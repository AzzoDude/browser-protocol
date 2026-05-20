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
    /// https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml
    /// https://usb.org/developers
    key: i64,
    /// Manufacturer-specific data (Encoded as a base64 string when passed over JSON)
    data: Cow<'a, str>,
}

impl<'a> ManufacturerData<'a> {
    pub fn builder() -> ManufacturerDataBuilder<'a> { ManufacturerDataBuilder::default() }
    pub fn key(&self) -> i64 { self.key }
    pub fn data(&self) -> &str { self.data.as_ref() }
}

#[derive(Default)]
pub struct ManufacturerDataBuilder<'a> {
    key: Option<i64>,
    data: Option<Cow<'a, str>>,
}

impl<'a> ManufacturerDataBuilder<'a> {
    /// Company identifier
    /// https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml
    /// https://usb.org/developers
    pub fn key(mut self, key: i64) -> Self { self.key = Some(key); self }
    /// Manufacturer-specific data (Encoded as a base64 string when passed over JSON)
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    pub fn build(self) -> ManufacturerData<'a> {
        ManufacturerData {
            key: self.key.unwrap_or_default(),
            data: self.data.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    txPower: Option<i64>,
    /// Key is the company identifier and the value is an array of bytes of
    /// manufacturer specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturerData: Option<Vec<ManufacturerData<'a>>>,
}

impl<'a> ScanRecord<'a> {
    pub fn builder() -> ScanRecordBuilder<'a> { ScanRecordBuilder::default() }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    pub fn uuids(&self) -> Option<&[Cow<'a, str>]> { self.uuids.as_deref() }
    pub fn appearance(&self) -> Option<i64> { self.appearance }
    pub fn txPower(&self) -> Option<i64> { self.txPower }
    pub fn manufacturerData(&self) -> Option<&[ManufacturerData<'a>]> { self.manufacturerData.as_deref() }
}

#[derive(Default)]
pub struct ScanRecordBuilder<'a> {
    name: Option<Cow<'a, str>>,
    uuids: Option<Vec<Cow<'a, str>>>,
    appearance: Option<i64>,
    txPower: Option<i64>,
    manufacturerData: Option<Vec<ManufacturerData<'a>>>,
}

impl<'a> ScanRecordBuilder<'a> {
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn uuids(mut self, uuids: Vec<Cow<'a, str>>) -> Self { self.uuids = Some(uuids); self }
    /// Stores the external appearance description of the device.
    pub fn appearance(mut self, appearance: i64) -> Self { self.appearance = Some(appearance); self }
    /// Stores the transmission power of a broadcasting device.
    pub fn txPower(mut self, txPower: i64) -> Self { self.txPower = Some(txPower); self }
    /// Key is the company identifier and the value is an array of bytes of
    /// manufacturer specific data.
    pub fn manufacturerData(mut self, manufacturerData: Vec<ManufacturerData<'a>>) -> Self { self.manufacturerData = Some(manufacturerData); self }
    pub fn build(self) -> ScanRecord<'a> {
        ScanRecord {
            name: self.name,
            uuids: self.uuids,
            appearance: self.appearance,
            txPower: self.txPower,
            manufacturerData: self.manufacturerData,
        }
    }
}

/// Stores the advertisement packet information that is sent by a Bluetooth device.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScanEntry<'a> {
    deviceAddress: Cow<'a, str>,
    rssi: i64,
    scanRecord: ScanRecord<'a>,
}

impl<'a> ScanEntry<'a> {
    pub fn builder() -> ScanEntryBuilder<'a> { ScanEntryBuilder::default() }
    pub fn deviceAddress(&self) -> &str { self.deviceAddress.as_ref() }
    pub fn rssi(&self) -> i64 { self.rssi }
    pub fn scanRecord(&self) -> &ScanRecord<'a> { &self.scanRecord }
}

