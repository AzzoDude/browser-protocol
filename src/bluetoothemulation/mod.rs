//! This domain allows configuring virtual Bluetooth devices to test
//! the web-bluetooth API.

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

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
    /// <https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml>
    /// <https://usb.org/developers>

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

/// Set the state of the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSimulatedCentralStateParams {
    /// State of the simulated central.

    pub state: CentralState,
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

/// Simulates an advertisement packet described in |entry| being received by
/// the central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulateAdvertisementParams {

    pub entry: ScanEntry,
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

/// Removes the service respresented by |serviceId| from the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveServiceParams {

    pub serviceId: String,
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

/// Removes the characteristic respresented by |characteristicId| from the
/// simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCharacteristicParams {

    pub characteristicId: String,
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

/// Removes the descriptor with |descriptorId| from the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDescriptorParams {

    pub descriptorId: String,
}

/// Simulates a GATT disconnection from the peripheral with |address|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulateGATTDisconnectionParams {

    pub address: String,
}
