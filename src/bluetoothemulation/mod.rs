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
    pub fn builder(key: i64, data: impl Into<Cow<'a, str>>) -> ManufacturerDataBuilder<'a> {
        ManufacturerDataBuilder {
            key: key,
            data: data.into(),
        }
    }
    pub fn key(&self) -> i64 { self.key }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    txPower: Option<i64>,
    /// Key is the company identifier and the value is an array of bytes of
    /// manufacturer specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturerData: Option<Vec<ManufacturerData<'a>>>,
}

impl<'a> ScanRecord<'a> {
    pub fn builder() -> ScanRecordBuilder<'a> {
        ScanRecordBuilder {
            name: None,
            uuids: None,
            appearance: None,
            txPower: None,
            manufacturerData: None,
        }
    }
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
    pub fn builder(deviceAddress: impl Into<Cow<'a, str>>, rssi: i64, scanRecord: ScanRecord<'a>) -> ScanEntryBuilder<'a> {
        ScanEntryBuilder {
            deviceAddress: deviceAddress.into(),
            rssi: rssi,
            scanRecord: scanRecord,
        }
    }
    pub fn deviceAddress(&self) -> &str { self.deviceAddress.as_ref() }
    pub fn rssi(&self) -> i64 { self.rssi }
    pub fn scanRecord(&self) -> &ScanRecord<'a> { &self.scanRecord }
}


pub struct ScanEntryBuilder<'a> {
    deviceAddress: Cow<'a, str>,
    rssi: i64,
    scanRecord: ScanRecord<'a>,
}

