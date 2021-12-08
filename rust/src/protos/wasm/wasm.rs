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
//! Generated file from `terra/wasm/v1beta1/wasm.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct Params {
    // message fields
    pub max_contract_size: u64,
    pub max_contract_gas: u64,
    pub max_contract_msg_size: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Params {
    fn default() -> &'a Params {
        <Params as ::protobuf::Message>::default_instance()
    }
}

impl Params {
    pub fn new() -> Params {
        ::std::default::Default::default()
    }

    // uint64 max_contract_size = 1;


    pub fn get_max_contract_size(&self) -> u64 {
        self.max_contract_size
    }
    pub fn clear_max_contract_size(&mut self) {
        self.max_contract_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_contract_size(&mut self, v: u64) {
        self.max_contract_size = v;
    }

    // uint64 max_contract_gas = 2;


    pub fn get_max_contract_gas(&self) -> u64 {
        self.max_contract_gas
    }
    pub fn clear_max_contract_gas(&mut self) {
        self.max_contract_gas = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_contract_gas(&mut self, v: u64) {
        self.max_contract_gas = v;
    }

    // uint64 max_contract_msg_size = 3;


    pub fn get_max_contract_msg_size(&self) -> u64 {
        self.max_contract_msg_size
    }
    pub fn clear_max_contract_msg_size(&mut self) {
        self.max_contract_msg_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_contract_msg_size(&mut self, v: u64) {
        self.max_contract_msg_size = v;
    }
}

impl ::protobuf::Message for Params {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.max_contract_size = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.max_contract_gas = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.max_contract_msg_size = tmp;
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
        if self.max_contract_size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.max_contract_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.max_contract_gas != 0 {
            my_size += ::protobuf::rt::value_size(2, self.max_contract_gas, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.max_contract_msg_size != 0 {
            my_size += ::protobuf::rt::value_size(3, self.max_contract_msg_size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.max_contract_size != 0 {
            os.write_uint64(1, self.max_contract_size)?;
        }
        if self.max_contract_gas != 0 {
            os.write_uint64(2, self.max_contract_gas)?;
        }
        if self.max_contract_msg_size != 0 {
            os.write_uint64(3, self.max_contract_msg_size)?;
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

    fn new() -> Params {
        Params::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "max_contract_size",
                |m: &Params| { &m.max_contract_size },
                |m: &mut Params| { &mut m.max_contract_size },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "max_contract_gas",
                |m: &Params| { &m.max_contract_gas },
                |m: &mut Params| { &mut m.max_contract_gas },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "max_contract_msg_size",
                |m: &Params| { &m.max_contract_msg_size },
                |m: &mut Params| { &mut m.max_contract_msg_size },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Params>(
                "Params",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Params {
        static instance: ::protobuf::rt::LazyV2<Params> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Params::new)
    }
}

impl ::protobuf::Clear for Params {
    fn clear(&mut self) {
        self.max_contract_size = 0;
        self.max_contract_gas = 0;
        self.max_contract_msg_size = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Params {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Params {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CodeInfo {
    // message fields
    pub code_id: u64,
    pub code_hash: ::std::vec::Vec<u8>,
    pub creator: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CodeInfo {
    fn default() -> &'a CodeInfo {
        <CodeInfo as ::protobuf::Message>::default_instance()
    }
}

impl CodeInfo {
    pub fn new() -> CodeInfo {
        ::std::default::Default::default()
    }

    // uint64 code_id = 1;


    pub fn get_code_id(&self) -> u64 {
        self.code_id
    }
    pub fn clear_code_id(&mut self) {
        self.code_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_code_id(&mut self, v: u64) {
        self.code_id = v;
    }

    // bytes code_hash = 2;


    pub fn get_code_hash(&self) -> &[u8] {
        &self.code_hash
    }
    pub fn clear_code_hash(&mut self) {
        self.code_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_code_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.code_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_code_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.code_hash
    }

    // Take field
    pub fn take_code_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.code_hash, ::std::vec::Vec::new())
    }

    // string creator = 3;


    pub fn get_creator(&self) -> &str {
        &self.creator
    }
    pub fn clear_creator(&mut self) {
        self.creator.clear();
    }

    // Param is passed by value, moved
    pub fn set_creator(&mut self, v: ::std::string::String) {
        self.creator = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creator(&mut self) -> &mut ::std::string::String {
        &mut self.creator
    }

    // Take field
    pub fn take_creator(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.creator, ::std::string::String::new())
    }
}

impl ::protobuf::Message for CodeInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.code_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.code_hash)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.creator)?;
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
        if self.code_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.code_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.code_hash);
        }
        if !self.creator.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.creator);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.code_id != 0 {
            os.write_uint64(1, self.code_id)?;
        }
        if !self.code_hash.is_empty() {
            os.write_bytes(2, &self.code_hash)?;
        }
        if !self.creator.is_empty() {
            os.write_string(3, &self.creator)?;
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

    fn new() -> CodeInfo {
        CodeInfo::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "code_id",
                |m: &CodeInfo| { &m.code_id },
                |m: &mut CodeInfo| { &mut m.code_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "code_hash",
                |m: &CodeInfo| { &m.code_hash },
                |m: &mut CodeInfo| { &mut m.code_hash },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "creator",
                |m: &CodeInfo| { &m.creator },
                |m: &mut CodeInfo| { &mut m.creator },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CodeInfo>(
                "CodeInfo",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CodeInfo {
        static instance: ::protobuf::rt::LazyV2<CodeInfo> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CodeInfo::new)
    }
}

impl ::protobuf::Clear for CodeInfo {
    fn clear(&mut self) {
        self.code_id = 0;
        self.code_hash.clear();
        self.creator.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CodeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CodeInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ContractInfo {
    // message fields
    pub address: ::std::string::String,
    pub creator: ::std::string::String,
    pub admin: ::std::string::String,
    pub code_id: u64,
    pub init_msg: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ContractInfo {
    fn default() -> &'a ContractInfo {
        <ContractInfo as ::protobuf::Message>::default_instance()
    }
}

impl ContractInfo {
    pub fn new() -> ContractInfo {
        ::std::default::Default::default()
    }

    // string address = 1;


    pub fn get_address(&self) -> &str {
        &self.address
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    // string creator = 2;


    pub fn get_creator(&self) -> &str {
        &self.creator
    }
    pub fn clear_creator(&mut self) {
        self.creator.clear();
    }

    // Param is passed by value, moved
    pub fn set_creator(&mut self, v: ::std::string::String) {
        self.creator = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creator(&mut self) -> &mut ::std::string::String {
        &mut self.creator
    }

    // Take field
    pub fn take_creator(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.creator, ::std::string::String::new())
    }

    // string admin = 3;


    pub fn get_admin(&self) -> &str {
        &self.admin
    }
    pub fn clear_admin(&mut self) {
        self.admin.clear();
    }

    // Param is passed by value, moved
    pub fn set_admin(&mut self, v: ::std::string::String) {
        self.admin = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_admin(&mut self) -> &mut ::std::string::String {
        &mut self.admin
    }

    // Take field
    pub fn take_admin(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.admin, ::std::string::String::new())
    }

    // uint64 code_id = 4;


    pub fn get_code_id(&self) -> u64 {
        self.code_id
    }
    pub fn clear_code_id(&mut self) {
        self.code_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_code_id(&mut self, v: u64) {
        self.code_id = v;
    }

    // bytes init_msg = 5;


    pub fn get_init_msg(&self) -> &[u8] {
        &self.init_msg
    }
    pub fn clear_init_msg(&mut self) {
        self.init_msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_init_msg(&mut self, v: ::std::vec::Vec<u8>) {
        self.init_msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_init_msg(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.init_msg
    }

    // Take field
    pub fn take_init_msg(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.init_msg, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for ContractInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.creator)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.admin)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.code_id = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.init_msg)?;
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
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.address);
        }
        if !self.creator.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.creator);
        }
        if !self.admin.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.admin);
        }
        if self.code_id != 0 {
            my_size += ::protobuf::rt::value_size(4, self.code_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.init_msg.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.init_msg);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.address.is_empty() {
            os.write_string(1, &self.address)?;
        }
        if !self.creator.is_empty() {
            os.write_string(2, &self.creator)?;
        }
        if !self.admin.is_empty() {
            os.write_string(3, &self.admin)?;
        }
        if self.code_id != 0 {
            os.write_uint64(4, self.code_id)?;
        }
        if !self.init_msg.is_empty() {
            os.write_bytes(5, &self.init_msg)?;
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

    fn new() -> ContractInfo {
        ContractInfo::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "address",
                |m: &ContractInfo| { &m.address },
                |m: &mut ContractInfo| { &mut m.address },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "creator",
                |m: &ContractInfo| { &m.creator },
                |m: &mut ContractInfo| { &mut m.creator },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "admin",
                |m: &ContractInfo| { &m.admin },
                |m: &mut ContractInfo| { &mut m.admin },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "code_id",
                |m: &ContractInfo| { &m.code_id },
                |m: &mut ContractInfo| { &mut m.code_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "init_msg",
                |m: &ContractInfo| { &m.init_msg },
                |m: &mut ContractInfo| { &mut m.init_msg },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ContractInfo>(
                "ContractInfo",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ContractInfo {
        static instance: ::protobuf::rt::LazyV2<ContractInfo> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ContractInfo::new)
    }
}

impl ::protobuf::Clear for ContractInfo {
    fn clear(&mut self) {
        self.address.clear();
        self.creator.clear();
        self.admin.clear();
        self.code_id = 0;
        self.init_msg.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ContractInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContractInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dterra/wasm/v1beta1/wasm.proto\x12\x12terra.wasm.v1beta1\x1a\x14gog\
    oproto/gogo.proto\x1a\x1ecosmos/base/v1beta1/coin.proto\"\xf8\x01\n\x06P\
    arams\x12H\n\x11max_contract_size\x18\x01\x20\x01(\x04R\x0fmaxContractSi\
    zeB\x1c\xf2\xde\x1f\x18yaml:\"max_contract_size\"\x12E\n\x10max_contract\
    _gas\x18\x02\x20\x01(\x04R\x0emaxContractGasB\x1b\xf2\xde\x1f\x17yaml:\"\
    max_contract_gas\"\x12S\n\x15max_contract_msg_size\x18\x03\x20\x01(\x04R\
    \x12maxContractMsgSizeB\x20\xf2\xde\x1f\x1cyaml:\"max_contract_msg_size\
    \":\x08\xe8\xa0\x1f\x01\x98\xa0\x1f\0\"\xa2\x01\n\x08CodeInfo\x125\n\x07\
    code_id\x18\x01\x20\x01(\x04R\x06codeIdB\x1c\xf2\xde\x1f\x0eyaml:\"code_\
    id\"\xe2\xde\x1f\x06CodeID\x121\n\tcode_hash\x18\x02\x20\x01(\x0cR\x08co\
    deHashB\x14\xf2\xde\x1f\x10yaml:\"code_hash\"\x12,\n\x07creator\x18\x03\
    \x20\x01(\tR\x07creatorB\x12\xf2\xde\x1f\x0eyaml:\"creator\"\"\x9b\x02\n\
    \x0cContractInfo\x12,\n\x07address\x18\x01\x20\x01(\tR\x07addressB\x12\
    \xf2\xde\x1f\x0eyaml:\"address\"\x12,\n\x07creator\x18\x02\x20\x01(\tR\
    \x07creatorB\x12\xf2\xde\x1f\x0eyaml:\"creator\"\x12&\n\x05admin\x18\x03\
    \x20\x01(\tR\x05adminB\x10\xf2\xde\x1f\x0cyaml:\"admin\"\x125\n\x07code_\
    id\x18\x04\x20\x01(\x04R\x06codeIdB\x1c\xf2\xde\x1f\x0eyaml:\"code_id\"\
    \xe2\xde\x1f\x06CodeID\x12J\n\x08init_msg\x18\x05\x20\x01(\x0cR\x07initM\
    sgB/\xf2\xde\x1f\x0fyaml:\"init_msg\"\xfa\xde\x1f\x18encoding/json.RawMe\
    ssage:\x04\xe8\xa0\x1f\x01B*Z(github.com/terra-money/core/x/wasm/typesb\
    \x06proto3\
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
