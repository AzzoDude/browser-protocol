use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TouchPoint {
    /// X coordinate of the event relative to the main frame's viewport in CSS pixels.
    x: f64,
    /// Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    /// the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    y: f64,
    /// X radius of the touch area (default: 1.0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "radiusX")]
    radius_x: Option<f64>,
    /// Y radius of the touch area (default: 1.0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "radiusY")]
    radius_y: Option<f64>,
    /// Rotation angle (default: 0.0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rotationAngle")]
    rotation_angle: Option<f64>,
    /// Force (default: 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<f64>,
    /// The normalized tangential pressure, which has a range of \[-1,1\] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tangentialPressure")]
    tangential_pressure: Option<f64>,
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range \[-90,90\], a positive tiltX is to the right (default: 0)
    #[serde(skip_serializing_if = "Option::is_none", rename = "tiltX")]
    tilt_x: Option<f64>,
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range \[-90,90\], a positive tiltY is towards the user (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tiltY")]
    tilt_y: Option<f64>,
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range \[0,359\] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    twist: Option<i64>,
    /// Identifier used to track touch sources between events, must be unique within an event.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<f64>,
}

impl TouchPoint {
    /// Creates a builder for this type with the required parameters:
    /// * `x`: X coordinate of the event relative to the main frame's viewport in CSS pixels.
    /// * `y`: Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    pub fn builder(x: f64, y: f64) -> TouchPointBuilder {
        TouchPointBuilder {
            x: x,
            y: y,
            radius_x: None,
            radius_y: None,
            rotation_angle: None,
            force: None,
            tangential_pressure: None,
            tilt_x: None,
            tilt_y: None,
            twist: None,
            id: None,
        }
    }
    /// X coordinate of the event relative to the main frame's viewport in CSS pixels.
    pub fn x(&self) -> f64 { self.x }
    /// Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    /// the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    pub fn y(&self) -> f64 { self.y }
    /// X radius of the touch area (default: 1.0).
    pub fn radius_x(&self) -> Option<f64> { self.radius_x }
    /// Y radius of the touch area (default: 1.0).
    pub fn radius_y(&self) -> Option<f64> { self.radius_y }
    /// Rotation angle (default: 0.0).
    pub fn rotation_angle(&self) -> Option<f64> { self.rotation_angle }
    /// Force (default: 1.0).
    pub fn force(&self) -> Option<f64> { self.force }
    /// The normalized tangential pressure, which has a range of \[-1,1\] (default: 0).
    pub fn tangential_pressure(&self) -> Option<f64> { self.tangential_pressure }
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range \[-90,90\], a positive tiltX is to the right (default: 0)
    pub fn tilt_x(&self) -> Option<f64> { self.tilt_x }
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range \[-90,90\], a positive tiltY is towards the user (default: 0).
    pub fn tilt_y(&self) -> Option<f64> { self.tilt_y }
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range \[0,359\] (default: 0).
    pub fn twist(&self) -> Option<i64> { self.twist }
    /// Identifier used to track touch sources between events, must be unique within an event.
    pub fn id(&self) -> Option<f64> { self.id }
}


pub struct TouchPointBuilder {
    x: f64,
    y: f64,
    radius_x: Option<f64>,
    radius_y: Option<f64>,
    rotation_angle: Option<f64>,
    force: Option<f64>,
    tangential_pressure: Option<f64>,
    tilt_x: Option<f64>,
    tilt_y: Option<f64>,
    twist: Option<i64>,
    id: Option<f64>,
}

impl TouchPointBuilder {
    /// X radius of the touch area (default: 1.0).
    pub fn radius_x(mut self, radius_x: f64) -> Self { self.radius_x = Some(radius_x); self }
    /// Y radius of the touch area (default: 1.0).
    pub fn radius_y(mut self, radius_y: f64) -> Self { self.radius_y = Some(radius_y); self }
    /// Rotation angle (default: 0.0).
    pub fn rotation_angle(mut self, rotation_angle: f64) -> Self { self.rotation_angle = Some(rotation_angle); self }
    /// Force (default: 1.0).
    pub fn force(mut self, force: f64) -> Self { self.force = Some(force); self }
    /// The normalized tangential pressure, which has a range of \[-1,1\] (default: 0).
    pub fn tangential_pressure(mut self, tangential_pressure: f64) -> Self { self.tangential_pressure = Some(tangential_pressure); self }
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range \[-90,90\], a positive tiltX is to the right (default: 0)
    pub fn tilt_x(mut self, tilt_x: f64) -> Self { self.tilt_x = Some(tilt_x); self }
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range \[-90,90\], a positive tiltY is towards the user (default: 0).
    pub fn tilt_y(mut self, tilt_y: f64) -> Self { self.tilt_y = Some(tilt_y); self }
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range \[0,359\] (default: 0).
    pub fn twist(mut self, twist: i64) -> Self { self.twist = Some(twist); self }
    /// Identifier used to track touch sources between events, must be unique within an event.
    pub fn id(mut self, id: f64) -> Self { self.id = Some(id); self }
    pub fn build(self) -> TouchPoint {
        TouchPoint {
            x: self.x,
            y: self.y,
            radius_x: self.radius_x,
            radius_y: self.radius_y,
            rotation_angle: self.rotation_angle,
            force: self.force,
            tangential_pressure: self.tangential_pressure,
            tilt_x: self.tilt_x,
            tilt_y: self.tilt_y,
            twist: self.twist,
            id: self.id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum GestureSourceType {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "touch")]
    Touch,
    #[serde(rename = "mouse")]
    Mouse,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MouseButton {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "back")]
    Back,
    #[serde(rename = "forward")]
    Forward,
}

/// UTC time in seconds, counted from January 1, 1970.

pub type TimeSinceEpoch = f64;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DragDataItem<'a> {
    /// Mime type of the dragged data.
    #[serde(rename = "mimeType")]
    mime_type: Cow<'a, str>,
    /// Depending of the value of 'mimeType', it contains the dragged link,
    /// text, HTML markup or any other data.
    data: Cow<'a, str>,
    /// Title associated with a link. Only valid when 'mimeType' == "text/uri-list".
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Cow<'a, str>>,
    /// Stores the base URL for the contained markup. Only valid when 'mimeType'
    /// == "text/html".
    #[serde(skip_serializing_if = "Option::is_none", rename = "baseURL")]
    base_url: Option<Cow<'a, str>>,
}

impl<'a> DragDataItem<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `mime_type`: Mime type of the dragged data.
    /// * `data`: Depending of the value of `mimeType`, it contains the dragged link, text, HTML markup or any other data.
    pub fn builder(mime_type: impl Into<Cow<'a, str>>, data: impl Into<Cow<'a, str>>) -> DragDataItemBuilder<'a> {
        DragDataItemBuilder {
            mime_type: mime_type.into(),
            data: data.into(),
            title: None,
            base_url: None,
        }
    }
    /// Mime type of the dragged data.
    pub fn mime_type(&self) -> &str { self.mime_type.as_ref() }
    /// Depending of the value of 'mimeType', it contains the dragged link,
    /// text, HTML markup or any other data.
    pub fn data(&self) -> &str { self.data.as_ref() }
    /// Title associated with a link. Only valid when 'mimeType' == "text/uri-list".
    pub fn title(&self) -> Option<&str> { self.title.as_deref() }
    /// Stores the base URL for the contained markup. Only valid when 'mimeType'
    /// == "text/html".
    pub fn base_url(&self) -> Option<&str> { self.base_url.as_deref() }
}


