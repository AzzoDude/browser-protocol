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
    pub fn builder() -> CreditCardBuilder<'a> { CreditCardBuilder::default() }
    pub fn number(&self) -> &str { self.number.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn expiryMonth(&self) -> &str { self.expiryMonth.as_ref() }
    pub fn expiryYear(&self) -> &str { self.expiryYear.as_ref() }
    pub fn cvc(&self) -> &str { self.cvc.as_ref() }
}

#[derive(Default)]
pub struct CreditCardBuilder<'a> {
    number: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    expiryMonth: Option<Cow<'a, str>>,
    expiryYear: Option<Cow<'a, str>>,
    cvc: Option<Cow<'a, str>>,
}

impl<'a> CreditCardBuilder<'a> {
    /// 16-digit credit card number.
    pub fn number(mut self, number: impl Into<Cow<'a, str>>) -> Self { self.number = Some(number.into()); self }
    /// Name of the credit card owner.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// 2-digit expiry month.
    pub fn expiryMonth(mut self, expiryMonth: impl Into<Cow<'a, str>>) -> Self { self.expiryMonth = Some(expiryMonth.into()); self }
    /// 4-digit expiry year.
    pub fn expiryYear(mut self, expiryYear: impl Into<Cow<'a, str>>) -> Self { self.expiryYear = Some(expiryYear.into()); self }
    /// 3-digit card verification code.
    pub fn cvc(mut self, cvc: impl Into<Cow<'a, str>>) -> Self { self.cvc = Some(cvc.into()); self }
    pub fn build(self) -> CreditCard<'a> {
        CreditCard {
            number: self.number.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            expiryMonth: self.expiryMonth.unwrap_or_default(),
            expiryYear: self.expiryYear.unwrap_or_default(),
            cvc: self.cvc.unwrap_or_default(),
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
    pub fn builder() -> AddressFieldBuilder<'a> { AddressFieldBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct AddressFieldBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> AddressFieldBuilder<'a> {
    /// address field name, for example GIVEN_NAME.
    /// The full list of supported field names:
    /// https://source.chromium.org/chromium/chromium/src/+/main:components/autofill/core/browser/field_types.cc;l=38
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// address field value, for example Jon Doe.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> AddressField<'a> {
        AddressField {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
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
    pub fn builder() -> AddressFieldsBuilder<'a> { AddressFieldsBuilder::default() }
    pub fn fields(&self) -> &[AddressField<'a>] { &self.fields }
}

#[derive(Default)]
pub struct AddressFieldsBuilder<'a> {
    fields: Option<Vec<AddressField<'a>>>,
}

impl<'a> AddressFieldsBuilder<'a> {
    pub fn fields(mut self, fields: Vec<AddressField<'a>>) -> Self { self.fields = Some(fields); self }
    pub fn build(self) -> AddressFields<'a> {
        AddressFields {
            fields: self.fields.unwrap_or_default(),
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
    pub fn builder() -> AddressBuilder<'a> { AddressBuilder::default() }
    pub fn fields(&self) -> &[AddressField<'a>] { &self.fields }
}

#[derive(Default)]
pub struct AddressBuilder<'a> {
    fields: Option<Vec<AddressField<'a>>>,
}

impl<'a> AddressBuilder<'a> {
    /// fields and values defining an address.
    pub fn fields(mut self, fields: Vec<AddressField<'a>>) -> Self { self.fields = Some(fields); self }
    pub fn build(self) -> Address<'a> {
        Address {
            fields: self.fields.unwrap_or_default(),
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
    pub fn builder() -> AddressUIBuilder<'a> { AddressUIBuilder::default() }
    pub fn addressFields(&self) -> &[AddressFields<'a>] { &self.addressFields }
}

#[derive(Default)]
pub struct AddressUIBuilder<'a> {
    addressFields: Option<Vec<AddressFields<'a>>>,
}

impl<'a> AddressUIBuilder<'a> {
    /// A two dimension array containing the representation of values from an address profile.
    pub fn addressFields(mut self, addressFields: Vec<AddressFields<'a>>) -> Self { self.addressFields = Some(addressFields); self }
    pub fn build(self) -> AddressUI<'a> {
        AddressUI {
            addressFields: self.addressFields.unwrap_or_default(),
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
    pub fn builder() -> FilledFieldBuilder<'a> { FilledFieldBuilder::default() }
    pub fn htmlType(&self) -> &str { self.htmlType.as_ref() }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn autofillType(&self) -> &str { self.autofillType.as_ref() }
    pub fn fillingStrategy(&self) -> &FillingStrategy { &self.fillingStrategy }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
    pub fn fieldId(&self) -> &crate::dom::BackendNodeId { &self.fieldId }
}

#[derive(Default)]
pub struct FilledFieldBuilder<'a> {
    htmlType: Option<Cow<'a, str>>,
    id: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    autofillType: Option<Cow<'a, str>>,
    fillingStrategy: Option<FillingStrategy>,
    frameId: Option<crate::page::FrameId<'a>>,
    fieldId: Option<crate::dom::BackendNodeId>,
}

impl<'a> FilledFieldBuilder<'a> {
    /// The type of the field, e.g text, password etc.
    pub fn htmlType(mut self, htmlType: impl Into<Cow<'a, str>>) -> Self { self.htmlType = Some(htmlType.into()); self }
    /// the html id
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    /// the html name
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// the field value
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// The actual field type, e.g FAMILY_NAME
    pub fn autofillType(mut self, autofillType: impl Into<Cow<'a, str>>) -> Self { self.autofillType = Some(autofillType.into()); self }
    /// The filling strategy
    pub fn fillingStrategy(mut self, fillingStrategy: FillingStrategy) -> Self { self.fillingStrategy = Some(fillingStrategy); self }
    /// The frame the field belongs to
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// The form field's DOM node
    pub fn fieldId(mut self, fieldId: crate::dom::BackendNodeId) -> Self { self.fieldId = Some(fieldId); self }
    pub fn build(self) -> FilledField<'a> {
        FilledField {
            htmlType: self.htmlType.unwrap_or_default(),
            id: self.id.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
            autofillType: self.autofillType.unwrap_or_default(),
            fillingStrategy: self.fillingStrategy.unwrap_or_default(),
            frameId: self.frameId.unwrap_or_default(),
            fieldId: self.fieldId.unwrap_or_default(),
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
    pub fn builder() -> TriggerParamsBuilder<'a> { TriggerParamsBuilder::default() }
    pub fn fieldId(&self) -> &crate::dom::BackendNodeId { &self.fieldId }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
    pub fn card(&self) -> Option<&CreditCard<'a>> { self.card.as_ref() }
    pub fn address(&self) -> Option<&Address<'a>> { self.address.as_ref() }
}

#[derive(Default)]
pub struct TriggerParamsBuilder<'a> {
    fieldId: Option<crate::dom::BackendNodeId>,
    frameId: Option<crate::page::FrameId<'a>>,
    card: Option<CreditCard<'a>>,
    address: Option<Address<'a>>,
}

impl<'a> TriggerParamsBuilder<'a> {
    /// Identifies a field that serves as an anchor for autofill.
    pub fn fieldId(mut self, fieldId: crate::dom::BackendNodeId) -> Self { self.fieldId = Some(fieldId); self }
    /// Identifies the frame that field belongs to.
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    /// Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with 'address'.
    pub fn card(mut self, card: CreditCard<'a>) -> Self { self.card = Some(card); self }
    /// Address to fill out the form. Address data is not saved. Mutually exclusive with 'card'.
    pub fn address(mut self, address: Address<'a>) -> Self { self.address = Some(address); self }
    pub fn build(self) -> TriggerParams<'a> {
        TriggerParams {
            fieldId: self.fieldId.unwrap_or_default(),
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
    pub fn builder() -> SetAddressesParamsBuilder<'a> { SetAddressesParamsBuilder::default() }
    pub fn addresses(&self) -> &[Address<'a>] { &self.addresses }
}

#[derive(Default)]
pub struct SetAddressesParamsBuilder<'a> {
    addresses: Option<Vec<Address<'a>>>,
}

impl<'a> SetAddressesParamsBuilder<'a> {
    pub fn addresses(mut self, addresses: Vec<Address<'a>>) -> Self { self.addresses = Some(addresses); self }
    pub fn build(self) -> SetAddressesParams<'a> {
        SetAddressesParams {
            addresses: self.addresses.unwrap_or_default(),
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

impl DisableParams { pub const METHOD: &'static str = "Autofill.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Autofill.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct EnableParamsBuilder {}

impl EnableParamsBuilder {
    pub fn build(self) -> EnableParams {
        EnableParams {}
    }
}

impl EnableParams { pub const METHOD: &'static str = "Autofill.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Autofill.enable";
    type Response = crate::EmptyReturns;
}
