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
//! Generated file from `tendermint/version/types.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct App {
    // message fields
    pub protocol: u64,
    pub software: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a App {
    fn default() -> &'a App {
        <App as ::protobuf::Message>::default_instance()
    }
}

impl App {
    pub fn new() -> App {
        ::std::default::Default::default()
    }

    // uint64 protocol = 1;


    pub fn get_protocol(&self) -> u64 {
        self.protocol
    }
    pub fn clear_protocol(&mut self) {
        self.protocol = 0;
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: u64) {
        self.protocol = v;
    }

    // string software = 2;


    pub fn get_software(&self) -> &str {
        &self.software
    }
    pub fn clear_software(&mut self) {
        self.software.clear();
    }

    // Param is passed by value, moved
    pub fn set_software(&mut self, v: ::std::string::String) {
        self.software = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_software(&mut self) -> &mut ::std::string::String {
        &mut self.software
    }

    // Take field
    pub fn take_software(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.software, ::std::string::String::new())
    }
}

impl ::protobuf::Message for App {
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
                    self.protocol = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.software)?;
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
        if self.protocol != 0 {
            my_size += ::protobuf::rt::value_size(1, self.protocol, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.software.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.software);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.protocol != 0 {
            os.write_uint64(1, self.protocol)?;
        }
        if !self.software.is_empty() {
            os.write_string(2, &self.software)?;
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

    fn new() -> App {
        App::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "protocol",
                |m: &App| { &m.protocol },
                |m: &mut App| { &mut m.protocol },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "software",
                |m: &App| { &m.software },
                |m: &mut App| { &mut m.software },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<App>(
                "App",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static App {
        static instance: ::protobuf::rt::LazyV2<App> = ::protobuf::rt::LazyV2::INIT;
        instance.get(App::new)
    }
}

impl ::protobuf::Clear for App {
    fn clear(&mut self) {
        self.protocol = 0;
        self.software.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for App {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for App {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Consensus {
    // message fields
    pub block: u64,
    pub app: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Consensus {
    fn default() -> &'a Consensus {
        <Consensus as ::protobuf::Message>::default_instance()
    }
}

impl Consensus {
    pub fn new() -> Consensus {
        ::std::default::Default::default()
    }

    // uint64 block = 1;


    pub fn get_block(&self) -> u64 {
        self.block
    }
    pub fn clear_block(&mut self) {
        self.block = 0;
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: u64) {
        self.block = v;
    }

    // uint64 app = 2;


    pub fn get_app(&self) -> u64 {
        self.app
    }
    pub fn clear_app(&mut self) {
        self.app = 0;
    }

    // Param is passed by value, moved
    pub fn set_app(&mut self, v: u64) {
        self.app = v;
    }
}

impl ::protobuf::Message for Consensus {
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
                    self.block = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.app = tmp;
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
        if self.block != 0 {
            my_size += ::protobuf::rt::value_size(1, self.block, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.app != 0 {
            my_size += ::protobuf::rt::value_size(2, self.app, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.block != 0 {
            os.write_uint64(1, self.block)?;
        }
        if self.app != 0 {
            os.write_uint64(2, self.app)?;
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

    fn new() -> Consensus {
        Consensus::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "block",
                |m: &Consensus| { &m.block },
                |m: &mut Consensus| { &mut m.block },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "app",
                |m: &Consensus| { &m.app },
                |m: &mut Consensus| { &mut m.app },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Consensus>(
                "Consensus",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Consensus {
        static instance: ::protobuf::rt::LazyV2<Consensus> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Consensus::new)
    }
}

impl ::protobuf::Clear for Consensus {
    fn clear(&mut self) {
        self.block = 0;
        self.app = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Consensus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Consensus {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1etendermint/version/types.proto\x12\x12tendermint.version\x1a\x14go\
    goproto/gogo.proto\"=\n\x03App\x12\x1a\n\x08protocol\x18\x01\x20\x01(\
    \x04R\x08protocol\x12\x1a\n\x08software\x18\x02\x20\x01(\tR\x08software\
    \"9\n\tConsensus\x12\x14\n\x05block\x18\x01\x20\x01(\x04R\x05block\x12\
    \x10\n\x03app\x18\x02\x20\x01(\x04R\x03app:\x04\xe8\xa0\x1f\x01B;Z9githu\
    b.com/tendermint/tendermint/proto/tendermint/versionb\x06proto3\
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
