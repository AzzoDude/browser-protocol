//! Defines commands and events for Autofill.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreditCard<'a> {
    /// 16-digit credit card number.
    number: Cow<'a, str>,
    /// Name of the credit card owner.
    name: Cow<'a, str>,
    /// 2-digit expiry month.
    expiryMonth: Cow<'a, str>,
    /// 4-digit expiry year.
    expiryYear: Cow<'a, str>,
    /// 3-digit card verification code.
    cvc: Cow<'a, str>,
}

impl<'a> CreditCard<'a> {
    pub fn builder(number: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, expiryMonth: impl Into<Cow<'a, str>>, expiryYear: impl Into<Cow<'a, str>>, cvc: impl Into<Cow<'a, str>>) -> CreditCardBuilder<'a> {
        CreditCardBuilder {
            number: number.into(),
            name: name.into(),
            expiryMonth: expiryMonth.into(),
            expiryYear: expiryYear.into(),
            cvc: cvc.into(),
        }
    }
    pub fn number(&self) -> &str { self.number.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn expiryMonth(&self) -> &str { self.expiryMonth.as_ref() }
    pub fn expiryYear(&self) -> &str { self.expiryYear.as_ref() }
    pub fn cvc(&self) -> &str { self.cvc.as_ref() }
}


pub struct CreditCardBuilder<'a> {
    number: Cow<'a, str>,
    name: Cow<'a, str>,
    expiryMonth: Cow<'a, str>,
    expiryYear: Cow<'a, str>,
    cvc: Cow<'a, str>,
}

impl<'a> CreditCardBuilder<'a> {
    pub fn build(self) -> CreditCard<'a> {
        CreditCard {
            number: self.number,
            name: self.name,
            expiryMonth: self.expiryMonth,
            expiryYear: self.expiryYear,
            cvc: self.cvc,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddressField<'a> {
    /// address field name, for example GIVEN_NAME.
    /// The full list of supported field names:
    /// https://source.chromium.org/chromium/chromium/src/+/main:components/autofill/core/browser/field_types.cc;l=38
    name: Cow<'a, str>,
    /// address field value, for example Jon Doe.
    value: Cow<'a, str>,
}

impl<'a> AddressField<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> AddressFieldBuilder<'a> {
        AddressFieldBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct AddressFieldBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> AddressFieldBuilder<'a> {
    pub fn build(self) -> AddressField<'a> {
        AddressField {
            name: self.name,
            value: self.value,
        }
    }
}

/// A list of address fields.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddressFields<'a> {
    fields: Vec<AddressField<'a>>,
}

impl<'a> AddressFields<'a> {
    pub fn builder(fields: Vec<AddressField<'a>>) -> AddressFieldsBuilder<'a> {
        AddressFieldsBuilder {
            fields: fields,
        }
    }
    pub fn fields(&self) -> &[AddressField<'a>] { &self.fields }
}


pub struct AddressFieldsBuilder<'a> {
    fields: Vec<AddressField<'a>>,
}

impl<'a> AddressFieldsBuilder<'a> {
    pub fn build(self) -> AddressFields<'a> {
        AddressFields {
            fields: self.fields,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Address<'a> {
    /// fields and values defining an address.
    fields: Vec<AddressField<'a>>,
}

impl<'a> Address<'a> {
    pub fn builder(fields: Vec<AddressField<'a>>) -> AddressBuilder<'a> {
        AddressBuilder {
            fields: fields,
        }
    }
    pub fn fields(&self) -> &[AddressField<'a>] { &self.fields }
}


pub struct AddressBuilder<'a> {
    fields: Vec<AddressField<'a>>,
}

impl<'a> AddressBuilder<'a> {
    pub fn build(self) -> Address<'a> {
        Address {
            fields: self.fields,
        }
    }
}

/// Defines how an address can be displayed like in chrome://settings/addresses.
/// Address UI is a two dimensional array, each inner array is an "address information line", and when rendered in a UI surface should be displayed as such.
/// The following address UI for instance:
/// [[{name: "GIVE_NAME", value: "Jon"}, {name: "FAMILY_NAME", value: "Doe"}], [{name: "CITY", value: "Munich"}, {name: "ZIP", value: "81456"}]]
/// should allow the receiver to render:
/// Jon Doe
/// Munich 81456

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddressUI<'a> {
    /// A two dimension array containing the representation of values from an address profile.
    addressFields: Vec<AddressFields<'a>>,
}

impl<'a> AddressUI<'a> {
    pub fn builder(addressFields: Vec<AddressFields<'a>>) -> AddressUIBuilder<'a> {
        AddressUIBuilder {
            addressFields: addressFields,
        }
    }
    pub fn addressFields(&self) -> &[AddressFields<'a>] { &self.addressFields }
}


pub struct AddressUIBuilder<'a> {
    addressFields: Vec<AddressFields<'a>>,
}

impl<'a> AddressUIBuilder<'a> {
    pub fn build(self) -> AddressUI<'a> {
        AddressUI {
            addressFields: self.addressFields,
        }
    }
}

/// Specified whether a filled field was done so by using the html autocomplete attribute or autofill heuristics.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum FillingStrategy {
    #[default]
    #[serde(rename = "autocompleteAttribute")]
    AutocompleteAttribute,
    #[serde(rename = "autofillInferred")]
    AutofillInferred,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilledField<'a> {
    /// The type of the field, e.g text, password etc.
    htmlType: Cow<'a, str>,
    /// the html id
    id: Cow<'a, str>,
    /// the html name
    name: Cow<'a, str>,
    /// the field value
    value: Cow<'a, str>,
    /// The actual field type, e.g FAMILY_NAME
    autofillType: Cow<'a, str>,
    /// The filling strategy
    fillingStrategy: FillingStrategy,
    /// The frame the field belongs to
    frameId: crate::page::FrameId<'a>,
    /// The form field's DOM node
    fieldId: crate::dom::BackendNodeId,
}

