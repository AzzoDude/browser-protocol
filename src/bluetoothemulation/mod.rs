//! This domain allows configuring virtual Bluetooth devices to test
//! the web-bluetooth API.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturerData<'a> {
    /// Company identifier
    /// <https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml>
    /// <https://usb.org/developers>
    key: i64,
    /// Manufacturer-specific data (Encoded as a base64 string when passed over JSON)
    data: Cow<'a, str>,
}

impl<'a> ManufacturerData<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `key`: Company identifier <https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml> <https://usb.org/developers>
    /// * `data`: Manufacturer-specific data (Encoded as a base64 string when passed over JSON)
    pub fn builder(key: i64, data: impl Into<Cow<'a, str>>) -> ManufacturerDataBuilder<'a> {
        ManufacturerDataBuilder {
            key: key,
            data: data.into(),
        }
    }
    /// Company identifier
    /// <https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml>
    /// <https://usb.org/developers>
    pub fn key(&self) -> i64 { self.key }
    /// Manufacturer-specific data (Encoded as a base64 string when passed over JSON)
    pub fn data(&self) -> &str { self.data.as_ref() }
}


pub struct ManufacturerDataBuilder<'a> {
    key: i64,
    data: Cow<'a, str>,
}

impl<'a> ManufacturerDataBuilder<'a> {
    pub fn build(self) -> ManufacturerData<'a> {
        ManufacturerData {
            key: self.key,
            data: self.data,
        }
    }
}

/// Stores the byte data of the advertisement packet sent by a Bluetooth device.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScanRecord<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uuids: Option<Vec<Cow<'a, str>>>,
    /// Stores the external appearance description of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    appearance: Option<i64>,
    /// Stores the transmission power of a broadcasting device.
    #[serde(skip_serializing_if = "Option::is_none", rename = "txPower")]
    tx_power: Option<i64>,
    /// Key is the company identifier and the value is an array of bytes of
    /// manufacturer specific data.
    #[serde(skip_serializing_if = "Option::is_none", rename = "manufacturerData")]
    manufacturer_data: Option<Vec<ManufacturerData<'a>>>,
}

impl<'a> ScanRecord<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> ScanRecordBuilder<'a> {
        ScanRecordBuilder {
            name: None,
            uuids: None,
            appearance: None,
            tx_power: None,
            manufacturer_data: None,
        }
    }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    pub fn uuids(&self) -> Option<&[Cow<'a, str>]> { self.uuids.as_deref() }
    /// Stores the external appearance description of the device.
    pub fn appearance(&self) -> Option<i64> { self.appearance }
    /// Stores the transmission power of a broadcasting device.
    pub fn tx_power(&self) -> Option<i64> { self.tx_power }
    /// Key is the company identifier and the value is an array of bytes of
    /// manufacturer specific data.
    pub fn manufacturer_data(&self) -> Option<&[ManufacturerData<'a>]> { self.manufacturer_data.as_deref() }
}

#[derive(Default)]
pub struct ScanRecordBuilder<'a> {
    name: Option<Cow<'a, str>>,
    uuids: Option<Vec<Cow<'a, str>>>,
    appearance: Option<i64>,
    tx_power: Option<i64>,
    manufacturer_data: Option<Vec<ManufacturerData<'a>>>,
}

impl<'a> ScanRecordBuilder<'a> {
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn uuids(mut self, uuids: Vec<Cow<'a, str>>) -> Self { self.uuids = Some(uuids); self }
    /// Stores the external appearance description of the device.
    pub fn appearance(mut self, appearance: i64) -> Self { self.appearance = Some(appearance); self }
    /// Stores the transmission power of a broadcasting device.
    pub fn tx_power(mut self, tx_power: i64) -> Self { self.tx_power = Some(tx_power); self }
    /// Key is the company identifier and the value is an array of bytes of
    /// manufacturer specific data.
    pub fn manufacturer_data(mut self, manufacturer_data: Vec<ManufacturerData<'a>>) -> Self { self.manufacturer_data = Some(manufacturer_data); self }
    pub fn build(self) -> ScanRecord<'a> {
        ScanRecord {
            name: self.name,
            uuids: self.uuids,
            appearance: self.appearance,
            tx_power: self.tx_power,
            manufacturer_data: self.manufacturer_data,
        }
    }
}

/// Stores the advertisement packet information that is sent by a Bluetooth device.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScanEntry<'a> {
    #[serde(rename = "deviceAddress")]
    device_address: Cow<'a, str>,
    rssi: i64,
    #[serde(rename = "scanRecord")]
    scan_record: ScanRecord<'a>,
}