pub struct DragDataItemBuilder<'a> {
    mime_type: Cow<'a, str>,
    data: Cow<'a, str>,
    title: Option<Cow<'a, str>>,
    base_url: Option<Cow<'a, str>>,
}

impl<'a> DragDataItemBuilder<'a> {
    /// Title associated with a link. Only valid when 'mimeType' == "text/uri-list".
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self { self.title = Some(title.into()); self }
    /// Stores the base URL for the contained markup. Only valid when 'mimeType'
    /// == "text/html".
    pub fn base_url(mut self, base_url: impl Into<Cow<'a, str>>) -> Self { self.base_url = Some(base_url.into()); self }
    pub fn build(self) -> DragDataItem<'a> {
        DragDataItem {
            mime_type: self.mime_type,
            data: self.data,
            title: self.title,
            base_url: self.base_url,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DragData<'a> {
    items: Vec<DragDataItem<'a>>,
    /// List of filenames that should be included when dropping
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Vec<Cow<'a, str>>>,
    /// Bit field representing allowed drag operations. Copy = 1, Link = 2, Move = 16
    #[serde(rename = "dragOperationsMask")]
    drag_operations_mask: i64,
}

impl<'a> DragData<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `items`: 
    /// * `drag_operations_mask`: Bit field representing allowed drag operations. Copy = 1, Link = 2, Move = 16
    pub fn builder(items: Vec<DragDataItem<'a>>, drag_operations_mask: i64) -> DragDataBuilder<'a> {
        DragDataBuilder {
            items: items,
            files: None,
            drag_operations_mask: drag_operations_mask,
        }
    }
    pub fn items(&self) -> &[DragDataItem<'a>] { &self.items }
    /// List of filenames that should be included when dropping
    pub fn files(&self) -> Option<&[Cow<'a, str>]> { self.files.as_deref() }
    /// Bit field representing allowed drag operations. Copy = 1, Link = 2, Move = 16
    pub fn drag_operations_mask(&self) -> i64 { self.drag_operations_mask }
}


pub struct DragDataBuilder<'a> {
    items: Vec<DragDataItem<'a>>,
    files: Option<Vec<Cow<'a, str>>>,
    drag_operations_mask: i64,
}

impl<'a> DragDataBuilder<'a> {
    /// List of filenames that should be included when dropping
    pub fn files(mut self, files: Vec<Cow<'a, str>>) -> Self { self.files = Some(files); self }
    pub fn build(self) -> DragData<'a> {
        DragData {
            items: self.items,
            files: self.files,
            drag_operations_mask: self.drag_operations_mask,
        }
    }
}

/// Dispatches a drag event into the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchDragEventParams<'a> {
    /// Type of the drag event.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// X coordinate of the event relative to the main frame's viewport in CSS pixels.
    x: f64,
    /// Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    /// the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    y: f64,
    data: DragData<'a>,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    modifiers: Option<i64>,
}

impl<'a> DispatchDragEventParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Type of the drag event.
    /// * `x`: X coordinate of the event relative to the main frame's viewport in CSS pixels.
    /// * `y`: Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    /// * `data`: 
    pub fn builder(type_: impl Into<Cow<'a, str>>, x: f64, y: f64, data: DragData<'a>) -> DispatchDragEventParamsBuilder<'a> {
        DispatchDragEventParamsBuilder {
            type_: type_.into(),
            x: x,
            y: y,
            data: data,
            modifiers: None,
        }
    }
    /// Type of the drag event.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// X coordinate of the event relative to the main frame's viewport in CSS pixels.
    pub fn x(&self) -> f64 { self.x }
    /// Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    /// the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    pub fn y(&self) -> f64 { self.y }
    pub fn data(&self) -> &DragData<'a> { &self.data }
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(&self) -> Option<i64> { self.modifiers }
}


pub struct DispatchDragEventParamsBuilder<'a> {
    type_: Cow<'a, str>,
    x: f64,
    y: f64,
    data: DragData<'a>,
    modifiers: Option<i64>,
}

impl<'a> DispatchDragEventParamsBuilder<'a> {
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(mut self, modifiers: i64) -> Self { self.modifiers = Some(modifiers); self }
    pub fn build(self) -> DispatchDragEventParams<'a> {
        DispatchDragEventParams {
            type_: self.type_,
            x: self.x,
            y: self.y,
            data: self.data,
            modifiers: self.modifiers,
        }
    }
}

impl<'a> DispatchDragEventParams<'a> { pub const METHOD: &'static str = "Input.dispatchDragEvent"; }

impl<'a> crate::CdpCommand<'a> for DispatchDragEventParams<'a> {
    const METHOD: &'static str = "Input.dispatchDragEvent";
    type Response = crate::EmptyReturns;
}

/// Dispatches a key event to the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchKeyEventParams<'a> {
    /// Type of the key event.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    modifiers: Option<i64>,
    /// Time at which the event occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<TimeSinceEpoch>,
    /// Text as generated by processing a virtual key code with a keyboard layout. Not needed for
    /// for 'keyUp' and 'rawKeyDown' events (default: "")
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Cow<'a, str>>,
    /// Text that would have been generated by the keyboard if no modifiers were pressed (except for
    /// shift). Useful for shortcut (accelerator) key handling (default: "").
    #[serde(skip_serializing_if = "Option::is_none", rename = "unmodifiedText")]
    unmodified_text: Option<Cow<'a, str>>,
    /// Unique key identifier (e.g., 'U+0041') (default: "").
    #[serde(skip_serializing_if = "Option::is_none", rename = "keyIdentifier")]
    key_identifier: Option<Cow<'a, str>>,
    /// Unique DOM defined string value for each physical key (e.g., 'KeyA') (default: "").
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<Cow<'a, str>>,
    /// Unique DOM defined string value describing the meaning of the key in the context of active
    /// modifiers, keyboard layout, etc (e.g., 'AltGr') (default: "").
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<Cow<'a, str>>,
    /// Windows virtual key code (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "windowsVirtualKeyCode")]
    windows_virtual_key_code: Option<i64>,
    /// Native virtual key code (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "nativeVirtualKeyCode")]
    native_virtual_key_code: Option<i64>,
    /// Whether the event was generated from auto repeat (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "autoRepeat")]
    auto_repeat: Option<bool>,
    /// Whether the event was generated from the keypad (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "isKeypad")]
    is_keypad: Option<bool>,
    /// Whether the event was a system key event (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "isSystemKey")]
    is_system_key: Option<bool>,
    /// Whether the event was from the left or right side of the keyboard. 1=Left, 2=Right (default:
    /// 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<i64>,
    /// Editing commands to send with the key event (e.g., 'selectAll') (default: \[\]).
    /// These are related to but not equal the command names used in 'document.execCommand' and NSStandardKeyBindingResponding.
    /// See <https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/editing/commands/editor_command_names.h> for valid command names.
    #[serde(skip_serializing_if = "Option::is_none")]
    commands: Option<Vec<Cow<'a, str>>>,
}

