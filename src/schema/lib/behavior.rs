// Handwritten operational behavior for the authority-verified owner Mirror Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// supplies only current-stage behavior: structural traits over the ordinary
// producer's shared representation, readable Dotos roles, and the allocated
// Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};
use signal_standard::schema::lib::{WireShape, WireShapeError, WireValue};

fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 {
        return Err(WireShapeError);
    }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self {
                Self(payload)
            }
            pub fn payload(&self) -> &$inner {
                &self.0
            }
            pub fn into_payload(self) -> $inner {
                self.0
            }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                self.0.to_wire()
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                Ok(Self(<$inner as WireShape>::from_wire(value)?))
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = 0usize $(+ {
                    let _ = stringify!($field);
                    1usize
                })*;
                #[allow(unused_mut, unused_variables)]
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_newtype!(z2VR1a, z2VXab);
wire_external_newtype!(z2VWZw, signal_mirror::schema::lib::z2Ve8p);
wire_struct!(z2VRXG { field_0: signal_standard::schema::lib::z2VXNY, field_1: z2VQot });
wire_enum!(z2VTJB { unit { 0 => z2VcDy : "AllStores" } unary { 1 => z2VXE8(signal_mirror::schema::lib::z2Ve8p) : "Store" } });
wire_struct!(z2VXxR {  });
wire_external_newtype!(z2VPES, std::string::String);
wire_enum!(z2VWt5 { unit {  } unary { 0 => z2VXXR(z2VWZw) : "RetireStore", 1 => z2VWC2(z2VWBn) : "RegisterStore", 2 => z2VNSh(z2VR1a) : "Configure", 3 => z2VMxr(z2VXxR) : "ObserveRegistry", 4 => z2Vctc(z2VXLU) : "SetRetention" } });
wire_external_newtype!(z2VbpU, signal_mirror::schema::lib::z2Ve8p);
wire_external_newtype!(z2VT8Z, signal_mirror::schema::lib::z2Ve8p);
wire_external_newtype!(z2VQot, u64);
wire_newtype!(z2VUYq, z2VRXG);
wire_external_newtype!(z2VQWA, Vec< z2VT8Z>);
wire_newtype!(z2VM9q, z2VRXG);
wire_enum!(z2VYSE { unit { 0 => z2VLwk : "StoreUnknown", 1 => z2VMiY : "LedgerFault", 2 => z2VY7i : "StoreAlreadyRegistered", 3 => z2VPC3 : "StoreNameInvalid" } unary {  } });
wire_enum!(z2VWXC { unit { 1 => z2VaQ8 : "KeepEverything" } unary { 0 => z2VXbW(z2VQrL) : "KeepLatestCheckpoints" } });
wire_struct!(z2VXLU { field_0: z2VTJB, field_1: z2VWXC });
wire_external_newtype!(z2Vcin, signal_mirror::schema::lib::z2Ve8p);
wire_enum!(z2VUH6 { unit {  } unary { 0 => z2VSig(z2VbpU) : "StoreRegistered", 1 => z2VN4H(z2VQWA) : "RegistryObserved", 2 => z2Vedh(z2VP3W) : "RetentionSet", 3 => z2VXCa(z2Vbcq) : "OrderRejected", 4 => z2VdCD(z2Vcin) : "StoreRetired", 5 => z2VQdi(z2VNz1) : "Configured" } });
wire_newtype!(z2VNz1, z2VXab);
wire_struct!(z2VXab { field_0: z2VPES, field_1: z2VM9q, field_2: z2VUYq, field_3: signal_standard::schema::lib::z2VaVE });
wire_external_newtype!(z2VQrL, u64);
wire_struct!(z2Vbcq { field_0: z2VYSE, field_1: z2VbDA });
wire_struct!(z2VQo2 { field_0: z2VYru, field_1: z2VXab });
wire_external_newtype!(z2VbDA, std::string::String);
wire_struct!(z2VWBn { field_0: signal_mirror::schema::lib::z2Ve8p, field_1: z2VMYP });
wire_enum!(z2VMYP { unit { 0 => z2Vf8Y : "Opaque", 1 => z2VbgN : "SemaVersionedLog" } unary {  } });
wire_struct!(z2VP3W { field_0: z2VTJB, field_1: z2VWXC });
wire_external_newtype!(z2VYru, std::string::String);

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer>
            for signal_standard::schema::lib::ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            signal_standard::schema::lib::ArchivedWireValue:
                RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <signal_standard::schema::lib::ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2VR1a);