impl<'a> ScanEntry<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `device_address`: 
    /// * `rssi`: 
    /// * `scan_record`: 
    pub fn builder(device_address: impl Into<Cow<'a, str>>, rssi: i64, scan_record: ScanRecord<'a>) -> ScanEntryBuilder<'a> {
        ScanEntryBuilder {
            device_address: device_address.into(),
            rssi: rssi,
            scan_record: scan_record,
        }
    }
    pub fn device_address(&self) -> &str { self.device_address.as_ref() }
    pub fn rssi(&self) -> i64 { self.rssi }
    pub fn scan_record(&self) -> &ScanRecord<'a> { &self.scan_record }
}


pub struct ScanEntryBuilder<'a> {
    device_address: Cow<'a, str>,
    rssi: i64,
    scan_record: ScanRecord<'a>,
}

impl<'a> ScanEntryBuilder<'a> {
    pub fn build(self) -> ScanEntry<'a> {
        ScanEntry {
            device_address: self.device_address,
            rssi: self.rssi,
            scan_record: self.scan_record,
        }
    }
}

/// Describes the properties of a characteristic. This follows Bluetooth Core
/// Specification BT 4.2 Vol 3 Part G 3.3.1. Characteristic Properties.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "writeWithoutResponse")]
    write_without_response: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indicate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "authenticatedSignedWrites")]
    authenticated_signed_writes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "extendedProperties")]
    extended_properties: Option<bool>,
}

impl CharacteristicProperties {
    /// Creates a builder for this type.
    pub fn builder() -> CharacteristicPropertiesBuilder {
        CharacteristicPropertiesBuilder {
            broadcast: None,
            read: None,
            write_without_response: None,
            write: None,
            notify: None,
            indicate: None,
            authenticated_signed_writes: None,
            extended_properties: None,
        }
    }
    pub fn broadcast(&self) -> Option<bool> { self.broadcast }
    pub fn read(&self) -> Option<bool> { self.read }
    pub fn write_without_response(&self) -> Option<bool> { self.write_without_response }
    pub fn write(&self) -> Option<bool> { self.write }
    pub fn notify(&self) -> Option<bool> { self.notify }
    pub fn indicate(&self) -> Option<bool> { self.indicate }
    pub fn authenticated_signed_writes(&self) -> Option<bool> { self.authenticated_signed_writes }
    pub fn extended_properties(&self) -> Option<bool> { self.extended_properties }
}

#[derive(Default)]
pub struct CharacteristicPropertiesBuilder {
    broadcast: Option<bool>,
    read: Option<bool>,
    write_without_response: Option<bool>,
    write: Option<bool>,
    notify: Option<bool>,
    indicate: Option<bool>,
    authenticated_signed_writes: Option<bool>,
    extended_properties: Option<bool>,
}

impl CharacteristicPropertiesBuilder {
    pub fn broadcast(mut self, broadcast: bool) -> Self { self.broadcast = Some(broadcast); self }
    pub fn read(mut self, read: bool) -> Self { self.read = Some(read); self }
    pub fn write_without_response(mut self, write_without_response: bool) -> Self { self.write_without_response = Some(write_without_response); self }
    pub fn write(mut self, write: bool) -> Self { self.write = Some(write); self }
    pub fn notify(mut self, notify: bool) -> Self { self.notify = Some(notify); self }
    pub fn indicate(mut self, indicate: bool) -> Self { self.indicate = Some(indicate); self }
    pub fn authenticated_signed_writes(mut self, authenticated_signed_writes: bool) -> Self { self.authenticated_signed_writes = Some(authenticated_signed_writes); self }
    pub fn extended_properties(mut self, extended_properties: bool) -> Self { self.extended_properties = Some(extended_properties); self }
    pub fn build(self) -> CharacteristicProperties {
        CharacteristicProperties {
            broadcast: self.broadcast,
            read: self.read,
            write_without_response: self.write_without_response,
            write: self.write,
            notify: self.notify,
            indicate: self.indicate,
            authenticated_signed_writes: self.authenticated_signed_writes,
            extended_properties: self.extended_properties,
        }
    }
}