impl<'a> DispatchKeyEventParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Type of the key event.
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> DispatchKeyEventParamsBuilder<'a> {
        DispatchKeyEventParamsBuilder {
            type_: type_.into(),
            modifiers: None,
            timestamp: None,
            text: None,
            unmodified_text: None,
            key_identifier: None,
            code: None,
            key: None,
            windows_virtual_key_code: None,
            native_virtual_key_code: None,
            auto_repeat: None,
            is_keypad: None,
            is_system_key: None,
            location: None,
            commands: None,
        }
    }
    /// Type of the key event.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(&self) -> Option<i64> { self.modifiers }
    /// Time at which the event occurred.
    pub fn timestamp(&self) -> Option<&TimeSinceEpoch> { self.timestamp.as_ref() }
    /// Text as generated by processing a virtual key code with a keyboard layout. Not needed for
    /// for 'keyUp' and 'rawKeyDown' events (default: "")
    pub fn text(&self) -> Option<&str> { self.text.as_deref() }
    /// Text that would have been generated by the keyboard if no modifiers were pressed (except for
    /// shift). Useful for shortcut (accelerator) key handling (default: "").
    pub fn unmodified_text(&self) -> Option<&str> { self.unmodified_text.as_deref() }
    /// Unique key identifier (e.g., 'U+0041') (default: "").
    pub fn key_identifier(&self) -> Option<&str> { self.key_identifier.as_deref() }
    /// Unique DOM defined string value for each physical key (e.g., 'KeyA') (default: "").
    pub fn code(&self) -> Option<&str> { self.code.as_deref() }
    /// Unique DOM defined string value describing the meaning of the key in the context of active
    /// modifiers, keyboard layout, etc (e.g., 'AltGr') (default: "").
    pub fn key(&self) -> Option<&str> { self.key.as_deref() }
    /// Windows virtual key code (default: 0).
    pub fn windows_virtual_key_code(&self) -> Option<i64> { self.windows_virtual_key_code }
    /// Native virtual key code (default: 0).
    pub fn native_virtual_key_code(&self) -> Option<i64> { self.native_virtual_key_code }
    /// Whether the event was generated from auto repeat (default: false).
    pub fn auto_repeat(&self) -> Option<bool> { self.auto_repeat }
    /// Whether the event was generated from the keypad (default: false).
    pub fn is_keypad(&self) -> Option<bool> { self.is_keypad }
    /// Whether the event was a system key event (default: false).
    pub fn is_system_key(&self) -> Option<bool> { self.is_system_key }
    /// Whether the event was from the left or right side of the keyboard. 1=Left, 2=Right (default:
    /// 0).
    pub fn location(&self) -> Option<i64> { self.location }
    /// Editing commands to send with the key event (e.g., 'selectAll') (default: \[\]).
    /// These are related to but not equal the command names used in 'document.execCommand' and NSStandardKeyBindingResponding.
    /// See <https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/editing/commands/editor_command_names.h> for valid command names.
    pub fn commands(&self) -> Option<&[Cow<'a, str>]> { self.commands.as_deref() }
}


pub struct DispatchKeyEventParamsBuilder<'a> {
    type_: Cow<'a, str>,
    modifiers: Option<i64>,
    timestamp: Option<TimeSinceEpoch>,
    text: Option<Cow<'a, str>>,
    unmodified_text: Option<Cow<'a, str>>,
    key_identifier: Option<Cow<'a, str>>,
    code: Option<Cow<'a, str>>,
    key: Option<Cow<'a, str>>,
    windows_virtual_key_code: Option<i64>,
    native_virtual_key_code: Option<i64>,
    auto_repeat: Option<bool>,
    is_keypad: Option<bool>,
    is_system_key: Option<bool>,
    location: Option<i64>,
    commands: Option<Vec<Cow<'a, str>>>,
}

impl<'a> DispatchKeyEventParamsBuilder<'a> {
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(mut self, modifiers: i64) -> Self { self.modifiers = Some(modifiers); self }
    /// Time at which the event occurred.
    pub fn timestamp(mut self, timestamp: TimeSinceEpoch) -> Self { self.timestamp = Some(timestamp); self }
    /// Text as generated by processing a virtual key code with a keyboard layout. Not needed for
    /// for 'keyUp' and 'rawKeyDown' events (default: "")
    pub fn text(mut self, text: impl Into<Cow<'a, str>>) -> Self { self.text = Some(text.into()); self }
    /// Text that would have been generated by the keyboard if no modifiers were pressed (except for
    /// shift). Useful for shortcut (accelerator) key handling (default: "").
    pub fn unmodified_text(mut self, unmodified_text: impl Into<Cow<'a, str>>) -> Self { self.unmodified_text = Some(unmodified_text.into()); self }
    /// Unique key identifier (e.g., 'U+0041') (default: "").
    pub fn key_identifier(mut self, key_identifier: impl Into<Cow<'a, str>>) -> Self { self.key_identifier = Some(key_identifier.into()); self }
    /// Unique DOM defined string value for each physical key (e.g., 'KeyA') (default: "").
    pub fn code(mut self, code: impl Into<Cow<'a, str>>) -> Self { self.code = Some(code.into()); self }
    /// Unique DOM defined string value describing the meaning of the key in the context of active
    /// modifiers, keyboard layout, etc (e.g., 'AltGr') (default: "").
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self { self.key = Some(key.into()); self }
    /// Windows virtual key code (default: 0).
    pub fn windows_virtual_key_code(mut self, windows_virtual_key_code: i64) -> Self { self.windows_virtual_key_code = Some(windows_virtual_key_code); self }
    /// Native virtual key code (default: 0).
    pub fn native_virtual_key_code(mut self, native_virtual_key_code: i64) -> Self { self.native_virtual_key_code = Some(native_virtual_key_code); self }
    /// Whether the event was generated from auto repeat (default: false).
    pub fn auto_repeat(mut self, auto_repeat: bool) -> Self { self.auto_repeat = Some(auto_repeat); self }
    /// Whether the event was generated from the keypad (default: false).
    pub fn is_keypad(mut self, is_keypad: bool) -> Self { self.is_keypad = Some(is_keypad); self }
    /// Whether the event was a system key event (default: false).
    pub fn is_system_key(mut self, is_system_key: bool) -> Self { self.is_system_key = Some(is_system_key); self }
    /// Whether the event was from the left or right side of the keyboard. 1=Left, 2=Right (default:
    /// 0).
    pub fn location(mut self, location: i64) -> Self { self.location = Some(location); self }
    /// Editing commands to send with the key event (e.g., 'selectAll') (default: \[\]).
    /// These are related to but not equal the command names used in 'document.execCommand' and NSStandardKeyBindingResponding.
    /// See <https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/editing/commands/editor_command_names.h> for valid command names.
    pub fn commands(mut self, commands: Vec<Cow<'a, str>>) -> Self { self.commands = Some(commands); self }
    pub fn build(self) -> DispatchKeyEventParams<'a> {
        DispatchKeyEventParams {
            type_: self.type_,
            modifiers: self.modifiers,
            timestamp: self.timestamp,
            text: self.text,
            unmodified_text: self.unmodified_text,
            key_identifier: self.key_identifier,
            code: self.code,
            key: self.key,
            windows_virtual_key_code: self.windows_virtual_key_code,
            native_virtual_key_code: self.native_virtual_key_code,
            auto_repeat: self.auto_repeat,
            is_keypad: self.is_keypad,
            is_system_key: self.is_system_key,
            location: self.location,
            commands: self.commands,
        }
    }
}

impl<'a> DispatchKeyEventParams<'a> { pub const METHOD: &'static str = "Input.dispatchKeyEvent"; }

impl<'a> crate::CdpCommand<'a> for DispatchKeyEventParams<'a> {
    const METHOD: &'static str = "Input.dispatchKeyEvent";
    type Response = crate::EmptyReturns;
}

/// This method emulates inserting text that doesn't come from a key press,
/// for example an emoji keyboard or an IME.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextParams<'a> {
    /// The text to insert.
    text: Cow<'a, str>,
}

impl<'a> InsertTextParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `text`: The text to insert.
    pub fn builder(text: impl Into<Cow<'a, str>>) -> InsertTextParamsBuilder<'a> {
        InsertTextParamsBuilder {
            text: text.into(),
        }
    }
    /// The text to insert.
    pub fn text(&self) -> &str { self.text.as_ref() }
}


pub struct InsertTextParamsBuilder<'a> {
    text: Cow<'a, str>,
}

impl<'a> InsertTextParamsBuilder<'a> {
    pub fn build(self) -> InsertTextParams<'a> {
        InsertTextParams {
            text: self.text,
        }
    }
}

impl<'a> InsertTextParams<'a> { pub const METHOD: &'static str = "Input.insertText"; }

impl<'a> crate::CdpCommand<'a> for InsertTextParams<'a> {
    const METHOD: &'static str = "Input.insertText";
    type Response = crate::EmptyReturns;
}

/// This method sets the current candidate text for IME.
/// Use imeCommitComposition to commit the final text.
/// Use imeSetComposition with empty string as text to cancel composition.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImeSetCompositionParams<'a> {
    /// The text to insert
    text: Cow<'a, str>,
    /// selection start
    #[serde(rename = "selectionStart")]
    selection_start: i64,
    /// selection end
    #[serde(rename = "selectionEnd")]
    selection_end: i64,
    /// replacement start
    #[serde(skip_serializing_if = "Option::is_none", rename = "replacementStart")]
    replacement_start: Option<i64>,
    /// replacement end
    #[serde(skip_serializing_if = "Option::is_none", rename = "replacementEnd")]
    replacement_end: Option<i64>,
}

