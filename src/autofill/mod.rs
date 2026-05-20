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
    #[serde(rename = "expiryMonth")]
    expiry_month: Cow<'a, str>,
    /// 4-digit expiry year.
    #[serde(rename = "expiryYear")]
    expiry_year: Cow<'a, str>,
    /// 3-digit card verification code.
    cvc: Cow<'a, str>,
}

impl<'a> CreditCard<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `number`: 16-digit credit card number.
    /// * `name`: Name of the credit card owner.
    /// * `expiry_month`: 2-digit expiry month.
    /// * `expiry_year`: 4-digit expiry year.
    /// * `cvc`: 3-digit card verification code.
    pub fn builder(number: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, expiry_month: impl Into<Cow<'a, str>>, expiry_year: impl Into<Cow<'a, str>>, cvc: impl Into<Cow<'a, str>>) -> CreditCardBuilder<'a> {
        CreditCardBuilder {
            number: number.into(),
            name: name.into(),
            expiry_month: expiry_month.into(),
            expiry_year: expiry_year.into(),
            cvc: cvc.into(),
        }
    }
    /// 16-digit credit card number.
    pub fn number(&self) -> &str { self.number.as_ref() }
    /// Name of the credit card owner.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// 2-digit expiry month.
    pub fn expiry_month(&self) -> &str { self.expiry_month.as_ref() }
    /// 4-digit expiry year.
    pub fn expiry_year(&self) -> &str { self.expiry_year.as_ref() }
    /// 3-digit card verification code.
    pub fn cvc(&self) -> &str { self.cvc.as_ref() }
}


pub struct CreditCardBuilder<'a> {
    number: Cow<'a, str>,
    name: Cow<'a, str>,
    expiry_month: Cow<'a, str>,
    expiry_year: Cow<'a, str>,
    cvc: Cow<'a, str>,
}

impl<'a> CreditCardBuilder<'a> {
    pub fn build(self) -> CreditCard<'a> {
        CreditCard {
            number: self.number,
            name: self.name,
            expiry_month: self.expiry_month,
            expiry_year: self.expiry_year,
            cvc: self.cvc,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddressField<'a> {
    /// address field name, for example GIVEN_NAME.
    /// The full list of supported field names:
    /// <https://source.chromium.org/chromium/chromium/src/+/main:components/autofill/core/browser/field_types.cc;l=38>
    name: Cow<'a, str>,
    /// address field value, for example Jon Doe.
    value: Cow<'a, str>,
}

impl<'a> AddressField<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: address field name, for example GIVEN_NAME. The full list of supported field names: <https://source.chromium.org/chromium/chromium/src/+/main:components/autofill/core/browser/field_types.cc;l=38>
    /// * `value`: address field value, for example Jon Doe.
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> AddressFieldBuilder<'a> {
        AddressFieldBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    /// address field name, for example GIVEN_NAME.
    /// The full list of supported field names:
    /// <https://source.chromium.org/chromium/chromium/src/+/main:components/autofill/core/browser/field_types.cc;l=38>
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// address field value, for example Jon Doe.
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
    /// Creates a builder for this type with the required parameters:
    /// * `fields`: 
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
    /// Creates a builder for this type with the required parameters:
    /// * `fields`: fields and values defining an address.
    pub fn builder(fields: Vec<AddressField<'a>>) -> AddressBuilder<'a> {
        AddressBuilder {
            fields: fields,
        }
    }
    /// fields and values defining an address.
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
/// \[\[{name: "GIVE_NAME", value: "Jon"}, {name: "FAMILY_NAME", value: "Doe"}\], \[{name: "CITY", value: "Munich"}, {name: "ZIP", value: "81456"}\]\]
/// should allow the receiver to render:
/// Jon Doe
/// Munich 81456

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddressUI<'a> {
    /// A two dimension array containing the representation of values from an address profile.
    #[serde(rename = "addressFields")]
    address_fields: Vec<AddressFields<'a>>,
}

impl<'a> AddressUI<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `address_fields`: A two dimension array containing the representation of values from an address profile.
    pub fn builder(address_fields: Vec<AddressFields<'a>>) -> AddressUIBuilder<'a> {
        AddressUIBuilder {
            address_fields: address_fields,
        }
    }
    /// A two dimension array containing the representation of values from an address profile.
    pub fn address_fields(&self) -> &[AddressFields<'a>] { &self.address_fields }
}


pub struct AddressUIBuilder<'a> {
    address_fields: Vec<AddressFields<'a>>,
}

impl<'a> AddressUIBuilder<'a> {
    pub fn build(self) -> AddressUI<'a> {
        AddressUI {
            address_fields: self.address_fields,
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
    #[serde(rename = "htmlType")]
    html_type: Cow<'a, str>,
    /// the html id
    id: Cow<'a, str>,
    /// the html name
    name: Cow<'a, str>,
    /// the field value
    value: Cow<'a, str>,
    /// The actual field type, e.g FAMILY_NAME
    #[serde(rename = "autofillType")]
    autofill_type: Cow<'a, str>,
    /// The filling strategy
    #[serde(rename = "fillingStrategy")]
    filling_strategy: FillingStrategy,
    /// The frame the field belongs to
    #[serde(rename = "frameId")]
    frame_id: crate::page::FrameId<'a>,
    /// The form field's DOM node
    #[serde(rename = "fieldId")]
    field_id: crate::dom::BackendNodeId,
}

impl<'a> FilledField<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `html_type`: The type of the field, e.g text, password etc.
    /// * `id`: the html id
    /// * `name`: the html name
    /// * `value`: the field value
    /// * `autofill_type`: The actual field type, e.g FAMILY_NAME
    /// * `filling_strategy`: The filling strategy
    /// * `frame_id`: The frame the field belongs to
    /// * `field_id`: The form field's DOM node
    pub fn builder(html_type: impl Into<Cow<'a, str>>, id: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>, autofill_type: impl Into<Cow<'a, str>>, filling_strategy: impl Into<FillingStrategy>, frame_id: crate::page::FrameId<'a>, field_id: crate::dom::BackendNodeId) -> FilledFieldBuilder<'a> {
        FilledFieldBuilder {
            html_type: html_type.into(),
            id: id.into(),
            name: name.into(),
            value: value.into(),
            autofill_type: autofill_type.into(),
            filling_strategy: filling_strategy.into(),
            frame_id: frame_id,
            field_id: field_id,
        }
    }
    /// The type of the field, e.g text, password etc.
    pub fn html_type(&self) -> &str { self.html_type.as_ref() }
    /// the html id
    pub fn id(&self) -> &str { self.id.as_ref() }
    /// the html name
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// the field value
    pub fn value(&self) -> &str { self.value.as_ref() }
    /// The actual field type, e.g FAMILY_NAME
    pub fn autofill_type(&self) -> &str { self.autofill_type.as_ref() }
    /// The filling strategy
    pub fn filling_strategy(&self) -> &FillingStrategy { &self.filling_strategy }
    /// The frame the field belongs to
    pub fn frame_id(&self) -> &crate::page::FrameId<'a> { &self.frame_id }
    /// The form field's DOM node
    pub fn field_id(&self) -> &crate::dom::BackendNodeId { &self.field_id }
}


pub struct FilledFieldBuilder<'a> {
    html_type: Cow<'a, str>,
    id: Cow<'a, str>,
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    autofill_type: Cow<'a, str>,
    filling_strategy: FillingStrategy,
    frame_id: crate::page::FrameId<'a>,
    field_id: crate::dom::BackendNodeId,
}

impl<'a> FilledFieldBuilder<'a> {
    pub fn build(self) -> FilledField<'a> {
        FilledField {
            html_type: self.html_type,
            id: self.id,
            name: self.name,
            value: self.value,
            autofill_type: self.autofill_type,
            filling_strategy: self.filling_strategy,
            frame_id: self.frame_id,
            field_id: self.field_id,
        }
    }
}