/// Enable the BluetoothEmulation domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// State of the simulated central.
    state: CentralState,
    /// If the simulated central supports low-energy.
    #[serde(rename = "leSupported")]
    le_supported: bool,
}

impl EnableParams {
    /// Creates a builder for this type with the required parameters:
    /// * `state`: State of the simulated central.
    /// * `le_supported`: If the simulated central supports low-energy.
    pub fn builder(state: impl Into<CentralState>, le_supported: bool) -> EnableParamsBuilder {
        EnableParamsBuilder {
            state: state.into(),
            le_supported: le_supported,
        }
    }
    /// State of the simulated central.
    pub fn state(&self) -> &CentralState { &self.state }
    /// If the simulated central supports low-energy.
    pub fn le_supported(&self) -> bool { self.le_supported }
}


pub struct EnableParamsBuilder {
    state: CentralState,
    le_supported: bool,
}

impl EnableParamsBuilder {
    pub fn build(self) -> EnableParams {
        EnableParams {
            state: self.state,
            le_supported: self.le_supported,
        }
    }
}

impl EnableParams { pub const METHOD: &'static str = "BluetoothEmulation.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "BluetoothEmulation.enable";
    type Response = crate::EmptyReturns;
}

/// Set the state of the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSimulatedCentralStateParams {
    /// State of the simulated central.
    state: CentralState,
}

impl SetSimulatedCentralStateParams {
    /// Creates a builder for this type with the required parameters:
    /// * `state`: State of the simulated central.
    pub fn builder(state: impl Into<CentralState>) -> SetSimulatedCentralStateParamsBuilder {
        SetSimulatedCentralStateParamsBuilder {
            state: state.into(),
        }
    }
    /// State of the simulated central.
    pub fn state(&self) -> &CentralState { &self.state }
}


pub struct SetSimulatedCentralStateParamsBuilder {
    state: CentralState,
}

impl SetSimulatedCentralStateParamsBuilder {
    pub fn build(self) -> SetSimulatedCentralStateParams {
        SetSimulatedCentralStateParams {
            state: self.state,
        }
    }
}

impl SetSimulatedCentralStateParams { pub const METHOD: &'static str = "BluetoothEmulation.setSimulatedCentralState"; }

impl<'a> crate::CdpCommand<'a> for SetSimulatedCentralStateParams {
    const METHOD: &'static str = "BluetoothEmulation.setSimulatedCentralState";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "BluetoothEmulation.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "BluetoothEmulation.disable";
    type Response = crate::EmptyReturns;
}

/// Simulates a peripheral with |address|, |name| and |knownServiceUuids|
/// that has already been connected to the system.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePreconnectedPeripheralParams<'a> {
    address: Cow<'a, str>,
    name: Cow<'a, str>,
    #[serde(rename = "manufacturerData")]
    manufacturer_data: Vec<ManufacturerData<'a>>,
    #[serde(rename = "knownServiceUuids")]
    known_service_uuids: Vec<Cow<'a, str>>,
}

impl<'a> SimulatePreconnectedPeripheralParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `address`: 
    /// * `name`: 
    /// * `manufacturer_data`: 
    /// * `known_service_uuids`: 
    pub fn builder(address: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, manufacturer_data: Vec<ManufacturerData<'a>>, known_service_uuids: Vec<Cow<'a, str>>) -> SimulatePreconnectedPeripheralParamsBuilder<'a> {
        SimulatePreconnectedPeripheralParamsBuilder {
            address: address.into(),
            name: name.into(),
            manufacturer_data: manufacturer_data,
            known_service_uuids: known_service_uuids,
        }
    }
    pub fn address(&self) -> &str { self.address.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn manufacturer_data(&self) -> &[ManufacturerData<'a>] { &self.manufacturer_data }
    pub fn known_service_uuids(&self) -> &[Cow<'a, str>] { &self.known_service_uuids }
}


pub struct SimulatePreconnectedPeripheralParamsBuilder<'a> {
    address: Cow<'a, str>,
    name: Cow<'a, str>,
    manufacturer_data: Vec<ManufacturerData<'a>>,
    known_service_uuids: Vec<Cow<'a, str>>,
}

impl<'a> SimulatePreconnectedPeripheralParamsBuilder<'a> {
    pub fn build(self) -> SimulatePreconnectedPeripheralParams<'a> {
        SimulatePreconnectedPeripheralParams {
            address: self.address,
            name: self.name,
            manufacturer_data: self.manufacturer_data,
            known_service_uuids: self.known_service_uuids,
        }
    }
}

impl<'a> SimulatePreconnectedPeripheralParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.simulatePreconnectedPeripheral"; }

impl<'a> crate::CdpCommand<'a> for SimulatePreconnectedPeripheralParams<'a> {
    const METHOD: &'static str = "BluetoothEmulation.simulatePreconnectedPeripheral";
    type Response = crate::EmptyReturns;
}

/// Simulates an advertisement packet described in |entry| being received by
/// the central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulateAdvertisementParams<'a> {
    entry: ScanEntry<'a>,
}

impl<'a> SimulateAdvertisementParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `entry`: 
    pub fn builder(entry: ScanEntry<'a>) -> SimulateAdvertisementParamsBuilder<'a> {
        SimulateAdvertisementParamsBuilder {
            entry: entry,
        }
    }
    pub fn entry(&self) -> &ScanEntry<'a> { &self.entry }
}


pub struct SimulateAdvertisementParamsBuilder<'a> {
    entry: ScanEntry<'a>,
}

impl<'a> SimulateAdvertisementParamsBuilder<'a> {
    pub fn build(self) -> SimulateAdvertisementParams<'a> {
        SimulateAdvertisementParams {
            entry: self.entry,
        }
    }
}

impl<'a> SimulateAdvertisementParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.simulateAdvertisement"; }

impl<'a> crate::CdpCommand<'a> for SimulateAdvertisementParams<'a> {
    const METHOD: &'static str = "BluetoothEmulation.simulateAdvertisement";
    type Response = crate::EmptyReturns;
}

/// Simulates the response code from the peripheral with |address| for a
/// GATT operation of |type|. The |code| value follows the HCI Error Codes from
/// Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulateGATTOperationResponseParams<'a> {
    address: Cow<'a, str>,
    #[serde(rename = "type")]
    type_: GATTOperationType,
    code: i64,
}

impl<'a> SimulateGATTOperationResponseParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `address`: 
    /// * `type_`: 
    /// * `code`: 
    pub fn builder(address: impl Into<Cow<'a, str>>, type_: impl Into<GATTOperationType>, code: i64) -> SimulateGATTOperationResponseParamsBuilder<'a> {
        SimulateGATTOperationResponseParamsBuilder {
            address: address.into(),
            type_: type_.into(),
            code: code,
        }
    }
    pub fn address(&self) -> &str { self.address.as_ref() }
    pub fn type_(&self) -> &GATTOperationType { &self.type_ }
    pub fn code(&self) -> i64 { self.code }
}


pub struct SimulateGATTOperationResponseParamsBuilder<'a> {
    address: Cow<'a, str>,
    type_: GATTOperationType,
    code: i64,
}

impl<'a> SimulateGATTOperationResponseParamsBuilder<'a> {
    pub fn build(self) -> SimulateGATTOperationResponseParams<'a> {
        SimulateGATTOperationResponseParams {
            address: self.address,
            type_: self.type_,
            code: self.code,
        }
    }
}

impl<'a> SimulateGATTOperationResponseParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.simulateGATTOperationResponse"; }

