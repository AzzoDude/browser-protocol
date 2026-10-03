//! Defines commands and events for Autofill.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CreditCard<'a> {
    /// 16-digit credit card number.
    pub number: Cow<'a, str>,
    /// Name of the credit card owner.
    pub name: Cow<'a, str>,
    /// 2-digit expiry month.
    #[serde(rename = "expiryMonth")]
    pub expiry_month: Cow<'a, str>,
    /// 4-digit expiry year.
    #[serde(rename = "expiryYear")]
    pub expiry_year: Cow<'a, str>,
    /// 3-digit card verification code.
    pub cvc: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddressField<'a> {
    /// address field name, for example GIVEN_NAME.
    /// The full list of supported field names:
    /// <https://source.chromium.org/chromium/chromium/src/+/main:components/autofill/core/browser/field_types.cc;l=38>
    pub name: Cow<'a, str>,
    /// address field value, for example Jon Doe.
    pub value: Cow<'a, str>,
}
/// A list of address fields.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddressFields<'a> {
    pub fields: Vec<AddressField<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Address<'a> {
    /// fields and values defining an address.
    pub fields: Vec<AddressField<'a>>,
}
/// Defines how an address can be displayed like in chrome://settings/addresses.
/// Address UI is a two dimensional array, each inner array is an "address information line", and when rendered in a UI surface should be displayed as such.
/// The following address UI for instance:
/// \[\[{name: "GIVE_NAME", value: "Jon"}, {name: "FAMILY_NAME", value: "Doe"}\], \[{name: "CITY", value: "Munich"}, {name: "ZIP", value: "81456"}\]\]
/// should allow the receiver to render:
/// Jon Doe
/// Munich 81456

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddressUI<'a> {
    /// A two dimension array containing the representation of values from an address profile.
    #[serde(rename = "addressFields")]
    pub address_fields: Vec<AddressFields<'a>>,
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


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FilledField<'a> {
    /// The type of the field, e.g text, password etc.
    #[serde(rename = "htmlType")]
    pub html_type: Cow<'a, str>,
    /// the html id
    pub id: Cow<'a, str>,
    /// the html name
    pub name: Cow<'a, str>,
    /// the field value
    pub value: Cow<'a, str>,
    /// The actual field type, e.g FAMILY_NAME
    #[serde(rename = "autofillType")]
    pub autofill_type: Cow<'a, str>,
    /// The filling strategy
    #[serde(rename = "fillingStrategy")]
    pub filling_strategy: FillingStrategy,
    /// The frame the field belongs to
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
    /// The form field's DOM node
    #[serde(rename = "fieldId")]
    pub field_id: crate::dom::BackendNodeId,
}
/// Trigger autofill on a form identified by the fieldId.
/// If the field and related form cannot be autofilled, returns an error.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Autofill.trigger")]
pub struct TriggerParams<'a> {
    /// Identifies a field that serves as an anchor for autofill.
    #[serde(rename = "fieldId")]
    pub field_id: crate::dom::BackendNodeId,
    /// Identifies the frame that field belongs to.
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
    /// Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with 'address'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreditCard<'a>>,
    /// Address to fill out the form. Address data is not saved. Mutually exclusive with 'card'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address<'a>>,
}
/// Set addresses so that developers can verify their forms implementation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Autofill.setAddresses")]
pub struct SetAddressesParams<'a> {
    pub addresses: Vec<Address<'a>>,
}
/// Disables autofill domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Autofill.disable")]
pub struct DisableParams {

}
/// Enables autofill domain notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Autofill.enable")]
pub struct EnableParams {

}
/// Emitted when an address form is filled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Autofill.addressFormFilled")]
pub struct AddressFormFilled<'a> {
    /// Information about the fields that were filled
    #[serde(rename = "filledFields")]
    pub filled_fields: Vec<FilledField<'a>>,
    /// An UI representation of the address used to fill the form.
    /// Consists of a 2D array where each child represents an address/profile line.
    #[serde(rename = "addressUi")]
    pub address_ui: AddressUI<'a>,
}