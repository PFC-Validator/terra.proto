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
//! Generated file from `cosmos/base/store/v1beta1/snapshot.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotItem {
    // message oneof groups
    pub item: ::std::option::Option<SnapshotItem_oneof_item>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SnapshotItem {
    fn default() -> &'a SnapshotItem {
        <SnapshotItem as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum SnapshotItem_oneof_item {
    store(SnapshotStoreItem),
    iavl(SnapshotIAVLItem),
}

impl SnapshotItem {
    pub fn new() -> SnapshotItem {
        ::std::default::Default::default()
    }

    // .cosmos.base.store.v1beta1.SnapshotStoreItem store = 1;


    pub fn get_store(&self) -> &SnapshotStoreItem {
        match self.item {
            ::std::option::Option::Some(SnapshotItem_oneof_item::store(ref v)) => v,
            _ => <SnapshotStoreItem as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_store(&mut self) {
        self.item = ::std::option::Option::None;
    }

    pub fn has_store(&self) -> bool {
        match self.item {
            ::std::option::Option::Some(SnapshotItem_oneof_item::store(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_store(&mut self, v: SnapshotStoreItem) {
        self.item = ::std::option::Option::Some(SnapshotItem_oneof_item::store(v))
    }

    // Mutable pointer to the field.
    pub fn mut_store(&mut self) -> &mut SnapshotStoreItem {
        if let ::std::option::Option::Some(SnapshotItem_oneof_item::store(_)) = self.item {
        } else {
            self.item = ::std::option::Option::Some(SnapshotItem_oneof_item::store(SnapshotStoreItem::new()));
        }
        match self.item {
            ::std::option::Option::Some(SnapshotItem_oneof_item::store(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_store(&mut self) -> SnapshotStoreItem {
        if self.has_store() {
            match self.item.take() {
                ::std::option::Option::Some(SnapshotItem_oneof_item::store(v)) => v,
                _ => panic!(),
            }
        } else {
            SnapshotStoreItem::new()
        }
    }

    // .cosmos.base.store.v1beta1.SnapshotIAVLItem iavl = 2;


    pub fn get_iavl(&self) -> &SnapshotIAVLItem {
        match self.item {
            ::std::option::Option::Some(SnapshotItem_oneof_item::iavl(ref v)) => v,
            _ => <SnapshotIAVLItem as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_iavl(&mut self) {
        self.item = ::std::option::Option::None;
    }

    pub fn has_iavl(&self) -> bool {
        match self.item {
            ::std::option::Option::Some(SnapshotItem_oneof_item::iavl(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_iavl(&mut self, v: SnapshotIAVLItem) {
        self.item = ::std::option::Option::Some(SnapshotItem_oneof_item::iavl(v))
    }

    // Mutable pointer to the field.
    pub fn mut_iavl(&mut self) -> &mut SnapshotIAVLItem {
        if let ::std::option::Option::Some(SnapshotItem_oneof_item::iavl(_)) = self.item {
        } else {
            self.item = ::std::option::Option::Some(SnapshotItem_oneof_item::iavl(SnapshotIAVLItem::new()));
        }
        match self.item {
            ::std::option::Option::Some(SnapshotItem_oneof_item::iavl(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_iavl(&mut self) -> SnapshotIAVLItem {
        if self.has_iavl() {
            match self.item.take() {
                ::std::option::Option::Some(SnapshotItem_oneof_item::iavl(v)) => v,
                _ => panic!(),
            }
        } else {
            SnapshotIAVLItem::new()
        }
    }
}

impl ::protobuf::Message for SnapshotItem {
    fn is_initialized(&self) -> bool {
        if let Some(SnapshotItem_oneof_item::store(ref v)) = self.item {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(SnapshotItem_oneof_item::iavl(ref v)) = self.item {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.item = ::std::option::Option::Some(SnapshotItem_oneof_item::store(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.item = ::std::option::Option::Some(SnapshotItem_oneof_item::iavl(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.item {
            match v {
                &SnapshotItem_oneof_item::store(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &SnapshotItem_oneof_item::iavl(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.item {
            match v {
                &SnapshotItem_oneof_item::store(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &SnapshotItem_oneof_item::iavl(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

    fn new() -> SnapshotItem {
        SnapshotItem::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SnapshotStoreItem>(
                "store",
                SnapshotItem::has_store,
                SnapshotItem::get_store,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SnapshotIAVLItem>(
                "iavl",
                SnapshotItem::has_iavl,
                SnapshotItem::get_iavl,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SnapshotItem>(
                "SnapshotItem",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SnapshotItem {
        static instance: ::protobuf::rt::LazyV2<SnapshotItem> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SnapshotItem::new)
    }
}

impl ::protobuf::Clear for SnapshotItem {
    fn clear(&mut self) {
        self.item = ::std::option::Option::None;
        self.item = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotItem {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotStoreItem {
    // message fields
    pub name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SnapshotStoreItem {
    fn default() -> &'a SnapshotStoreItem {
        <SnapshotStoreItem as ::protobuf::Message>::default_instance()
    }
}

impl SnapshotStoreItem {
    pub fn new() -> SnapshotStoreItem {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }
}

impl ::protobuf::Message for SnapshotStoreItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

    fn new() -> SnapshotStoreItem {
        SnapshotStoreItem::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &SnapshotStoreItem| { &m.name },
                |m: &mut SnapshotStoreItem| { &mut m.name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SnapshotStoreItem>(
                "SnapshotStoreItem",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SnapshotStoreItem {
        static instance: ::protobuf::rt::LazyV2<SnapshotStoreItem> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SnapshotStoreItem::new)
    }
}

impl ::protobuf::Clear for SnapshotStoreItem {
    fn clear(&mut self) {
        self.name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotStoreItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotStoreItem {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotIAVLItem {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    pub version: i64,
    pub height: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SnapshotIAVLItem {
    fn default() -> &'a SnapshotIAVLItem {
        <SnapshotIAVLItem as ::protobuf::Message>::default_instance()
    }
}

impl SnapshotIAVLItem {
    pub fn new() -> SnapshotIAVLItem {
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

    // bytes value = 2;


    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    // int64 version = 3;


    pub fn get_version(&self) -> i64 {
        self.version
    }
    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i64) {
        self.version = v;
    }

    // int32 height = 4;


    pub fn get_height(&self) -> i32 {
        self.height
    }
    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i32) {
        self.height = v;
    }
}

impl ::protobuf::Message for SnapshotIAVLItem {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.version = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.height = tmp;
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
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(3, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(4, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
        }
        if self.version != 0 {
            os.write_int64(3, self.version)?;
        }
        if self.height != 0 {
            os.write_int32(4, self.height)?;
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

    fn new() -> SnapshotIAVLItem {
        SnapshotIAVLItem::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "key",
                |m: &SnapshotIAVLItem| { &m.key },
                |m: &mut SnapshotIAVLItem| { &mut m.key },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "value",
                |m: &SnapshotIAVLItem| { &m.value },
                |m: &mut SnapshotIAVLItem| { &mut m.value },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "version",
                |m: &SnapshotIAVLItem| { &m.version },
                |m: &mut SnapshotIAVLItem| { &mut m.version },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "height",
                |m: &SnapshotIAVLItem| { &m.height },
                |m: &mut SnapshotIAVLItem| { &mut m.height },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SnapshotIAVLItem>(
                "SnapshotIAVLItem",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SnapshotIAVLItem {
        static instance: ::protobuf::rt::LazyV2<SnapshotIAVLItem> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SnapshotIAVLItem::new)
    }
}

impl ::protobuf::Clear for SnapshotIAVLItem {
    fn clear(&mut self) {
        self.key.clear();
        self.value.clear();
        self.version = 0;
        self.height = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotIAVLItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotIAVLItem {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(cosmos/base/store/v1beta1/snapshot.proto\x12\x19cosmos.base.store.v1b\
    eta1\x1a\x14gogoproto/gogo.proto\"\xa9\x01\n\x0cSnapshotItem\x12D\n\x05s\
    tore\x18\x01\x20\x01(\x0b2,.cosmos.base.store.v1beta1.SnapshotStoreItemH\
    \0R\x05store\x12K\n\x04iavl\x18\x02\x20\x01(\x0b2+.cosmos.base.store.v1b\
    eta1.SnapshotIAVLItemH\0R\x04iavlB\x08\xe2\xde\x1f\x04IAVLB\x06\n\x04ite\
    m\"'\n\x11SnapshotStoreItem\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04nam\
    e\"l\n\x10SnapshotIAVLItem\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\
    \x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05value\x12\x18\n\x07version\
    \x18\x03\x20\x01(\x03R\x07version\x12\x16\n\x06height\x18\x04\x20\x01(\
    \x05R\x06heightB*Z(github.com/cosmos/cosmos-sdk/store/typesb\x06proto3\
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