impl<'a> ImeSetCompositionParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `text`: The text to insert
    /// * `selection_start`: selection start
    /// * `selection_end`: selection end
    pub fn builder(text: impl Into<Cow<'a, str>>, selection_start: i64, selection_end: i64) -> ImeSetCompositionParamsBuilder<'a> {
        ImeSetCompositionParamsBuilder {
            text: text.into(),
            selection_start: selection_start,
            selection_end: selection_end,
            replacement_start: None,
            replacement_end: None,
        }
    }
    /// The text to insert
    pub fn text(&self) -> &str { self.text.as_ref() }
    /// selection start
    pub fn selection_start(&self) -> i64 { self.selection_start }
    /// selection end
    pub fn selection_end(&self) -> i64 { self.selection_end }
    /// replacement start
    pub fn replacement_start(&self) -> Option<i64> { self.replacement_start }
    /// replacement end
    pub fn replacement_end(&self) -> Option<i64> { self.replacement_end }
}


pub struct ImeSetCompositionParamsBuilder<'a> {
    text: Cow<'a, str>,
    selection_start: i64,
    selection_end: i64,
    replacement_start: Option<i64>,
    replacement_end: Option<i64>,
}

impl<'a> ImeSetCompositionParamsBuilder<'a> {
    /// replacement start
    pub fn replacement_start(mut self, replacement_start: i64) -> Self { self.replacement_start = Some(replacement_start); self }
    /// replacement end
    pub fn replacement_end(mut self, replacement_end: i64) -> Self { self.replacement_end = Some(replacement_end); self }
    pub fn build(self) -> ImeSetCompositionParams<'a> {
        ImeSetCompositionParams {
            text: self.text,
            selection_start: self.selection_start,
            selection_end: self.selection_end,
            replacement_start: self.replacement_start,
            replacement_end: self.replacement_end,
        }
    }
}

impl<'a> ImeSetCompositionParams<'a> { pub const METHOD: &'static str = "Input.imeSetComposition"; }

impl<'a> crate::CdpCommand<'a> for ImeSetCompositionParams<'a> {
    const METHOD: &'static str = "Input.imeSetComposition";
    type Response = crate::EmptyReturns;
}

/// Dispatches a mouse event to the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchMouseEventParams<'a> {
    /// Type of the mouse event.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// X coordinate of the event relative to the main frame's viewport in CSS pixels.
    x: f64,
    /// Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    /// the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    y: f64,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    modifiers: Option<i64>,
    /// Time at which the event occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<TimeSinceEpoch>,
    /// Mouse button (default: "none").
    #[serde(skip_serializing_if = "Option::is_none")]
    button: Option<MouseButton>,
    /// A number indicating which buttons are pressed on the mouse when a mouse event is triggered.
    /// Left=1, Right=2, Middle=4, Back=8, Forward=16, None=0.
    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<i64>,
    /// Number of times the mouse button was clicked (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "clickCount")]
    click_count: Option<u64>,
    /// The normalized pressure, which has a range of \[0,1\] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<f64>,
    /// The normalized tangential pressure, which has a range of \[-1,1\] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tangentialPressure")]
    tangential_pressure: Option<f64>,
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range \[-90,90\], a positive tiltX is to the right (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tiltX")]
    tilt_x: Option<f64>,
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range \[-90,90\], a positive tiltY is towards the user (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tiltY")]
    tilt_y: Option<f64>,
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range \[0,359\] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    twist: Option<i64>,
    /// X delta in CSS pixels for mouse wheel event (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "deltaX")]
    delta_x: Option<f64>,
    /// Y delta in CSS pixels for mouse wheel event (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "deltaY")]
    delta_y: Option<f64>,
    /// Pointer type (default: "mouse").
    #[serde(skip_serializing_if = "Option::is_none", rename = "pointerType")]
    pointer_type: Option<Cow<'a, str>>,
}

impl<'a> DispatchMouseEventParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Type of the mouse event.
    /// * `x`: X coordinate of the event relative to the main frame's viewport in CSS pixels.
    /// * `y`: Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    pub fn builder(type_: impl Into<Cow<'a, str>>, x: f64, y: f64) -> DispatchMouseEventParamsBuilder<'a> {
        DispatchMouseEventParamsBuilder {
            type_: type_.into(),
            x: x,
            y: y,
            modifiers: None,
            timestamp: None,
            button: None,
            buttons: None,
            click_count: None,
            force: None,
            tangential_pressure: None,
            tilt_x: None,
            tilt_y: None,
            twist: None,
            delta_x: None,
            delta_y: None,
            pointer_type: None,
        }
    }
    /// Type of the mouse event.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// X coordinate of the event relative to the main frame's viewport in CSS pixels.
    pub fn x(&self) -> f64 { self.x }
    /// Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    /// the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    pub fn y(&self) -> f64 { self.y }
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(&self) -> Option<i64> { self.modifiers }
    /// Time at which the event occurred.
    pub fn timestamp(&self) -> Option<&TimeSinceEpoch> { self.timestamp.as_ref() }
    /// Mouse button (default: "none").
    pub fn button(&self) -> Option<&MouseButton> { self.button.as_ref() }
    /// A number indicating which buttons are pressed on the mouse when a mouse event is triggered.
    /// Left=1, Right=2, Middle=4, Back=8, Forward=16, None=0.
    pub fn buttons(&self) -> Option<i64> { self.buttons }
    /// Number of times the mouse button was clicked (default: 0).
    pub fn click_count(&self) -> Option<u64> { self.click_count }
    /// The normalized pressure, which has a range of \[0,1\] (default: 0).
    pub fn force(&self) -> Option<f64> { self.force }
    /// The normalized tangential pressure, which has a range of \[-1,1\] (default: 0).
    pub fn tangential_pressure(&self) -> Option<f64> { self.tangential_pressure }
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range \[-90,90\], a positive tiltX is to the right (default: 0).
    pub fn tilt_x(&self) -> Option<f64> { self.tilt_x }
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range \[-90,90\], a positive tiltY is towards the user (default: 0).
    pub fn tilt_y(&self) -> Option<f64> { self.tilt_y }
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range \[0,359\] (default: 0).
    pub fn twist(&self) -> Option<i64> { self.twist }
    /// X delta in CSS pixels for mouse wheel event (default: 0).
    pub fn delta_x(&self) -> Option<f64> { self.delta_x }
    /// Y delta in CSS pixels for mouse wheel event (default: 0).
    pub fn delta_y(&self) -> Option<f64> { self.delta_y }
    /// Pointer type (default: "mouse").
    pub fn pointer_type(&self) -> Option<&str> { self.pointer_type.as_deref() }
}