impl<'a> ScanEntryBuilder<'a> {
    pub fn build(self) -> ScanEntry<'a> {
        ScanEntry {
            deviceAddress: self.deviceAddress,
            rssi: self.rssi,
            scanRecord: self.scanRecord,
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
    pub fn builder() -> CharacteristicPropertiesBuilder {
        CharacteristicPropertiesBuilder {
            broadcast: None,
            read: None,
            writeWithoutResponse: None,
            write: None,
            notify: None,
            indicate: None,
            authenticatedSignedWrites: None,
            extendedProperties: None,
        }
    }
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
    pub fn builder(state: impl Into<CentralState>, leSupported: bool) -> EnableParamsBuilder {
        EnableParamsBuilder {
            state: state.into(),
            leSupported: leSupported,
        }
    }
    pub fn state(&self) -> &CentralState { &self.state }
    pub fn leSupported(&self) -> bool { self.leSupported }
}


pub struct EnableParamsBuilder {
    state: CentralState,
    leSupported: bool,
}

impl EnableParamsBuilder {
    pub fn build(self) -> EnableParams {
        EnableParams {
            state: self.state,
            leSupported: self.leSupported,
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
    pub fn builder(state: impl Into<CentralState>) -> SetSimulatedCentralStateParamsBuilder {
        SetSimulatedCentralStateParamsBuilder {
            state: state.into(),
        }
    }
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
    manufacturerData: Vec<ManufacturerData<'a>>,
    knownServiceUuids: Vec<Cow<'a, str>>,
}

impl<'a> SimulatePreconnectedPeripheralParams<'a> {
    pub fn builder(address: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, manufacturerData: Vec<ManufacturerData<'a>>, knownServiceUuids: Vec<Cow<'a, str>>) -> SimulatePreconnectedPeripheralParamsBuilder<'a> {
        SimulatePreconnectedPeripheralParamsBuilder {
            address: address.into(),
            name: name.into(),
            manufacturerData: manufacturerData,
            knownServiceUuids: knownServiceUuids,
        }
    }
    pub fn address(&self) -> &str { self.address.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn manufacturerData(&self) -> &[ManufacturerData<'a>] { &self.manufacturerData }
    pub fn knownServiceUuids(&self) -> &[Cow<'a, str>] { &self.knownServiceUuids }
}


pub struct SimulatePreconnectedPeripheralParamsBuilder<'a> {
    address: Cow<'a, str>,
    name: Cow<'a, str>,
    manufacturerData: Vec<ManufacturerData<'a>>,
    knownServiceUuids: Vec<Cow<'a, str>>,
}

impl<'a> SimulatePreconnectedPeripheralParamsBuilder<'a> {
    pub fn build(self) -> SimulatePreconnectedPeripheralParams<'a> {
        SimulatePreconnectedPeripheralParams {
            address: self.address,
            name: self.name,
            manufacturerData: self.manufacturerData,
            knownServiceUuids: self.knownServiceUuids,
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
    characteristicId: Cow<'a, str>,
    #[serde(rename = "type")]
    type_: CharacteristicOperationType,
    code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateCharacteristicOperationResponseParams<'a> {
    pub fn builder(characteristicId: impl Into<Cow<'a, str>>, type_: impl Into<CharacteristicOperationType>, code: i64) -> SimulateCharacteristicOperationResponseParamsBuilder<'a> {
        SimulateCharacteristicOperationResponseParamsBuilder {
            characteristicId: characteristicId.into(),
            type_: type_.into(),
            code: code,
            data: None,
        }
    }
    pub fn characteristicId(&self) -> &str { self.characteristicId.as_ref() }
    pub fn type_(&self) -> &CharacteristicOperationType { &self.type_ }
    pub fn code(&self) -> i64 { self.code }
    pub fn data(&self) -> Option<&str> { self.data.as_deref() }
}


pub struct SimulateCharacteristicOperationResponseParamsBuilder<'a> {
    characteristicId: Cow<'a, str>,
    type_: CharacteristicOperationType,
    code: i64,
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateCharacteristicOperationResponseParamsBuilder<'a> {
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    pub fn build(self) -> SimulateCharacteristicOperationResponseParams<'a> {
        SimulateCharacteristicOperationResponseParams {
            characteristicId: self.characteristicId,
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
    descriptorId: Cow<'a, str>,
    #[serde(rename = "type")]
    type_: DescriptorOperationType,
    code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateDescriptorOperationResponseParams<'a> {
    pub fn builder(descriptorId: impl Into<Cow<'a, str>>, type_: impl Into<DescriptorOperationType>, code: i64) -> SimulateDescriptorOperationResponseParamsBuilder<'a> {
        SimulateDescriptorOperationResponseParamsBuilder {
            descriptorId: descriptorId.into(),
            type_: type_.into(),
            code: code,
            data: None,
        }
    }
    pub fn descriptorId(&self) -> &str { self.descriptorId.as_ref() }
    pub fn type_(&self) -> &DescriptorOperationType { &self.type_ }
    pub fn code(&self) -> i64 { self.code }
    pub fn data(&self) -> Option<&str> { self.data.as_deref() }
}


pub struct SimulateDescriptorOperationResponseParamsBuilder<'a> {
    descriptorId: Cow<'a, str>,
    type_: DescriptorOperationType,
    code: i64,
    data: Option<Cow<'a, str>>,
}

impl<'a> SimulateDescriptorOperationResponseParamsBuilder<'a> {
    pub fn data(mut self, data: impl Into<Cow<'a, str>>) -> Self { self.data = Some(data.into()); self }
    pub fn build(self) -> SimulateDescriptorOperationResponseParams<'a> {
        SimulateDescriptorOperationResponseParams {
            descriptorId: self.descriptorId,
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
    serviceUuid: Cow<'a, str>,
}

impl<'a> AddServiceParams<'a> {
    pub fn builder(address: impl Into<Cow<'a, str>>, serviceUuid: impl Into<Cow<'a, str>>) -> AddServiceParamsBuilder<'a> {
        AddServiceParamsBuilder {
            address: address.into(),
            serviceUuid: serviceUuid.into(),
        }
    }
    pub fn address(&self) -> &str { self.address.as_ref() }
    pub fn serviceUuid(&self) -> &str { self.serviceUuid.as_ref() }
}


pub struct AddServiceParamsBuilder<'a> {
    address: Cow<'a, str>,
    serviceUuid: Cow<'a, str>,
}

impl<'a> AddServiceParamsBuilder<'a> {
    pub fn build(self) -> AddServiceParams<'a> {
        AddServiceParams {
            address: self.address,
            serviceUuid: self.serviceUuid,
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
    pub fn builder(serviceId: impl Into<Cow<'a, str>>) -> AddServiceReturnsBuilder<'a> {
        AddServiceReturnsBuilder {
            serviceId: serviceId.into(),
        }
    }
    pub fn serviceId(&self) -> &str { self.serviceId.as_ref() }
}


pub struct AddServiceReturnsBuilder<'a> {
    serviceId: Cow<'a, str>,
}

impl<'a> AddServiceReturnsBuilder<'a> {
    pub fn build(self) -> AddServiceReturns<'a> {
        AddServiceReturns {
            serviceId: self.serviceId,
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
    pub fn builder(serviceId: impl Into<Cow<'a, str>>) -> RemoveServiceParamsBuilder<'a> {
        RemoveServiceParamsBuilder {
            serviceId: serviceId.into(),
        }
    }
    pub fn serviceId(&self) -> &str { self.serviceId.as_ref() }
}


pub struct RemoveServiceParamsBuilder<'a> {
    serviceId: Cow<'a, str>,
}

impl<'a> RemoveServiceParamsBuilder<'a> {
    pub fn build(self) -> RemoveServiceParams<'a> {
        RemoveServiceParams {
            serviceId: self.serviceId,
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
    pub fn builder(serviceId: impl Into<Cow<'a, str>>, characteristicUuid: impl Into<Cow<'a, str>>, properties: CharacteristicProperties) -> AddCharacteristicParamsBuilder<'a> {
        AddCharacteristicParamsBuilder {
            serviceId: serviceId.into(),
            characteristicUuid: characteristicUuid.into(),
            properties: properties,
        }
    }
    pub fn serviceId(&self) -> &str { self.serviceId.as_ref() }
    pub fn characteristicUuid(&self) -> &str { self.characteristicUuid.as_ref() }
    pub fn properties(&self) -> &CharacteristicProperties { &self.properties }
}


pub struct AddCharacteristicParamsBuilder<'a> {
    serviceId: Cow<'a, str>,
    characteristicUuid: Cow<'a, str>,
    properties: CharacteristicProperties,
}

impl<'a> AddCharacteristicParamsBuilder<'a> {
    pub fn build(self) -> AddCharacteristicParams<'a> {
        AddCharacteristicParams {
            serviceId: self.serviceId,
            characteristicUuid: self.characteristicUuid,
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
    characteristicId: Cow<'a, str>,
}

impl<'a> AddCharacteristicReturns<'a> {
    pub fn builder(characteristicId: impl Into<Cow<'a, str>>) -> AddCharacteristicReturnsBuilder<'a> {
        AddCharacteristicReturnsBuilder {
            characteristicId: characteristicId.into(),
        }
    }
    pub fn characteristicId(&self) -> &str { self.characteristicId.as_ref() }
}


pub struct AddCharacteristicReturnsBuilder<'a> {
    characteristicId: Cow<'a, str>,
}

impl<'a> AddCharacteristicReturnsBuilder<'a> {
    pub fn build(self) -> AddCharacteristicReturns<'a> {
        AddCharacteristicReturns {
            characteristicId: self.characteristicId,
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
    pub fn builder(characteristicId: impl Into<Cow<'a, str>>) -> RemoveCharacteristicParamsBuilder<'a> {
        RemoveCharacteristicParamsBuilder {
            characteristicId: characteristicId.into(),
        }
    }
    pub fn characteristicId(&self) -> &str { self.characteristicId.as_ref() }
}


pub struct RemoveCharacteristicParamsBuilder<'a> {
    characteristicId: Cow<'a, str>,
}

impl<'a> RemoveCharacteristicParamsBuilder<'a> {
    pub fn build(self) -> RemoveCharacteristicParams<'a> {
        RemoveCharacteristicParams {
            characteristicId: self.characteristicId,
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
    pub fn builder(characteristicId: impl Into<Cow<'a, str>>, descriptorUuid: impl Into<Cow<'a, str>>) -> AddDescriptorParamsBuilder<'a> {
        AddDescriptorParamsBuilder {
            characteristicId: characteristicId.into(),
            descriptorUuid: descriptorUuid.into(),
        }
    }
    pub fn characteristicId(&self) -> &str { self.characteristicId.as_ref() }
    pub fn descriptorUuid(&self) -> &str { self.descriptorUuid.as_ref() }
}


pub struct AddDescriptorParamsBuilder<'a> {
    characteristicId: Cow<'a, str>,
    descriptorUuid: Cow<'a, str>,
}

impl<'a> AddDescriptorParamsBuilder<'a> {
    pub fn build(self) -> AddDescriptorParams<'a> {
        AddDescriptorParams {
            characteristicId: self.characteristicId,
            descriptorUuid: self.descriptorUuid,
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
    pub fn builder(descriptorId: impl Into<Cow<'a, str>>) -> AddDescriptorReturnsBuilder<'a> {
        AddDescriptorReturnsBuilder {
            descriptorId: descriptorId.into(),
        }
    }
    pub fn descriptorId(&self) -> &str { self.descriptorId.as_ref() }
}


pub struct AddDescriptorReturnsBuilder<'a> {
    descriptorId: Cow<'a, str>,
}

impl<'a> AddDescriptorReturnsBuilder<'a> {
    pub fn build(self) -> AddDescriptorReturns<'a> {
        AddDescriptorReturns {
            descriptorId: self.descriptorId,
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
    pub fn builder(descriptorId: impl Into<Cow<'a, str>>) -> RemoveDescriptorParamsBuilder<'a> {
        RemoveDescriptorParamsBuilder {
            descriptorId: descriptorId.into(),
        }
    }
    pub fn descriptorId(&self) -> &str { self.descriptorId.as_ref() }
}


pub struct RemoveDescriptorParamsBuilder<'a> {
    descriptorId: Cow<'a, str>,
}

impl<'a> RemoveDescriptorParamsBuilder<'a> {
    pub fn build(self) -> RemoveDescriptorParams<'a> {
        RemoveDescriptorParams {
            descriptorId: self.descriptorId,
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