impl<'a> crate::CdpCommand<'a> for SimulateGATTOperationResponseParams<'a> {
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
pub struct SimulateCharacteristicOperationResponseParams<'a> {
    #[serde(rename = "characteristicId")]
    characteristic_id: Cow<'a, str>,
    #[serde(rename = "type")]
    type_: CharacteristicOperationType,
    code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateCharacteristicOperationResponseParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `characteristic_id`: 
    /// * `type_`: 
    /// * `code`: 
    pub fn builder(characteristic_id: impl Into<Cow<'a, str>>, type_: impl Into<CharacteristicOperationType>, code: i64) -> SimulateCharacteristicOperationResponseParamsBuilder<'a> {
        SimulateCharacteristicOperationResponseParamsBuilder {
            characteristic_id: characteristic_id.into(),
            type_: type_.into(),
            code: code,
            data: None,
        }
    }
    pub fn characteristic_id(&self) -> &str { self.characteristic_id.as_ref() }
    pub fn type_(&self) -> &CharacteristicOperationType { &self.type_ }
    pub fn code(&self) -> i64 { self.code }
    pub fn data(&self) -> Option<&str> { self.data.as_deref() }
}


pub struct SimulateCharacteristicOperationResponseParamsBuilder<'a> {
    characteristic_id: Cow<'a, str>,
    type_: CharacteristicOperationType,
    code: i64,
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateCharacteristicOperationResponseParamsBuilder<'a> {
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    pub fn build(self) -> SimulateCharacteristicOperationResponseParams<'a> {
        SimulateCharacteristicOperationResponseParams {
            characteristic_id: self.characteristic_id,
            type_: self.type_,
            code: self.code,
            data: self.data,
        }
    }
}

impl<'a> SimulateCharacteristicOperationResponseParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.simulateCharacteristicOperationResponse"; }

impl<'a> crate::CdpCommand<'a> for SimulateCharacteristicOperationResponseParams<'a> {
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
pub struct SimulateDescriptorOperationResponseParams<'a> {
    #[serde(rename = "descriptorId")]
    descriptor_id: Cow<'a, str>,
    #[serde(rename = "type")]
    type_: DescriptorOperationType,
    code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateDescriptorOperationResponseParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `descriptor_id`: 
    /// * `type_`: 
    /// * `code`: 
    pub fn builder(descriptor_id: impl Into<Cow<'a, str>>, type_: impl Into<DescriptorOperationType>, code: i64) -> SimulateDescriptorOperationResponseParamsBuilder<'a> {
        SimulateDescriptorOperationResponseParamsBuilder {
            descriptor_id: descriptor_id.into(),
            type_: type_.into(),
            code: code,
            data: None,
        }
    }
    pub fn descriptor_id(&self) -> &str { self.descriptor_id.as_ref() }
    pub fn type_(&self) -> &DescriptorOperationType { &self.type_ }
    pub fn code(&self) -> i64 { self.code }
    pub fn data(&self) -> Option<&str> { self.data.as_deref() }
}


pub struct SimulateDescriptorOperationResponseParamsBuilder<'a> {
    descriptor_id: Cow<'a, str>,
    type_: DescriptorOperationType,
    code: i64,
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateDescriptorOperationResponseParamsBuilder<'a> {
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    pub fn build(self) -> SimulateDescriptorOperationResponseParams<'a> {
        SimulateDescriptorOperationResponseParams {
            descriptor_id: self.descriptor_id,
            type_: self.type_,
            code: self.code,
            data: self.data,
        }
    }
}

impl<'a> SimulateDescriptorOperationResponseParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.simulateDescriptorOperationResponse"; }

impl<'a> crate::CdpCommand<'a> for SimulateDescriptorOperationResponseParams<'a> {
    const METHOD: &'static str = "BluetoothEmulation.simulateDescriptorOperationResponse";
    type Response = crate::EmptyReturns;
}

/// Adds a service with |serviceUuid| to the peripheral with |address|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddServiceParams<'a> {
    address: Cow<'a, str>,
    #[serde(rename = "serviceUuid")]
    service_uuid: Cow<'a, str>,
}

impl<'a> AddServiceParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `address`: 
    /// * `service_uuid`: 
    pub fn builder(address: impl Into<Cow<'a, str>>, service_uuid: impl Into<Cow<'a, str>>) -> AddServiceParamsBuilder<'a> {
        AddServiceParamsBuilder {
            address: address.into(),
            service_uuid: service_uuid.into(),
        }
    }
    pub fn address(&self) -> &str { self.address.as_ref() }
    pub fn service_uuid(&self) -> &str { self.service_uuid.as_ref() }
}