pub struct DispatchMouseEventParamsBuilder<'a> {
    type_: Cow<'a, str>,
    x: f64,
    y: f64,
    modifiers: Option<i64>,
    timestamp: Option<TimeSinceEpoch>,
    button: Option<MouseButton>,
    buttons: Option<i64>,
    click_count: Option<u64>,
    force: Option<f64>,
    tangential_pressure: Option<f64>,
    tilt_x: Option<f64>,
    tilt_y: Option<f64>,
    twist: Option<i64>,
    delta_x: Option<f64>,
    delta_y: Option<f64>,
    pointer_type: Option<Cow<'a, str>>,
}

impl<'a> DispatchMouseEventParamsBuilder<'a> {
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(mut self, modifiers: i64) -> Self { self.modifiers = Some(modifiers); self }
    /// Time at which the event occurred.
    pub fn timestamp(mut self, timestamp: TimeSinceEpoch) -> Self { self.timestamp = Some(timestamp); self }
    /// Mouse button (default: "none").
    pub fn button(mut self, button: impl Into<MouseButton>) -> Self { self.button = Some(button.into()); self }
    /// A number indicating which buttons are pressed on the mouse when a mouse event is triggered.
    /// Left=1, Right=2, Middle=4, Back=8, Forward=16, None=0.
    pub fn buttons(mut self, buttons: i64) -> Self { self.buttons = Some(buttons); self }
    /// Number of times the mouse button was clicked (default: 0).
    pub fn click_count(mut self, click_count: u64) -> Self { self.click_count = Some(click_count); self }
    /// The normalized pressure, which has a range of \[0,1\] (default: 0).
    pub fn force(mut self, force: f64) -> Self { self.force = Some(force); self }
    /// The normalized tangential pressure, which has a range of \[-1,1\] (default: 0).
    pub fn tangential_pressure(mut self, tangential_pressure: f64) -> Self { self.tangential_pressure = Some(tangential_pressure); self }
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range \[-90,90\], a positive tiltX is to the right (default: 0).
    pub fn tilt_x(mut self, tilt_x: f64) -> Self { self.tilt_x = Some(tilt_x); self }
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range \[-90,90\], a positive tiltY is towards the user (default: 0).
    pub fn tilt_y(mut self, tilt_y: f64) -> Self { self.tilt_y = Some(tilt_y); self }
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range \[0,359\] (default: 0).
    pub fn twist(mut self, twist: i64) -> Self { self.twist = Some(twist); self }
    /// X delta in CSS pixels for mouse wheel event (default: 0).
    pub fn delta_x(mut self, delta_x: f64) -> Self { self.delta_x = Some(delta_x); self }
    /// Y delta in CSS pixels for mouse wheel event (default: 0).
    pub fn delta_y(mut self, delta_y: f64) -> Self { self.delta_y = Some(delta_y); self }
    /// Pointer type (default: "mouse").
    pub fn pointer_type(mut self, pointer_type: impl Into<Cow<'a, str>>) -> Self { self.pointer_type = Some(pointer_type.into()); self }
    pub fn build(self) -> DispatchMouseEventParams<'a> {
        DispatchMouseEventParams {
            type_: self.type_,
            x: self.x,
            y: self.y,
            modifiers: self.modifiers,
            timestamp: self.timestamp,
            button: self.button,
            buttons: self.buttons,
            click_count: self.click_count,
            force: self.force,
            tangential_pressure: self.tangential_pressure,
            tilt_x: self.tilt_x,
            tilt_y: self.tilt_y,
            twist: self.twist,
            delta_x: self.delta_x,
            delta_y: self.delta_y,
            pointer_type: self.pointer_type,
        }
    }
}

impl<'a> DispatchMouseEventParams<'a> { pub const METHOD: &'static str = "Input.dispatchMouseEvent"; }

impl<'a> crate::CdpCommand<'a> for DispatchMouseEventParams<'a> {
    const METHOD: &'static str = "Input.dispatchMouseEvent";
    type Response = crate::EmptyReturns;
}

/// Dispatches a touch event to the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchTouchEventParams<'a> {
    /// Type of the touch event. TouchEnd and TouchCancel must not contain any touch points, while
    /// TouchStart and TouchMove must contains at least one.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// Active touch points on the touch device. One event per any changed point (compared to
    /// previous touch event in a sequence) is generated, emulating pressing/moving/releasing points
    /// one by one.
    #[serde(rename = "touchPoints")]
    touch_points: Vec<TouchPoint>,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    modifiers: Option<i64>,
    /// Time at which the event occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<TimeSinceEpoch>,
}

impl<'a> DispatchTouchEventParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Type of the touch event. TouchEnd and TouchCancel must not contain any touch points, while TouchStart and TouchMove must contains at least one.
    /// * `touch_points`: Active touch points on the touch device. One event per any changed point (compared to previous touch event in a sequence) is generated, emulating pressing/moving/releasing points one by one.
    pub fn builder(type_: impl Into<Cow<'a, str>>, touch_points: Vec<TouchPoint>) -> DispatchTouchEventParamsBuilder<'a> {
        DispatchTouchEventParamsBuilder {
            type_: type_.into(),
            touch_points: touch_points,
            modifiers: None,
            timestamp: None,
        }
    }
    /// Type of the touch event. TouchEnd and TouchCancel must not contain any touch points, while
    /// TouchStart and TouchMove must contains at least one.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// Active touch points on the touch device. One event per any changed point (compared to
    /// previous touch event in a sequence) is generated, emulating pressing/moving/releasing points
    /// one by one.
    pub fn touch_points(&self) -> &[TouchPoint] { &self.touch_points }
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(&self) -> Option<i64> { self.modifiers }
    /// Time at which the event occurred.
    pub fn timestamp(&self) -> Option<&TimeSinceEpoch> { self.timestamp.as_ref() }
}


pub struct DispatchTouchEventParamsBuilder<'a> {
    type_: Cow<'a, str>,
    touch_points: Vec<TouchPoint>,
    modifiers: Option<i64>,
    timestamp: Option<TimeSinceEpoch>,
}