archive_root!(z2VWZw);
archive_root!(z2VRXG);
archive_root!(z2VTJB);
archive_root!(z2VXxR);
archive_root!(z2VPES);
archive_root!(z2VWt5);
archive_root!(z2VbpU);
archive_root!(z2VT8Z);
archive_root!(z2VQot);
archive_root!(z2VUYq);
archive_root!(z2VQWA);
archive_root!(z2VM9q);
archive_root!(z2VYSE);
archive_root!(z2VWXC);
archive_root!(z2VXLU);
archive_root!(z2Vcin);
archive_root!(z2VUH6);
archive_root!(z2VNz1);
archive_root!(z2VXab);
archive_root!(z2VQrL);
archive_root!(z2Vbcq);
archive_root!(z2VQo2);
archive_root!(z2VbDA);
archive_root!(z2VWBn);
archive_root!(z2VMYP);
archive_root!(z2VP3W);
archive_root!(z2VYru);


pub enum ContractMarker {}

impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(10) {
            Ok(value) => value,
            Err(_) => panic!("contract ID is allocated"),
        },
        match signal_frame::WireRevision::try_new(2) {
            Ok(value) => value,
            Err(_) => panic!("wire revision is allocated"),
        },
    );
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineRefusalReason {
    Rejected,
    Unavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EngineRefusal {
    pub reason: EngineRefusalReason,
    pub detail: std::string::String,
}

impl EngineRefusal {
    pub fn rejected(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Rejected, detail }
    }

    pub fn unavailable(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Unavailable, detail }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")]
    FrameEncode,
    #[error("failed to decode bound signal frame")]
    ArchiveDecode,
    #[error("unexpected signal frame body")]
    UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")]
    OperationCount { found: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputRoute {
    RetireStore,
    RegisterStore,
    Configure,
    ObserveRegistry,
    SetRetention,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputRoute {
    StoreRegistered,
    RegistryObserved,
    RetentionSet,
    OrderRejected,
    StoreRetired,
    Configured,
}

impl z2VWt5 {
    pub fn route(&self) -> InputRoute {
        match self {
            Self::z2VXXR(_) => InputRoute::RetireStore,
            Self::z2VWC2(_) => InputRoute::RegisterStore,
            Self::z2VNSh(_) => InputRoute::Configure,
            Self::z2VMxr(_) => InputRoute::ObserveRegistry,
            Self::z2Vctc(_) => InputRoute::SetRetention,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(0),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        Frame::new(
            route,
            FrameBody::Request {
                exchange,
                request: signal_frame::Request::from_payload(self),
            },
        )
    }

    pub fn encode_request_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl z2VUH6 {
    pub fn route(&self) -> OutputRoute {
        match self {
            Self::z2VSig(_) => OutputRoute::StoreRegistered,
            Self::z2VN4H(_) => OutputRoute::RegistryObserved,
            Self::z2Vedh(_) => OutputRoute::RetentionSet,
            Self::z2VXCa(_) => OutputRoute::OrderRejected,
            Self::z2VdCD(_) => OutputRoute::StoreRetired,
            Self::z2VQdi(_) => OutputRoute::Configured,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(1),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        let reply = signal_frame::Reply::committed(
            signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self)),
        );
        Frame::new(route, FrameBody::Reply { exchange, reply })
    }

    pub fn encode_reply_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_reply_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl signal_frame::RequestPayload for z2VWt5 {}

impl signal_frame::SignalOperationHeads for z2VWt5 {
    const HEADS: &'static [&'static str] = &["RetireStore", "RegisterStore", "Configure", "ObserveRegistry", "SetRetention"];
}

impl signal_frame::LogVariant for z2VWt5 {
    fn log_variant(&self) -> u64 {
        let route = self.wire_route();
        u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8)
    }
}

pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, z2VWt5, z2VUH6>;
pub type FrameBody = signal_frame::ExchangeFrameBody<z2VWt5, z2VUH6>;
pub type Request = signal_frame::Request<z2VWt5>;
pub type ReplyEnvelope = signal_frame::Reply<z2VUH6>;
pub type RequestBuilder = signal_frame::RequestBuilder<z2VWt5>;

impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> {
        Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode)
    }

    pub fn decode_single_request(
        bytes: &[u8],
    ) -> Result<(signal_frame::ExchangeIdentifier, z2VWt5), SignalFrameError> {
        match Self::decode_frame(bytes)?.into_body() {
            FrameBody::Request { exchange, request } => {
                let found = request.payloads().len();
                if found != 1 {
                    return Err(SignalFrameError::OperationCount { found });
                }
                Ok((exchange, request.payloads.into_head()))
            }
            _ => Err(SignalFrameError::UnexpectedFrameBody),
        }
    }
}
