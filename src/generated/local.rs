// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Recipe {
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub base_session_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub changes: ::prost::alloc::vec::Vec<Change>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Change {
    #[prost(enumeration = "change::ChangeType", tag = "1")]
    pub change_type: i32,
    #[prost(string, tag = "2")]
    pub file_path: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub blob_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Change`.
pub mod change {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ChangeType {
        Add = 0,
        Modify = 1,
        Delete = 2,
    }
    impl ChangeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ChangeType::Add => "ADD",
                ChangeType::Modify => "MODIFY",
                ChangeType::Delete => "DELETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADD" => Some(Self::Add),
                "MODIFY" => Some(Self::Modify),
                "DELETE" => Some(Self::Delete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blob {
    #[prost(string, tag = "1")]
    pub blob_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub file_path: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub size: u64,
    #[prost(string, tag = "4")]
    pub checksum: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobList {
    #[prost(message, repeated, tag = "1")]
    pub blobs: ::prost::alloc::vec::Vec<Blob>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