pub struct AddServiceParamsBuilder<'a> {
    address: Cow<'a, str>,
    service_uuid: Cow<'a, str>,
}

impl<'a> AddServiceParamsBuilder<'a> {
    pub fn build(self) -> AddServiceParams<'a> {
        AddServiceParams {
            address: self.address,
            service_uuid: self.service_uuid,
        }
    }
}

/// Adds a service with |serviceUuid| to the peripheral with |address|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddServiceReturns<'a> {
    /// An identifier that uniquely represents this service.
    #[serde(rename = "serviceId")]
    service_id: Cow<'a, str>,
}

impl<'a> AddServiceReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `service_id`: An identifier that uniquely represents this service.
    pub fn builder(service_id: impl Into<Cow<'a, str>>) -> AddServiceReturnsBuilder<'a> {
        AddServiceReturnsBuilder {
            service_id: service_id.into(),
        }
    }
    /// An identifier that uniquely represents this service.
    pub fn service_id(&self) -> &str { self.service_id.as_ref() }
}


pub struct AddServiceReturnsBuilder<'a> {
    service_id: Cow<'a, str>,
}

impl<'a> AddServiceReturnsBuilder<'a> {
    pub fn build(self) -> AddServiceReturns<'a> {
        AddServiceReturns {
            service_id: self.service_id,
        }
    }
}

impl<'a> AddServiceParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.addService"; }

impl<'a> crate::CdpCommand<'a> for AddServiceParams<'a> {
    const METHOD: &'static str = "BluetoothEmulation.addService";
    type Response = AddServiceReturns<'a>;
}

/// Removes the service respresented by |serviceId| from the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveServiceParams<'a> {
    #[serde(rename = "serviceId")]
    service_id: Cow<'a, str>,
}

impl<'a> RemoveServiceParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `service_id`: 
    pub fn builder(service_id: impl Into<Cow<'a, str>>) -> RemoveServiceParamsBuilder<'a> {
        RemoveServiceParamsBuilder {
            service_id: service_id.into(),
        }
    }
    pub fn service_id(&self) -> &str { self.service_id.as_ref() }
}


pub struct RemoveServiceParamsBuilder<'a> {
    service_id: Cow<'a, str>,
}

impl<'a> RemoveServiceParamsBuilder<'a> {
    pub fn build(self) -> RemoveServiceParams<'a> {
        RemoveServiceParams {
            service_id: self.service_id,
        }
    }
}

impl<'a> RemoveServiceParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.removeService"; }

impl<'a> crate::CdpCommand<'a> for RemoveServiceParams<'a> {
    const METHOD: &'static str = "BluetoothEmulation.removeService";
    type Response = crate::EmptyReturns;
}

/// Adds a characteristic with |characteristicUuid| and |properties| to the
/// service represented by |serviceId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddCharacteristicParams<'a> {
    #[serde(rename = "serviceId")]
    service_id: Cow<'a, str>,
    #[serde(rename = "characteristicUuid")]
    characteristic_uuid: Cow<'a, str>,
    properties: CharacteristicProperties,
}

impl<'a> AddCharacteristicParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `service_id`: 
    /// * `characteristic_uuid`: 
    /// * `properties`: 
    pub fn builder(service_id: impl Into<Cow<'a, str>>, characteristic_uuid: impl Into<Cow<'a, str>>, properties: CharacteristicProperties) -> AddCharacteristicParamsBuilder<'a> {
        AddCharacteristicParamsBuilder {
            service_id: service_id.into(),
            characteristic_uuid: characteristic_uuid.into(),
            properties: properties,
        }
    }
    pub fn service_id(&self) -> &str { self.service_id.as_ref() }
    pub fn characteristic_uuid(&self) -> &str { self.characteristic_uuid.as_ref() }
    pub fn properties(&self) -> &CharacteristicProperties { &self.properties }
}


pub struct AddCharacteristicParamsBuilder<'a> {
    service_id: Cow<'a, str>,
    characteristic_uuid: Cow<'a, str>,
    properties: CharacteristicProperties,
}

