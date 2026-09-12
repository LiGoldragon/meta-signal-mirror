#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type ConfigureRequest = DaemonConfiguration;
#[rustfmt::skip]
pub type StoreRetirement = signal_mirror::StoreName;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LocalSocketBinding {
    pub socket_path: signal::SocketPath,
    pub socket_mode: SocketMode,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RetentionScope {
    AllStores,
    Store(signal_mirror::StoreName),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RegistryQuery {}
#[rustfmt::skip]
pub type StoragePath = String;
#[rustfmt::skip]
pub type RegistrationReceipt = signal_mirror::StoreName;
#[rustfmt::skip]
pub type RetirementReceipt = signal_mirror::StoreName;
#[rustfmt::skip]
pub type RegisteredStore = signal_mirror::StoreName;
#[rustfmt::skip]
pub type SocketMode = i64;
#[rustfmt::skip]
pub type MetaSocketBinding = LocalSocketBinding;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RegistryListing {
    pub registered_store_vector: std::vec::Vec<RegisteredStore>,
}
#[rustfmt::skip]
pub type WorkingSocketBinding = LocalSocketBinding;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum OrderRejectionReason {
    StoreUnknown,
    LedgerFault,
    StoreAlreadyRegistered,
    StoreNameInvalid,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RetentionRule {
    KeepLatestCheckpoints(CheckpointKeepCount),
    KeepEverything,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RetentionOrder {
    pub retention_scope: RetentionScope,
    pub retention_rule: RetentionRule,
}
#[rustfmt::skip]
pub type ConfigurationReceipt = DaemonConfiguration;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DaemonConfiguration {
    pub storage_path: StoragePath,
    pub working_socket_binding: WorkingSocketBinding,
    pub meta_socket_binding: MetaSocketBinding,
    pub network_endpoint: signal::NetworkEndpoint,
}
#[rustfmt::skip]
pub type CheckpointKeepCount = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OrderRejection {
    pub order_rejection_reason: OrderRejectionReason,
    pub rejection_detail: RejectionDetail,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigurationWrite {
    pub configuration_archive_path: ConfigurationArchivePath,
    pub daemon_configuration: DaemonConfiguration,
}
#[rustfmt::skip]
pub type RejectionDetail = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StoreRegistration {
    pub store_name: signal_mirror::StoreName,
    pub content_addressing: ContentAddressing,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ContentAddressing {
    Opaque,
    SemaVersionedLog,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RetentionReceipt {
    pub retention_scope: RetentionScope,
    pub retention_rule: RetentionRule,
}
#[rustfmt::skip]
pub type ConfigurationArchivePath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    RetireStore(StoreRetirement),
    RegisterStore(StoreRegistration),
    Configure(ConfigureRequest),
    ObserveRegistry(RegistryQuery),
    SetRetention(RetentionOrder),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    StoreRegistered(RegistrationReceipt),
    RegistryObserved(RegistryListing),
    RetentionSet(RetentionReceipt),
    OrderRejected(OrderRejection),
    StoreRetired(RetirementReceipt),
    Configured(ConfigurationReceipt),
}
