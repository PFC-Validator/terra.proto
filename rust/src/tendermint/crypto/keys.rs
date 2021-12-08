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
//! Generated file from `tendermint/crypto/keys.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct PublicKey {
    // message oneof groups
    pub sum: ::std::option::Option<PublicKey_oneof_sum>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PublicKey {
    fn default() -> &'a PublicKey {
        <PublicKey as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum PublicKey_oneof_sum {
    ed25519(::std::vec::Vec<u8>),
    secp256k1(::std::vec::Vec<u8>),
}

impl PublicKey {
    pub fn new() -> PublicKey {
        ::std::default::Default::default()
    }

    // bytes ed25519 = 1;


    pub fn get_ed25519(&self) -> &[u8] {
        match self.sum {
            ::std::option::Option::Some(PublicKey_oneof_sum::ed25519(ref v)) => v,
            _ => &[],
        }
    }
    pub fn clear_ed25519(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_ed25519(&self) -> bool {
        match self.sum {
            ::std::option::Option::Some(PublicKey_oneof_sum::ed25519(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ed25519(&mut self, v: ::std::vec::Vec<u8>) {
        self.sum = ::std::option::Option::Some(PublicKey_oneof_sum::ed25519(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ed25519(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(PublicKey_oneof_sum::ed25519(_)) = self.sum {
        } else {
            self.sum = ::std::option::Option::Some(PublicKey_oneof_sum::ed25519(::std::vec::Vec::new()));
        }
        match self.sum {
            ::std::option::Option::Some(PublicKey_oneof_sum::ed25519(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ed25519(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_ed25519() {
            match self.sum.take() {
                ::std::option::Option::Some(PublicKey_oneof_sum::ed25519(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    // bytes secp256k1 = 2;


    pub fn get_secp256k1(&self) -> &[u8] {
        match self.sum {
            ::std::option::Option::Some(PublicKey_oneof_sum::secp256k1(ref v)) => v,
            _ => &[],
        }
    }
    pub fn clear_secp256k1(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_secp256k1(&self) -> bool {
        match self.sum {
            ::std::option::Option::Some(PublicKey_oneof_sum::secp256k1(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_secp256k1(&mut self, v: ::std::vec::Vec<u8>) {
        self.sum = ::std::option::Option::Some(PublicKey_oneof_sum::secp256k1(v))
    }

    // Mutable pointer to the field.
    pub fn mut_secp256k1(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(PublicKey_oneof_sum::secp256k1(_)) = self.sum {
        } else {
            self.sum = ::std::option::Option::Some(PublicKey_oneof_sum::secp256k1(::std::vec::Vec::new()));
        }
        match self.sum {
            ::std::option::Option::Some(PublicKey_oneof_sum::secp256k1(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_secp256k1(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_secp256k1() {
            match self.sum.take() {
                ::std::option::Option::Some(PublicKey_oneof_sum::secp256k1(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }
}

impl ::protobuf::Message for PublicKey {
    fn is_initialized(&self) -> bool {
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
                    self.sum = ::std::option::Option::Some(PublicKey_oneof_sum::ed25519(is.read_bytes()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.sum = ::std::option::Option::Some(PublicKey_oneof_sum::secp256k1(is.read_bytes()?));
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
                &PublicKey_oneof_sum::ed25519(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &PublicKey_oneof_sum::secp256k1(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(2, &v);
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
                &PublicKey_oneof_sum::ed25519(ref v) => {
                    os.write_bytes(1, v)?;
                },
                &PublicKey_oneof_sum::secp256k1(ref v) => {
                    os.write_bytes(2, v)?;
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

    fn new() -> PublicKey {
        PublicKey::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                "ed25519",
                PublicKey::has_ed25519,
                PublicKey::get_ed25519,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                "secp256k1",
                PublicKey::has_secp256k1,
                PublicKey::get_secp256k1,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PublicKey>(
                "PublicKey",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PublicKey {
        static instance: ::protobuf::rt::LazyV2<PublicKey> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PublicKey::new)
    }
}

impl ::protobuf::Clear for PublicKey {
    fn clear(&mut self) {
        self.sum = ::std::option::Option::None;
        self.sum = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PublicKey {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1ctendermint/crypto/keys.proto\x12\x11tendermint.crypto\x1a\x14gogop\
    roto/gogo.proto\"X\n\tPublicKey\x12\x1a\n\x07ed25519\x18\x01\x20\x01(\
    \x0cH\0R\x07ed25519\x12\x1e\n\tsecp256k1\x18\x02\x20\x01(\x0cH\0R\tsecp2\
    56k1B\x05\n\x03sum:\x08\xe8\xa1\x1f\x01\xe8\xa0\x1f\x01B:Z8github.com/te\
    ndermint/tendermint/proto/tendermint/cryptob\x06proto3\
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
