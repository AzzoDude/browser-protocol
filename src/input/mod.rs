use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TouchPoint {
    /// X coordinate of the event relative to the main frame's viewport in CSS pixels.

    pub x: f64,
    /// Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    /// the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.

    pub y: f64,
    /// X radius of the touch area (default: 1.0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub radiusX: Option<f64>,
    /// Y radius of the touch area (default: 1.0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub radiusY: Option<f64>,
    /// Rotation angle (default: 0.0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotationAngle: Option<f64>,
    /// Force (default: 1.0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<f64>,
    /// The normalized tangential pressure, which has a range of \[-1,1\] (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tangentialPressure: Option<f64>,
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range \[-90,90\], a positive tiltX is to the right (default: 0)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiltX: Option<f64>,
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range \[-90,90\], a positive tiltY is towards the user (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiltY: Option<f64>,
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range \[0,359\] (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub twist: Option<i64>,
    /// Identifier used to track touch sources between events, must be unique within an event.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum GestureSourceType {
    #[default]
    Default,
    Touch,
    Mouse,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MouseButton {
    #[default]
    None,
    Left,
    Middle,
    Right,
    Back,
    Forward,
}

/// UTC time in seconds, counted from January 1, 1970.

pub type TimeSinceEpoch = f64;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DragDataItem {
    /// Mime type of the dragged data.

    pub mimeType: String,
    /// Depending of the value of 'mimeType', it contains the dragged link,
    /// text, HTML markup or any other data.

    pub data: String,
    /// Title associated with a link. Only valid when 'mimeType' == "text/uri-list".

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Stores the base URL for the contained markup. Only valid when 'mimeType'
    /// == "text/html".

    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseURL: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DragData {

    pub items: Vec<DragDataItem>,
    /// List of filenames that should be included when dropping

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    /// Bit field representing allowed drag operations. Copy = 1, Link = 2, Move = 16

    pub dragOperationsMask: i64,
}

/// Dispatches a drag event into the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchDragEventParams {
    /// Type of the drag event.

    #[serde(rename = "type")]
    pub type_: String,
    /// X coordinate of the event relative to the main frame's viewport in CSS pixels.

    pub x: f64,
    /// Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    /// the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.

    pub y: f64,

    pub data: DragData,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<i64>,
}

/// Dispatches a key event to the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchKeyEventParams {
    /// Type of the key event.

    #[serde(rename = "type")]
    pub type_: String,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<i64>,
    /// Time at which the event occurred.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<TimeSinceEpoch>,
    /// Text as generated by processing a virtual key code with a keyboard layout. Not needed for
    /// for 'keyUp' and 'rawKeyDown' events (default: "")

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Text that would have been generated by the keyboard if no modifiers were pressed (except for
    /// shift). Useful for shortcut (accelerator) key handling (default: "").

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmodifiedText: Option<String>,
    /// Unique key identifier (e.g., 'U+0041') (default: "").

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyIdentifier: Option<String>,
    /// Unique DOM defined string value for each physical key (e.g., 'KeyA') (default: "").

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Unique DOM defined string value describing the meaning of the key in the context of active
    /// modifiers, keyboard layout, etc (e.g., 'AltGr') (default: "").

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Windows virtual key code (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub windowsVirtualKeyCode: Option<i64>,
    /// Native virtual key code (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nativeVirtualKeyCode: Option<i64>,
    /// Whether the event was generated from auto repeat (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoRepeat: Option<bool>,
    /// Whether the event was generated from the keypad (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isKeypad: Option<bool>,
    /// Whether the event was a system key event (default: false).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isSystemKey: Option<bool>,
    /// Whether the event was from the left or right side of the keyboard. 1=Left, 2=Right (default:
    /// 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<i64>,
    /// Editing commands to send with the key event (e.g., 'selectAll') (default: \[\]).
    /// These are related to but not equal the command names used in 'document.execCommand' and NSStandardKeyBindingResponding.
    /// See <https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/editing/commands/editor_command_names.h> for valid command names.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
}

/// This method emulates inserting text that doesn't come from a key press,
/// for example an emoji keyboard or an IME.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextParams {
    /// The text to insert.

    pub text: String,
}

/// This method sets the current candidate text for IME.
/// Use imeCommitComposition to commit the final text.
/// Use imeSetComposition with empty string as text to cancel composition.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImeSetCompositionParams {
    /// The text to insert

    pub text: String,
    /// selection start

    pub selectionStart: i64,
    /// selection end

    pub selectionEnd: i64,
    /// replacement start

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacementStart: Option<i64>,
    /// replacement end

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacementEnd: Option<i64>,
}

/// Dispatches a mouse event to the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchMouseEventParams {
    /// Type of the mouse event.

    #[serde(rename = "type")]
    pub type_: String,
    /// X coordinate of the event relative to the main frame's viewport in CSS pixels.

    pub x: f64,
    /// Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    /// the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.

    pub y: f64,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<i64>,
    /// Time at which the event occurred.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<TimeSinceEpoch>,
    /// Mouse button (default: "none").

    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<MouseButton>,
    /// A number indicating which buttons are pressed on the mouse when a mouse event is triggered.
    /// Left=1, Right=2, Middle=4, Back=8, Forward=16, None=0.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<i64>,
    /// Number of times the mouse button was clicked (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clickCount: Option<u64>,
    /// The normalized pressure, which has a range of \[0,1\] (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<f64>,
    /// The normalized tangential pressure, which has a range of \[-1,1\] (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tangentialPressure: Option<f64>,
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range \[-90,90\], a positive tiltX is to the right (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiltX: Option<f64>,
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range \[-90,90\], a positive tiltY is towards the user (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiltY: Option<f64>,
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range \[0,359\] (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub twist: Option<i64>,
    /// X delta in CSS pixels for mouse wheel event (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deltaX: Option<f64>,
    /// Y delta in CSS pixels for mouse wheel event (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deltaY: Option<f64>,
    /// Pointer type (default: "mouse").

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointerType: Option<String>,
}

/// Dispatches a touch event to the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispatchTouchEventParams {
    /// Type of the touch event. TouchEnd and TouchCancel must not contain any touch points, while
    /// TouchStart and TouchMove must contains at least one.

    #[serde(rename = "type")]
    pub type_: String,
    /// Active touch points on the touch device. One event per any changed point (compared to
    /// previous touch event in a sequence) is generated, emulating pressing/moving/releasing points
    /// one by one.

    pub touchPoints: Vec<TouchPoint>,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<i64>,
    /// Time at which the event occurred.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<TimeSinceEpoch>,
}

/// Emulates touch event from the mouse event parameters.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmulateTouchFromMouseEventParams {
    /// Type of the mouse event.

    #[serde(rename = "type")]
    pub type_: String,
    /// X coordinate of the mouse pointer in DIP.

    pub x: i32,
    /// Y coordinate of the mouse pointer in DIP.

    pub y: i32,
    /// Mouse button. Only "none", "left", "right" are supported.

    pub button: MouseButton,
    /// Time at which the event occurred (default: current time).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<TimeSinceEpoch>,
    /// X delta in DIP for mouse wheel event (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deltaX: Option<f64>,
    /// Y delta in DIP for mouse wheel event (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deltaY: Option<f64>,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<i64>,
    /// Number of times the mouse button was clicked (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clickCount: Option<u64>,
}

/// Ignores input events (useful while auditing page).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetIgnoreInputEventsParams {
    /// Ignores input events processing when set to true.

    pub ignore: bool,
}

/// Prevents default drag and drop behavior and instead emits 'Input.dragIntercepted' events.
/// Drag and drop behavior can be directly controlled via 'Input.dispatchDragEvent'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInterceptDragsParams {

    pub enabled: bool,
}

