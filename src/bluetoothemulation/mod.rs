//! This domain allows configuring virtual Bluetooth devices to test
//! the web-bluetooth API.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Indicates the various states of Central.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CentralState {
    #[default]
    #[serde(rename = "absent")]
    Absent,
    #[serde(rename = "powered-off")]
    PoweredOff,
    #[serde(rename = "powered-on")]
    PoweredOn,
}

/// Indicates the various types of GATT event.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum GATTOperationType {
    #[default]
    #[serde(rename = "connection")]
    Connection,
    #[serde(rename = "discovery")]
    Discovery,
}

/// Indicates the various types of characteristic write.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CharacteristicWriteType {
    #[default]
    #[serde(rename = "write-default-deprecated")]
    WriteDefaultDeprecated,
    #[serde(rename = "write-with-response")]
    WriteWithResponse,
    #[serde(rename = "write-without-response")]
    WriteWithoutResponse,
}

/// Indicates the various types of characteristic operation.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CharacteristicOperationType {
    #[default]
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "subscribe-to-notifications")]
    SubscribeToNotifications,
    #[serde(rename = "unsubscribe-from-notifications")]
    UnsubscribeFromNotifications,
}

/// Indicates the various types of descriptor operation.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DescriptorOperationType {
    #[default]
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

/// Stores the manufacturer data

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturerData<'a> {
    /// Company identifier
    /// <https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml>
    /// <https://usb.org/developers>
    pub key: i64,
    /// Manufacturer-specific data (Encoded as a base64 string when passed over JSON)
    pub data: Cow<'a, str>,
}
/// Stores the byte data of the advertisement packet sent by a Bluetooth device.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScanRecord<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuids: Option<Vec<Cow<'a, str>>>,
    /// Stores the external appearance description of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appearance: Option<i64>,
    /// Stores the transmission power of a broadcasting device.
    #[serde(skip_serializing_if = "Option::is_none", rename = "txPower")]
    pub tx_power: Option<i64>,
    /// Key is the company identifier and the value is an array of bytes of
    /// manufacturer specific data.
    #[serde(skip_serializing_if = "Option::is_none", rename = "manufacturerData")]
    pub manufacturer_data: Option<Vec<ManufacturerData<'a>>>,
}
/// Stores the advertisement packet information that is sent by a Bluetooth device.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScanEntry<'a> {
    #[serde(rename = "deviceAddress")]
    pub device_address: Cow<'a, str>,
    pub rssi: i64,
    #[serde(rename = "scanRecord")]
    pub scan_record: ScanRecord<'a>,
}
/// Describes the properties of a characteristic. This follows Bluetooth Core
/// Specification BT 4.2 Vol 3 Part G 3.3.1. Characteristic Properties.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "writeWithoutResponse")]
    pub write_without_response: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "authenticatedSignedWrites")]
    pub authenticated_signed_writes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "extendedProperties")]
    pub extended_properties: Option<bool>,
}
/// Enable the BluetoothEmulation domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.enable")]
pub struct EnableParams {
    /// State of the simulated central.
    pub state: CentralState,
    /// If the simulated central supports low-energy.
    #[serde(rename = "leSupported")]
    pub le_supported: bool,
}
/// Set the state of the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.setSimulatedCentralState")]
pub struct SetSimulatedCentralStateParams {
    /// State of the simulated central.
    pub state: CentralState,
}
/// Disable the BluetoothEmulation domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.disable")]
pub struct DisableParams {

}
/// Simulates a peripheral with |address|, |name| and |knownServiceUuids|
/// that has already been connected to the system.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.simulatePreconnectedPeripheral")]
pub struct SimulatePreconnectedPeripheralParams<'a> {
    pub address: Cow<'a, str>,
    pub name: Cow<'a, str>,
    #[serde(rename = "manufacturerData")]
    pub manufacturer_data: Vec<ManufacturerData<'a>>,
    #[serde(rename = "knownServiceUuids")]
    pub known_service_uuids: Vec<Cow<'a, str>>,
}
/// Simulates an advertisement packet described in |entry| being received by
/// the central.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.simulateAdvertisement")]
pub struct SimulateAdvertisementParams<'a> {
    pub entry: ScanEntry<'a>,
}
/// Simulates the response code from the peripheral with |address| for a
/// GATT operation of |type|. The |code| value follows the HCI Error Codes from
/// Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.simulateGATTOperationResponse")]
pub struct SimulateGATTOperationResponseParams<'a> {
    pub address: Cow<'a, str>,
    #[serde(rename = "type")]
    pub type_: GATTOperationType,
    pub code: i64,
}
/// Simulates the response from the characteristic with |characteristicId| for a
/// characteristic operation of |type|. The |code| value follows the Error
/// Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
/// The |data| is expected to exist when simulating a successful read operation
/// response.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.simulateCharacteristicOperationResponse")]
pub struct SimulateCharacteristicOperationResponseParams<'a> {
    #[serde(rename = "characteristicId")]
    pub characteristic_id: Cow<'a, str>,
    #[serde(rename = "type")]
    pub type_: CharacteristicOperationType,
    pub code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Cow<'a, str>>,
}
/// Simulates the response from the descriptor with |descriptorId| for a
/// descriptor operation of |type|. The |code| value follows the Error
/// Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
/// The |data| is expected to exist when simulating a successful read operation
/// response.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.simulateDescriptorOperationResponse")]
pub struct SimulateDescriptorOperationResponseParams<'a> {
    #[serde(rename = "descriptorId")]
    pub descriptor_id: Cow<'a, str>,
    #[serde(rename = "type")]
    pub type_: DescriptorOperationType,
    pub code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Cow<'a, str>>,
}
/// Adds a service with |serviceUuid| to the peripheral with |address|.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.addService", response = "AddServiceReturns<'a>")]
pub struct AddServiceParams<'a> {
    pub address: Cow<'a, str>,
    #[serde(rename = "serviceUuid")]
    pub service_uuid: Cow<'a, str>,
}
/// Adds a service with |serviceUuid| to the peripheral with |address|.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddServiceReturns<'a> {
    /// An identifier that uniquely represents this service.
    #[serde(rename = "serviceId")]
    pub service_id: Cow<'a, str>,
}
/// Removes the service respresented by |serviceId| from the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.removeService")]
pub struct RemoveServiceParams<'a> {
    #[serde(rename = "serviceId")]
    pub service_id: Cow<'a, str>,
}
/// Adds a characteristic with |characteristicUuid| and |properties| to the
/// service represented by |serviceId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.addCharacteristic", response = "AddCharacteristicReturns<'a>")]
pub struct AddCharacteristicParams<'a> {
    #[serde(rename = "serviceId")]
    pub service_id: Cow<'a, str>,
    #[serde(rename = "characteristicUuid")]
    pub characteristic_uuid: Cow<'a, str>,
    pub properties: CharacteristicProperties,
}
/// Adds a characteristic with |characteristicUuid| and |properties| to the
/// service represented by |serviceId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddCharacteristicReturns<'a> {
    /// An identifier that uniquely represents this characteristic.
    #[serde(rename = "characteristicId")]
    pub characteristic_id: Cow<'a, str>,
}
/// Removes the characteristic respresented by |characteristicId| from the
/// simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.removeCharacteristic")]
pub struct RemoveCharacteristicParams<'a> {
    #[serde(rename = "characteristicId")]
    pub characteristic_id: Cow<'a, str>,
}
/// Adds a descriptor with |descriptorUuid| to the characteristic respresented
/// by |characteristicId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.addDescriptor", response = "AddDescriptorReturns<'a>")]
pub struct AddDescriptorParams<'a> {
    #[serde(rename = "characteristicId")]
    pub characteristic_id: Cow<'a, str>,
    #[serde(rename = "descriptorUuid")]
    pub descriptor_uuid: Cow<'a, str>,
}
/// Adds a descriptor with |descriptorUuid| to the characteristic respresented
/// by |characteristicId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddDescriptorReturns<'a> {
    /// An identifier that uniquely represents this descriptor.
    #[serde(rename = "descriptorId")]
    pub descriptor_id: Cow<'a, str>,
}
/// Removes the descriptor with |descriptorId| from the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.removeDescriptor")]
pub struct RemoveDescriptorParams<'a> {
    #[serde(rename = "descriptorId")]
    pub descriptor_id: Cow<'a, str>,
}
/// Simulates a GATT disconnection from the peripheral with |address|.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.simulateGATTDisconnection")]
pub struct SimulateGATTDisconnectionParams<'a> {
    pub address: Cow<'a, str>,
}
/// Event for when a GATT operation of |type| to the peripheral with |address|
/// happened.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.gattOperationReceived")]
pub struct GattOperationReceived<'a> {
    pub address: Cow<'a, str>,
    #[serde(rename = "type")]
    pub type_: GATTOperationType,
}
/// Event for when a characteristic operation of |type| to the characteristic
/// respresented by |characteristicId| happened. |data| and |writeType| is
/// expected to exist when |type| is write.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.characteristicOperationReceived")]
pub struct CharacteristicOperationReceived<'a> {
    #[serde(rename = "characteristicId")]
    pub characteristic_id: Cow<'a, str>,
    #[serde(rename = "type")]
    pub type_: CharacteristicOperationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "writeType")]
    pub write_type: Option<CharacteristicWriteType>,
}
/// Event for when a descriptor operation of |type| to the descriptor
/// respresented by |descriptorId| happened. |data| is expected to exist when
/// |type| is write.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "BluetoothEmulation.descriptorOperationReceived")]
pub struct DescriptorOperationReceived<'a> {
    #[serde(rename = "descriptorId")]
    pub descriptor_id: Cow<'a, str>,
    #[serde(rename = "type")]
    pub type_: DescriptorOperationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Cow<'a, str>>,
}