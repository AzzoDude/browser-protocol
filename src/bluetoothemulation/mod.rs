use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! This domain allows configuring virtual Bluetooth devices to test
//! the web-bluetooth API.

/// Indicates the various states of Central.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CentralState {
    #[default]
    Absent,
    PoweredOff,
    PoweredOn,
}

/// Indicates the various types of GATT event.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum GATTOperationType {
    #[default]
    Connection,
    Discovery,
}

/// Indicates the various types of characteristic write.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CharacteristicWriteType {
    #[default]
    WriteDefaultDeprecated,
    WriteWithResponse,
    WriteWithoutResponse,
}

/// Indicates the various types of characteristic operation.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CharacteristicOperationType {
    #[default]
    Read,
    Write,
    SubscribeToNotifications,
    UnsubscribeFromNotifications,
}

/// Indicates the various types of descriptor operation.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DescriptorOperationType {
    #[default]
    Read,
    Write,
}

/// Stores the manufacturer data

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturerData {
    /// Company identifier
    /// https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml
    /// https://usb.org/developers

    pub key: i64,
    /// Manufacturer-specific data (Encoded as a base64 string when passed over JSON)

    pub data: String,
}

/// Stores the byte data of the advertisement packet sent by a Bluetooth device.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScanRecord {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuids: Option<Vec<String>>,
    /// Stores the external appearance description of the device.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub appearance: Option<i64>,
    /// Stores the transmission power of a broadcasting device.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txPower: Option<i64>,
    /// Key is the company identifier and the value is an array of bytes of
    /// manufacturer specific data.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturerData: Option<Vec<ManufacturerData>>,
}

/// Stores the advertisement packet information that is sent by a Bluetooth device.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScanEntry {

    pub deviceAddress: String,

    pub rssi: i64,

    pub scanRecord: ScanRecord,
}

/// Describes the properties of a characteristic. This follows Bluetooth Core
/// Specification BT 4.2 Vol 3 Part G 3.3.1. Characteristic Properties.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicProperties {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub writeWithoutResponse: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicate: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticatedSignedWrites: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extendedProperties: Option<bool>,
}

/// Enable the BluetoothEmulation domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// State of the simulated central.

    pub state: CentralState,
    /// If the simulated central supports low-energy.

    pub leSupported: bool,
}

impl EnableParams { pub const METHOD: &'static str = "BluetoothEmulation.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "BluetoothEmulation.enable";
    type Response = crate::EmptyReturns;
}

/// Set the state of the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSimulatedCentralStateParams {
    /// State of the simulated central.

    pub state: CentralState,
}

impl SetSimulatedCentralStateParams { pub const METHOD: &'static str = "BluetoothEmulation.setSimulatedCentralState"; }

impl crate::CdpCommand for SetSimulatedCentralStateParams {
    const METHOD: &'static str = "BluetoothEmulation.setSimulatedCentralState";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "BluetoothEmulation.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "BluetoothEmulation.disable";
    type Response = crate::EmptyReturns;
}

/// Simulates a peripheral with |address|, |name| and |knownServiceUuids|
/// that has already been connected to the system.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePreconnectedPeripheralParams {

    pub address: String,

    pub name: String,

    pub manufacturerData: Vec<ManufacturerData>,

    pub knownServiceUuids: Vec<String>,
}

impl SimulatePreconnectedPeripheralParams { pub const METHOD: &'static str = "BluetoothEmulation.simulatePreconnectedPeripheral"; }

impl crate::CdpCommand for SimulatePreconnectedPeripheralParams {
    const METHOD: &'static str = "BluetoothEmulation.simulatePreconnectedPeripheral";
    type Response = crate::EmptyReturns;
}

/// Simulates an advertisement packet described in |entry| being received by
/// the central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulateAdvertisementParams {

    pub entry: ScanEntry,
}

impl SimulateAdvertisementParams { pub const METHOD: &'static str = "BluetoothEmulation.simulateAdvertisement"; }

impl crate::CdpCommand for SimulateAdvertisementParams {
    const METHOD: &'static str = "BluetoothEmulation.simulateAdvertisement";
    type Response = crate::EmptyReturns;
}

/// Simulates the response code from the peripheral with |address| for a
/// GATT operation of |type|. The |code| value follows the HCI Error Codes from
/// Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulateGATTOperationResponseParams {

    pub address: String,

    #[serde(rename = "type")]
    pub type_: GATTOperationType,

    pub code: i64,
}

impl SimulateGATTOperationResponseParams { pub const METHOD: &'static str = "BluetoothEmulation.simulateGATTOperationResponse"; }

impl crate::CdpCommand for SimulateGATTOperationResponseParams {
    const METHOD: &'static str = "BluetoothEmulation.simulateGATTOperationResponse";
    type Response = crate::EmptyReturns;
}

/// Simulates the response from the characteristic with |characteristicId| for a
/// characteristic operation of |type|. The |code| value follows the Error
/// Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
/// The |data| is expected to exist when simulating a successful read operation
/// response.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulateCharacteristicOperationResponseParams {

    pub characteristicId: String,

    #[serde(rename = "type")]
    pub type_: CharacteristicOperationType,

    pub code: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

impl SimulateCharacteristicOperationResponseParams { pub const METHOD: &'static str = "BluetoothEmulation.simulateCharacteristicOperationResponse"; }

impl crate::CdpCommand for SimulateCharacteristicOperationResponseParams {
    const METHOD: &'static str = "BluetoothEmulation.simulateCharacteristicOperationResponse";
    type Response = crate::EmptyReturns;
}

/// Simulates the response from the descriptor with |descriptorId| for a
/// descriptor operation of |type|. The |code| value follows the Error
/// Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
/// The |data| is expected to exist when simulating a successful read operation
/// response.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulateDescriptorOperationResponseParams {

    pub descriptorId: String,

    #[serde(rename = "type")]
    pub type_: DescriptorOperationType,

    pub code: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

impl SimulateDescriptorOperationResponseParams { pub const METHOD: &'static str = "BluetoothEmulation.simulateDescriptorOperationResponse"; }

impl crate::CdpCommand for SimulateDescriptorOperationResponseParams {
    const METHOD: &'static str = "BluetoothEmulation.simulateDescriptorOperationResponse";
    type Response = crate::EmptyReturns;
}

/// Adds a service with |serviceUuid| to the peripheral with |address|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddServiceParams {

    pub address: String,

    pub serviceUuid: String,
}

/// Adds a service with |serviceUuid| to the peripheral with |address|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddServiceReturns {
    /// An identifier that uniquely represents this service.

    pub serviceId: String,
}

impl AddServiceParams { pub const METHOD: &'static str = "BluetoothEmulation.addService"; }

impl crate::CdpCommand for AddServiceParams {
    const METHOD: &'static str = "BluetoothEmulation.addService";
    type Response = AddServiceReturns;
}

/// Removes the service respresented by |serviceId| from the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveServiceParams {

    pub serviceId: String,
}

impl RemoveServiceParams { pub const METHOD: &'static str = "BluetoothEmulation.removeService"; }

impl crate::CdpCommand for RemoveServiceParams {
    const METHOD: &'static str = "BluetoothEmulation.removeService";
    type Response = crate::EmptyReturns;
}

/// Adds a characteristic with |characteristicUuid| and |properties| to the
/// service represented by |serviceId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddCharacteristicParams {

    pub serviceId: String,

    pub characteristicUuid: String,

    pub properties: CharacteristicProperties,
}

/// Adds a characteristic with |characteristicUuid| and |properties| to the
/// service represented by |serviceId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddCharacteristicReturns {
    /// An identifier that uniquely represents this characteristic.

    pub characteristicId: String,
}

impl AddCharacteristicParams { pub const METHOD: &'static str = "BluetoothEmulation.addCharacteristic"; }

impl crate::CdpCommand for AddCharacteristicParams {
    const METHOD: &'static str = "BluetoothEmulation.addCharacteristic";
    type Response = AddCharacteristicReturns;
}

/// Removes the characteristic respresented by |characteristicId| from the
/// simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCharacteristicParams {

    pub characteristicId: String,
}

impl RemoveCharacteristicParams { pub const METHOD: &'static str = "BluetoothEmulation.removeCharacteristic"; }

impl crate::CdpCommand for RemoveCharacteristicParams {
    const METHOD: &'static str = "BluetoothEmulation.removeCharacteristic";
    type Response = crate::EmptyReturns;
}

/// Adds a descriptor with |descriptorUuid| to the characteristic respresented
/// by |characteristicId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddDescriptorParams {

    pub characteristicId: String,

    pub descriptorUuid: String,
}

/// Adds a descriptor with |descriptorUuid| to the characteristic respresented
/// by |characteristicId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddDescriptorReturns {
    /// An identifier that uniquely represents this descriptor.

    pub descriptorId: String,
}

impl AddDescriptorParams { pub const METHOD: &'static str = "BluetoothEmulation.addDescriptor"; }

impl crate::CdpCommand for AddDescriptorParams {
    const METHOD: &'static str = "BluetoothEmulation.addDescriptor";
    type Response = AddDescriptorReturns;
}

/// Removes the descriptor with |descriptorId| from the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDescriptorParams {

    pub descriptorId: String,
}

impl RemoveDescriptorParams { pub const METHOD: &'static str = "BluetoothEmulation.removeDescriptor"; }

impl crate::CdpCommand for RemoveDescriptorParams {
    const METHOD: &'static str = "BluetoothEmulation.removeDescriptor";
    type Response = crate::EmptyReturns;
}

/// Simulates a GATT disconnection from the peripheral with |address|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulateGATTDisconnectionParams {

    pub address: String,
}

impl SimulateGATTDisconnectionParams { pub const METHOD: &'static str = "BluetoothEmulation.simulateGATTDisconnection"; }

impl crate::CdpCommand for SimulateGATTDisconnectionParams {
    const METHOD: &'static str = "BluetoothEmulation.simulateGATTDisconnection";
    type Response = crate::EmptyReturns;
}
