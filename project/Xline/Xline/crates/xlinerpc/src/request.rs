//! Request types for RPC communication

use prost::Message;

use crate::{
    MetaData,
    codec::{BinaryCodec, Codec, DecodeError, EncodeError},
};

/// Generic RPC request wrapper
///
/// Separates metadata (headers, auth tokens) from the actual request data.
/// T should be a protobuf Message type, and usually as RequestWrapper.
#[derive(Debug, Clone, PartialEq)]
pub struct Request<T: Message> {
    /// The actual request data
    data: T,
    /// Metadata (headers, tokens, etc.)
    meta: MetaData,
}

impl<T: Message> Request<T> {
    /// Create a new request with data and metadata
    #[must_use]
    #[inline]
    pub fn new(data: T, meta: MetaData) -> Self {
        Self { data, meta }
    }

    /// Create a new request with only data (empty metadata)
    #[must_use]
    #[inline]
    pub fn from_data(data: T) -> Self {
        Self {
            data,
            meta: MetaData::new(),
        }
    }

    /// Create a new request with data and a token
    #[must_use]
    #[inline]
    pub fn with_token<V: crate::IntoMetadataBytes>(data: T, token: V) -> Self {
        let mut meta = MetaData::new();
        meta.set_token(token);
        Self { data, meta }
    }

    /// Get a reference to the data
    #[must_use]
    #[inline]
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Get a mutable reference to the data
    #[inline]
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Get a reference to the metadata
    #[must_use]
    #[inline]
    pub fn meta(&self) -> &MetaData {
        &self.meta
    }

    /// Get a mutable reference to the metadata
    #[inline]
    pub fn meta_mut(&mut self) -> &mut MetaData {
        &mut self.meta
    }

    /// Get the authentication token if present
    #[must_use]
    #[inline]
    pub fn token(&self) -> Option<&str> {
        self.meta.token()
    }

    /// Decompose into data and metadata
    #[must_use]
    #[inline]
    pub fn into_parts(self) -> (T, MetaData) {
        (self.data, self.meta)
    }

    /// Encode the request to bytes using the default BinaryCodec
    ///
    /// # Errors
    /// Returns error if encoding fails
    #[inline]
    pub fn encode_to_vec(&self) -> Result<Vec<u8>, EncodeError> {
        self.encode_with(&BinaryCodec)
    }

    /// Encode the request to bytes using a custom codec
    ///
    /// # Errors
    /// Returns error if encoding fails
    pub fn encode_with<C: Codec>(&self, codec: &C) -> Result<Vec<u8>, EncodeError> {
        codec.encode(&self.data, &self.meta)
    }

    /// Decode a request from bytes using the default BinaryCodec
    ///
    /// # Errors
    /// Returns error if decoding fails
    #[inline]
    pub fn decode_from_slice(bytes: &[u8]) -> Result<Self, DecodeError>
    where
        T: Default,
    {
        Self::decode_with(bytes, &BinaryCodec)
    }

    /// Decode a request from bytes using a custom codec
    ///
    /// # Errors
    /// Returns error if decoding fails
    pub fn decode_with<C: Codec>(bytes: &[u8], codec: &C) -> Result<Self, DecodeError>
    where
        T: Default,
    {
        let (data, meta) = codec.decode(bytes)?;
        Ok(Self { data, meta })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Message)]
    struct TestMessage {
        #[prost(string, tag = "1")]
        name: String,
        #[prost(int32, tag = "2")]
        value: i32,
    }

    #[test]
    fn test_request_from_data() {
        let msg = TestMessage {
            name: "test".to_string(),
            value: 42,
        };
        let req = Request::from_data(msg.clone());

        assert_eq!(req.data().name, "test");
        assert_eq!(req.data().value, 42);
        assert!(req.meta().is_empty());
        assert_eq!(req.token(), None);
    }

    #[test]
    fn test_request_with_token() {
        let msg = TestMessage {
            name: "test".to_string(),
            value: 123,
        };
        let req = Request::with_token(msg, "my-token");

        assert_eq!(req.token(), Some("my-token"));
    }

    #[test]
    fn test_request_encode_decode() {
        let msg = TestMessage {
            name: "hello".to_string(),
            value: 999,
        };

        let mut meta = MetaData::new();
        meta.set_token("auth-123");
        meta.insert("trace-id", "trace-456");

        let request = Request::new(msg, meta);

        // Encode
        let encoded = request.encode_to_vec().expect("encode failed");
        assert!(!encoded.is_empty());

        // Decode
        let decoded = Request::<TestMessage>::decode_from_slice(&encoded).expect("decode failed");

        assert_eq!(decoded.data().name, "hello");
        assert_eq!(decoded.data().value, 999);
        assert_eq!(decoded.token(), Some("auth-123"));
        assert_eq!(
            decoded.meta().get("trace-id"),
            Some(b"trace-456".as_slice())
        );
    }

    #[test]
    fn test_request_encode_decode_empty_meta() {
        let msg = TestMessage {
            name: "test".to_string(),
            value: 1,
        };
        let request = Request::from_data(msg);

        let encoded = request.encode_to_vec().unwrap();
        let decoded = Request::<TestMessage>::decode_from_slice(&encoded).unwrap();

        assert_eq!(decoded.data().name, "test");
        assert_eq!(decoded.data().value, 1);
        assert!(decoded.meta().is_empty());
    }

    #[test]
    fn test_request_encode_with_custom_codec() {
        let msg = TestMessage {
            name: "test".to_string(),
            value: 42,
        };
        let request = Request::from_data(msg);

        let codec = BinaryCodec::new();
        let encoded = request.encode_with(&codec).unwrap();
        let decoded = Request::<TestMessage>::decode_with(&encoded, &codec).unwrap();

        assert_eq!(decoded.data().name, "test");
        assert_eq!(decoded.data().value, 42);
    }

    #[test]
    fn test_request_into_parts() {
        let msg = TestMessage {
            name: "test".to_string(),
            value: 42,
        };
        let meta = MetaData::with_entry("key", "value");
        let request = Request::new(msg, meta);

        let (data, meta_out) = request.into_parts();
        assert_eq!(data.name, "test");
        assert_eq!(data.value, 42);
        assert_eq!(meta_out.get("key"), Some(b"value".as_slice()));
    }
}