impl<'a> AddCharacteristicParamsBuilder<'a> {
    pub fn build(self) -> AddCharacteristicParams<'a> {
        AddCharacteristicParams {
            service_id: self.service_id,
            characteristic_uuid: self.characteristic_uuid,
            properties: self.properties,
        }
    }
}

/// Adds a characteristic with |characteristicUuid| and |properties| to the
/// service represented by |serviceId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddCharacteristicReturns<'a> {
    /// An identifier that uniquely represents this characteristic.
    #[serde(rename = "characteristicId")]
    characteristic_id: Cow<'a, str>,
}

impl<'a> AddCharacteristicReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `characteristic_id`: An identifier that uniquely represents this characteristic.
    pub fn builder(characteristic_id: impl Into<Cow<'a, str>>) -> AddCharacteristicReturnsBuilder<'a> {
        AddCharacteristicReturnsBuilder {
            characteristic_id: characteristic_id.into(),
        }
    }
    /// An identifier that uniquely represents this characteristic.
    pub fn characteristic_id(&self) -> &str { self.characteristic_id.as_ref() }
}


pub struct AddCharacteristicReturnsBuilder<'a> {
    characteristic_id: Cow<'a, str>,
}

impl<'a> AddCharacteristicReturnsBuilder<'a> {
    pub fn build(self) -> AddCharacteristicReturns<'a> {
        AddCharacteristicReturns {
            characteristic_id: self.characteristic_id,
        }
    }
}

impl<'a> AddCharacteristicParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.addCharacteristic"; }

impl<'a> crate::CdpCommand<'a> for AddCharacteristicParams<'a> {
    const METHOD: &'static str = "BluetoothEmulation.addCharacteristic";
    type Response = AddCharacteristicReturns<'a>;
}

/// Removes the characteristic respresented by |characteristicId| from the
/// simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCharacteristicParams<'a> {
    #[serde(rename = "characteristicId")]
    characteristic_id: Cow<'a, str>,
}

impl<'a> RemoveCharacteristicParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `characteristic_id`: 
    pub fn builder(characteristic_id: impl Into<Cow<'a, str>>) -> RemoveCharacteristicParamsBuilder<'a> {
        RemoveCharacteristicParamsBuilder {
            characteristic_id: characteristic_id.into(),
        }
    }
    pub fn characteristic_id(&self) -> &str { self.characteristic_id.as_ref() }
}


pub struct RemoveCharacteristicParamsBuilder<'a> {
    characteristic_id: Cow<'a, str>,
}

impl<'a> RemoveCharacteristicParamsBuilder<'a> {
    pub fn build(self) -> RemoveCharacteristicParams<'a> {
        RemoveCharacteristicParams {
            characteristic_id: self.characteristic_id,
        }
    }
}

impl<'a> RemoveCharacteristicParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.removeCharacteristic"; }

impl<'a> crate::CdpCommand<'a> for RemoveCharacteristicParams<'a> {
    const METHOD: &'static str = "BluetoothEmulation.removeCharacteristic";
    type Response = crate::EmptyReturns;
}

/// Adds a descriptor with |descriptorUuid| to the characteristic respresented
/// by |characteristicId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddDescriptorParams<'a> {
    #[serde(rename = "characteristicId")]
    characteristic_id: Cow<'a, str>,
    #[serde(rename = "descriptorUuid")]
    descriptor_uuid: Cow<'a, str>,
}

impl<'a> AddDescriptorParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `characteristic_id`: 
    /// * `descriptor_uuid`: 
    pub fn builder(characteristic_id: impl Into<Cow<'a, str>>, descriptor_uuid: impl Into<Cow<'a, str>>) -> AddDescriptorParamsBuilder<'a> {
        AddDescriptorParamsBuilder {
            characteristic_id: characteristic_id.into(),
            descriptor_uuid: descriptor_uuid.into(),
        }
    }
    pub fn characteristic_id(&self) -> &str { self.characteristic_id.as_ref() }
    pub fn descriptor_uuid(&self) -> &str { self.descriptor_uuid.as_ref() }
}


pub struct AddDescriptorParamsBuilder<'a> {
    characteristic_id: Cow<'a, str>,
    descriptor_uuid: Cow<'a, str>,
}