impl<'a> DispatchTouchEventParamsBuilder<'a> {
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(mut self, modifiers: i64) -> Self { self.modifiers = Some(modifiers); self }
    /// Time at which the event occurred.
    pub fn timestamp(mut self, timestamp: TimeSinceEpoch) -> Self { self.timestamp = Some(timestamp); self }
    pub fn build(self) -> DispatchTouchEventParams<'a> {
        DispatchTouchEventParams {
            type_: self.type_,
            touch_points: self.touch_points,
            modifiers: self.modifiers,
            timestamp: self.timestamp,
        }
    }
}

impl<'a> DispatchTouchEventParams<'a> { pub const METHOD: &'static str = "Input.dispatchTouchEvent"; }

impl<'a> crate::CdpCommand<'a> for DispatchTouchEventParams<'a> {
    const METHOD: &'static str = "Input.dispatchTouchEvent";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CancelDraggingParams {}

impl CancelDraggingParams { pub const METHOD: &'static str = "Input.cancelDragging"; }

impl<'a> crate::CdpCommand<'a> for CancelDraggingParams {
    const METHOD: &'static str = "Input.cancelDragging";
    type Response = crate::EmptyReturns;
}

/// Emulates touch event from the mouse event parameters.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmulateTouchFromMouseEventParams<'a> {
    /// Type of the mouse event.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// X coordinate of the mouse pointer in DIP.
    x: i32,
    /// Y coordinate of the mouse pointer in DIP.
    y: i32,
    /// Mouse button. Only "none", "left", "right" are supported.
    button: MouseButton,
    /// Time at which the event occurred (default: current time).
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<TimeSinceEpoch>,
    /// X delta in DIP for mouse wheel event (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "deltaX")]
    delta_x: Option<f64>,
    /// Y delta in DIP for mouse wheel event (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "deltaY")]
    delta_y: Option<f64>,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    modifiers: Option<i64>,
    /// Number of times the mouse button was clicked (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "clickCount")]
    click_count: Option<u64>,
}

impl<'a> EmulateTouchFromMouseEventParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Type of the mouse event.
    /// * `x`: X coordinate of the mouse pointer in DIP.
    /// * `y`: Y coordinate of the mouse pointer in DIP.
    /// * `button`: Mouse button. Only "none", "left", "right" are supported.
    pub fn builder(type_: impl Into<Cow<'a, str>>, x: i32, y: i32, button: impl Into<MouseButton>) -> EmulateTouchFromMouseEventParamsBuilder<'a> {
        EmulateTouchFromMouseEventParamsBuilder {
            type_: type_.into(),
            x: x,
            y: y,
            button: button.into(),
            timestamp: None,
            delta_x: None,
            delta_y: None,
            modifiers: None,
            click_count: None,
        }
    }
    /// Type of the mouse event.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// X coordinate of the mouse pointer in DIP.
    pub fn x(&self) -> i32 { self.x }
    /// Y coordinate of the mouse pointer in DIP.
    pub fn y(&self) -> i32 { self.y }
    /// Mouse button. Only "none", "left", "right" are supported.
    pub fn button(&self) -> &MouseButton { &self.button }
    /// Time at which the event occurred (default: current time).
    pub fn timestamp(&self) -> Option<&TimeSinceEpoch> { self.timestamp.as_ref() }
    /// X delta in DIP for mouse wheel event (default: 0).
    pub fn delta_x(&self) -> Option<f64> { self.delta_x }
    /// Y delta in DIP for mouse wheel event (default: 0).
    pub fn delta_y(&self) -> Option<f64> { self.delta_y }
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(&self) -> Option<i64> { self.modifiers }
    /// Number of times the mouse button was clicked (default: 0).
    pub fn click_count(&self) -> Option<u64> { self.click_count }
}


pub struct EmulateTouchFromMouseEventParamsBuilder<'a> {
    type_: Cow<'a, str>,
    x: i32,
    y: i32,
    button: MouseButton,
    timestamp: Option<TimeSinceEpoch>,
    delta_x: Option<f64>,
    delta_y: Option<f64>,
    modifiers: Option<i64>,
    click_count: Option<u64>,
}

impl<'a> EmulateTouchFromMouseEventParamsBuilder<'a> {
    /// Time at which the event occurred (default: current time).
    pub fn timestamp(mut self, timestamp: TimeSinceEpoch) -> Self { self.timestamp = Some(timestamp); self }
    /// X delta in DIP for mouse wheel event (default: 0).
    pub fn delta_x(mut self, delta_x: f64) -> Self { self.delta_x = Some(delta_x); self }
    /// Y delta in DIP for mouse wheel event (default: 0).
    pub fn delta_y(mut self, delta_y: f64) -> Self { self.delta_y = Some(delta_y); self }
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(mut self, modifiers: i64) -> Self { self.modifiers = Some(modifiers); self }
    /// Number of times the mouse button was clicked (default: 0).
    pub fn click_count(mut self, click_count: u64) -> Self { self.click_count = Some(click_count); self }
    pub fn build(self) -> EmulateTouchFromMouseEventParams<'a> {
        EmulateTouchFromMouseEventParams {
            type_: self.type_,
            x: self.x,
            y: self.y,
            button: self.button,
            timestamp: self.timestamp,
            delta_x: self.delta_x,
            delta_y: self.delta_y,
            modifiers: self.modifiers,
            click_count: self.click_count,
        }
    }
}

impl<'a> EmulateTouchFromMouseEventParams<'a> { pub const METHOD: &'static str = "Input.emulateTouchFromMouseEvent"; }

impl<'a> crate::CdpCommand<'a> for EmulateTouchFromMouseEventParams<'a> {
    const METHOD: &'static str = "Input.emulateTouchFromMouseEvent";
    type Response = crate::EmptyReturns;
}

/// Ignores input events (useful while auditing page).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetIgnoreInputEventsParams {
    /// Ignores input events processing when set to true.
    ignore: bool,
}

impl SetIgnoreInputEventsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `ignore`: Ignores input events processing when set to true.
    pub fn builder(ignore: bool) -> SetIgnoreInputEventsParamsBuilder {
        SetIgnoreInputEventsParamsBuilder {
            ignore: ignore,
        }
    }
    /// Ignores input events processing when set to true.
    pub fn ignore(&self) -> bool { self.ignore }
}


pub struct SetIgnoreInputEventsParamsBuilder {
    ignore: bool,
}

impl SetIgnoreInputEventsParamsBuilder {
    pub fn build(self) -> SetIgnoreInputEventsParams {
        SetIgnoreInputEventsParams {
            ignore: self.ignore,
        }
    }
}

impl SetIgnoreInputEventsParams { pub const METHOD: &'static str = "Input.setIgnoreInputEvents"; }

impl<'a> crate::CdpCommand<'a> for SetIgnoreInputEventsParams {
    const METHOD: &'static str = "Input.setIgnoreInputEvents";
    type Response = crate::EmptyReturns;
}

/// Prevents default drag and drop behavior and instead emits 'Input.dragIntercepted' events.
/// Drag and drop behavior can be directly controlled via 'Input.dispatchDragEvent'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInterceptDragsParams {
    enabled: bool,
}

impl SetInterceptDragsParams {
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: 
    pub fn builder(enabled: bool) -> SetInterceptDragsParamsBuilder {
        SetInterceptDragsParamsBuilder {
            enabled: enabled,
        }
    }
    pub fn enabled(&self) -> bool { self.enabled }
}


pub struct SetInterceptDragsParamsBuilder {
    enabled: bool,
}

