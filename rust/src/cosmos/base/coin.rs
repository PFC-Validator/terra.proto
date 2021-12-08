// This file is generated by rust-protobuf 2.25.1. Do not edit
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
//! Generated file from `cosmos/base/v1beta1/coin.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct Coin {
    // message fields
    pub denom: ::std::string::String,
    pub amount: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Coin {
    fn default() -> &'a Coin {
        <Coin as ::protobuf::Message>::default_instance()
    }
}

impl Coin {
    pub fn new() -> Coin {
        ::std::default::Default::default()
    }

    // string denom = 1;


    pub fn get_denom(&self) -> &str {
        &self.denom
    }
    pub fn clear_denom(&mut self) {
        self.denom.clear();
    }

    // Param is passed by value, moved
    pub fn set_denom(&mut self, v: ::std::string::String) {
        self.denom = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_denom(&mut self) -> &mut ::std::string::String {
        &mut self.denom
    }

    // Take field
    pub fn take_denom(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.denom, ::std::string::String::new())
    }

    // string amount = 2;


    pub fn get_amount(&self) -> &str {
        &self.amount
    }
    pub fn clear_amount(&mut self) {
        self.amount.clear();
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: ::std::string::String) {
        self.amount = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_amount(&mut self) -> &mut ::std::string::String {
        &mut self.amount
    }

    // Take field
    pub fn take_amount(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.amount, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Coin {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.denom)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.amount)?;
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
        if !self.denom.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.denom);
        }
        if !self.amount.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.amount);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.denom.is_empty() {
            os.write_string(1, &self.denom)?;
        }
        if !self.amount.is_empty() {
            os.write_string(2, &self.amount)?;
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

    fn new() -> Coin {
        Coin::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "denom",
                |m: &Coin| { &m.denom },
                |m: &mut Coin| { &mut m.denom },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "amount",
                |m: &Coin| { &m.amount },
                |m: &mut Coin| { &mut m.amount },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Coin>(
                "Coin",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Coin {
        static instance: ::protobuf::rt::LazyV2<Coin> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Coin::new)
    }
}

impl ::protobuf::Clear for Coin {
    fn clear(&mut self) {
        self.denom.clear();
        self.amount.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Coin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Coin {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DecCoin {
    // message fields
    pub denom: ::std::string::String,
    pub amount: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DecCoin {
    fn default() -> &'a DecCoin {
        <DecCoin as ::protobuf::Message>::default_instance()
    }
}

impl DecCoin {
    pub fn new() -> DecCoin {
        ::std::default::Default::default()
    }

    // string denom = 1;


    pub fn get_denom(&self) -> &str {
        &self.denom
    }
    pub fn clear_denom(&mut self) {
        self.denom.clear();
    }

    // Param is passed by value, moved
    pub fn set_denom(&mut self, v: ::std::string::String) {
        self.denom = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_denom(&mut self) -> &mut ::std::string::String {
        &mut self.denom
    }

    // Take field
    pub fn take_denom(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.denom, ::std::string::String::new())
    }

    // string amount = 2;


    pub fn get_amount(&self) -> &str {
        &self.amount
    }
    pub fn clear_amount(&mut self) {
        self.amount.clear();
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: ::std::string::String) {
        self.amount = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_amount(&mut self) -> &mut ::std::string::String {
        &mut self.amount
    }

    // Take field
    pub fn take_amount(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.amount, ::std::string::String::new())
    }
}

impl ::protobuf::Message for DecCoin {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.denom)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.amount)?;
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
        if !self.denom.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.denom);
        }
        if !self.amount.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.amount);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.denom.is_empty() {
            os.write_string(1, &self.denom)?;
        }
        if !self.amount.is_empty() {
            os.write_string(2, &self.amount)?;
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

    fn new() -> DecCoin {
        DecCoin::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "denom",
                |m: &DecCoin| { &m.denom },
                |m: &mut DecCoin| { &mut m.denom },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "amount",
                |m: &DecCoin| { &m.amount },
                |m: &mut DecCoin| { &mut m.amount },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DecCoin>(
                "DecCoin",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DecCoin {
        static instance: ::protobuf::rt::LazyV2<DecCoin> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DecCoin::new)
    }
}

impl ::protobuf::Clear for DecCoin {
    fn clear(&mut self) {
        self.denom.clear();
        self.amount.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DecCoin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DecCoin {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IntProto {
    // message fields
    pub int: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a IntProto {
    fn default() -> &'a IntProto {
        <IntProto as ::protobuf::Message>::default_instance()
    }
}

impl IntProto {
    pub fn new() -> IntProto {
        ::std::default::Default::default()
    }

    // string int = 1;


    pub fn get_int(&self) -> &str {
        &self.int
    }
    pub fn clear_int(&mut self) {
        self.int.clear();
    }

    // Param is passed by value, moved
    pub fn set_int(&mut self, v: ::std::string::String) {
        self.int = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_int(&mut self) -> &mut ::std::string::String {
        &mut self.int
    }

    // Take field
    pub fn take_int(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.int, ::std::string::String::new())
    }
}

impl ::protobuf::Message for IntProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.int)?;
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
        if !self.int.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.int);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.int.is_empty() {
            os.write_string(1, &self.int)?;
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

    fn new() -> IntProto {
        IntProto::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "int",
                |m: &IntProto| { &m.int },
                |m: &mut IntProto| { &mut m.int },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<IntProto>(
                "IntProto",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static IntProto {
        static instance: ::protobuf::rt::LazyV2<IntProto> = ::protobuf::rt::LazyV2::INIT;
        instance.get(IntProto::new)
    }
}

impl ::protobuf::Clear for IntProto {
    fn clear(&mut self) {
        self.int.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IntProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IntProto {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DecProto {
    // message fields
    pub dec: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DecProto {
    fn default() -> &'a DecProto {
        <DecProto as ::protobuf::Message>::default_instance()
    }
}

impl DecProto {
    pub fn new() -> DecProto {
        ::std::default::Default::default()
    }

    // string dec = 1;


    pub fn get_dec(&self) -> &str {
        &self.dec
    }
    pub fn clear_dec(&mut self) {
        self.dec.clear();
    }

    // Param is passed by value, moved
    pub fn set_dec(&mut self, v: ::std::string::String) {
        self.dec = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dec(&mut self) -> &mut ::std::string::String {
        &mut self.dec
    }

    // Take field
    pub fn take_dec(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.dec, ::std::string::String::new())
    }
}

impl ::protobuf::Message for DecProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.dec)?;
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
        if !self.dec.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.dec);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.dec.is_empty() {
            os.write_string(1, &self.dec)?;
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

    fn new() -> DecProto {
        DecProto::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "dec",
                |m: &DecProto| { &m.dec },
                |m: &mut DecProto| { &mut m.dec },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DecProto>(
                "DecProto",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DecProto {
        static instance: ::protobuf::rt::LazyV2<DecProto> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DecProto::new)
    }
}

impl ::protobuf::Clear for DecProto {
    fn clear(&mut self) {
        self.dec.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DecProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DecProto {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1ecosmos/base/v1beta1/coin.proto\x12\x13cosmos.base.v1beta1\x1a\x14g\
    ogoproto/gogo.proto\x1a\x19cosmos_proto/cosmos.proto\"U\n\x04Coin\x12\
    \x14\n\x05denom\x18\x01\x20\x01(\tR\x05denom\x121\n\x06amount\x18\x02\
    \x20\x01(\tR\x06amountB\x19\xd2\xb4-\ncosmos.Int\xda\xde\x1f\x03Int\xc8\
    \xde\x1f\0:\x04\xe8\xa0\x1f\x01\"X\n\x07DecCoin\x12\x14\n\x05denom\x18\
    \x01\x20\x01(\tR\x05denom\x121\n\x06amount\x18\x02\x20\x01(\tR\x06amount\
    B\x19\xd2\xb4-\ncosmos.Dec\xda\xde\x1f\x03Dec\xc8\xde\x1f\0:\x04\xe8\xa0\
    \x1f\x01\"7\n\x08IntProto\x12+\n\x03int\x18\x01\x20\x01(\tR\x03intB\x19\
    \xd2\xb4-\ncosmos.Int\xda\xde\x1f\x03Int\xc8\xde\x1f\0\"7\n\x08DecProto\
    \x12+\n\x03dec\x18\x01\x20\x01(\tR\x03decB\x19\xd2\xb4-\ncosmos.Dec\xda\
    \xde\x1f\x03Dec\xc8\xde\x1f\0B,Z\"github.com/cosmos/cosmos-sdk/types\xd8\
    \xe1\x1e\0\x80\xe2\x1e\0b\x06proto3\
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