impl<'a> AddDescriptorParamsBuilder<'a> {
    pub fn build(self) -> AddDescriptorParams<'a> {
        AddDescriptorParams {
            characteristic_id: self.characteristic_id,
            descriptor_uuid: self.descriptor_uuid,
        }
    }
}

/// Adds a descriptor with |descriptorUuid| to the characteristic respresented
/// by |characteristicId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddDescriptorReturns<'a> {
    /// An identifier that uniquely represents this descriptor.
    #[serde(rename = "descriptorId")]
    descriptor_id: Cow<'a, str>,
}

impl<'a> AddDescriptorReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `descriptor_id`: An identifier that uniquely represents this descriptor.
    pub fn builder(descriptor_id: impl Into<Cow<'a, str>>) -> AddDescriptorReturnsBuilder<'a> {
        AddDescriptorReturnsBuilder {
            descriptor_id: descriptor_id.into(),
        }
    }
    /// An identifier that uniquely represents this descriptor.
    pub fn descriptor_id(&self) -> &str { self.descriptor_id.as_ref() }
}


pub struct AddDescriptorReturnsBuilder<'a> {
    descriptor_id: Cow<'a, str>,
}

impl<'a> AddDescriptorReturnsBuilder<'a> {
    pub fn build(self) -> AddDescriptorReturns<'a> {
        AddDescriptorReturns {
            descriptor_id: self.descriptor_id,
        }
    }
}

impl<'a> AddDescriptorParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.addDescriptor"; }

impl<'a> crate::CdpCommand<'a> for AddDescriptorParams<'a> {
    const METHOD: &'static str = "BluetoothEmulation.addDescriptor";
    type Response = AddDescriptorReturns<'a>;
}

/// Removes the descriptor with |descriptorId| from the simulated central.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDescriptorParams<'a> {
    #[serde(rename = "descriptorId")]
    descriptor_id: Cow<'a, str>,
}

impl<'a> RemoveDescriptorParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `descriptor_id`: 
    pub fn builder(descriptor_id: impl Into<Cow<'a, str>>) -> RemoveDescriptorParamsBuilder<'a> {
        RemoveDescriptorParamsBuilder {
            descriptor_id: descriptor_id.into(),
        }
    }
    pub fn descriptor_id(&self) -> &str { self.descriptor_id.as_ref() }
}


pub struct RemoveDescriptorParamsBuilder<'a> {
    descriptor_id: Cow<'a, str>,
}

impl<'a> RemoveDescriptorParamsBuilder<'a> {
    pub fn build(self) -> RemoveDescriptorParams<'a> {
        RemoveDescriptorParams {
            descriptor_id: self.descriptor_id,
        }
    }
}

impl<'a> RemoveDescriptorParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.removeDescriptor"; }

impl<'a> crate::CdpCommand<'a> for RemoveDescriptorParams<'a> {
    const METHOD: &'static str = "BluetoothEmulation.removeDescriptor";
    type Response = crate::EmptyReturns;
}

/// Simulates a GATT disconnection from the peripheral with |address|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulateGATTDisconnectionParams<'a> {
    address: Cow<'a, str>,
}

impl<'a> SimulateGATTDisconnectionParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `address`: 
    pub fn builder(address: impl Into<Cow<'a, str>>) -> SimulateGATTDisconnectionParamsBuilder<'a> {
        SimulateGATTDisconnectionParamsBuilder {
            address: address.into(),
        }
    }
    pub fn address(&self) -> &str { self.address.as_ref() }
}


pub struct SimulateGATTDisconnectionParamsBuilder<'a> {
    address: Cow<'a, str>,
}

impl<'a> SimulateGATTDisconnectionParamsBuilder<'a> {
    pub fn build(self) -> SimulateGATTDisconnectionParams<'a> {
        SimulateGATTDisconnectionParams {
            address: self.address,
        }
    }
}

impl<'a> SimulateGATTDisconnectionParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.simulateGATTDisconnection"; }

impl<'a> crate::CdpCommand<'a> for SimulateGATTDisconnectionParams<'a> {
    const METHOD: &'static str = "BluetoothEmulation.simulateGATTDisconnection";
    type Response = crate::EmptyReturns;
}
