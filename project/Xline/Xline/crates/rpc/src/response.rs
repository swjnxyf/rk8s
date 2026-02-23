//! Response types for RPC communication

use prost::Message;

use crate::{MetaData, codec::{Codec, BinaryCodec, EncodeError, DecodeError}};

/// Generic RPC response wrapper
///
/// Separates metadata (headers, status) from the actual response data.
/// T should be a protobuf Message type, and usually as ResponseWrapper.
#[derive(Debug, Clone, PartialEq)]
pub struct Response<T: Message> {
    /// The actual response data
    data: T,
    /// Metadata (headers, status, etc.)
    meta: MetaData,
}

impl<T: Message> Response<T> {
    /// Create a new response with data and metadata
    #[must_use]
    #[inline]
    pub fn new(data: T, meta: MetaData) -> Self {
        Self { data, meta }
    }

    /// Create a new response with only data (empty metadata)
    #[must_use]
    #[inline]
    pub fn from_data(data: T) -> Self {
        Self {
            data,
            meta: MetaData::new(),
        }
    }

    /// Create a new response with data and status metadata
    #[must_use]
    #[inline]
    pub fn with_status<V: crate::IntoMetadataBytes>(data: T, status: V) -> Self {
        let mut meta = MetaData::new();
        meta.insert("status", status);
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

    /// Get the status if present
    #[must_use]
    #[inline]
    pub fn status(&self) -> Option<&str> {
        self.meta.get_str("status").and_then(Result::ok)
    }

    /// Decompose into data and metadata
    #[must_use]
    #[inline]
    pub fn into_parts(self) -> (T, MetaData) {
        (self.data, self.meta)
    }

    /// Encode the response to bytes using the default BinaryCodec
    ///
    /// # Errors
    /// Returns error if encoding fails
    #[inline]
    pub fn encode_to_vec(&self) -> Result<Vec<u8>, EncodeError> {
        self.encode_with(&BinaryCodec)
    }

    /// Encode the response to bytes using a custom codec
    ///
    /// # Errors
    /// Returns error if encoding fails
    pub fn encode_with<C: Codec>(&self, codec: &C) -> Result<Vec<u8>, EncodeError> {
        codec.encode(&self.data, &self.meta)
    }

    /// Decode a response from bytes using the default BinaryCodec
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

    /// Decode a response from bytes using a custom codec
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
        result: String,
        #[prost(int32, tag = "2")]
        code: i32,
    }

    #[test]
    fn test_response_from_data() {
        let msg = TestMessage {
            result: "success".to_string(),
            code: 200,
        };
        let resp = Response::from_data(msg.clone());

        assert_eq!(resp.data().result, "success");
        assert_eq!(resp.data().code, 200);
        assert!(resp.meta().is_empty());
        assert_eq!(resp.status(), None);
    }

    #[test]
    fn test_response_with_status() {
        let msg = TestMessage {
            result: "ok".to_string(),
            code: 200,
        };
        let resp = Response::with_status(msg, "OK");

        assert_eq!(resp.status(), Some("OK"));
    }

    #[test]
    fn test_response_encode_decode() {
        let msg = TestMessage {
            result: "hello".to_string(),
            code: 100,
        };

        let mut meta = MetaData::new();
        meta.insert("status", "OK");
        meta.insert("request-id", "req-789");

        let response = Response::new(msg, meta);

        // Encode
        let encoded = response.encode_to_vec().expect("encode failed");
        assert!(!encoded.is_empty());

        // Decode
        let decoded = Response::<TestMessage>::decode_from_slice(&encoded).expect("decode failed");

        assert_eq!(decoded.data().result, "hello");
        assert_eq!(decoded.data().code, 100);
        assert_eq!(decoded.status(), Some("OK"));
        assert_eq!(
            decoded.meta().get("request-id"),
            Some(b"req-789".as_slice())
        );
    }

    #[test]
    fn test_response_encode_decode_empty_meta() {
        let msg = TestMessage {
            result: "test".to_string(),
            code: 1,
        };
        let response = Response::from_data(msg);

        let encoded = response.encode_to_vec().unwrap();
        let decoded = Response::<TestMessage>::decode_from_slice(&encoded).unwrap();

        assert_eq!(decoded.data().result, "test");
        assert_eq!(decoded.data().code, 1);
        assert!(decoded.meta().is_empty());
    }

    #[test]
    fn test_response_encode_with_custom_codec() {
        let msg = TestMessage {
            result: "test".to_string(),
            code: 42,
        };
        let response = Response::from_data(msg);

        let codec = BinaryCodec::new();
        let encoded = response.encode_with(&codec).unwrap();
        let decoded = Response::<TestMessage>::decode_with(&encoded, &codec).unwrap();

        assert_eq!(decoded.data().result, "test");
        assert_eq!(decoded.data().code, 42);
    }

    #[test]
    fn test_response_into_parts() {
        let msg = TestMessage {
            result: "test".to_string(),
            code: 42,
        };
        let meta = MetaData::with_entry("status", "OK");
        let response = Response::new(msg, meta);

        let (data, meta_out) = response.into_parts();
        assert_eq!(data.result, "test");
        assert_eq!(data.code, 42);
        assert_eq!(meta_out.get("status"), Some(b"OK".as_slice()));
    }
}
