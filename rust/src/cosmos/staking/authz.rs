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
//! Generated file from `cosmos/staking/v1beta1/authz.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct StakeAuthorization {
    // message fields
    pub max_tokens: ::protobuf::SingularPtrField<super::coin::Coin>,
    pub authorization_type: AuthorizationType,
    // message oneof groups
    pub validators: ::std::option::Option<StakeAuthorization_oneof_validators>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StakeAuthorization {
    fn default() -> &'a StakeAuthorization {
        <StakeAuthorization as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum StakeAuthorization_oneof_validators {
    allow_list(StakeAuthorization_Validators),
    deny_list(StakeAuthorization_Validators),
}

impl StakeAuthorization {
    pub fn new() -> StakeAuthorization {
        ::std::default::Default::default()
    }

    // .cosmos.base.v1beta1.Coin max_tokens = 1;


    pub fn get_max_tokens(&self) -> &super::coin::Coin {
        self.max_tokens.as_ref().unwrap_or_else(|| <super::coin::Coin as ::protobuf::Message>::default_instance())
    }
    pub fn clear_max_tokens(&mut self) {
        self.max_tokens.clear();
    }

    pub fn has_max_tokens(&self) -> bool {
        self.max_tokens.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_tokens(&mut self, v: super::coin::Coin) {
        self.max_tokens = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_tokens(&mut self) -> &mut super::coin::Coin {
        if self.max_tokens.is_none() {
            self.max_tokens.set_default();
        }
        self.max_tokens.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_tokens(&mut self) -> super::coin::Coin {
        self.max_tokens.take().unwrap_or_else(|| super::coin::Coin::new())
    }

    // .cosmos.staking.v1beta1.StakeAuthorization.Validators allow_list = 2;


    pub fn get_allow_list(&self) -> &StakeAuthorization_Validators {
        match self.validators {
            ::std::option::Option::Some(StakeAuthorization_oneof_validators::allow_list(ref v)) => v,
            _ => <StakeAuthorization_Validators as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_allow_list(&mut self) {
        self.validators = ::std::option::Option::None;
    }

    pub fn has_allow_list(&self) -> bool {
        match self.validators {
            ::std::option::Option::Some(StakeAuthorization_oneof_validators::allow_list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_allow_list(&mut self, v: StakeAuthorization_Validators) {
        self.validators = ::std::option::Option::Some(StakeAuthorization_oneof_validators::allow_list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_allow_list(&mut self) -> &mut StakeAuthorization_Validators {
        if let ::std::option::Option::Some(StakeAuthorization_oneof_validators::allow_list(_)) = self.validators {
        } else {
            self.validators = ::std::option::Option::Some(StakeAuthorization_oneof_validators::allow_list(StakeAuthorization_Validators::new()));
        }
        match self.validators {
            ::std::option::Option::Some(StakeAuthorization_oneof_validators::allow_list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_allow_list(&mut self) -> StakeAuthorization_Validators {
        if self.has_allow_list() {
            match self.validators.take() {
                ::std::option::Option::Some(StakeAuthorization_oneof_validators::allow_list(v)) => v,
                _ => panic!(),
            }
        } else {
            StakeAuthorization_Validators::new()
        }
    }

    // .cosmos.staking.v1beta1.StakeAuthorization.Validators deny_list = 3;


    pub fn get_deny_list(&self) -> &StakeAuthorization_Validators {
        match self.validators {
            ::std::option::Option::Some(StakeAuthorization_oneof_validators::deny_list(ref v)) => v,
            _ => <StakeAuthorization_Validators as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_deny_list(&mut self) {
        self.validators = ::std::option::Option::None;
    }

    pub fn has_deny_list(&self) -> bool {
        match self.validators {
            ::std::option::Option::Some(StakeAuthorization_oneof_validators::deny_list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_deny_list(&mut self, v: StakeAuthorization_Validators) {
        self.validators = ::std::option::Option::Some(StakeAuthorization_oneof_validators::deny_list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_deny_list(&mut self) -> &mut StakeAuthorization_Validators {
        if let ::std::option::Option::Some(StakeAuthorization_oneof_validators::deny_list(_)) = self.validators {
        } else {
            self.validators = ::std::option::Option::Some(StakeAuthorization_oneof_validators::deny_list(StakeAuthorization_Validators::new()));
        }
        match self.validators {
            ::std::option::Option::Some(StakeAuthorization_oneof_validators::deny_list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_deny_list(&mut self) -> StakeAuthorization_Validators {
        if self.has_deny_list() {
            match self.validators.take() {
                ::std::option::Option::Some(StakeAuthorization_oneof_validators::deny_list(v)) => v,
                _ => panic!(),
            }
        } else {
            StakeAuthorization_Validators::new()
        }
    }

    // .cosmos.staking.v1beta1.AuthorizationType authorization_type = 4;


    pub fn get_authorization_type(&self) -> AuthorizationType {
        self.authorization_type
    }
    pub fn clear_authorization_type(&mut self) {
        self.authorization_type = AuthorizationType::AUTHORIZATION_TYPE_UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_authorization_type(&mut self, v: AuthorizationType) {
        self.authorization_type = v;
    }
}

impl ::protobuf::Message for StakeAuthorization {
    fn is_initialized(&self) -> bool {
        for v in &self.max_tokens {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(StakeAuthorization_oneof_validators::allow_list(ref v)) = self.validators {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(StakeAuthorization_oneof_validators::deny_list(ref v)) = self.validators {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_tokens)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.validators = ::std::option::Option::Some(StakeAuthorization_oneof_validators::allow_list(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.validators = ::std::option::Option::Some(StakeAuthorization_oneof_validators::deny_list(is.read_message()?));
                },
                4 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.authorization_type, 4, &mut self.unknown_fields)?
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
        if let Some(ref v) = self.max_tokens.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.authorization_type != AuthorizationType::AUTHORIZATION_TYPE_UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(4, self.authorization_type);
        }
        if let ::std::option::Option::Some(ref v) = self.validators {
            match v {
                &StakeAuthorization_oneof_validators::allow_list(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &StakeAuthorization_oneof_validators::deny_list(ref v) => {
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
        if let Some(ref v) = self.max_tokens.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.authorization_type != AuthorizationType::AUTHORIZATION_TYPE_UNSPECIFIED {
            os.write_enum(4, ::protobuf::ProtobufEnum::value(&self.authorization_type))?;
        }
        if let ::std::option::Option::Some(ref v) = self.validators {
            match v {
                &StakeAuthorization_oneof_validators::allow_list(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &StakeAuthorization_oneof_validators::deny_list(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> StakeAuthorization {
        StakeAuthorization::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::coin::Coin>>(
                "max_tokens",
                |m: &StakeAuthorization| { &m.max_tokens },
                |m: &mut StakeAuthorization| { &mut m.max_tokens },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, StakeAuthorization_Validators>(
                "allow_list",
                StakeAuthorization::has_allow_list,
                StakeAuthorization::get_allow_list,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, StakeAuthorization_Validators>(
                "deny_list",
                StakeAuthorization::has_deny_list,
                StakeAuthorization::get_deny_list,
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AuthorizationType>>(
                "authorization_type",
                |m: &StakeAuthorization| { &m.authorization_type },
                |m: &mut StakeAuthorization| { &mut m.authorization_type },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StakeAuthorization>(
                "StakeAuthorization",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StakeAuthorization {
        static instance: ::protobuf::rt::LazyV2<StakeAuthorization> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StakeAuthorization::new)
    }
}

impl ::protobuf::Clear for StakeAuthorization {
    fn clear(&mut self) {
        self.max_tokens.clear();
        self.validators = ::std::option::Option::None;
        self.validators = ::std::option::Option::None;
        self.authorization_type = AuthorizationType::AUTHORIZATION_TYPE_UNSPECIFIED;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StakeAuthorization {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StakeAuthorization {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StakeAuthorization_Validators {
    // message fields
    pub address: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StakeAuthorization_Validators {
    fn default() -> &'a StakeAuthorization_Validators {
        <StakeAuthorization_Validators as ::protobuf::Message>::default_instance()
    }
}

impl StakeAuthorization_Validators {
    pub fn new() -> StakeAuthorization_Validators {
        ::std::default::Default::default()
    }

    // repeated string address = 1;


    pub fn get_address(&self) -> &[::std::string::String] {
        &self.address
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.address = v;
    }

    // Mutable pointer to the field.
    pub fn mut_address(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.address, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for StakeAuthorization_Validators {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.address)?;
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
        for value in &self.address {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.address {
            os.write_string(1, &v)?;
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

    fn new() -> StakeAuthorization_Validators {
        StakeAuthorization_Validators::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "address",
                |m: &StakeAuthorization_Validators| { &m.address },
                |m: &mut StakeAuthorization_Validators| { &mut m.address },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StakeAuthorization_Validators>(
                "StakeAuthorization.Validators",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StakeAuthorization_Validators {
        static instance: ::protobuf::rt::LazyV2<StakeAuthorization_Validators> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StakeAuthorization_Validators::new)
    }
}

impl ::protobuf::Clear for StakeAuthorization_Validators {
    fn clear(&mut self) {
        self.address.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StakeAuthorization_Validators {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StakeAuthorization_Validators {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AuthorizationType {
    AUTHORIZATION_TYPE_UNSPECIFIED = 0,
    AUTHORIZATION_TYPE_DELEGATE = 1,
    AUTHORIZATION_TYPE_UNDELEGATE = 2,
    AUTHORIZATION_TYPE_REDELEGATE = 3,
}

impl ::protobuf::ProtobufEnum for AuthorizationType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AuthorizationType> {
        match value {
            0 => ::std::option::Option::Some(AuthorizationType::AUTHORIZATION_TYPE_UNSPECIFIED),
            1 => ::std::option::Option::Some(AuthorizationType::AUTHORIZATION_TYPE_DELEGATE),
            2 => ::std::option::Option::Some(AuthorizationType::AUTHORIZATION_TYPE_UNDELEGATE),
            3 => ::std::option::Option::Some(AuthorizationType::AUTHORIZATION_TYPE_REDELEGATE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AuthorizationType] = &[
            AuthorizationType::AUTHORIZATION_TYPE_UNSPECIFIED,
            AuthorizationType::AUTHORIZATION_TYPE_DELEGATE,
            AuthorizationType::AUTHORIZATION_TYPE_UNDELEGATE,
            AuthorizationType::AUTHORIZATION_TYPE_REDELEGATE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<AuthorizationType>("AuthorizationType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for AuthorizationType {
}

impl ::std::default::Default for AuthorizationType {
    fn default() -> Self {
        AuthorizationType::AUTHORIZATION_TYPE_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthorizationType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"cosmos/staking/v1beta1/authz.proto\x12\x16cosmos.staking.v1beta1\x1a\
    \x14gogoproto/gogo.proto\x1a\x19cosmos_proto/cosmos.proto\x1a\x1ecosmos/\
    base/v1beta1/coin.proto\"\xcc\x03\n\x12StakeAuthorization\x12e\n\nmax_to\
    kens\x18\x01\x20\x01(\x0b2\x19.cosmos.base.v1beta1.CoinR\tmaxTokensB+\
    \xaa\xdf\x1f'github.com/cosmos/cosmos-sdk/types.Coin\x12V\n\nallow_list\
    \x18\x02\x20\x01(\x0b25.cosmos.staking.v1beta1.StakeAuthorization.Valida\
    torsH\0R\tallowList\x12T\n\tdeny_list\x18\x03\x20\x01(\x0b25.cosmos.stak\
    ing.v1beta1.StakeAuthorization.ValidatorsH\0R\x08denyList\x12X\n\x12auth\
    orization_type\x18\x04\x20\x01(\x0e2).cosmos.staking.v1beta1.Authorizati\
    onTypeR\x11authorizationType\x1a&\n\nValidators\x12\x18\n\x07address\x18\
    \x01\x20\x03(\tR\x07addressB\x0c\n\nvalidators:\x11\xca\xb4-\rAuthorizat\
    ion*\x9e\x01\n\x11AuthorizationType\x12\"\n\x1eAUTHORIZATION_TYPE_UNSPEC\
    IFIED\x10\0\x12\x1f\n\x1bAUTHORIZATION_TYPE_DELEGATE\x10\x01\x12!\n\x1dA\
    UTHORIZATION_TYPE_UNDELEGATE\x10\x02\x12!\n\x1dAUTHORIZATION_TYPE_REDELE\
    GATE\x10\x03B.Z,github.com/cosmos/cosmos-sdk/x/staking/typesb\x06proto3\
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