impl<'a> FilledField<'a> {
    pub fn builder(htmlType: impl Into<Cow<'a, str>>, id: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>, autofillType: impl Into<Cow<'a, str>>, fillingStrategy: impl Into<FillingStrategy>, frameId: crate::page::FrameId<'a>, fieldId: crate::dom::BackendNodeId) -> FilledFieldBuilder<'a> {
        FilledFieldBuilder {
            htmlType: htmlType.into(),
            id: id.into(),
            name: name.into(),
            value: value.into(),
            autofillType: autofillType.into(),
            fillingStrategy: fillingStrategy.into(),
            frameId: frameId,
            fieldId: fieldId,
        }
    }
    pub fn htmlType(&self) -> &str { self.htmlType.as_ref() }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn autofillType(&self) -> &str { self.autofillType.as_ref() }
    pub fn fillingStrategy(&self) -> &FillingStrategy { &self.fillingStrategy }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
    pub fn fieldId(&self) -> &crate::dom::BackendNodeId { &self.fieldId }
}


pub struct FilledFieldBuilder<'a> {
    htmlType: Cow<'a, str>,
    id: Cow<'a, str>,
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    autofillType: Cow<'a, str>,
    fillingStrategy: FillingStrategy,
    frameId: crate::page::FrameId<'a>,
    fieldId: crate::dom::BackendNodeId,
}

impl<'a> FilledFieldBuilder<'a> {
    pub fn build(self) -> FilledField<'a> {
        FilledField {
            htmlType: self.htmlType,
            id: self.id,
            name: self.name,
            value: self.value,
            autofillType: self.autofillType,
            fillingStrategy: self.fillingStrategy,
            frameId: self.frameId,
            fieldId: self.fieldId,
        }
    }
}

/// Trigger autofill on a form identified by the fieldId.
/// If the field and related form cannot be autofilled, returns an error.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerParams<'a> {
    /// Identifies a field that serves as an anchor for autofill.
    fieldId: crate::dom::BackendNodeId,
    /// Identifies the frame that field belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<crate::page::FrameId<'a>>,
    /// Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with 'address'.
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<CreditCard<'a>>,
    /// Address to fill out the form. Address data is not saved. Mutually exclusive with 'card'.
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Address<'a>>,
}

impl<'a> TriggerParams<'a> {
    pub fn builder(fieldId: crate::dom::BackendNodeId) -> TriggerParamsBuilder<'a> {
        TriggerParamsBuilder {
            fieldId: fieldId,
            frameId: None,
            card: None,
            address: None,
        }
    }
    pub fn fieldId(&self) -> &crate::dom::BackendNodeId { &self.fieldId }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
    pub fn card(&self) -> Option<&CreditCard<'a>> { self.card.as_ref() }
    pub fn address(&self) -> Option<&Address<'a>> { self.address.as_ref() }
}


pub struct TriggerParamsBuilder<'a> {
    fieldId: crate::dom::BackendNodeId,
    frameId: Option<crate::page::FrameId<'a>>,
    card: Option<CreditCard<'a>>,
    address: Option<Address<'a>>,
}

impl<'a> TriggerParamsBuilder<'a> {
    /// Identifies the frame that field belongs to.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with 'address'.
    pub fn card(mut self, card: CreditCard<'a>) -> Self { self.card = Some(card); self }
    /// Address to fill out the form. Address data is not saved. Mutually exclusive with 'card'.
    pub fn address(mut self, address: Address<'a>) -> Self { self.address = Some(address); self }
    pub fn build(self) -> TriggerParams<'a> {
        TriggerParams {
            fieldId: self.fieldId,
            frameId: self.frameId,
            card: self.card,
            address: self.address,
        }
    }
}

impl<'a> TriggerParams<'a> { pub const METHOD: &'static str = "Autofill.trigger"; }

impl<'a> crate::CdpCommand<'a> for TriggerParams<'a> {
    const METHOD: &'static str = "Autofill.trigger";
    type Response = crate::EmptyReturns;
}

/// Set addresses so that developers can verify their forms implementation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAddressesParams<'a> {
    addresses: Vec<Address<'a>>,
}

impl<'a> SetAddressesParams<'a> {
    pub fn builder(addresses: Vec<Address<'a>>) -> SetAddressesParamsBuilder<'a> {
        SetAddressesParamsBuilder {
            addresses: addresses,
        }
    }
    pub fn addresses(&self) -> &[Address<'a>] { &self.addresses }
}


pub struct SetAddressesParamsBuilder<'a> {
    addresses: Vec<Address<'a>>,
}

impl<'a> SetAddressesParamsBuilder<'a> {
    pub fn build(self) -> SetAddressesParams<'a> {
        SetAddressesParams {
            addresses: self.addresses,
        }
    }
}

impl<'a> SetAddressesParams<'a> { pub const METHOD: &'static str = "Autofill.setAddresses"; }

impl<'a> crate::CdpCommand<'a> for SetAddressesParams<'a> {
    const METHOD: &'static str = "Autofill.setAddresses";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Autofill.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Autofill.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Autofill.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Autofill.enable";
    type Response = crate::EmptyReturns;
}
