use http::Request;
use http_body::Body;
use prost::Message;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio_stream::Stream;
use tonic::{Status, body::BoxBody, codec::EnabledCompressionEncodings, codegen::BoxFuture};
use tower::Service;
use xlinerpc::{
    Request as XlineRequest, Response as XlineResponse, Status as XlineStatus,
    Streaming as XlineStreaming,
};

fn h2_unimplemented_response() -> http::Response<BoxBody> {
    let mut response = http::Response::new(tonic::body::empty_body());
    let headers = response.headers_mut();
    let _prev = headers.insert(Status::GRPC_STATUS, http::HeaderValue::from_static("12"));
    let _prev = headers.insert(
        http::header::CONTENT_TYPE,
        tonic::metadata::GRPC_CONTENT_TYPE,
    );
    response
}

#[derive(Clone)]
pub(crate) struct WithEncodingOption<T> {
    svc: Arc<T>,
    accept_compression_encodings: EnabledCompressionEncodings,
    send_compression_encodings: EnabledCompressionEncodings,
    max_decoding_message_size: Option<usize>,
    max_encoding_message_size: Option<usize>,
}

impl<T> WithEncodingOption<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self::from_arc(Arc::new(inner))
    }

    pub(crate) fn from_arc(inner: Arc<T>) -> Self {
        Self {
            svc: inner,
            accept_compression_encodings: Default::default(),
            send_compression_encodings: Default::default(),
            max_decoding_message_size: None,
            max_encoding_message_size: None,
        }
    }

    // /// Enable decompressing requests with the given encoding.
    // pub(crate) fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
    //     self.accept_compression_encodings.enable(encoding);
    //     self
    // }
    // /// Compress responses with the given encoding, if the client supports it.
    // pub(crate) fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
    //     self.send_compression_encodings.enable(encoding);
    //     self
    // }
    // /// Limits the maximum size of a decoded message.
    // ///
    // /// Default: `4MB`
    // pub(crate) fn max_decoding_message_size(mut self, limit: usize) -> Self {
    //     self.max_decoding_message_size = Some(limit);
    //     self
    // }
    // /// Limits the maximum size of an encoded message.
    // ///
    // /// Default: `usize::MAX`
    // pub(crate) fn max_encoding_message_size(mut self, limit: usize) -> Self {
    //     self.max_encoding_message_size = Some(limit);
    //     self
    // }
}

#[derive(Clone)]
pub(crate) struct MakeUnarySVC<SVC, Input, Output> {
    inner: SVC,
    _1: std::marker::PhantomData<Input>,
    _2: std::marker::PhantomData<Output>,
}

impl<SVC, Input, Output> MakeUnarySVC<SVC, Input, Output>
where
    SVC: Clone,
{
    pub(crate) fn new(service: SVC) -> Self {
        MakeUnarySVC {
            inner: service,
            _1: std::marker::PhantomData,
            _2: std::marker::PhantomData,
        }
    }
}

impl<B, SVC, Input, Output> Service<Request<B>>
    for WithEncodingOption<MakeUnarySVC<SVC, Input, Output>>
where
    Input: Message + Default + Send + 'static,
    Output: Message + Default + Send + 'static + Clone,
    SVC: Service<XlineRequest<Input>, Response = XlineResponse<Output>, Error = XlineStatus>
        + Clone
        + 'static
        + Send
        + Sync,
    SVC::Future: Send,
    B: Body + Send + 'static,
    B::Error: Into<super::Error> + Send + 'static,
{
    type Response = http::Response<BoxBody>;
    type Error = std::convert::Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let _ = (
            &self.svc.inner,
            self.accept_compression_encodings,
            self.send_compression_encodings,
            self.max_decoding_message_size,
            self.max_encoding_message_size,
        );
        let _ = request;
        let fut = async move { Ok(h2_unimplemented_response()) };
        Box::pin(fut)
    }
}

#[derive(Clone)]
pub(crate) struct MakeStreamingSvc<SVC, Input, Output> {
    inner: SVC,
    _1: std::marker::PhantomData<Input>,
    _2: std::marker::PhantomData<Output>,
}

impl<SVC, Input, Output> MakeStreamingSvc<SVC, Input, Output>
where
    SVC: Clone,
{
    pub(crate) fn new(service: SVC) -> Self {
        MakeStreamingSvc {
            inner: service,
            _1: std::marker::PhantomData,
            _2: std::marker::PhantomData,
        }
    }
}

