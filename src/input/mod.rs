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
    #[serde(skip_serializing_if = "Option::is_none")]
    radiusX: Option<f64>,
    /// Y radius of the touch area (default: 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    radiusY: Option<f64>,
    /// Rotation angle (default: 0.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    rotationAngle: Option<f64>,
    /// Force (default: 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<f64>,
    /// The normalized tangential pressure, which has a range of [-1,1] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    tangentialPressure: Option<f64>,
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range [-90,90], a positive tiltX is to the right (default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    tiltX: Option<f64>,
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range [-90,90], a positive tiltY is towards the user (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    tiltY: Option<f64>,
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range [0,359] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    twist: Option<i64>,
    /// Identifier used to track touch sources between events, must be unique within an event.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<f64>,
}

impl TouchPoint {
    pub fn builder(x: f64, y: f64) -> TouchPointBuilder {
        TouchPointBuilder {
            x: x,
            y: y,
            radiusX: None,
            radiusY: None,
            rotationAngle: None,
            force: None,
            tangentialPressure: None,
            tiltX: None,
            tiltY: None,
            twist: None,
            id: None,
        }
    }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn radiusX(&self) -> Option<f64> { self.radiusX }
    pub fn radiusY(&self) -> Option<f64> { self.radiusY }
    pub fn rotationAngle(&self) -> Option<f64> { self.rotationAngle }
    pub fn force(&self) -> Option<f64> { self.force }
    pub fn tangentialPressure(&self) -> Option<f64> { self.tangentialPressure }
    pub fn tiltX(&self) -> Option<f64> { self.tiltX }
    pub fn tiltY(&self) -> Option<f64> { self.tiltY }
    pub fn twist(&self) -> Option<i64> { self.twist }
    pub fn id(&self) -> Option<f64> { self.id }
}


pub struct TouchPointBuilder {
    x: f64,
    y: f64,
    radiusX: Option<f64>,
    radiusY: Option<f64>,
    rotationAngle: Option<f64>,
    force: Option<f64>,
    tangentialPressure: Option<f64>,
    tiltX: Option<f64>,
    tiltY: Option<f64>,
    twist: Option<i64>,
    id: Option<f64>,
}

impl TouchPointBuilder {
    /// X radius of the touch area (default: 1.0).
    pub fn radiusX(mut self, radiusX: f64) -> Self { self.radiusX = Some(radiusX); self }
    /// Y radius of the touch area (default: 1.0).
    pub fn radiusY(mut self, radiusY: f64) -> Self { self.radiusY = Some(radiusY); self }
    /// Rotation angle (default: 0.0).
    pub fn rotationAngle(mut self, rotationAngle: f64) -> Self { self.rotationAngle = Some(rotationAngle); self }
    /// Force (default: 1.0).
    pub fn force(mut self, force: f64) -> Self { self.force = Some(force); self }
    /// The normalized tangential pressure, which has a range of [-1,1] (default: 0).
    pub fn tangentialPressure(mut self, tangentialPressure: f64) -> Self { self.tangentialPressure = Some(tangentialPressure); self }
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range [-90,90], a positive tiltX is to the right (default: 0)
    pub fn tiltX(mut self, tiltX: f64) -> Self { self.tiltX = Some(tiltX); self }
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range [-90,90], a positive tiltY is towards the user (default: 0).
    pub fn tiltY(mut self, tiltY: f64) -> Self { self.tiltY = Some(tiltY); self }
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range [0,359] (default: 0).
    pub fn twist(mut self, twist: i64) -> Self { self.twist = Some(twist); self }
    /// Identifier used to track touch sources between events, must be unique within an event.
    pub fn id(mut self, id: f64) -> Self { self.id = Some(id); self }
    pub fn build(self) -> TouchPoint {
        TouchPoint {
            x: self.x,
            y: self.y,
            radiusX: self.radiusX,
            radiusY: self.radiusY,
            rotationAngle: self.rotationAngle,
            force: self.force,
            tangentialPressure: self.tangentialPressure,
            tiltX: self.tiltX,
            tiltY: self.tiltY,
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
    mimeType: Cow<'a, str>,
    /// Depending of the value of 'mimeType', it contains the dragged link,
    /// text, HTML markup or any other data.
    data: Cow<'a, str>,
    /// Title associated with a link. Only valid when 'mimeType' == "text/uri-list".
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Cow<'a, str>>,
    /// Stores the base URL for the contained markup. Only valid when 'mimeType'
    /// == "text/html".
    #[serde(skip_serializing_if = "Option::is_none")]
    baseURL: Option<Cow<'a, str>>,
}

impl<'a> DragDataItem<'a> {
    pub fn builder(mimeType: impl Into<Cow<'a, str>>, data: impl Into<Cow<'a, str>>) -> DragDataItemBuilder<'a> {
        DragDataItemBuilder {
            mimeType: mimeType.into(),
            data: data.into(),
            title: None,
            baseURL: None,
        }
    }
    pub fn mimeType(&self) -> &str { self.mimeType.as_ref() }
    pub fn data(&self) -> &str { self.data.as_ref() }
    pub fn title(&self) -> Option<&str> { self.title.as_deref() }
    pub fn baseURL(&self) -> Option<&str> { self.baseURL.as_deref() }
}


pub struct DragDataItemBuilder<'a> {
    mimeType: Cow<'a, str>,
    data: Cow<'a, str>,
    title: Option<Cow<'a, str>>,
    baseURL: Option<Cow<'a, str>>,
}

impl<'a> DragDataItemBuilder<'a> {
    /// Title associated with a link. Only valid when 'mimeType' == "text/uri-list".
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self { self.title = Some(title.into()); self }
    /// Stores the base URL for the contained markup. Only valid when 'mimeType'
    /// == "text/html".
    pub fn baseURL(mut self, baseURL: impl Into<Cow<'a, str>>) -> Self { self.baseURL = Some(baseURL.into()); self }
    pub fn build(self) -> DragDataItem<'a> {
        DragDataItem {
            mimeType: self.mimeType,
            data: self.data,
            title: self.title,
            baseURL: self.baseURL,
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
    dragOperationsMask: i64,
}

impl<'a> DragData<'a> {
    pub fn builder(items: Vec<DragDataItem<'a>>, dragOperationsMask: i64) -> DragDataBuilder<'a> {
        DragDataBuilder {
            items: items,
            files: None,
            dragOperationsMask: dragOperationsMask,
        }
    }
    pub fn items(&self) -> &[DragDataItem<'a>] { &self.items }
    pub fn files(&self) -> Option<&[Cow<'a, str>]> { self.files.as_deref() }
    pub fn dragOperationsMask(&self) -> i64 { self.dragOperationsMask }
}


pub struct DragDataBuilder<'a> {
    items: Vec<DragDataItem<'a>>,
    files: Option<Vec<Cow<'a, str>>>,
    dragOperationsMask: i64,
}

impl<'a> DragDataBuilder<'a> {
    /// List of filenames that should be included when dropping
    pub fn files(mut self, files: Vec<Cow<'a, str>>) -> Self { self.files = Some(files); self }
    pub fn build(self) -> DragData<'a> {
        DragData {
            items: self.items,
            files: self.files,
            dragOperationsMask: self.dragOperationsMask,
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
    pub fn builder(type_: impl Into<Cow<'a, str>>, x: f64, y: f64, data: DragData<'a>) -> DispatchDragEventParamsBuilder<'a> {
        DispatchDragEventParamsBuilder {
            type_: type_.into(),
            x: x,
            y: y,
            data: data,
            modifiers: None,
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn data(&self) -> &DragData<'a> { &self.data }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    unmodifiedText: Option<Cow<'a, str>>,
    /// Unique key identifier (e.g., 'U+0041') (default: "").
    #[serde(skip_serializing_if = "Option::is_none")]
    keyIdentifier: Option<Cow<'a, str>>,
    /// Unique DOM defined string value for each physical key (e.g., 'KeyA') (default: "").
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<Cow<'a, str>>,
    /// Unique DOM defined string value describing the meaning of the key in the context of active
    /// modifiers, keyboard layout, etc (e.g., 'AltGr') (default: "").
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<Cow<'a, str>>,
    /// Windows virtual key code (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    windowsVirtualKeyCode: Option<i64>,
    /// Native virtual key code (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    nativeVirtualKeyCode: Option<i64>,
    /// Whether the event was generated from auto repeat (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    autoRepeat: Option<bool>,
    /// Whether the event was generated from the keypad (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    isKeypad: Option<bool>,
    /// Whether the event was a system key event (default: false).
    #[serde(skip_serializing_if = "Option::is_none")]
    isSystemKey: Option<bool>,
    /// Whether the event was from the left or right side of the keyboard. 1=Left, 2=Right (default:
    /// 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<i64>,
    /// Editing commands to send with the key event (e.g., 'selectAll') (default: []).
    /// These are related to but not equal the command names used in 'document.execCommand' and NSStandardKeyBindingResponding.
    /// See https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/editing/commands/editor_command_names.h for valid command names.
    #[serde(skip_serializing_if = "Option::is_none")]
    commands: Option<Vec<Cow<'a, str>>>,
}

impl<'a> DispatchKeyEventParams<'a> {
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> DispatchKeyEventParamsBuilder<'a> {
        DispatchKeyEventParamsBuilder {
            type_: type_.into(),
            modifiers: None,
            timestamp: None,
            text: None,
            unmodifiedText: None,
            keyIdentifier: None,
            code: None,
            key: None,
            windowsVirtualKeyCode: None,
            nativeVirtualKeyCode: None,
            autoRepeat: None,
            isKeypad: None,
            isSystemKey: None,
            location: None,
            commands: None,
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn modifiers(&self) -> Option<i64> { self.modifiers }
    pub fn timestamp(&self) -> Option<&TimeSinceEpoch> { self.timestamp.as_ref() }
    pub fn text(&self) -> Option<&str> { self.text.as_deref() }
    pub fn unmodifiedText(&self) -> Option<&str> { self.unmodifiedText.as_deref() }
    pub fn keyIdentifier(&self) -> Option<&str> { self.keyIdentifier.as_deref() }
    pub fn code(&self) -> Option<&str> { self.code.as_deref() }
    pub fn key(&self) -> Option<&str> { self.key.as_deref() }
    pub fn windowsVirtualKeyCode(&self) -> Option<i64> { self.windowsVirtualKeyCode }
    pub fn nativeVirtualKeyCode(&self) -> Option<i64> { self.nativeVirtualKeyCode }
    pub fn autoRepeat(&self) -> Option<bool> { self.autoRepeat }
    pub fn isKeypad(&self) -> Option<bool> { self.isKeypad }
    pub fn isSystemKey(&self) -> Option<bool> { self.isSystemKey }
    pub fn location(&self) -> Option<i64> { self.location }
    pub fn commands(&self) -> Option<&[Cow<'a, str>]> { self.commands.as_deref() }
}


pub struct DispatchKeyEventParamsBuilder<'a> {
    type_: Cow<'a, str>,
    modifiers: Option<i64>,
    timestamp: Option<TimeSinceEpoch>,
    text: Option<Cow<'a, str>>,
    unmodifiedText: Option<Cow<'a, str>>,
    keyIdentifier: Option<Cow<'a, str>>,
    code: Option<Cow<'a, str>>,
    key: Option<Cow<'a, str>>,
    windowsVirtualKeyCode: Option<i64>,
    nativeVirtualKeyCode: Option<i64>,
    autoRepeat: Option<bool>,
    isKeypad: Option<bool>,
    isSystemKey: Option<bool>,
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
    pub fn unmodifiedText(mut self, unmodifiedText: impl Into<Cow<'a, str>>) -> Self { self.unmodifiedText = Some(unmodifiedText.into()); self }
    /// Unique key identifier (e.g., 'U+0041') (default: "").
    pub fn keyIdentifier(mut self, keyIdentifier: impl Into<Cow<'a, str>>) -> Self { self.keyIdentifier = Some(keyIdentifier.into()); self }
    /// Unique DOM defined string value for each physical key (e.g., 'KeyA') (default: "").
    pub fn code(mut self, code: impl Into<Cow<'a, str>>) -> Self { self.code = Some(code.into()); self }
    /// Unique DOM defined string value describing the meaning of the key in the context of active
    /// modifiers, keyboard layout, etc (e.g., 'AltGr') (default: "").
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self { self.key = Some(key.into()); self }
    /// Windows virtual key code (default: 0).
    pub fn windowsVirtualKeyCode(mut self, windowsVirtualKeyCode: i64) -> Self { self.windowsVirtualKeyCode = Some(windowsVirtualKeyCode); self }
    /// Native virtual key code (default: 0).
    pub fn nativeVirtualKeyCode(mut self, nativeVirtualKeyCode: i64) -> Self { self.nativeVirtualKeyCode = Some(nativeVirtualKeyCode); self }
    /// Whether the event was generated from auto repeat (default: false).
    pub fn autoRepeat(mut self, autoRepeat: bool) -> Self { self.autoRepeat = Some(autoRepeat); self }
    /// Whether the event was generated from the keypad (default: false).
    pub fn isKeypad(mut self, isKeypad: bool) -> Self { self.isKeypad = Some(isKeypad); self }
    /// Whether the event was a system key event (default: false).
    pub fn isSystemKey(mut self, isSystemKey: bool) -> Self { self.isSystemKey = Some(isSystemKey); self }
    /// Whether the event was from the left or right side of the keyboard. 1=Left, 2=Right (default:
    /// 0).
    pub fn location(mut self, location: i64) -> Self { self.location = Some(location); self }
    /// Editing commands to send with the key event (e.g., 'selectAll') (default: []).
    /// These are related to but not equal the command names used in 'document.execCommand' and NSStandardKeyBindingResponding.
    /// See https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/editing/commands/editor_command_names.h for valid command names.
    pub fn commands(mut self, commands: Vec<Cow<'a, str>>) -> Self { self.commands = Some(commands); self }
    pub fn build(self) -> DispatchKeyEventParams<'a> {
        DispatchKeyEventParams {
            type_: self.type_,
            modifiers: self.modifiers,
            timestamp: self.timestamp,
            text: self.text,
            unmodifiedText: self.unmodifiedText,
            keyIdentifier: self.keyIdentifier,
            code: self.code,
            key: self.key,
            windowsVirtualKeyCode: self.windowsVirtualKeyCode,
            nativeVirtualKeyCode: self.nativeVirtualKeyCode,
            autoRepeat: self.autoRepeat,
            isKeypad: self.isKeypad,
            isSystemKey: self.isSystemKey,
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
    pub fn builder(text: impl Into<Cow<'a, str>>) -> InsertTextParamsBuilder<'a> {
        InsertTextParamsBuilder {
            text: text.into(),
        }
    }
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
    selectionStart: i64,
    /// selection end
    selectionEnd: i64,
    /// replacement start
    #[serde(skip_serializing_if = "Option::is_none")]
    replacementStart: Option<i64>,
    /// replacement end
    #[serde(skip_serializing_if = "Option::is_none")]
    replacementEnd: Option<i64>,
}

impl<'a> ImeSetCompositionParams<'a> {
    pub fn builder(text: impl Into<Cow<'a, str>>, selectionStart: i64, selectionEnd: i64) -> ImeSetCompositionParamsBuilder<'a> {
        ImeSetCompositionParamsBuilder {
            text: text.into(),
            selectionStart: selectionStart,
            selectionEnd: selectionEnd,
            replacementStart: None,
            replacementEnd: None,
        }
    }
    pub fn text(&self) -> &str { self.text.as_ref() }
    pub fn selectionStart(&self) -> i64 { self.selectionStart }
    pub fn selectionEnd(&self) -> i64 { self.selectionEnd }
    pub fn replacementStart(&self) -> Option<i64> { self.replacementStart }
    pub fn replacementEnd(&self) -> Option<i64> { self.replacementEnd }
}


pub struct ImeSetCompositionParamsBuilder<'a> {
    text: Cow<'a, str>,
    selectionStart: i64,
    selectionEnd: i64,
    replacementStart: Option<i64>,
    replacementEnd: Option<i64>,
}

impl<'a> ImeSetCompositionParamsBuilder<'a> {
    /// replacement start
    pub fn replacementStart(mut self, replacementStart: i64) -> Self { self.replacementStart = Some(replacementStart); self }
    /// replacement end
    pub fn replacementEnd(mut self, replacementEnd: i64) -> Self { self.replacementEnd = Some(replacementEnd); self }
    pub fn build(self) -> ImeSetCompositionParams<'a> {
        ImeSetCompositionParams {
            text: self.text,
            selectionStart: self.selectionStart,
            selectionEnd: self.selectionEnd,
            replacementStart: self.replacementStart,
            replacementEnd: self.replacementEnd,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    clickCount: Option<u64>,
    /// The normalized pressure, which has a range of [0,1] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<f64>,
    /// The normalized tangential pressure, which has a range of [-1,1] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    tangentialPressure: Option<f64>,
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range [-90,90], a positive tiltX is to the right (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    tiltX: Option<f64>,
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range [-90,90], a positive tiltY is towards the user (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    tiltY: Option<f64>,
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range [0,359] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    twist: Option<i64>,
    /// X delta in CSS pixels for mouse wheel event (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    deltaX: Option<f64>,
    /// Y delta in CSS pixels for mouse wheel event (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    deltaY: Option<f64>,
    /// Pointer type (default: "mouse").
    #[serde(skip_serializing_if = "Option::is_none")]
    pointerType: Option<Cow<'a, str>>,
}

impl<'a> DispatchMouseEventParams<'a> {
    pub fn builder(type_: impl Into<Cow<'a, str>>, x: f64, y: f64) -> DispatchMouseEventParamsBuilder<'a> {
        DispatchMouseEventParamsBuilder {
            type_: type_.into(),
            x: x,
            y: y,
            modifiers: None,
            timestamp: None,
            button: None,
            buttons: None,
            clickCount: None,
            force: None,
            tangentialPressure: None,
            tiltX: None,
            tiltY: None,
            twist: None,
            deltaX: None,
            deltaY: None,
            pointerType: None,
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn modifiers(&self) -> Option<i64> { self.modifiers }
    pub fn timestamp(&self) -> Option<&TimeSinceEpoch> { self.timestamp.as_ref() }
    pub fn button(&self) -> Option<&MouseButton> { self.button.as_ref() }
    pub fn buttons(&self) -> Option<i64> { self.buttons }
    pub fn clickCount(&self) -> Option<u64> { self.clickCount }
    pub fn force(&self) -> Option<f64> { self.force }
    pub fn tangentialPressure(&self) -> Option<f64> { self.tangentialPressure }
    pub fn tiltX(&self) -> Option<f64> { self.tiltX }
    pub fn tiltY(&self) -> Option<f64> { self.tiltY }
    pub fn twist(&self) -> Option<i64> { self.twist }
    pub fn deltaX(&self) -> Option<f64> { self.deltaX }
    pub fn deltaY(&self) -> Option<f64> { self.deltaY }
    pub fn pointerType(&self) -> Option<&str> { self.pointerType.as_deref() }
}


pub struct DispatchMouseEventParamsBuilder<'a> {
    type_: Cow<'a, str>,
    x: f64,
    y: f64,
    modifiers: Option<i64>,
    timestamp: Option<TimeSinceEpoch>,
    button: Option<MouseButton>,
    buttons: Option<i64>,
    clickCount: Option<u64>,
    force: Option<f64>,
    tangentialPressure: Option<f64>,
    tiltX: Option<f64>,
    tiltY: Option<f64>,
    twist: Option<i64>,
    deltaX: Option<f64>,
    deltaY: Option<f64>,
    pointerType: Option<Cow<'a, str>>,
}

impl<'a> DispatchMouseEventParamsBuilder<'a> {
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(mut self, modifiers: i64) -> Self { self.modifiers = Some(modifiers); self }
    /// Time at which the event occurred.
    pub fn timestamp(mut self, timestamp: TimeSinceEpoch) -> Self { self.timestamp = Some(timestamp); self }
    /// Mouse button (default: "none").
    pub fn button(mut self, button: MouseButton) -> Self { self.button = Some(button); self }
    /// A number indicating which buttons are pressed on the mouse when a mouse event is triggered.
    /// Left=1, Right=2, Middle=4, Back=8, Forward=16, None=0.
    pub fn buttons(mut self, buttons: i64) -> Self { self.buttons = Some(buttons); self }
    /// Number of times the mouse button was clicked (default: 0).
    pub fn clickCount(mut self, clickCount: u64) -> Self { self.clickCount = Some(clickCount); self }
    /// The normalized pressure, which has a range of [0,1] (default: 0).
    pub fn force(mut self, force: f64) -> Self { self.force = Some(force); self }
    /// The normalized tangential pressure, which has a range of [-1,1] (default: 0).
    pub fn tangentialPressure(mut self, tangentialPressure: f64) -> Self { self.tangentialPressure = Some(tangentialPressure); self }
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range [-90,90], a positive tiltX is to the right (default: 0).
    pub fn tiltX(mut self, tiltX: f64) -> Self { self.tiltX = Some(tiltX); self }
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range [-90,90], a positive tiltY is towards the user (default: 0).
    pub fn tiltY(mut self, tiltY: f64) -> Self { self.tiltY = Some(tiltY); self }
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range [0,359] (default: 0).
    pub fn twist(mut self, twist: i64) -> Self { self.twist = Some(twist); self }
    /// X delta in CSS pixels for mouse wheel event (default: 0).
    pub fn deltaX(mut self, deltaX: f64) -> Self { self.deltaX = Some(deltaX); self }
    /// Y delta in CSS pixels for mouse wheel event (default: 0).
    pub fn deltaY(mut self, deltaY: f64) -> Self { self.deltaY = Some(deltaY); self }
    /// Pointer type (default: "mouse").
    pub fn pointerType(mut self, pointerType: impl Into<Cow<'a, str>>) -> Self { self.pointerType = Some(pointerType.into()); self }
    pub fn build(self) -> DispatchMouseEventParams<'a> {
        DispatchMouseEventParams {
            type_: self.type_,
            x: self.x,
            y: self.y,
            modifiers: self.modifiers,
            timestamp: self.timestamp,
            button: self.button,
            buttons: self.buttons,
            clickCount: self.clickCount,
            force: self.force,
            tangentialPressure: self.tangentialPressure,
            tiltX: self.tiltX,
            tiltY: self.tiltY,
            twist: self.twist,
            deltaX: self.deltaX,
            deltaY: self.deltaY,
            pointerType: self.pointerType,
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
    touchPoints: Vec<TouchPoint>,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    modifiers: Option<i64>,
    /// Time at which the event occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<TimeSinceEpoch>,
}

impl<'a> DispatchTouchEventParams<'a> {
    pub fn builder(type_: impl Into<Cow<'a, str>>, touchPoints: Vec<TouchPoint>) -> DispatchTouchEventParamsBuilder<'a> {
        DispatchTouchEventParamsBuilder {
            type_: type_.into(),
            touchPoints: touchPoints,
            modifiers: None,
            timestamp: None,
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn touchPoints(&self) -> &[TouchPoint] { &self.touchPoints }
    pub fn modifiers(&self) -> Option<i64> { self.modifiers }
    pub fn timestamp(&self) -> Option<&TimeSinceEpoch> { self.timestamp.as_ref() }
}


pub struct DispatchTouchEventParamsBuilder<'a> {
    type_: Cow<'a, str>,
    touchPoints: Vec<TouchPoint>,
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
            touchPoints: self.touchPoints,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    deltaX: Option<f64>,
    /// Y delta in DIP for mouse wheel event (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    deltaY: Option<f64>,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    modifiers: Option<i64>,
    /// Number of times the mouse button was clicked (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    clickCount: Option<u64>,
}

impl<'a> EmulateTouchFromMouseEventParams<'a> {
    pub fn builder(type_: impl Into<Cow<'a, str>>, x: i32, y: i32, button: MouseButton) -> EmulateTouchFromMouseEventParamsBuilder<'a> {
        EmulateTouchFromMouseEventParamsBuilder {
            type_: type_.into(),
            x: x,
            y: y,
            button: button,
            timestamp: None,
            deltaX: None,
            deltaY: None,
            modifiers: None,
            clickCount: None,
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn button(&self) -> &MouseButton { &self.button }
    pub fn timestamp(&self) -> Option<&TimeSinceEpoch> { self.timestamp.as_ref() }
    pub fn deltaX(&self) -> Option<f64> { self.deltaX }
    pub fn deltaY(&self) -> Option<f64> { self.deltaY }
    pub fn modifiers(&self) -> Option<i64> { self.modifiers }
    pub fn clickCount(&self) -> Option<u64> { self.clickCount }
}


pub struct EmulateTouchFromMouseEventParamsBuilder<'a> {
    type_: Cow<'a, str>,
    x: i32,
    y: i32,
    button: MouseButton,
    timestamp: Option<TimeSinceEpoch>,
    deltaX: Option<f64>,
    deltaY: Option<f64>,
    modifiers: Option<i64>,
    clickCount: Option<u64>,
}

impl<'a> EmulateTouchFromMouseEventParamsBuilder<'a> {
    /// Time at which the event occurred (default: current time).
    pub fn timestamp(mut self, timestamp: TimeSinceEpoch) -> Self { self.timestamp = Some(timestamp); self }
    /// X delta in DIP for mouse wheel event (default: 0).
    pub fn deltaX(mut self, deltaX: f64) -> Self { self.deltaX = Some(deltaX); self }
    /// Y delta in DIP for mouse wheel event (default: 0).
    pub fn deltaY(mut self, deltaY: f64) -> Self { self.deltaY = Some(deltaY); self }
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    pub fn modifiers(mut self, modifiers: i64) -> Self { self.modifiers = Some(modifiers); self }
    /// Number of times the mouse button was clicked (default: 0).
    pub fn clickCount(mut self, clickCount: u64) -> Self { self.clickCount = Some(clickCount); self }
    pub fn build(self) -> EmulateTouchFromMouseEventParams<'a> {
        EmulateTouchFromMouseEventParams {
            type_: self.type_,
            x: self.x,
            y: self.y,
            button: self.button,
            timestamp: self.timestamp,
            deltaX: self.deltaX,
            deltaY: self.deltaY,
            modifiers: self.modifiers,
            clickCount: self.clickCount,
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
    pub fn builder(ignore: bool) -> SetIgnoreInputEventsParamsBuilder {
        SetIgnoreInputEventsParamsBuilder {
            ignore: ignore,
        }
    }
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
    /// Relative scale factor after zooming (>1.0 zooms in, <1.0 zooms out).
    scaleFactor: f64,
    /// Relative pointer speed in pixels per second (default: 800).
    #[serde(skip_serializing_if = "Option::is_none")]
    relativeSpeed: Option<i64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    #[serde(skip_serializing_if = "Option::is_none")]
    gestureSourceType: Option<GestureSourceType>,
}

impl SynthesizePinchGestureParams {
    pub fn builder(x: f64, y: f64, scaleFactor: f64) -> SynthesizePinchGestureParamsBuilder {
        SynthesizePinchGestureParamsBuilder {
            x: x,
            y: y,
            scaleFactor: scaleFactor,
            relativeSpeed: None,
            gestureSourceType: None,
        }
    }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn scaleFactor(&self) -> f64 { self.scaleFactor }
    pub fn relativeSpeed(&self) -> Option<i64> { self.relativeSpeed }
    pub fn gestureSourceType(&self) -> Option<&GestureSourceType> { self.gestureSourceType.as_ref() }
}


pub struct SynthesizePinchGestureParamsBuilder {
    x: f64,
    y: f64,
    scaleFactor: f64,
    relativeSpeed: Option<i64>,
    gestureSourceType: Option<GestureSourceType>,
}

impl SynthesizePinchGestureParamsBuilder {
    /// Relative pointer speed in pixels per second (default: 800).
    pub fn relativeSpeed(mut self, relativeSpeed: i64) -> Self { self.relativeSpeed = Some(relativeSpeed); self }
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    pub fn gestureSourceType(mut self, gestureSourceType: GestureSourceType) -> Self { self.gestureSourceType = Some(gestureSourceType); self }
    pub fn build(self) -> SynthesizePinchGestureParams {
        SynthesizePinchGestureParams {
            x: self.x,
            y: self.y,
            scaleFactor: self.scaleFactor,
            relativeSpeed: self.relativeSpeed,
            gestureSourceType: self.gestureSourceType,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    xDistance: Option<f64>,
    /// The distance to scroll along the Y axis (positive to scroll up).
    #[serde(skip_serializing_if = "Option::is_none")]
    yDistance: Option<f64>,
    /// The number of additional pixels to scroll back along the X axis, in addition to the given
    /// distance.
    #[serde(skip_serializing_if = "Option::is_none")]
    xOverscroll: Option<f64>,
    /// The number of additional pixels to scroll back along the Y axis, in addition to the given
    /// distance.
    #[serde(skip_serializing_if = "Option::is_none")]
    yOverscroll: Option<f64>,
    /// Prevent fling (default: true).
    #[serde(skip_serializing_if = "Option::is_none")]
    preventFling: Option<bool>,
    /// Swipe speed in pixels per second (default: 800).
    #[serde(skip_serializing_if = "Option::is_none")]
    speed: Option<i64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    #[serde(skip_serializing_if = "Option::is_none")]
    gestureSourceType: Option<GestureSourceType>,
    /// The number of times to repeat the gesture (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    repeatCount: Option<u64>,
    /// The number of milliseconds delay between each repeat. (default: 250).
    #[serde(skip_serializing_if = "Option::is_none")]
    repeatDelayMs: Option<i64>,
    /// The name of the interaction markers to generate, if not empty (default: "").
    #[serde(skip_serializing_if = "Option::is_none")]
    interactionMarkerName: Option<Cow<'a, str>>,
}

impl<'a> SynthesizeScrollGestureParams<'a> {
    pub fn builder(x: f64, y: f64) -> SynthesizeScrollGestureParamsBuilder<'a> {
        SynthesizeScrollGestureParamsBuilder {
            x: x,
            y: y,
            xDistance: None,
            yDistance: None,
            xOverscroll: None,
            yOverscroll: None,
            preventFling: None,
            speed: None,
            gestureSourceType: None,
            repeatCount: None,
            repeatDelayMs: None,
            interactionMarkerName: None,
        }
    }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn xDistance(&self) -> Option<f64> { self.xDistance }
    pub fn yDistance(&self) -> Option<f64> { self.yDistance }
    pub fn xOverscroll(&self) -> Option<f64> { self.xOverscroll }
    pub fn yOverscroll(&self) -> Option<f64> { self.yOverscroll }
    pub fn preventFling(&self) -> Option<bool> { self.preventFling }
    pub fn speed(&self) -> Option<i64> { self.speed }
    pub fn gestureSourceType(&self) -> Option<&GestureSourceType> { self.gestureSourceType.as_ref() }
    pub fn repeatCount(&self) -> Option<u64> { self.repeatCount }
    pub fn repeatDelayMs(&self) -> Option<i64> { self.repeatDelayMs }
    pub fn interactionMarkerName(&self) -> Option<&str> { self.interactionMarkerName.as_deref() }
}


pub struct SynthesizeScrollGestureParamsBuilder<'a> {
    x: f64,
    y: f64,
    xDistance: Option<f64>,
    yDistance: Option<f64>,
    xOverscroll: Option<f64>,
    yOverscroll: Option<f64>,
    preventFling: Option<bool>,
    speed: Option<i64>,
    gestureSourceType: Option<GestureSourceType>,
    repeatCount: Option<u64>,
    repeatDelayMs: Option<i64>,
    interactionMarkerName: Option<Cow<'a, str>>,
}

impl<'a> SynthesizeScrollGestureParamsBuilder<'a> {
    /// The distance to scroll along the X axis (positive to scroll left).
    pub fn xDistance(mut self, xDistance: f64) -> Self { self.xDistance = Some(xDistance); self }
    /// The distance to scroll along the Y axis (positive to scroll up).
    pub fn yDistance(mut self, yDistance: f64) -> Self { self.yDistance = Some(yDistance); self }
    /// The number of additional pixels to scroll back along the X axis, in addition to the given
    /// distance.
    pub fn xOverscroll(mut self, xOverscroll: f64) -> Self { self.xOverscroll = Some(xOverscroll); self }
    /// The number of additional pixels to scroll back along the Y axis, in addition to the given
    /// distance.
    pub fn yOverscroll(mut self, yOverscroll: f64) -> Self { self.yOverscroll = Some(yOverscroll); self }
    /// Prevent fling (default: true).
    pub fn preventFling(mut self, preventFling: bool) -> Self { self.preventFling = Some(preventFling); self }
    /// Swipe speed in pixels per second (default: 800).
    pub fn speed(mut self, speed: i64) -> Self { self.speed = Some(speed); self }
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    pub fn gestureSourceType(mut self, gestureSourceType: GestureSourceType) -> Self { self.gestureSourceType = Some(gestureSourceType); self }
    /// The number of times to repeat the gesture (default: 0).
    pub fn repeatCount(mut self, repeatCount: u64) -> Self { self.repeatCount = Some(repeatCount); self }
    /// The number of milliseconds delay between each repeat. (default: 250).
    pub fn repeatDelayMs(mut self, repeatDelayMs: i64) -> Self { self.repeatDelayMs = Some(repeatDelayMs); self }
    /// The name of the interaction markers to generate, if not empty (default: "").
    pub fn interactionMarkerName(mut self, interactionMarkerName: impl Into<Cow<'a, str>>) -> Self { self.interactionMarkerName = Some(interactionMarkerName.into()); self }
    pub fn build(self) -> SynthesizeScrollGestureParams<'a> {
        SynthesizeScrollGestureParams {
            x: self.x,
            y: self.y,
            xDistance: self.xDistance,
            yDistance: self.yDistance,
            xOverscroll: self.xOverscroll,
            yOverscroll: self.yOverscroll,
            preventFling: self.preventFling,
            speed: self.speed,
            gestureSourceType: self.gestureSourceType,
            repeatCount: self.repeatCount,
            repeatDelayMs: self.repeatDelayMs,
            interactionMarkerName: self.interactionMarkerName,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    tapCount: Option<u64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    #[serde(skip_serializing_if = "Option::is_none")]
    gestureSourceType: Option<GestureSourceType>,
}

impl SynthesizeTapGestureParams {
    pub fn builder(x: f64, y: f64) -> SynthesizeTapGestureParamsBuilder {
        SynthesizeTapGestureParamsBuilder {
            x: x,
            y: y,
            duration: None,
            tapCount: None,
            gestureSourceType: None,
        }
    }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn duration(&self) -> Option<i64> { self.duration }
    pub fn tapCount(&self) -> Option<u64> { self.tapCount }
    pub fn gestureSourceType(&self) -> Option<&GestureSourceType> { self.gestureSourceType.as_ref() }
}


pub struct SynthesizeTapGestureParamsBuilder {
    x: f64,
    y: f64,
    duration: Option<i64>,
    tapCount: Option<u64>,
    gestureSourceType: Option<GestureSourceType>,
}

impl SynthesizeTapGestureParamsBuilder {
    /// Duration between touchdown and touchup events in ms (default: 50).
    pub fn duration(mut self, duration: i64) -> Self { self.duration = Some(duration); self }
    /// Number of times to perform the tap (e.g. 2 for double tap, default: 1).
    pub fn tapCount(mut self, tapCount: u64) -> Self { self.tapCount = Some(tapCount); self }
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    pub fn gestureSourceType(mut self, gestureSourceType: GestureSourceType) -> Self { self.gestureSourceType = Some(gestureSourceType); self }
    pub fn build(self) -> SynthesizeTapGestureParams {
        SynthesizeTapGestureParams {
            x: self.x,
            y: self.y,
            duration: self.duration,
            tapCount: self.tapCount,
            gestureSourceType: self.gestureSourceType,
        }
    }
}

impl SynthesizeTapGestureParams { pub const METHOD: &'static str = "Input.synthesizeTapGesture"; }

impl<'a> crate::CdpCommand<'a> for SynthesizeTapGestureParams {
    const METHOD: &'static str = "Input.synthesizeTapGesture";
    type Response = crate::EmptyReturns;
}