/// Trigger autofill on a form identified by the fieldId.
/// If the field and related form cannot be autofilled, returns an error.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerParams<'a> {
    /// Identifies a field that serves as an anchor for autofill.
    #[serde(rename = "fieldId")]
    field_id: crate::dom::BackendNodeId,
    /// Identifies the frame that field belongs to.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<crate::page::FrameId<'a>>,
    /// Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with 'address'.
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<CreditCard<'a>>,
    /// Address to fill out the form. Address data is not saved. Mutually exclusive with 'card'.
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Address<'a>>,
}

impl<'a> TriggerParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `field_id`: Identifies a field that serves as an anchor for autofill.
    pub fn builder(field_id: crate::dom::BackendNodeId) -> TriggerParamsBuilder<'a> {
        TriggerParamsBuilder {
            field_id: field_id,
            frame_id: None,
            card: None,
            address: None,
        }
    }
    /// Identifies a field that serves as an anchor for autofill.
    pub fn field_id(&self) -> &crate::dom::BackendNodeId { &self.field_id }
    /// Identifies the frame that field belongs to.
    pub fn frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.frame_id.as_ref() }
    /// Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with 'address'.
    pub fn card(&self) -> Option<&CreditCard<'a>> { self.card.as_ref() }
    /// Address to fill out the form. Address data is not saved. Mutually exclusive with 'card'.
    pub fn address(&self) -> Option<&Address<'a>> { self.address.as_ref() }
}


pub struct TriggerParamsBuilder<'a> {
    field_id: crate::dom::BackendNodeId,
    frame_id: Option<crate::page::FrameId<'a>>,
    card: Option<CreditCard<'a>>,
    address: Option<Address<'a>>,
}

impl<'a> TriggerParamsBuilder<'a> {
    /// Identifies the frame that field belongs to.
    pub fn frame_id(mut self, frame_id: crate::page::FrameId<'a>) -> Self { self.frame_id = Some(frame_id); self }
    /// Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with 'address'.
    pub fn card(mut self, card: CreditCard<'a>) -> Self { self.card = Some(card); self }
    /// Address to fill out the form. Address data is not saved. Mutually exclusive with 'card'.
    pub fn address(mut self, address: Address<'a>) -> Self { self.address = Some(address); self }
    pub fn build(self) -> TriggerParams<'a> {
        TriggerParams {
            field_id: self.field_id,
            frame_id: self.frame_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `addresses`: 
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