impl<B, SVC, Input, Output, RspStream> Service<Request<B>>
    for WithEncodingOption<MakeStreamingSvc<SVC, Input, Output>>
where
    Input: Message + Default + Send + 'static,
    Output: Message + Default + Send + 'static + Clone,
    RspStream: Stream<Item = Result<Output, XlineStatus>> + Send + 'static,
    SVC: Service<
            XlineRequest<XlineStreaming<Input>>,
            Response = XlineResponse<
                // RspStream<Output>
                RspStream,
            >,
            Error = XlineStatus,
        >
        + Clone
        + 'static
        + Send
        + Sync,
    SVC::Future: Send,
    B: Body + Send + 'static,
    B::Error: Into<super::Error> + Send + 'static,
{
    type Response = http::Response<BoxBody>;
    type Error = std::convert::Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let _ = (
            &self.svc.inner,
            self.accept_compression_encodings,
            self.send_compression_encodings,
            self.max_decoding_message_size,
            self.max_encoding_message_size,
        );
        let _ = request;
        let fut = async move { Ok(h2_unimplemented_response()) };
        Box::pin(fut)
    }
}

#[derive(Clone)]
pub(crate) struct MakeServerStreamingSvc<SVC, Input, Output> {
    inner: SVC,
    _1: std::marker::PhantomData<Input>,
    _2: std::marker::PhantomData<Output>,
}

impl<SVC, Input, Output> MakeServerStreamingSvc<SVC, Input, Output>
where
    SVC: Clone,
{
    pub(crate) fn new(service: SVC) -> Self {
        MakeServerStreamingSvc {
            inner: service,
            _1: std::marker::PhantomData,
            _2: std::marker::PhantomData,
        }
    }
}

impl<B, SVC, Input, Output, RspStream> Service<Request<B>>
    for WithEncodingOption<MakeServerStreamingSvc<SVC, Input, Output>>
where
    Input: Message + Default + Send + 'static,
    Output: Message + Default + Send + 'static + Clone,
    RspStream: Stream<Item = Result<Output, XlineStatus>> + Send + 'static,
    SVC: Service<XlineRequest<Input>, Response = XlineResponse<RspStream>, Error = XlineStatus>
        + Clone
        + 'static
        + Send
        + Sync,
    SVC::Future: Send,
    B: Body + Send + 'static,
    B::Error: Into<super::Error> + Send + 'static,
{
    type Response = http::Response<BoxBody>;
    type Error = std::convert::Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let _ = (
            &self.svc.inner,
            self.accept_compression_encodings,
            self.send_compression_encodings,
            self.max_decoding_message_size,
            self.max_encoding_message_size,
        );
        let _ = request;
        let fut = async move { Ok(h2_unimplemented_response()) };
        Box::pin(fut)
    }
}

#[derive(Clone)]
pub(crate) struct MakeClientStreamingSvc<SVC, Input, Output> {
    inner: SVC,
    _1: std::marker::PhantomData<Input>,
    _2: std::marker::PhantomData<Output>,
}

impl<SVC, Input, Output> MakeClientStreamingSvc<SVC, Input, Output>
where
    SVC: Clone,
{
    pub(crate) fn new(service: SVC) -> Self {
        MakeClientStreamingSvc {
            inner: service,
            _1: std::marker::PhantomData,
            _2: std::marker::PhantomData,
        }
    }
}

impl<B, SVC, Input, Output> Service<Request<B>>
    for WithEncodingOption<MakeClientStreamingSvc<SVC, Input, Output>>
where
    Input: Message + Default + Send + 'static,
    Output: Message + Default + Send + 'static + Clone,
    SVC: Service<
            XlineRequest<XlineStreaming<Input>>,
            Response = XlineResponse<Output>,
            Error = XlineStatus,
        >
        + Clone
        + 'static
        + Send
        + Sync,
    SVC::Future: Send,
    B: Body + Send + 'static,
    B::Error: Into<super::Error> + Send + 'static,
{
    type Response = http::Response<BoxBody>;
    type Error = std::convert::Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let _ = (
            &self.svc.inner,
            self.accept_compression_encodings,
            self.send_compression_encodings,
            self.max_decoding_message_size,
            self.max_encoding_message_size,
        );
        let _ = request;
        let fut = async move { Ok(h2_unimplemented_response()) };
        Box::pin(fut)
    }
}
