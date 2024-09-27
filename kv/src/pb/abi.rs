/// message定义类型，编译后一般转换为对应语言的Struct或某种Class类型
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof = "request::Command", tags = "1, 2")]
    pub command: ::core::option::Option<request::Command>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag = "1")]
        Get(super::RequestGet),
        #[prost(message, tag = "2")]
        Put(super::RequestPut),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestGet {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPut {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
