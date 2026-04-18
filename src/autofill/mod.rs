//! Defines commands and events for Autofill.

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreditCard {
    /// 16-digit credit card number.

    pub number: String,
    /// Name of the credit card owner.

    pub name: String,
    /// 2-digit expiry month.

    pub expiryMonth: String,
    /// 4-digit expiry year.

    pub expiryYear: String,
    /// 3-digit card verification code.

    pub cvc: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddressField {
    /// address field name, for example GIVEN_NAME.
    /// The full list of supported field names:
    /// <https://source.chromium.org/chromium/chromium/src/+/main:components/autofill/core/browser/field_types.cc;l=38>

    pub name: String,
    /// address field value, for example Jon Doe.

    pub value: String,
}

/// A list of address fields.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddressFields {

    pub fields: Vec<AddressField>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// fields and values defining an address.

    pub fields: Vec<AddressField>,
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
pub struct AddressUI {
    /// A two dimension array containing the representation of values from an address profile.

    pub addressFields: Vec<AddressFields>,
}

/// Specified whether a filled field was done so by using the html autocomplete attribute or autofill heuristics.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum FillingStrategy {
    #[default]
    AutocompleteAttribute,
    AutofillInferred,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilledField {
    /// The type of the field, e.g text, password etc.

    pub htmlType: String,
    /// the html id

    pub id: String,
    /// the html name

    pub name: String,
    /// the field value

    pub value: String,
    /// The actual field type, e.g FAMILY_NAME

    pub autofillType: String,
    /// The filling strategy

    pub fillingStrategy: FillingStrategy,
    /// The frame the field belongs to

    pub frameId: crate::page::FrameId,
    /// The form field's DOM node

    pub fieldId: crate::dom::BackendNodeId,
}

/// Trigger autofill on a form identified by the fieldId.
/// If the field and related form cannot be autofilled, returns an error.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerParams {
    /// Identifies a field that serves as an anchor for autofill.

    pub fieldId: crate::dom::BackendNodeId,
    /// Identifies the frame that field belongs to.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<crate::page::FrameId>,
    /// Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with 'address'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreditCard>,
    /// Address to fill out the form. Address data is not saved. Mutually exclusive with 'card'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

/// Set addresses so that developers can verify their forms implementation.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAddressesParams {

    pub addresses: Vec<Address>,
}
