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
//! Generated file from `tendermint/types/block.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct Block {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::types::Header>,
    pub data: ::protobuf::SingularPtrField<super::types::Data>,
    pub evidence: ::protobuf::SingularPtrField<super::evidence::EvidenceList>,
    pub last_commit: ::protobuf::SingularPtrField<super::types::Commit>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Block {
    fn default() -> &'a Block {
        <Block as ::protobuf::Message>::default_instance()
    }
}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    // .tendermint.types.Header header = 1;


    pub fn get_header(&self) -> &super::types::Header {
        self.header.as_ref().unwrap_or_else(|| <super::types::Header as ::protobuf::Message>::default_instance())
    }
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::types::Header) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::types::Header {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::types::Header {
        self.header.take().unwrap_or_else(|| super::types::Header::new())
    }

    // .tendermint.types.Data data = 2;


    pub fn get_data(&self) -> &super::types::Data {
        self.data.as_ref().unwrap_or_else(|| <super::types::Data as ::protobuf::Message>::default_instance())
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: super::types::Data) {
        self.data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut super::types::Data {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> super::types::Data {
        self.data.take().unwrap_or_else(|| super::types::Data::new())
    }

    // .tendermint.types.EvidenceList evidence = 3;


    pub fn get_evidence(&self) -> &super::evidence::EvidenceList {
        self.evidence.as_ref().unwrap_or_else(|| <super::evidence::EvidenceList as ::protobuf::Message>::default_instance())
    }
    pub fn clear_evidence(&mut self) {
        self.evidence.clear();
    }

    pub fn has_evidence(&self) -> bool {
        self.evidence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_evidence(&mut self, v: super::evidence::EvidenceList) {
        self.evidence = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_evidence(&mut self) -> &mut super::evidence::EvidenceList {
        if self.evidence.is_none() {
            self.evidence.set_default();
        }
        self.evidence.as_mut().unwrap()
    }

    // Take field
    pub fn take_evidence(&mut self) -> super::evidence::EvidenceList {
        self.evidence.take().unwrap_or_else(|| super::evidence::EvidenceList::new())
    }

    // .tendermint.types.Commit last_commit = 4;


    pub fn get_last_commit(&self) -> &super::types::Commit {
        self.last_commit.as_ref().unwrap_or_else(|| <super::types::Commit as ::protobuf::Message>::default_instance())
    }
    pub fn clear_last_commit(&mut self) {
        self.last_commit.clear();
    }

    pub fn has_last_commit(&self) -> bool {
        self.last_commit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_commit(&mut self, v: super::types::Commit) {
        self.last_commit = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_commit(&mut self) -> &mut super::types::Commit {
        if self.last_commit.is_none() {
            self.last_commit.set_default();
        }
        self.last_commit.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_commit(&mut self) -> super::types::Commit {
        self.last_commit.take().unwrap_or_else(|| super::types::Commit::new())
    }
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.data {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.evidence {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.last_commit {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.evidence)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.last_commit)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.evidence.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.last_commit.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.evidence.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.last_commit.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> Block {
        Block::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::Header>>(
                "header",
                |m: &Block| { &m.header },
                |m: &mut Block| { &mut m.header },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::Data>>(
                "data",
                |m: &Block| { &m.data },
                |m: &mut Block| { &mut m.data },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::evidence::EvidenceList>>(
                "evidence",
                |m: &Block| { &m.evidence },
                |m: &mut Block| { &mut m.evidence },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::Commit>>(
                "last_commit",
                |m: &Block| { &m.last_commit },
                |m: &mut Block| { &mut m.last_commit },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Block>(
                "Block",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Block {
        static instance: ::protobuf::rt::LazyV2<Block> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Block::new)
    }
}

impl ::protobuf::Clear for Block {
    fn clear(&mut self) {
        self.header.clear();
        self.data.clear();
        self.evidence.clear();
        self.last_commit.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Block {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1ctendermint/types/block.proto\x12\x10tendermint.types\x1a\x14gogopr\
    oto/gogo.proto\x1a\x1ctendermint/types/types.proto\x1a\x1ftendermint/typ\
    es/evidence.proto\"\xee\x01\n\x05Block\x126\n\x06header\x18\x01\x20\x01(\
    \x0b2\x18.tendermint.types.HeaderR\x06headerB\x04\xc8\xde\x1f\0\x120\n\
    \x04data\x18\x02\x20\x01(\x0b2\x16.tendermint.types.DataR\x04dataB\x04\
    \xc8\xde\x1f\0\x12@\n\x08evidence\x18\x03\x20\x01(\x0b2\x1e.tendermint.t\
    ypes.EvidenceListR\x08evidenceB\x04\xc8\xde\x1f\0\x129\n\x0blast_commit\
    \x18\x04\x20\x01(\x0b2\x18.tendermint.types.CommitR\nlastCommitB9Z7githu\
    b.com/tendermint/tendermint/proto/tendermint/typesb\x06proto3\
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