/// Synthesizes a pinch gesture over a time period by issuing appropriate touch events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizePinchGestureParams {
    /// X coordinate of the start of the gesture in CSS pixels.

    pub x: f64,
    /// Y coordinate of the start of the gesture in CSS pixels.

    pub y: f64,
    /// Relative scale factor after zooming (\>1.0 zooms in, \<1.0 zooms out).

    pub scaleFactor: f64,
    /// Relative pointer speed in pixels per second (default: 800).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relativeSpeed: Option<i64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gestureSourceType: Option<GestureSourceType>,
}

/// Synthesizes a scroll gesture over a time period by issuing appropriate touch events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizeScrollGestureParams {
    /// X coordinate of the start of the gesture in CSS pixels.

    pub x: f64,
    /// Y coordinate of the start of the gesture in CSS pixels.

    pub y: f64,
    /// The distance to scroll along the X axis (positive to scroll left).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xDistance: Option<f64>,
    /// The distance to scroll along the Y axis (positive to scroll up).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub yDistance: Option<f64>,
    /// The number of additional pixels to scroll back along the X axis, in addition to the given
    /// distance.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xOverscroll: Option<f64>,
    /// The number of additional pixels to scroll back along the Y axis, in addition to the given
    /// distance.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub yOverscroll: Option<f64>,
    /// Prevent fling (default: true).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preventFling: Option<bool>,
    /// Swipe speed in pixels per second (default: 800).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gestureSourceType: Option<GestureSourceType>,
    /// The number of times to repeat the gesture (default: 0).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeatCount: Option<u64>,
    /// The number of milliseconds delay between each repeat. (default: 250).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeatDelayMs: Option<i64>,
    /// The name of the interaction markers to generate, if not empty (default: "").

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactionMarkerName: Option<String>,
}

/// Synthesizes a tap gesture over a time period by issuing appropriate touch events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizeTapGestureParams {
    /// X coordinate of the start of the gesture in CSS pixels.

    pub x: f64,
    /// Y coordinate of the start of the gesture in CSS pixels.

    pub y: f64,
    /// Duration between touchdown and touchup events in ms (default: 50).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Number of times to perform the tap (e.g. 2 for double tap, default: 1).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tapCount: Option<u64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gestureSourceType: Option<GestureSourceType>,
}