#[derive(Default)]
pub struct ScanEntryBuilder<'a> {
    deviceAddress: Option<Cow<'a, str>>,
    rssi: Option<i64>,
    scanRecord: Option<ScanRecord<'a>>,
}

impl<'a> ScanEntryBuilder<'a> {
    pub fn deviceAddress(mut self, deviceAddress: impl Into<Cow<'a, str>>) -> Self { self.deviceAddress = Some(deviceAddress.into()); self }
    pub fn rssi(mut self, rssi: i64) -> Self { self.rssi = Some(rssi); self }
    pub fn scanRecord(mut self, scanRecord: ScanRecord<'a>) -> Self { self.scanRecord = Some(scanRecord); self }
    pub fn build(self) -> ScanEntry<'a> {
        ScanEntry {
            deviceAddress: self.deviceAddress.unwrap_or_default(),
            rssi: self.rssi.unwrap_or_default(),
            scanRecord: self.scanRecord.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    writeWithoutResponse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indicate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticatedSignedWrites: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extendedProperties: Option<bool>,
}

impl CharacteristicProperties {
    pub fn builder() -> CharacteristicPropertiesBuilder { CharacteristicPropertiesBuilder::default() }
    pub fn broadcast(&self) -> Option<bool> { self.broadcast }
    pub fn read(&self) -> Option<bool> { self.read }
    pub fn writeWithoutResponse(&self) -> Option<bool> { self.writeWithoutResponse }
    pub fn write(&self) -> Option<bool> { self.write }
    pub fn notify(&self) -> Option<bool> { self.notify }
    pub fn indicate(&self) -> Option<bool> { self.indicate }
    pub fn authenticatedSignedWrites(&self) -> Option<bool> { self.authenticatedSignedWrites }
    pub fn extendedProperties(&self) -> Option<bool> { self.extendedProperties }
}

#[derive(Default)]
pub struct CharacteristicPropertiesBuilder {
    broadcast: Option<bool>,
    read: Option<bool>,
    writeWithoutResponse: Option<bool>,
    write: Option<bool>,
    notify: Option<bool>,
    indicate: Option<bool>,
    authenticatedSignedWrites: Option<bool>,
    extendedProperties: Option<bool>,
}

impl CharacteristicPropertiesBuilder {
    pub fn broadcast(mut self, broadcast: bool) -> Self { self.broadcast = Some(broadcast); self }
    pub fn read(mut self, read: bool) -> Self { self.read = Some(read); self }
    pub fn writeWithoutResponse(mut self, writeWithoutResponse: bool) -> Self { self.writeWithoutResponse = Some(writeWithoutResponse); self }
    pub fn write(mut self, write: bool) -> Self { self.write = Some(write); self }
    pub fn notify(mut self, notify: bool) -> Self { self.notify = Some(notify); self }
    pub fn indicate(mut self, indicate: bool) -> Self { self.indicate = Some(indicate); self }
    pub fn authenticatedSignedWrites(mut self, authenticatedSignedWrites: bool) -> Self { self.authenticatedSignedWrites = Some(authenticatedSignedWrites); self }
    pub fn extendedProperties(mut self, extendedProperties: bool) -> Self { self.extendedProperties = Some(extendedProperties); self }
    pub fn build(self) -> CharacteristicProperties {
        CharacteristicProperties {
            broadcast: self.broadcast,
            read: self.read,
            writeWithoutResponse: self.writeWithoutResponse,
            write: self.write,
            notify: self.notify,
            indicate: self.indicate,
            authenticatedSignedWrites: self.authenticatedSignedWrites,
            extendedProperties: self.extendedProperties,
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
    leSupported: bool,
}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder { EnableParamsBuilder::default() }
    pub fn state(&self) -> &CentralState { &self.state }
    pub fn leSupported(&self) -> bool { self.leSupported }
}

#[derive(Default)]
pub struct EnableParamsBuilder {
    state: Option<CentralState>,
    leSupported: Option<bool>,
}

impl EnableParamsBuilder {
    /// State of the simulated central.
    pub fn state(mut self, state: CentralState) -> Self { self.state = Some(state); self }
    /// If the simulated central supports low-energy.
    pub fn leSupported(mut self, leSupported: bool) -> Self { self.leSupported = Some(leSupported); self }
    pub fn build(self) -> EnableParams {
        EnableParams {
            state: self.state.unwrap_or_default(),
            leSupported: self.leSupported.unwrap_or_default(),
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
    pub fn builder() -> SetSimulatedCentralStateParamsBuilder { SetSimulatedCentralStateParamsBuilder::default() }
    pub fn state(&self) -> &CentralState { &self.state }
}

#[derive(Default)]
pub struct SetSimulatedCentralStateParamsBuilder {
    state: Option<CentralState>,
}

impl SetSimulatedCentralStateParamsBuilder {
    /// State of the simulated central.
    pub fn state(mut self, state: CentralState) -> Self { self.state = Some(state); self }
    pub fn build(self) -> SetSimulatedCentralStateParams {
        SetSimulatedCentralStateParams {
            state: self.state.unwrap_or_default(),
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

impl DisableParams {
    pub fn builder() -> DisableParamsBuilder {
        DisableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct DisableParamsBuilder {}

impl DisableParamsBuilder {
    pub fn build(self) -> DisableParams {
        DisableParams {}
    }
}

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
    manufacturerData: Vec<ManufacturerData<'a>>,
    knownServiceUuids: Vec<Cow<'a, str>>,
}

impl<'a> SimulatePreconnectedPeripheralParams<'a> {
    pub fn builder() -> SimulatePreconnectedPeripheralParamsBuilder<'a> { SimulatePreconnectedPeripheralParamsBuilder::default() }
    pub fn address(&self) -> &str { self.address.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn manufacturerData(&self) -> &[ManufacturerData<'a>] { &self.manufacturerData }
    pub fn knownServiceUuids(&self) -> &[Cow<'a, str>] { &self.knownServiceUuids }
}

#[derive(Default)]
pub struct SimulatePreconnectedPeripheralParamsBuilder<'a> {
    address: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    manufacturerData: Option<Vec<ManufacturerData<'a>>>,
    knownServiceUuids: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SimulatePreconnectedPeripheralParamsBuilder<'a> {
    pub fn address(mut self, address: impl Into<Cow<'a, str>>) -> Self { self.address = Some(address.into()); self }
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn manufacturerData(mut self, manufacturerData: Vec<ManufacturerData<'a>>) -> Self { self.manufacturerData = Some(manufacturerData); self }
    pub fn knownServiceUuids(mut self, knownServiceUuids: Vec<Cow<'a, str>>) -> Self { self.knownServiceUuids = Some(knownServiceUuids); self }
    pub fn build(self) -> SimulatePreconnectedPeripheralParams<'a> {
        SimulatePreconnectedPeripheralParams {
            address: self.address.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            manufacturerData: self.manufacturerData.unwrap_or_default(),
            knownServiceUuids: self.knownServiceUuids.unwrap_or_default(),
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
    pub fn builder() -> SimulateAdvertisementParamsBuilder<'a> { SimulateAdvertisementParamsBuilder::default() }
    pub fn entry(&self) -> &ScanEntry<'a> { &self.entry }
}

#[derive(Default)]
pub struct SimulateAdvertisementParamsBuilder<'a> {
    entry: Option<ScanEntry<'a>>,
}

impl<'a> SimulateAdvertisementParamsBuilder<'a> {
    pub fn entry(mut self, entry: ScanEntry<'a>) -> Self { self.entry = Some(entry); self }
    pub fn build(self) -> SimulateAdvertisementParams<'a> {
        SimulateAdvertisementParams {
            entry: self.entry.unwrap_or_default(),
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
    pub fn builder() -> SimulateGATTOperationResponseParamsBuilder<'a> { SimulateGATTOperationResponseParamsBuilder::default() }
    pub fn address(&self) -> &str { self.address.as_ref() }
    pub fn type_(&self) -> &GATTOperationType { &self.type_ }
    pub fn code(&self) -> i64 { self.code }
}

#[derive(Default)]
pub struct SimulateGATTOperationResponseParamsBuilder<'a> {
    address: Option<Cow<'a, str>>,
    type_: Option<GATTOperationType>,
    code: Option<i64>,
}

impl<'a> SimulateGATTOperationResponseParamsBuilder<'a> {
    pub fn address(mut self, address: impl Into<Cow<'a, str>>) -> Self { self.address = Some(address.into()); self }
    pub fn type_(mut self, type_: GATTOperationType) -> Self { self.type_ = Some(type_); self }
    pub fn code(mut self, code: i64) -> Self { self.code = Some(code); self }
    pub fn build(self) -> SimulateGATTOperationResponseParams<'a> {
        SimulateGATTOperationResponseParams {
            address: self.address.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
            code: self.code.unwrap_or_default(),
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
    characteristicId: Cow<'a, str>,
    #[serde(rename = "type")]
    type_: CharacteristicOperationType,
    code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateCharacteristicOperationResponseParams<'a> {
    pub fn builder() -> SimulateCharacteristicOperationResponseParamsBuilder<'a> { SimulateCharacteristicOperationResponseParamsBuilder::default() }
    pub fn characteristicId(&self) -> &str { self.characteristicId.as_ref() }
    pub fn type_(&self) -> &CharacteristicOperationType { &self.type_ }
    pub fn code(&self) -> i64 { self.code }
    pub fn data(&self) -> Option<&str> { self.data.as_deref() }
}

#[derive(Default)]
pub struct SimulateCharacteristicOperationResponseParamsBuilder<'a> {
    characteristicId: Option<Cow<'a, str>>,
    type_: Option<CharacteristicOperationType>,
    code: Option<i64>,
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateCharacteristicOperationResponseParamsBuilder<'a> {
    pub fn characteristicId(mut self, characteristicId: impl Into<Cow<'a, str>>) -> Self { self.characteristicId = Some(characteristicId.into()); self }
    pub fn type_(mut self, type_: CharacteristicOperationType) -> Self { self.type_ = Some(type_); self }
    pub fn code(mut self, code: i64) -> Self { self.code = Some(code); self }
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    pub fn build(self) -> SimulateCharacteristicOperationResponseParams<'a> {
        SimulateCharacteristicOperationResponseParams {
            characteristicId: self.characteristicId.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
            code: self.code.unwrap_or_default(),
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
    descriptorId: Cow<'a, str>,
    #[serde(rename = "type")]
    type_: DescriptorOperationType,
    code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateDescriptorOperationResponseParams<'a> {
    pub fn builder() -> SimulateDescriptorOperationResponseParamsBuilder<'a> { SimulateDescriptorOperationResponseParamsBuilder::default() }
    pub fn descriptorId(&self) -> &str { self.descriptorId.as_ref() }
    pub fn type_(&self) -> &DescriptorOperationType { &self.type_ }
    pub fn code(&self) -> i64 { self.code }
    pub fn data(&self) -> Option<&str> { self.data.as_deref() }
}

#[derive(Default)]
pub struct SimulateDescriptorOperationResponseParamsBuilder<'a> {
    descriptorId: Option<Cow<'a, str>>,
    type_: Option<DescriptorOperationType>,
    code: Option<i64>,
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateDescriptorOperationResponseParamsBuilder<'a> {
    pub fn descriptorId(mut self, descriptorId: impl Into<Cow<'a, str>>) -> Self { self.descriptorId = Some(descriptorId.into()); self }
    pub fn type_(mut self, type_: DescriptorOperationType) -> Self { self.type_ = Some(type_); self }
    pub fn code(mut self, code: i64) -> Self { self.code = Some(code); self }
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    pub fn build(self) -> SimulateDescriptorOperationResponseParams<'a> {
        SimulateDescriptorOperationResponseParams {
            descriptorId: self.descriptorId.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
            code: self.code.unwrap_or_default(),
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
    serviceUuid: Cow<'a, str>,
}

impl<'a> AddServiceParams<'a> {
    pub fn builder() -> AddServiceParamsBuilder<'a> { AddServiceParamsBuilder::default() }
    pub fn address(&self) -> &str { self.address.as_ref() }
    pub fn serviceUuid(&self) -> &str { self.serviceUuid.as_ref() }
}

#[derive(Default)]
pub struct AddServiceParamsBuilder<'a> {
    address: Option<Cow<'a, str>>,
    serviceUuid: Option<Cow<'a, str>>,
}

impl<'a> AddServiceParamsBuilder<'a> {
    pub fn address(mut self, address: impl Into<Cow<'a, str>>) -> Self { self.address = Some(address.into()); self }
    pub fn serviceUuid(mut self, serviceUuid: impl Into<Cow<'a, str>>) -> Self { self.serviceUuid = Some(serviceUuid.into()); self }
    pub fn build(self) -> AddServiceParams<'a> {
        AddServiceParams {
            address: self.address.unwrap_or_default(),
            serviceUuid: self.serviceUuid.unwrap_or_default(),
        }
    }
}

/// Adds a service with |serviceUuid| to the peripheral with |address|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddServiceReturns<'a> {
    /// An identifier that uniquely represents this service.
    serviceId: Cow<'a, str>,
}

impl<'a> AddServiceReturns<'a> {
    pub fn builder() -> AddServiceReturnsBuilder<'a> { AddServiceReturnsBuilder::default() }
    pub fn serviceId(&self) -> &str { self.serviceId.as_ref() }
}

#[derive(Default)]
pub struct AddServiceReturnsBuilder<'a> {
    serviceId: Option<Cow<'a, str>>,
}

impl<'a> AddServiceReturnsBuilder<'a> {
    /// An identifier that uniquely represents this service.
    pub fn serviceId(mut self, serviceId: impl Into<Cow<'a, str>>) -> Self { self.serviceId = Some(serviceId.into()); self }
    pub fn build(self) -> AddServiceReturns<'a> {
        AddServiceReturns {
            serviceId: self.serviceId.unwrap_or_default(),
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
    serviceId: Cow<'a, str>,
}

impl<'a> RemoveServiceParams<'a> {
    pub fn builder() -> RemoveServiceParamsBuilder<'a> { RemoveServiceParamsBuilder::default() }
    pub fn serviceId(&self) -> &str { self.serviceId.as_ref() }
}

#[derive(Default)]
pub struct RemoveServiceParamsBuilder<'a> {
    serviceId: Option<Cow<'a, str>>,
}

impl<'a> RemoveServiceParamsBuilder<'a> {
    pub fn serviceId(mut self, serviceId: impl Into<Cow<'a, str>>) -> Self { self.serviceId = Some(serviceId.into()); self }
    pub fn build(self) -> RemoveServiceParams<'a> {
        RemoveServiceParams {
            serviceId: self.serviceId.unwrap_or_default(),
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
    serviceId: Cow<'a, str>,
    characteristicUuid: Cow<'a, str>,
    properties: CharacteristicProperties,
}

impl<'a> AddCharacteristicParams<'a> {
    pub fn builder() -> AddCharacteristicParamsBuilder<'a> { AddCharacteristicParamsBuilder::default() }
    pub fn serviceId(&self) -> &str { self.serviceId.as_ref() }
    pub fn characteristicUuid(&self) -> &str { self.characteristicUuid.as_ref() }
    pub fn properties(&self) -> &CharacteristicProperties { &self.properties }
}

#[derive(Default)]
pub struct AddCharacteristicParamsBuilder<'a> {
    serviceId: Option<Cow<'a, str>>,
    characteristicUuid: Option<Cow<'a, str>>,
    properties: Option<CharacteristicProperties>,
}

impl<'a> AddCharacteristicParamsBuilder<'a> {
    pub fn serviceId(mut self, serviceId: impl Into<Cow<'a, str>>) -> Self { self.serviceId = Some(serviceId.into()); self }
    pub fn characteristicUuid(mut self, characteristicUuid: impl Into<Cow<'a, str>>) -> Self { self.characteristicUuid = Some(characteristicUuid.into()); self }
    pub fn properties(mut self, properties: CharacteristicProperties) -> Self { self.properties = Some(properties); self }
    pub fn build(self) -> AddCharacteristicParams<'a> {
        AddCharacteristicParams {
            serviceId: self.serviceId.unwrap_or_default(),
            characteristicUuid: self.characteristicUuid.unwrap_or_default(),
            properties: self.properties.unwrap_or_default(),
        }
    }
}

/// Adds a characteristic with |characteristicUuid| and |properties| to the
/// service represented by |serviceId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddCharacteristicReturns<'a> {
    /// An identifier that uniquely represents this characteristic.
    characteristicId: Cow<'a, str>,
}

impl<'a> AddCharacteristicReturns<'a> {
    pub fn builder() -> AddCharacteristicReturnsBuilder<'a> { AddCharacteristicReturnsBuilder::default() }
    pub fn characteristicId(&self) -> &str { self.characteristicId.as_ref() }
}

#[derive(Default)]
pub struct AddCharacteristicReturnsBuilder<'a> {
    characteristicId: Option<Cow<'a, str>>,
}

impl<'a> AddCharacteristicReturnsBuilder<'a> {
    /// An identifier that uniquely represents this characteristic.
    pub fn characteristicId(mut self, characteristicId: impl Into<Cow<'a, str>>) -> Self { self.characteristicId = Some(characteristicId.into()); self }
    pub fn build(self) -> AddCharacteristicReturns<'a> {
        AddCharacteristicReturns {
            characteristicId: self.characteristicId.unwrap_or_default(),
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
    characteristicId: Cow<'a, str>,
}

impl<'a> RemoveCharacteristicParams<'a> {
    pub fn builder() -> RemoveCharacteristicParamsBuilder<'a> { RemoveCharacteristicParamsBuilder::default() }
    pub fn characteristicId(&self) -> &str { self.characteristicId.as_ref() }
}

#[derive(Default)]
pub struct RemoveCharacteristicParamsBuilder<'a> {
    characteristicId: Option<Cow<'a, str>>,
}

impl<'a> RemoveCharacteristicParamsBuilder<'a> {
    pub fn characteristicId(mut self, characteristicId: impl Into<Cow<'a, str>>) -> Self { self.characteristicId = Some(characteristicId.into()); self }
    pub fn build(self) -> RemoveCharacteristicParams<'a> {
        RemoveCharacteristicParams {
            characteristicId: self.characteristicId.unwrap_or_default(),
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
    characteristicId: Cow<'a, str>,
    descriptorUuid: Cow<'a, str>,
}

impl<'a> AddDescriptorParams<'a> {
    pub fn builder() -> AddDescriptorParamsBuilder<'a> { AddDescriptorParamsBuilder::default() }
    pub fn characteristicId(&self) -> &str { self.characteristicId.as_ref() }
    pub fn descriptorUuid(&self) -> &str { self.descriptorUuid.as_ref() }
}

#[derive(Default)]
pub struct AddDescriptorParamsBuilder<'a> {
    characteristicId: Option<Cow<'a, str>>,
    descriptorUuid: Option<Cow<'a, str>>,
}

impl<'a> AddDescriptorParamsBuilder<'a> {
    pub fn characteristicId(mut self, characteristicId: impl Into<Cow<'a, str>>) -> Self { self.characteristicId = Some(characteristicId.into()); self }
    pub fn descriptorUuid(mut self, descriptorUuid: impl Into<Cow<'a, str>>) -> Self { self.descriptorUuid = Some(descriptorUuid.into()); self }
    pub fn build(self) -> AddDescriptorParams<'a> {
        AddDescriptorParams {
            characteristicId: self.characteristicId.unwrap_or_default(),
            descriptorUuid: self.descriptorUuid.unwrap_or_default(),
        }
    }
}

/// Adds a descriptor with |descriptorUuid| to the characteristic respresented
/// by |characteristicId|.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddDescriptorReturns<'a> {
    /// An identifier that uniquely represents this descriptor.
    descriptorId: Cow<'a, str>,
}

impl<'a> AddDescriptorReturns<'a> {
    pub fn builder() -> AddDescriptorReturnsBuilder<'a> { AddDescriptorReturnsBuilder::default() }
    pub fn descriptorId(&self) -> &str { self.descriptorId.as_ref() }
}

#[derive(Default)]
pub struct AddDescriptorReturnsBuilder<'a> {
    descriptorId: Option<Cow<'a, str>>,
}

impl<'a> AddDescriptorReturnsBuilder<'a> {
    /// An identifier that uniquely represents this descriptor.
    pub fn descriptorId(mut self, descriptorId: impl Into<Cow<'a, str>>) -> Self { self.descriptorId = Some(descriptorId.into()); self }
    pub fn build(self) -> AddDescriptorReturns<'a> {
        AddDescriptorReturns {
            descriptorId: self.descriptorId.unwrap_or_default(),
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
    descriptorId: Cow<'a, str>,
}

impl<'a> RemoveDescriptorParams<'a> {
    pub fn builder() -> RemoveDescriptorParamsBuilder<'a> { RemoveDescriptorParamsBuilder::default() }
    pub fn descriptorId(&self) -> &str { self.descriptorId.as_ref() }
}

#[derive(Default)]
pub struct RemoveDescriptorParamsBuilder<'a> {
    descriptorId: Option<Cow<'a, str>>,
}

impl<'a> RemoveDescriptorParamsBuilder<'a> {
    pub fn descriptorId(mut self, descriptorId: impl Into<Cow<'a, str>>) -> Self { self.descriptorId = Some(descriptorId.into()); self }
    pub fn build(self) -> RemoveDescriptorParams<'a> {
        RemoveDescriptorParams {
            descriptorId: self.descriptorId.unwrap_or_default(),
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
    pub fn builder() -> SimulateGATTDisconnectionParamsBuilder<'a> { SimulateGATTDisconnectionParamsBuilder::default() }
    pub fn address(&self) -> &str { self.address.as_ref() }
}

#[derive(Default)]
pub struct SimulateGATTDisconnectionParamsBuilder<'a> {
    address: Option<Cow<'a, str>>,
}

impl<'a> SimulateGATTDisconnectionParamsBuilder<'a> {
    pub fn address(mut self, address: impl Into<Cow<'a, str>>) -> Self { self.address = Some(address.into()); self }
    pub fn build(self) -> SimulateGATTDisconnectionParams<'a> {
        SimulateGATTDisconnectionParams {
            address: self.address.unwrap_or_default(),
        }
    }
}

impl<'a> SimulateGATTDisconnectionParams<'a> { pub const METHOD: &'static str = "BluetoothEmulation.simulateGATTDisconnection"; }

impl<'a> crate::CdpCommand<'a> for SimulateGATTDisconnectionParams<'a> {
    const METHOD: &'static str = "BluetoothEmulation.simulateGATTDisconnection";
    type Response = crate::EmptyReturns;
}