impl SetInterceptDragsParamsBuilder {
    pub fn build(self) -> SetInterceptDragsParams {
        SetInterceptDragsParams {
            enabled: self.enabled,
        }
    }
}

impl SetInterceptDragsParams { pub const METHOD: &'static str = "Input.setInterceptDrags"; }

impl<'a> crate::CdpCommand<'a> for SetInterceptDragsParams {
    const METHOD: &'static str = "Input.setInterceptDrags";
    type Response = crate::EmptyReturns;
}

/// Synthesizes a pinch gesture over a time period by issuing appropriate touch events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizePinchGestureParams {
    /// X coordinate of the start of the gesture in CSS pixels.
    x: f64,
    /// Y coordinate of the start of the gesture in CSS pixels.
    y: f64,
    /// Relative scale factor after zooming (\>1.0 zooms in, \<1.0 zooms out).
    #[serde(rename = "scaleFactor")]
    scale_factor: f64,
    /// Relative pointer speed in pixels per second (default: 800).
    #[serde(skip_serializing_if = "Option::is_none", rename = "relativeSpeed")]
    relative_speed: Option<i64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gestureSourceType")]
    gesture_source_type: Option<GestureSourceType>,
}

impl SynthesizePinchGestureParams {
    /// Creates a builder for this type with the required parameters:
    /// * `x`: X coordinate of the start of the gesture in CSS pixels.
    /// * `y`: Y coordinate of the start of the gesture in CSS pixels.
    /// * `scale_factor`: Relative scale factor after zooming (\>1.0 zooms in, \<1.0 zooms out).
    pub fn builder(x: f64, y: f64, scale_factor: f64) -> SynthesizePinchGestureParamsBuilder {
        SynthesizePinchGestureParamsBuilder {
            x: x,
            y: y,
            scale_factor: scale_factor,
            relative_speed: None,
            gesture_source_type: None,
        }
    }
    /// X coordinate of the start of the gesture in CSS pixels.
    pub fn x(&self) -> f64 { self.x }
    /// Y coordinate of the start of the gesture in CSS pixels.
    pub fn y(&self) -> f64 { self.y }
    /// Relative scale factor after zooming (\>1.0 zooms in, \<1.0 zooms out).
    pub fn scale_factor(&self) -> f64 { self.scale_factor }
    /// Relative pointer speed in pixels per second (default: 800).
    pub fn relative_speed(&self) -> Option<i64> { self.relative_speed }
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    pub fn gesture_source_type(&self) -> Option<&GestureSourceType> { self.gesture_source_type.as_ref() }
}


pub struct SynthesizePinchGestureParamsBuilder {
    x: f64,
    y: f64,
    scale_factor: f64,
    relative_speed: Option<i64>,
    gesture_source_type: Option<GestureSourceType>,
}

impl SynthesizePinchGestureParamsBuilder {
    /// Relative pointer speed in pixels per second (default: 800).
    pub fn relative_speed(mut self, relative_speed: i64) -> Self { self.relative_speed = Some(relative_speed); self }
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    pub fn gesture_source_type(mut self, gesture_source_type: impl Into<GestureSourceType>) -> Self { self.gesture_source_type = Some(gesture_source_type.into()); self }
    pub fn build(self) -> SynthesizePinchGestureParams {
        SynthesizePinchGestureParams {
            x: self.x,
            y: self.y,
            scale_factor: self.scale_factor,
            relative_speed: self.relative_speed,
            gesture_source_type: self.gesture_source_type,
        }
    }
}

impl SynthesizePinchGestureParams { pub const METHOD: &'static str = "Input.synthesizePinchGesture"; }

impl<'a> crate::CdpCommand<'a> for SynthesizePinchGestureParams {
    const METHOD: &'static str = "Input.synthesizePinchGesture";
    type Response = crate::EmptyReturns;
}

/// Synthesizes a scroll gesture over a time period by issuing appropriate touch events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizeScrollGestureParams<'a> {
    /// X coordinate of the start of the gesture in CSS pixels.
    x: f64,
    /// Y coordinate of the start of the gesture in CSS pixels.
    y: f64,
    /// The distance to scroll along the X axis (positive to scroll left).
    #[serde(skip_serializing_if = "Option::is_none", rename = "xDistance")]
    x_distance: Option<f64>,
    /// The distance to scroll along the Y axis (positive to scroll up).
    #[serde(skip_serializing_if = "Option::is_none", rename = "yDistance")]
    y_distance: Option<f64>,
    /// The number of additional pixels to scroll back along the X axis, in addition to the given
    /// distance.
    #[serde(skip_serializing_if = "Option::is_none", rename = "xOverscroll")]
    x_overscroll: Option<f64>,
    /// The number of additional pixels to scroll back along the Y axis, in addition to the given
    /// distance.
    #[serde(skip_serializing_if = "Option::is_none", rename = "yOverscroll")]
    y_overscroll: Option<f64>,
    /// Prevent fling (default: true).
    #[serde(skip_serializing_if = "Option::is_none", rename = "preventFling")]
    prevent_fling: Option<bool>,
    /// Swipe speed in pixels per second (default: 800).
    #[serde(skip_serializing_if = "Option::is_none")]
    speed: Option<i64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gestureSourceType")]
    gesture_source_type: Option<GestureSourceType>,
    /// The number of times to repeat the gesture (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "repeatCount")]
    repeat_count: Option<u64>,
    /// The number of milliseconds delay between each repeat. (default: 250).
    #[serde(skip_serializing_if = "Option::is_none", rename = "repeatDelayMs")]
    repeat_delay_ms: Option<i64>,
    /// The name of the interaction markers to generate, if not empty (default: "").
    #[serde(skip_serializing_if = "Option::is_none", rename = "interactionMarkerName")]
    interaction_marker_name: Option<Cow<'a, str>>,
}

impl<'a> SynthesizeScrollGestureParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `x`: X coordinate of the start of the gesture in CSS pixels.
    /// * `y`: Y coordinate of the start of the gesture in CSS pixels.
    pub fn builder(x: f64, y: f64) -> SynthesizeScrollGestureParamsBuilder<'a> {
        SynthesizeScrollGestureParamsBuilder {
            x: x,
            y: y,
            x_distance: None,
            y_distance: None,
            x_overscroll: None,
            y_overscroll: None,
            prevent_fling: None,
            speed: None,
            gesture_source_type: None,
            repeat_count: None,
            repeat_delay_ms: None,
            interaction_marker_name: None,
        }
    }
    /// X coordinate of the start of the gesture in CSS pixels.
    pub fn x(&self) -> f64 { self.x }
    /// Y coordinate of the start of the gesture in CSS pixels.
    pub fn y(&self) -> f64 { self.y }
    /// The distance to scroll along the X axis (positive to scroll left).
    pub fn x_distance(&self) -> Option<f64> { self.x_distance }
    /// The distance to scroll along the Y axis (positive to scroll up).
    pub fn y_distance(&self) -> Option<f64> { self.y_distance }
    /// The number of additional pixels to scroll back along the X axis, in addition to the given
    /// distance.
    pub fn x_overscroll(&self) -> Option<f64> { self.x_overscroll }
    /// The number of additional pixels to scroll back along the Y axis, in addition to the given
    /// distance.
    pub fn y_overscroll(&self) -> Option<f64> { self.y_overscroll }
    /// Prevent fling (default: true).
    pub fn prevent_fling(&self) -> Option<bool> { self.prevent_fling }
    /// Swipe speed in pixels per second (default: 800).
    pub fn speed(&self) -> Option<i64> { self.speed }
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    pub fn gesture_source_type(&self) -> Option<&GestureSourceType> { self.gesture_source_type.as_ref() }
    /// The number of times to repeat the gesture (default: 0).
    pub fn repeat_count(&self) -> Option<u64> { self.repeat_count }
    /// The number of milliseconds delay between each repeat. (default: 250).
    pub fn repeat_delay_ms(&self) -> Option<i64> { self.repeat_delay_ms }
    /// The name of the interaction markers to generate, if not empty (default: "").
    pub fn interaction_marker_name(&self) -> Option<&str> { self.interaction_marker_name.as_deref() }
}


