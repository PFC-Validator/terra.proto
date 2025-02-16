// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `cosmos/base/query/v1beta1/pagination.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct PageRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub offset: u64,
    pub limit: u64,
    pub count_total: bool,
    pub reverse: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PageRequest {
    fn default() -> &'a PageRequest {
        <PageRequest as ::protobuf::Message>::default_instance()
    }
}

impl PageRequest {
    pub fn new() -> PageRequest {
        ::std::default::Default::default()
    }

    // bytes key = 1;


    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    // uint64 offset = 2;


    pub fn get_offset(&self) -> u64 {
        self.offset
    }
    pub fn clear_offset(&mut self) {
        self.offset = 0;
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: u64) {
        self.offset = v;
    }

    // uint64 limit = 3;


    pub fn get_limit(&self) -> u64 {
        self.limit
    }
    pub fn clear_limit(&mut self) {
        self.limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: u64) {
        self.limit = v;
    }

    // bool count_total = 4;


    pub fn get_count_total(&self) -> bool {
        self.count_total
    }
    pub fn clear_count_total(&mut self) {
        self.count_total = false;
    }

    // Param is passed by value, moved
    pub fn set_count_total(&mut self, v: bool) {
        self.count_total = v;
    }

    // bool reverse = 5;


    pub fn get_reverse(&self) -> bool {
        self.reverse
    }
    pub fn clear_reverse(&mut self) {
        self.reverse = false;
    }

    // Param is passed by value, moved
    pub fn set_reverse(&mut self, v: bool) {
        self.reverse = v;
    }
}

impl ::protobuf::Message for PageRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.offset = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.limit = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.count_total = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.reverse = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if self.offset != 0 {
            my_size += ::protobuf::rt::value_size(2, self.offset, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.limit != 0 {
            my_size += ::protobuf::rt::value_size(3, self.limit, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.count_total != false {
            my_size += 2;
        }
        if self.reverse != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if self.offset != 0 {
            os.write_uint64(2, self.offset)?;
        }
        if self.limit != 0 {
            os.write_uint64(3, self.limit)?;
        }
        if self.count_total != false {
            os.write_bool(4, self.count_total)?;
        }
        if self.reverse != false {
            os.write_bool(5, self.reverse)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PageRequest {
        PageRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "key",
                |m: &PageRequest| { &m.key },
                |m: &mut PageRequest| { &mut m.key },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "offset",
                |m: &PageRequest| { &m.offset },
                |m: &mut PageRequest| { &mut m.offset },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "limit",
                |m: &PageRequest| { &m.limit },
                |m: &mut PageRequest| { &mut m.limit },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "count_total",
                |m: &PageRequest| { &m.count_total },
                |m: &mut PageRequest| { &mut m.count_total },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "reverse",
                |m: &PageRequest| { &m.reverse },
                |m: &mut PageRequest| { &mut m.reverse },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PageRequest>(
                "PageRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PageRequest {
        static instance: ::protobuf::rt::LazyV2<PageRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PageRequest::new)
    }
}

impl ::protobuf::Clear for PageRequest {
    fn clear(&mut self) {
        self.key.clear();
        self.offset = 0;
        self.limit = 0;
        self.count_total = false;
        self.reverse = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PageRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PageRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PageResponse {
    // message fields
    pub next_key: ::std::vec::Vec<u8>,
    pub total: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PageResponse {
    fn default() -> &'a PageResponse {
        <PageResponse as ::protobuf::Message>::default_instance()
    }
}

impl PageResponse {
    pub fn new() -> PageResponse {
        ::std::default::Default::default()
    }

    // bytes next_key = 1;


    pub fn get_next_key(&self) -> &[u8] {
        &self.next_key
    }
    pub fn clear_next_key(&mut self) {
        self.next_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_next_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.next_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_next_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.next_key
    }

    // Take field
    pub fn take_next_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.next_key, ::std::vec::Vec::new())
    }

    // uint64 total = 2;


    pub fn get_total(&self) -> u64 {
        self.total
    }
    pub fn clear_total(&mut self) {
        self.total = 0;
    }

    // Param is passed by value, moved
    pub fn set_total(&mut self, v: u64) {
        self.total = v;
    }
}

impl ::protobuf::Message for PageResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.next_key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.next_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.next_key);
        }
        if self.total != 0 {
            my_size += ::protobuf::rt::value_size(2, self.total, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.next_key.is_empty() {
            os.write_bytes(1, &self.next_key)?;
        }
        if self.total != 0 {
            os.write_uint64(2, self.total)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PageResponse {
        PageResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "next_key",
                |m: &PageResponse| { &m.next_key },
                |m: &mut PageResponse| { &mut m.next_key },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "total",
                |m: &PageResponse| { &m.total },
                |m: &mut PageResponse| { &mut m.total },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PageResponse>(
                "PageResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PageResponse {
        static instance: ::protobuf::rt::LazyV2<PageResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PageResponse::new)
    }
}

impl ::protobuf::Clear for PageResponse {
    fn clear(&mut self) {
        self.next_key.clear();
        self.total = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PageResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PageResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*cosmos/base/query/v1beta1/pagination.proto\x12\x19cosmos.base.query.v\
    1beta1\"\x88\x01\n\x0bPageRequest\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\
    \x03key\x12\x16\n\x06offset\x18\x02\x20\x01(\x04R\x06offset\x12\x14\n\
    \x05limit\x18\x03\x20\x01(\x04R\x05limit\x12\x1f\n\x0bcount_total\x18\
    \x04\x20\x01(\x08R\ncountTotal\x12\x18\n\x07reverse\x18\x05\x20\x01(\x08\
    R\x07reverse\"?\n\x0cPageResponse\x12\x19\n\x08next_key\x18\x01\x20\x01(\
    \x0cR\x07nextKey\x12\x14\n\x05total\x18\x02\x20\x01(\x04R\x05totalB*Z(gi\
    thub.com/cosmos/cosmos-sdk/types/queryb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
