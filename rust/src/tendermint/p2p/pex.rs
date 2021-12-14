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
//! Generated file from `tendermint/p2p/pex.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct PexRequest {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PexRequest {
    fn default() -> &'a PexRequest {
        <PexRequest as ::protobuf::Message>::default_instance()
    }
}

impl PexRequest {
    pub fn new() -> PexRequest {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for PexRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
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

    fn new() -> PexRequest {
        PexRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PexRequest>(
                "PexRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PexRequest {
        static instance: ::protobuf::rt::LazyV2<PexRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PexRequest::new)
    }
}

impl ::protobuf::Clear for PexRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PexRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PexRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PexAddrs {
    // message fields
    pub addrs: ::protobuf::RepeatedField<super::types::NetAddress>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PexAddrs {
    fn default() -> &'a PexAddrs {
        <PexAddrs as ::protobuf::Message>::default_instance()
    }
}

impl PexAddrs {
    pub fn new() -> PexAddrs {
        ::std::default::Default::default()
    }

    // repeated .tendermint.p2p.NetAddress addrs = 1;


    pub fn get_addrs(&self) -> &[super::types::NetAddress] {
        &self.addrs
    }
    pub fn clear_addrs(&mut self) {
        self.addrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_addrs(&mut self, v: ::protobuf::RepeatedField<super::types::NetAddress>) {
        self.addrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_addrs(&mut self) -> &mut ::protobuf::RepeatedField<super::types::NetAddress> {
        &mut self.addrs
    }

    // Take field
    pub fn take_addrs(&mut self) -> ::protobuf::RepeatedField<super::types::NetAddress> {
        ::std::mem::replace(&mut self.addrs, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for PexAddrs {
    fn is_initialized(&self) -> bool {
        for v in &self.addrs {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.addrs)?;
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
        for value in &self.addrs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.addrs {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> PexAddrs {
        PexAddrs::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::NetAddress>>(
                "addrs",
                |m: &PexAddrs| { &m.addrs },
                |m: &mut PexAddrs| { &mut m.addrs },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PexAddrs>(
                "PexAddrs",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PexAddrs {
        static instance: ::protobuf::rt::LazyV2<PexAddrs> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PexAddrs::new)
    }
}

impl ::protobuf::Clear for PexAddrs {
    fn clear(&mut self) {
        self.addrs.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PexAddrs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PexAddrs {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message oneof groups
    pub sum: ::std::option::Option<Message_oneof_sum>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Message {
    fn default() -> &'a Message {
        <Message as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum Message_oneof_sum {
    pex_request(PexRequest),
    pex_addrs(PexAddrs),
}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    // .tendermint.p2p.PexRequest pex_request = 1;


    pub fn get_pex_request(&self) -> &PexRequest {
        match self.sum {
            ::std::option::Option::Some(Message_oneof_sum::pex_request(ref v)) => v,
            _ => <PexRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_pex_request(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_pex_request(&self) -> bool {
        match self.sum {
            ::std::option::Option::Some(Message_oneof_sum::pex_request(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pex_request(&mut self, v: PexRequest) {
        self.sum = ::std::option::Option::Some(Message_oneof_sum::pex_request(v))
    }

    // Mutable pointer to the field.
    pub fn mut_pex_request(&mut self) -> &mut PexRequest {
        if let ::std::option::Option::Some(Message_oneof_sum::pex_request(_)) = self.sum {
        } else {
            self.sum = ::std::option::Option::Some(Message_oneof_sum::pex_request(PexRequest::new()));
        }
        match self.sum {
            ::std::option::Option::Some(Message_oneof_sum::pex_request(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pex_request(&mut self) -> PexRequest {
        if self.has_pex_request() {
            match self.sum.take() {
                ::std::option::Option::Some(Message_oneof_sum::pex_request(v)) => v,
                _ => panic!(),
            }
        } else {
            PexRequest::new()
        }
    }

    // .tendermint.p2p.PexAddrs pex_addrs = 2;


    pub fn get_pex_addrs(&self) -> &PexAddrs {
        match self.sum {
            ::std::option::Option::Some(Message_oneof_sum::pex_addrs(ref v)) => v,
            _ => <PexAddrs as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_pex_addrs(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_pex_addrs(&self) -> bool {
        match self.sum {
            ::std::option::Option::Some(Message_oneof_sum::pex_addrs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pex_addrs(&mut self, v: PexAddrs) {
        self.sum = ::std::option::Option::Some(Message_oneof_sum::pex_addrs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_pex_addrs(&mut self) -> &mut PexAddrs {
        if let ::std::option::Option::Some(Message_oneof_sum::pex_addrs(_)) = self.sum {
        } else {
            self.sum = ::std::option::Option::Some(Message_oneof_sum::pex_addrs(PexAddrs::new()));
        }
        match self.sum {
            ::std::option::Option::Some(Message_oneof_sum::pex_addrs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pex_addrs(&mut self) -> PexAddrs {
        if self.has_pex_addrs() {
            match self.sum.take() {
                ::std::option::Option::Some(Message_oneof_sum::pex_addrs(v)) => v,
                _ => panic!(),
            }
        } else {
            PexAddrs::new()
        }
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        if let Some(Message_oneof_sum::pex_request(ref v)) = self.sum {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_sum::pex_addrs(ref v)) = self.sum {
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
                    self.sum = ::std::option::Option::Some(Message_oneof_sum::pex_request(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.sum = ::std::option::Option::Some(Message_oneof_sum::pex_addrs(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.sum {
            match v {
                &Message_oneof_sum::pex_request(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_sum::pex_addrs(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.sum {
            match v {
                &Message_oneof_sum::pex_request(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_sum::pex_addrs(ref v) => {
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

    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PexRequest>(
                "pex_request",
                Message::has_pex_request,
                Message::get_pex_request,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PexAddrs>(
                "pex_addrs",
                Message::has_pex_addrs,
                Message::get_pex_addrs,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Message>(
                "Message",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Message {
        static instance: ::protobuf::rt::LazyV2<Message> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Message::new)
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.sum = ::std::option::Option::None;
        self.sum = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18tendermint/p2p/pex.proto\x12\x0etendermint.p2p\x1a\x1atendermint/p\
    2p/types.proto\x1a\x14gogoproto/gogo.proto\"\x0c\n\nPexRequest\"B\n\x08P\
    exAddrs\x126\n\x05addrs\x18\x01\x20\x03(\x0b2\x1a.tendermint.p2p.NetAddr\
    essR\x05addrsB\x04\xc8\xde\x1f\0\"\x88\x01\n\x07Message\x12=\n\x0bpex_re\
    quest\x18\x01\x20\x01(\x0b2\x1a.tendermint.p2p.PexRequestH\0R\npexReques\
    t\x127\n\tpex_addrs\x18\x02\x20\x01(\x0b2\x18.tendermint.p2p.PexAddrsH\0\
    R\x08pexAddrsB\x05\n\x03sumB7Z5github.com/tendermint/tendermint/proto/te\
    ndermint/p2pb\x06proto3\
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