pub struct SynthesizeScrollGestureParamsBuilder<'a> {
    x: f64,
    y: f64,
    x_distance: Option<f64>,
    y_distance: Option<f64>,
    x_overscroll: Option<f64>,
    y_overscroll: Option<f64>,
    prevent_fling: Option<bool>,
    speed: Option<i64>,
    gesture_source_type: Option<GestureSourceType>,
    repeat_count: Option<u64>,
    repeat_delay_ms: Option<i64>,
    interaction_marker_name: Option<Cow<'a, str>>,
}

impl<'a> SynthesizeScrollGestureParamsBuilder<'a> {
    /// The distance to scroll along the X axis (positive to scroll left).
    pub fn x_distance(mut self, x_distance: f64) -> Self { self.x_distance = Some(x_distance); self }
    /// The distance to scroll along the Y axis (positive to scroll up).
    pub fn y_distance(mut self, y_distance: f64) -> Self { self.y_distance = Some(y_distance); self }
    /// The number of additional pixels to scroll back along the X axis, in addition to the given
    /// distance.
    pub fn x_overscroll(mut self, x_overscroll: f64) -> Self { self.x_overscroll = Some(x_overscroll); self }
    /// The number of additional pixels to scroll back along the Y axis, in addition to the given
    /// distance.
    pub fn y_overscroll(mut self, y_overscroll: f64) -> Self { self.y_overscroll = Some(y_overscroll); self }
    /// Prevent fling (default: true).
    pub fn prevent_fling(mut self, prevent_fling: bool) -> Self { self.prevent_fling = Some(prevent_fling); self }
    /// Swipe speed in pixels per second (default: 800).
    pub fn speed(mut self, speed: i64) -> Self { self.speed = Some(speed); self }
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    pub fn gesture_source_type(mut self, gesture_source_type: impl Into<GestureSourceType>) -> Self { self.gesture_source_type = Some(gesture_source_type.into()); self }
    /// The number of times to repeat the gesture (default: 0).
    pub fn repeat_count(mut self, repeat_count: u64) -> Self { self.repeat_count = Some(repeat_count); self }
    /// The number of milliseconds delay between each repeat. (default: 250).
    pub fn repeat_delay_ms(mut self, repeat_delay_ms: i64) -> Self { self.repeat_delay_ms = Some(repeat_delay_ms); self }
    /// The name of the interaction markers to generate, if not empty (default: "").
    pub fn interaction_marker_name(mut self, interaction_marker_name: impl Into<Cow<'a, str>>) -> Self { self.interaction_marker_name = Some(interaction_marker_name.into()); self }
    pub fn build(self) -> SynthesizeScrollGestureParams<'a> {
        SynthesizeScrollGestureParams {
            x: self.x,
            y: self.y,
            x_distance: self.x_distance,
            y_distance: self.y_distance,
            x_overscroll: self.x_overscroll,
            y_overscroll: self.y_overscroll,
            prevent_fling: self.prevent_fling,
            speed: self.speed,
            gesture_source_type: self.gesture_source_type,
            repeat_count: self.repeat_count,
            repeat_delay_ms: self.repeat_delay_ms,
            interaction_marker_name: self.interaction_marker_name,
        }
    }
}

impl<'a> SynthesizeScrollGestureParams<'a> { pub const METHOD: &'static str = "Input.synthesizeScrollGesture"; }

impl<'a> crate::CdpCommand<'a> for SynthesizeScrollGestureParams<'a> {
    const METHOD: &'static str = "Input.synthesizeScrollGesture";
    type Response = crate::EmptyReturns;
}

/// Synthesizes a tap gesture over a time period by issuing appropriate touch events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizeTapGestureParams {
    /// X coordinate of the start of the gesture in CSS pixels.
    x: f64,
    /// Y coordinate of the start of the gesture in CSS pixels.
    y: f64,
    /// Duration between touchdown and touchup events in ms (default: 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<i64>,
    /// Number of times to perform the tap (e.g. 2 for double tap, default: 1).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tapCount")]
    tap_count: Option<u64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gestureSourceType")]
    gesture_source_type: Option<GestureSourceType>,
}

impl SynthesizeTapGestureParams {
    /// Creates a builder for this type with the required parameters:
    /// * `x`: X coordinate of the start of the gesture in CSS pixels.
    /// * `y`: Y coordinate of the start of the gesture in CSS pixels.
    pub fn builder(x: f64, y: f64) -> SynthesizeTapGestureParamsBuilder {
        SynthesizeTapGestureParamsBuilder {
            x: x,
            y: y,
            duration: None,
            tap_count: None,
            gesture_source_type: None,
        }
    }
    /// X coordinate of the start of the gesture in CSS pixels.
    pub fn x(&self) -> f64 { self.x }
    /// Y coordinate of the start of the gesture in CSS pixels.
    pub fn y(&self) -> f64 { self.y }
    /// Duration between touchdown and touchup events in ms (default: 50).
    pub fn duration(&self) -> Option<i64> { self.duration }
    /// Number of times to perform the tap (e.g. 2 for double tap, default: 1).
    pub fn tap_count(&self) -> Option<u64> { self.tap_count }
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    pub fn gesture_source_type(&self) -> Option<&GestureSourceType> { self.gesture_source_type.as_ref() }
}


pub struct SynthesizeTapGestureParamsBuilder {
    x: f64,
    y: f64,
    duration: Option<i64>,
    tap_count: Option<u64>,
    gesture_source_type: Option<GestureSourceType>,
}

impl SynthesizeTapGestureParamsBuilder {
    /// Duration between touchdown and touchup events in ms (default: 50).
    pub fn duration(mut self, duration: i64) -> Self { self.duration = Some(duration); self }
    /// Number of times to perform the tap (e.g. 2 for double tap, default: 1).
    pub fn tap_count(mut self, tap_count: u64) -> Self { self.tap_count = Some(tap_count); self }
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    pub fn gesture_source_type(mut self, gesture_source_type: impl Into<GestureSourceType>) -> Self { self.gesture_source_type = Some(gesture_source_type.into()); self }
    pub fn build(self) -> SynthesizeTapGestureParams {
        SynthesizeTapGestureParams {
            x: self.x,
            y: self.y,
            duration: self.duration,
            tap_count: self.tap_count,
            gesture_source_type: self.gesture_source_type,
        }
    }
}

impl SynthesizeTapGestureParams { pub const METHOD: &'static str = "Input.synthesizeTapGesture"; }

impl<'a> crate::CdpCommand<'a> for SynthesizeTapGestureParams {
    const METHOD: &'static str = "Input.synthesizeTapGesture";
    type Response = crate::EmptyReturns;
}
