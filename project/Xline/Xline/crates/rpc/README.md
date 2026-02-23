# RPC 模块

## 概述

`rpc` 模块为 Xline 提供自定义的 RPC 传输层抽象，用于实现：

1. **路由分发** - 通过 `Request<T>` 和 `Response<T>` 泛型包装器分离数据和元数据
2. **自定义编解码** - 基于二进制格式的高效编解码协议，支持从 gRPC/tonic 迁移到 QUIC/GM-QUIC

## 核心组件

### 数据结构

- **`MetaData`** - 元数据容器，存储键值对（如认证令牌、追踪 ID 等）
  - 内部使用 `HashMap<Vec<u8>, Vec<u8>>` 支持二进制数据
  - 提供便捷方法如 `token()`、`set_token()` 等
  - 限制最多 255 个条目，每个键/值最大 64KB

- **`Request<T>`** - 请求包装器
  - `data: T` - Protobuf 消息数据（私有字段）
  - `meta: MetaData` - 请求元数据（私有字段）
  - 通过访问器方法 `data()`、`meta()` 等访问

- **`Response<T>`** - 响应包装器
  - 结构与 `Request<T>` 类似
  - 提供 `status()` 方法获取响应状态

### 编解码器

- **`Codec` trait** - 编解码器接口
  - `encode<T: Message>(&self, data: &T, meta: &MetaData) -> Result<Vec<u8>, EncodeError>`
  - `decode<T: Message + Default>(&self, bytes: &[u8]) -> Result<(T, MetaData), DecodeError>`

- **`BinaryCodec`** - 默认二进制编解码器实现

#### 二进制格式规范

```
整体格式:
[meta_len: u32 (big-endian)][metadata][protobuf_data]

Metadata 格式:
[count: u8][key_len: u16][key_bytes][value_len: u16][value_bytes]...
```

特点：
- 使用长度前缀，支持包含 `0x00` 的二进制数据
- 元数据限制：最多 255 个条目（u8::MAX）
- 键/值限制：每个最大 65535 字节（u16::MAX）

## 使用示例

### 创建请求

```rust
use rpc::{Request, MetaData};
use prost::Message;

// 创建带有 token 的请求
let request = Request::with_token(my_proto_message, "auth-token-123");

// 创建带有自定义元数据的请求
let mut meta = MetaData::new();
meta.insert("trace-id", "trace-456");
meta.insert("client-id", "client-789");
let request = Request::new(my_proto_message, meta);
```

### 编码和解码

```rust
use rpc::{Request, BinaryCodec, Codec};

// 编码
let codec = BinaryCodec::new();
let bytes = request.encode_to_vec()?;

// 解码
let decoded: Request<MyMessage> = Request::decode_from_slice(&bytes)?;

// 使用自定义编解码器
let bytes = request.encode_with(&codec)?;
let decoded = Request::decode_with(&bytes, &codec)?;
```

### 元数据操作

```rust
use rpc::MetaData;

let mut meta = MetaData::new();

// 插入字符串
meta.insert("key", "value");

// 插入二进制数据
meta.insert(vec![0x01, 0x02], vec![0xff, 0xfe]);

// 读取数据
if let Some(value) = meta.get("key") {
    // value 是 &[u8]
}

// 读取为 UTF-8 字符串
if let Some(Ok(value)) = meta.get_str("key") {
    // value 是 &str
}

// 获取认证令牌
if let Some(token) = meta.token() {
    println!("Token: {}", token);
}
```

## 设计原则

### 为什么不合并 Request 和 Response？

虽然 `Request<T>` 和 `Response<T>` 结构相似，但它们具有不同的语义：
- Request 对应 Xline 的 `RequestWrapper` 枚举
- Response 对应 Xline 的 `ResponseWrapper` 枚举
- 类型分离使编译器能够在类型层面区分请求和响应

### 错误处理策略

- **编码限制使用 `assert!`** - 超过 255 个条目或 64KB 键/值会 panic
  - 原因：这些是协议级别的硬性限制，违反表示程序逻辑错误
  - 用户应在业务层确保不超过限制

## 与 xlineapi 的关系

- **`xlineapi`** - 定义 Protobuf 消息类型和 gRPC 服务接口
- **`rpc`** - 提供传输层抽象和编解码，用于从 gRPC 迁移到 QUIC

两者配合使用：`xlineapi` 定义 **what**（消息格式），`rpc` 定义 **how**（传输方式）。

## 测试

运行所有测试：

```bash
cargo test -p rpc
```

测试覆盖：
- 基本编解码（空元数据、正常元数据、大量元数据）
- 边界条件（255 个条目、64KB 键/值）
- 错误情况（截断数据、损坏的 protobuf、无效格式）
- 二进制数据（包含 null 字节、非 UTF-8 数据）
- Unicode 支持（中文、emoji）
