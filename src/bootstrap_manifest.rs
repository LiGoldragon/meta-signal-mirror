//! Explicit producer-owned bootstrap authority state for the owner Mirror Interface.
//!
//! Every identity and canonical-order value below is an already-minted opaque
//! seat. None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    122, 170, 232, 9, 239, 42, 110, 115, 216, 171, 94, 12, 232, 55, 178, 119, 149, 52, 162, 152,
    250, 169, 154, 26, 139, 215, 135, 75, 212, 131, 33, 126,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 12001;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 25372;

pub const INTERFACE_SEAT: AuthoritySeat = AuthoritySeat::new("Interface", 8264, 0x2eeacf51523612fc);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 5747, 0xc71146ad699d4fbb);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 41949, 0xb4d095caba333205);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 29388, 0x2cc0a7e37bac27a0);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 16743, 0xca9caac0af99fa0d);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 6295, 0x3718304fd0419895);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 40166, 0x208c4aacac76c8cb);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 18080, 0x08dad4652aff94fd);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 217, 0xacfa66bb9a5ad61c);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 6394, 0xca492ea7a69aa845);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 53237, 0x4508f14b171c6d91);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 11969, 0x889ab491f5cd612c);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 35431, 0x02dba3aef9f10445);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 8187, 0x2edd562aaec726bb);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 64541, 0x97a923fd7d7ad413);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 4990, 0x279c94b97694c70a);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    15181, 16242, 9332, 63683, 5233, 34532, 54257, 57150, 36261, 22108,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "StoragePath", 11667, 0x35fd210e28d17917),
    DeclarationSeat::new(None, "SocketMode", 16971, 0x6287a0852e021a60),
    DeclarationSeat::new(None, "LocalSocketBinding", 19371, 0x0cbe39c390f9e193),
    DeclarationSeat::new(None, "WorkingSocketBinding", 4672, 0x829195e3c2ff635c),
    DeclarationSeat::new(None, "MetaSocketBinding", 29554, 0x6cb76653f416aee0),
    DeclarationSeat::new(None, "DaemonConfiguration", 39748, 0xd7cede4e2cc44672),
    DeclarationSeat::new(None, "ConfigureRequest", 17649, 0x0b06405a5023536b),
    DeclarationSeat::new(None, "ConfigurationReceipt", 10830, 0xd4d0bb41f3e2638d),
    DeclarationSeat::new(None, "ContentAddressing", 5980, 0xeff284e5d8bd6468),
    DeclarationSeat::new(Some(5980), "Opaque", 65149, 0x3922ce6004831334),
    DeclarationSeat::new(Some(5980), "SemaVersionedLog", 53539, 0x4eea9578f1465cc0),
    DeclarationSeat::new(None, "StoreRegistration", 35061, 0xe957ff0090826d28),
    DeclarationSeat::new(None, "RegistrationReceipt", 54009, 0x5d8e0f85e8390c7c),
    DeclarationSeat::new(None, "StoreRetirement", 36346, 0x0bf919b0b99cf349),
    DeclarationSeat::new(None, "RetirementReceipt", 57043, 0xadd2ebc49e530166),
    DeclarationSeat::new(None, "CheckpointKeepCount", 17113, 0xd8a048030d82b707),
    DeclarationSeat::new(None, "RetentionScope", 25340, 0x1618c52fa10c3d5d),
    DeclarationSeat::new(Some(25340), "Store", 38561, 0x99c70a29e0d8f19a),
    DeclarationSeat::new(Some(25340), "AllStores", 55372, 0x1382432fbb6123f2),
    DeclarationSeat::new(None, "RetentionRule", 36187, 0xa166aa5d78c3a095),
    DeclarationSeat::new(Some(36187), "KeepEverything", 49233, 0x64ba84d7090f5b93),
    DeclarationSeat::new(
        Some(36187),
        "KeepLatestCheckpoints",
        39801,
        0x1f1d8800433ebf31,
    ),
    DeclarationSeat::new(None, "RetentionOrder", 38929, 0xa18b3f9094585ab0),
    DeclarationSeat::new(None, "RetentionReceipt", 11033, 0xf52a7bc4b46b2d7d),
    DeclarationSeat::new(None, "RegistryQuery", 41014, 0x2869d2d1f62bad62),
    DeclarationSeat::new(None, "RegisteredStore", 24782, 0x60e6712b7bc93970),
    DeclarationSeat::new(None, "RegistryListing", 15943, 0x73cc9d2ad9bbf593),
    DeclarationSeat::new(None, "OrderRejectionReason", 42627, 0x82dee1177d6bcee0),
    DeclarationSeat::new(Some(42627), "StoreUnknown", 3971, 0x2a0f85cd309123fa),
    DeclarationSeat::new(
        Some(42627),
        "StoreAlreadyRegistered",
        41553,
        0xa3d95fb96d950e8a,
    ),
    DeclarationSeat::new(Some(42627), "StoreNameInvalid", 11528, 0xc3505c83cce8256c),
    DeclarationSeat::new(Some(42627), "LedgerFault", 6569, 0x59ca8bfdd7c7575e),
    DeclarationSeat::new(None, "RejectionDetail", 51961, 0xe5b466243664f434),
    DeclarationSeat::new(None, "OrderRejection", 53334, 0xdb207ddf90a7cc58),
    DeclarationSeat::new(None, "ConfigurationArchivePath", 44058, 0xfda09483a05ed500),
    DeclarationSeat::new(None, "ConfigurationWrite", 16921, 0xdf24dfb2cdf859b9),
    DeclarationSeat::new(None, "MetaMirrorRequest", 37398, 0x55b31ddcdca93377),
    DeclarationSeat::new(Some(37398), "Configure", 9014, 0x9306dfafe5f4b169),
    DeclarationSeat::new(Some(37398), "RegisterStore", 35075, 0x2c50a0d66a19f72f),
    DeclarationSeat::new(Some(37398), "RetireStore", 39564, 0x0e27e8b963a5d2cb),
    DeclarationSeat::new(Some(37398), "SetRetention", 57613, 0xf0594ae22646c94a),
    DeclarationSeat::new(Some(37398), "ObserveRegistry", 7399, 0xca8aeab76d972fa8),
    DeclarationSeat::new(None, "MetaMirrorReply", 28641, 0xbbc6c50ed4eee80f),
    DeclarationSeat::new(Some(28641), "Configured", 16381, 0xffcbea6a52967d03),
    DeclarationSeat::new(Some(28641), "StoreRegistered", 23397, 0x0243418b0a5c703d),
    DeclarationSeat::new(Some(28641), "StoreRetired", 58634, 0xef5201968041909d),
    DeclarationSeat::new(Some(28641), "RetentionSet", 63476, 0x601aad1af64af823),
    DeclarationSeat::new(Some(28641), "RegistryObserved", 7714, 0x58eec6695d0ad66f),
    DeclarationSeat::new(Some(28641), "OrderRejected", 38471, 0xe4da715f1680b3a0),
];
