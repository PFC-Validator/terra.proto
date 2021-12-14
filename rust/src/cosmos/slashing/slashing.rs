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
//! Generated file from `cosmos/slashing/v1beta1/slashing.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct ValidatorSigningInfo {
    // message fields
    pub address: ::std::string::String,
    pub start_height: i64,
    pub index_offset: i64,
    pub jailed_until: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub tombstoned: bool,
    pub missed_blocks_counter: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ValidatorSigningInfo {
    fn default() -> &'a ValidatorSigningInfo {
        <ValidatorSigningInfo as ::protobuf::Message>::default_instance()
    }
}

impl ValidatorSigningInfo {
    pub fn new() -> ValidatorSigningInfo {
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

    // int64 start_height = 2;


    pub fn get_start_height(&self) -> i64 {
        self.start_height
    }
    pub fn clear_start_height(&mut self) {
        self.start_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_height(&mut self, v: i64) {
        self.start_height = v;
    }

    // int64 index_offset = 3;


    pub fn get_index_offset(&self) -> i64 {
        self.index_offset
    }
    pub fn clear_index_offset(&mut self) {
        self.index_offset = 0;
    }

    // Param is passed by value, moved
    pub fn set_index_offset(&mut self, v: i64) {
        self.index_offset = v;
    }

    // .google.protobuf.Timestamp jailed_until = 4;


    pub fn get_jailed_until(&self) -> &::protobuf::well_known_types::Timestamp {
        self.jailed_until.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_jailed_until(&mut self) {
        self.jailed_until.clear();
    }

    pub fn has_jailed_until(&self) -> bool {
        self.jailed_until.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jailed_until(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.jailed_until = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jailed_until(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.jailed_until.is_none() {
            self.jailed_until.set_default();
        }
        self.jailed_until.as_mut().unwrap()
    }

    // Take field
    pub fn take_jailed_until(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.jailed_until.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // bool tombstoned = 5;


    pub fn get_tombstoned(&self) -> bool {
        self.tombstoned
    }
    pub fn clear_tombstoned(&mut self) {
        self.tombstoned = false;
    }

    // Param is passed by value, moved
    pub fn set_tombstoned(&mut self, v: bool) {
        self.tombstoned = v;
    }

    // int64 missed_blocks_counter = 6;


    pub fn get_missed_blocks_counter(&self) -> i64 {
        self.missed_blocks_counter
    }
    pub fn clear_missed_blocks_counter(&mut self) {
        self.missed_blocks_counter = 0;
    }

    // Param is passed by value, moved
    pub fn set_missed_blocks_counter(&mut self, v: i64) {
        self.missed_blocks_counter = v;
    }
}

impl ::protobuf::Message for ValidatorSigningInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.jailed_until {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.start_height = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.index_offset = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jailed_until)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.tombstoned = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.missed_blocks_counter = tmp;
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
        if self.start_height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.start_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.index_offset != 0 {
            my_size += ::protobuf::rt::value_size(3, self.index_offset, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.jailed_until.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.tombstoned != false {
            my_size += 2;
        }
        if self.missed_blocks_counter != 0 {
            my_size += ::protobuf::rt::value_size(6, self.missed_blocks_counter, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.address.is_empty() {
            os.write_string(1, &self.address)?;
        }
        if self.start_height != 0 {
            os.write_int64(2, self.start_height)?;
        }
        if self.index_offset != 0 {
            os.write_int64(3, self.index_offset)?;
        }
        if let Some(ref v) = self.jailed_until.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.tombstoned != false {
            os.write_bool(5, self.tombstoned)?;
        }
        if self.missed_blocks_counter != 0 {
            os.write_int64(6, self.missed_blocks_counter)?;
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

    fn new() -> ValidatorSigningInfo {
        ValidatorSigningInfo::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "address",
                |m: &ValidatorSigningInfo| { &m.address },
                |m: &mut ValidatorSigningInfo| { &mut m.address },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "start_height",
                |m: &ValidatorSigningInfo| { &m.start_height },
                |m: &mut ValidatorSigningInfo| { &mut m.start_height },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "index_offset",
                |m: &ValidatorSigningInfo| { &m.index_offset },
                |m: &mut ValidatorSigningInfo| { &mut m.index_offset },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "jailed_until",
                |m: &ValidatorSigningInfo| { &m.jailed_until },
                |m: &mut ValidatorSigningInfo| { &mut m.jailed_until },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "tombstoned",
                |m: &ValidatorSigningInfo| { &m.tombstoned },
                |m: &mut ValidatorSigningInfo| { &mut m.tombstoned },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "missed_blocks_counter",
                |m: &ValidatorSigningInfo| { &m.missed_blocks_counter },
                |m: &mut ValidatorSigningInfo| { &mut m.missed_blocks_counter },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ValidatorSigningInfo>(
                "ValidatorSigningInfo",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ValidatorSigningInfo {
        static instance: ::protobuf::rt::LazyV2<ValidatorSigningInfo> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ValidatorSigningInfo::new)
    }
}

impl ::protobuf::Clear for ValidatorSigningInfo {
    fn clear(&mut self) {
        self.address.clear();
        self.start_height = 0;
        self.index_offset = 0;
        self.jailed_until.clear();
        self.tombstoned = false;
        self.missed_blocks_counter = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ValidatorSigningInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ValidatorSigningInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Params {
    // message fields
    pub signed_blocks_window: i64,
    pub min_signed_per_window: ::std::vec::Vec<u8>,
    pub downtime_jail_duration: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub slash_fraction_double_sign: ::std::vec::Vec<u8>,
    pub slash_fraction_downtime: ::std::vec::Vec<u8>,
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

    // int64 signed_blocks_window = 1;


    pub fn get_signed_blocks_window(&self) -> i64 {
        self.signed_blocks_window
    }
    pub fn clear_signed_blocks_window(&mut self) {
        self.signed_blocks_window = 0;
    }

    // Param is passed by value, moved
    pub fn set_signed_blocks_window(&mut self, v: i64) {
        self.signed_blocks_window = v;
    }

    // bytes min_signed_per_window = 2;


    pub fn get_min_signed_per_window(&self) -> &[u8] {
        &self.min_signed_per_window
    }
    pub fn clear_min_signed_per_window(&mut self) {
        self.min_signed_per_window.clear();
    }

    // Param is passed by value, moved
    pub fn set_min_signed_per_window(&mut self, v: ::std::vec::Vec<u8>) {
        self.min_signed_per_window = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_min_signed_per_window(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.min_signed_per_window
    }

    // Take field
    pub fn take_min_signed_per_window(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.min_signed_per_window, ::std::vec::Vec::new())
    }

    // .google.protobuf.Duration downtime_jail_duration = 3;


    pub fn get_downtime_jail_duration(&self) -> &::protobuf::well_known_types::Duration {
        self.downtime_jail_duration.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Duration as ::protobuf::Message>::default_instance())
    }
    pub fn clear_downtime_jail_duration(&mut self) {
        self.downtime_jail_duration.clear();
    }

    pub fn has_downtime_jail_duration(&self) -> bool {
        self.downtime_jail_duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_downtime_jail_duration(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.downtime_jail_duration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_downtime_jail_duration(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.downtime_jail_duration.is_none() {
            self.downtime_jail_duration.set_default();
        }
        self.downtime_jail_duration.as_mut().unwrap()
    }

    // Take field
    pub fn take_downtime_jail_duration(&mut self) -> ::protobuf::well_known_types::Duration {
        self.downtime_jail_duration.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    // bytes slash_fraction_double_sign = 4;


    pub fn get_slash_fraction_double_sign(&self) -> &[u8] {
        &self.slash_fraction_double_sign
    }
    pub fn clear_slash_fraction_double_sign(&mut self) {
        self.slash_fraction_double_sign.clear();
    }

    // Param is passed by value, moved
    pub fn set_slash_fraction_double_sign(&mut self, v: ::std::vec::Vec<u8>) {
        self.slash_fraction_double_sign = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slash_fraction_double_sign(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.slash_fraction_double_sign
    }

    // Take field
    pub fn take_slash_fraction_double_sign(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.slash_fraction_double_sign, ::std::vec::Vec::new())
    }

    // bytes slash_fraction_downtime = 5;


    pub fn get_slash_fraction_downtime(&self) -> &[u8] {
        &self.slash_fraction_downtime
    }
    pub fn clear_slash_fraction_downtime(&mut self) {
        self.slash_fraction_downtime.clear();
    }

    // Param is passed by value, moved
    pub fn set_slash_fraction_downtime(&mut self, v: ::std::vec::Vec<u8>) {
        self.slash_fraction_downtime = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slash_fraction_downtime(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.slash_fraction_downtime
    }

    // Take field
    pub fn take_slash_fraction_downtime(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.slash_fraction_downtime, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Params {
    fn is_initialized(&self) -> bool {
        for v in &self.downtime_jail_duration {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.signed_blocks_window = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.min_signed_per_window)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.downtime_jail_duration)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.slash_fraction_double_sign)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.slash_fraction_downtime)?;
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
        if self.signed_blocks_window != 0 {
            my_size += ::protobuf::rt::value_size(1, self.signed_blocks_window, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.min_signed_per_window.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.min_signed_per_window);
        }
        if let Some(ref v) = self.downtime_jail_duration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.slash_fraction_double_sign.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.slash_fraction_double_sign);
        }
        if !self.slash_fraction_downtime.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.slash_fraction_downtime);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.signed_blocks_window != 0 {
            os.write_int64(1, self.signed_blocks_window)?;
        }
        if !self.min_signed_per_window.is_empty() {
            os.write_bytes(2, &self.min_signed_per_window)?;
        }
        if let Some(ref v) = self.downtime_jail_duration.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.slash_fraction_double_sign.is_empty() {
            os.write_bytes(4, &self.slash_fraction_double_sign)?;
        }
        if !self.slash_fraction_downtime.is_empty() {
            os.write_bytes(5, &self.slash_fraction_downtime)?;
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
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "signed_blocks_window",
                |m: &Params| { &m.signed_blocks_window },
                |m: &mut Params| { &mut m.signed_blocks_window },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "min_signed_per_window",
                |m: &Params| { &m.min_signed_per_window },
                |m: &mut Params| { &mut m.min_signed_per_window },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                "downtime_jail_duration",
                |m: &Params| { &m.downtime_jail_duration },
                |m: &mut Params| { &mut m.downtime_jail_duration },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "slash_fraction_double_sign",
                |m: &Params| { &m.slash_fraction_double_sign },
                |m: &mut Params| { &mut m.slash_fraction_double_sign },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "slash_fraction_downtime",
                |m: &Params| { &m.slash_fraction_downtime },
                |m: &mut Params| { &mut m.slash_fraction_downtime },
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
        self.signed_blocks_window = 0;
        self.min_signed_per_window.clear();
        self.downtime_jail_duration.clear();
        self.slash_fraction_double_sign.clear();
        self.slash_fraction_downtime.clear();
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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&cosmos/slashing/v1beta1/slashing.proto\x12\x17cosmos.slashing.v1beta1\
    \x1a\x14gogoproto/gogo.proto\x1a\x1egoogle/protobuf/duration.proto\x1a\
    \x1fgoogle/protobuf/timestamp.proto\"\x88\x03\n\x14ValidatorSigningInfo\
    \x12\x18\n\x07address\x18\x01\x20\x01(\tR\x07address\x12:\n\x0cstart_hei\
    ght\x18\x02\x20\x01(\x03R\x0bstartHeightB\x17\xf2\xde\x1f\x13yaml:\"star\
    t_height\"\x12:\n\x0cindex_offset\x18\x03\x20\x01(\x03R\x0bindexOffsetB\
    \x17\xf2\xde\x1f\x13yaml:\"index_offset\"\x12^\n\x0cjailed_until\x18\x04\
    \x20\x01(\x0b2\x1a.google.protobuf.TimestampR\x0bjailedUntilB\x1f\xf2\
    \xde\x1f\x13yaml:\"jailed_until\"\x90\xdf\x1f\x01\xc8\xde\x1f\0\x12\x1e\
    \n\ntombstoned\x18\x05\x20\x01(\x08R\ntombstoned\x12T\n\x15missed_blocks\
    _counter\x18\x06\x20\x01(\x03R\x13missedBlocksCounterB\x20\xf2\xde\x1f\
    \x1cyaml:\"missed_blocks_counter\":\x08\xe8\xa0\x1f\x01\x98\xa0\x1f\0\"\
    \xf9\x04\n\x06Params\x12Q\n\x14signed_blocks_window\x18\x01\x20\x01(\x03\
    R\x12signedBlocksWindowB\x1f\xf2\xde\x1f\x1byaml:\"signed_blocks_window\
    \"\x12\x81\x01\n\x15min_signed_per_window\x18\x02\x20\x01(\x0cR\x12minSi\
    gnedPerWindowBN\xf2\xde\x1f\x1cyaml:\"min_signed_per_window\"\xda\xde\
    \x1f&github.com/cosmos/cosmos-sdk/types.Dec\xc8\xde\x1f\0\x12z\n\x16down\
    time_jail_duration\x18\x03\x20\x01(\x0b2\x19.google.protobuf.DurationR\
    \x14downtimeJailDurationB)\xf2\xde\x1f\x1dyaml:\"downtime_jail_duration\
    \"\xc8\xde\x1f\0\x98\xdf\x1f\x01\x12\x90\x01\n\x1aslash_fraction_double_\
    sign\x18\x04\x20\x01(\x0cR\x17slashFractionDoubleSignBS\xf2\xde\x1f!yaml\
    :\"slash_fraction_double_sign\"\xda\xde\x1f&github.com/cosmos/cosmos-sdk\
    /types.Dec\xc8\xde\x1f\0\x12\x88\x01\n\x17slash_fraction_downtime\x18\
    \x05\x20\x01(\x0cR\x15slashFractionDowntimeBP\xf2\xde\x1f\x1eyaml:\"slas\
    h_fraction_downtime\"\xda\xde\x1f&github.com/cosmos/cosmos-sdk/types.Dec\
    \xc8\xde\x1f\0B3Z-github.com/cosmos/cosmos-sdk/x/slashing/types\xa8\xe2\
    \x1e\x01b\x06proto3\
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