use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TouchPoint {
    /// X coordinate of the event relative to the main frame's viewport in CSS pixels.
    pub x: f64,
    /// Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    /// the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    pub y: f64,
    /// X radius of the touch area (default: 1.0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "radiusX")]
    pub radius_x: Option<f64>,
    /// Y radius of the touch area (default: 1.0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "radiusY")]
    pub radius_y: Option<f64>,
    /// Rotation angle (default: 0.0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "rotationAngle")]
    pub rotation_angle: Option<f64>,
    /// Force (default: 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<f64>,
    /// The normalized tangential pressure, which has a range of \[-1,1\] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tangentialPressure")]
    pub tangential_pressure: Option<f64>,
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range \[-90,90\], a positive tiltX is to the right (default: 0)
    #[serde(skip_serializing_if = "Option::is_none", rename = "tiltX")]
    pub tilt_x: Option<f64>,
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range \[-90,90\], a positive tiltY is towards the user (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tiltY")]
    pub tilt_y: Option<f64>,
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


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DragDataItem<'a> {
    /// Mime type of the dragged data.
    #[serde(rename = "mimeType")]
    pub mime_type: Cow<'a, str>,
    /// Depending of the value of 'mimeType', it contains the dragged link,
    /// text, HTML markup or any other data.
    pub data: Cow<'a, str>,
    /// Title associated with a link. Only valid when 'mimeType' == "text/uri-list".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'a, str>>,
    /// Stores the base URL for the contained markup. Only valid when 'mimeType'
    /// == "text/html".
    #[serde(skip_serializing_if = "Option::is_none", rename = "baseURL")]
    pub base_url: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DragData<'a> {
    pub items: Vec<DragDataItem<'a>>,
    /// List of filenames that should be included when dropping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<Cow<'a, str>>>,
    /// Bit field representing allowed drag operations. Copy = 1, Link = 2, Move = 16
    #[serde(rename = "dragOperationsMask")]
    pub drag_operations_mask: i64,
}
/// Dispatches a drag event into the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.dispatchDragEvent")]
pub struct DispatchDragEventParams<'a> {
    /// Type of the drag event.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// X coordinate of the event relative to the main frame's viewport in CSS pixels.
    pub x: f64,
    /// Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    /// the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    pub y: f64,
    pub data: DragData<'a>,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<i64>,
}
/// Dispatches a key event to the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.dispatchKeyEvent")]
pub struct DispatchKeyEventParams<'a> {
    /// Type of the key event.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
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
    pub text: Option<Cow<'a, str>>,
    /// Text that would have been generated by the keyboard if no modifiers were pressed (except for
    /// shift). Useful for shortcut (accelerator) key handling (default: "").
    #[serde(skip_serializing_if = "Option::is_none", rename = "unmodifiedText")]
    pub unmodified_text: Option<Cow<'a, str>>,
    /// Unique key identifier (e.g., 'U+0041') (default: "").
    #[serde(skip_serializing_if = "Option::is_none", rename = "keyIdentifier")]
    pub key_identifier: Option<Cow<'a, str>>,
    /// Unique DOM defined string value for each physical key (e.g., 'KeyA') (default: "").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Cow<'a, str>>,
    /// Unique DOM defined string value describing the meaning of the key in the context of active
    /// modifiers, keyboard layout, etc (e.g., 'AltGr') (default: "").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Cow<'a, str>>,
    /// Windows virtual key code (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "windowsVirtualKeyCode")]
    pub windows_virtual_key_code: Option<i64>,
    /// Native virtual key code (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "nativeVirtualKeyCode")]
    pub native_virtual_key_code: Option<i64>,
    /// Whether the event was generated from auto repeat (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "autoRepeat")]
    pub auto_repeat: Option<bool>,
    /// Whether the event was generated from the keypad (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "isKeypad")]
    pub is_keypad: Option<bool>,
    /// Whether the event was a system key event (default: false).
    #[serde(skip_serializing_if = "Option::is_none", rename = "isSystemKey")]
    pub is_system_key: Option<bool>,
    /// Whether the event was from the left or right side of the keyboard. 1=Left, 2=Right (default:
    /// 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<i64>,
    /// Editing commands to send with the key event (e.g., 'selectAll') (default: \[\]).
    /// These are related to but not equal the command names used in 'document.execCommand' and NSStandardKeyBindingResponding.
    /// See <https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/editing/commands/editor_command_names.h> for valid command names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<Cow<'a, str>>>,
}
/// This method emulates inserting text that doesn't come from a key press,
/// for example an emoji keyboard or an IME.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.insertText")]
pub struct InsertTextParams<'a> {
    /// The text to insert.
    pub text: Cow<'a, str>,
}
/// This method sets the current candidate text for IME.
/// Use imeCommitComposition to commit the final text.
/// Use imeSetComposition with empty string as text to cancel composition.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.imeSetComposition")]
pub struct ImeSetCompositionParams<'a> {
    /// The text to insert
    pub text: Cow<'a, str>,
    /// selection start
    #[serde(rename = "selectionStart")]
    pub selection_start: i64,
    /// selection end
    #[serde(rename = "selectionEnd")]
    pub selection_end: i64,
    /// replacement start
    #[serde(skip_serializing_if = "Option::is_none", rename = "replacementStart")]
    pub replacement_start: Option<i64>,
    /// replacement end
    #[serde(skip_serializing_if = "Option::is_none", rename = "replacementEnd")]
    pub replacement_end: Option<i64>,
}
/// Dispatches a mouse event to the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.dispatchMouseEvent")]
pub struct DispatchMouseEventParams<'a> {
    /// Type of the mouse event.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "clickCount")]
    pub click_count: Option<u64>,
    /// The normalized pressure, which has a range of \[0,1\] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<f64>,
    /// The normalized tangential pressure, which has a range of \[-1,1\] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tangentialPressure")]
    pub tangential_pressure: Option<f64>,
    /// The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range \[-90,90\], a positive tiltX is to the right (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tiltX")]
    pub tilt_x: Option<f64>,
    /// The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range \[-90,90\], a positive tiltY is towards the user (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tiltY")]
    pub tilt_y: Option<f64>,
    /// The clockwise rotation of a pen stylus around its own major axis, in degrees in the range \[0,359\] (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twist: Option<i64>,
    /// X delta in CSS pixels for mouse wheel event (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "deltaX")]
    pub delta_x: Option<f64>,
    /// Y delta in CSS pixels for mouse wheel event (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "deltaY")]
    pub delta_y: Option<f64>,
    /// Pointer type (default: "mouse").
    #[serde(skip_serializing_if = "Option::is_none", rename = "pointerType")]
    pub pointer_type: Option<Cow<'a, str>>,
}
/// Dispatches a touch event to the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.dispatchTouchEvent")]
pub struct DispatchTouchEventParams<'a> {
    /// Type of the touch event. TouchEnd and TouchCancel must not contain any touch points, while
    /// TouchStart and TouchMove must contains at least one.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Active touch points on the touch device. One event per any changed point (compared to
    /// previous touch event in a sequence) is generated, emulating pressing/moving/releasing points
    /// one by one.
    #[serde(rename = "touchPoints")]
    pub touch_points: Vec<TouchPoint>,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<i64>,
    /// Time at which the event occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<TimeSinceEpoch>,
}
/// Cancels any active dragging in the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.cancelDragging")]
pub struct CancelDraggingParams {

}
/// Emulates touch event from the mouse event parameters.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.emulateTouchFromMouseEvent")]
pub struct EmulateTouchFromMouseEventParams<'a> {
    /// Type of the mouse event.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "deltaX")]
    pub delta_x: Option<f64>,
    /// Y delta in DIP for mouse wheel event (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "deltaY")]
    pub delta_y: Option<f64>,
    /// Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    /// (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<i64>,
    /// Number of times the mouse button was clicked (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "clickCount")]
    pub click_count: Option<u64>,
}
/// Ignores input events (useful while auditing page).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.setIgnoreInputEvents")]
pub struct SetIgnoreInputEventsParams {
    /// Ignores input events processing when set to true.
    pub ignore: bool,
}
/// Prevents default drag and drop behavior and instead emits 'Input.dragIntercepted' events.
/// Drag and drop behavior can be directly controlled via 'Input.dispatchDragEvent'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.setInterceptDrags")]
pub struct SetInterceptDragsParams {
    pub enabled: bool,
}
/// Synthesizes a pinch gesture over a time period by issuing appropriate touch events.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.synthesizePinchGesture")]
pub struct SynthesizePinchGestureParams {
    /// X coordinate of the start of the gesture in CSS pixels.
    pub x: f64,
    /// Y coordinate of the start of the gesture in CSS pixels.
    pub y: f64,
    /// Relative scale factor after zooming (\>1.0 zooms in, \<1.0 zooms out).
    #[serde(rename = "scaleFactor")]
    pub scale_factor: f64,
    /// Relative pointer speed in pixels per second (default: 800).
    #[serde(skip_serializing_if = "Option::is_none", rename = "relativeSpeed")]
    pub relative_speed: Option<i64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gestureSourceType")]
    pub gesture_source_type: Option<GestureSourceType>,
}
/// Synthesizes a scroll gesture over a time period by issuing appropriate touch events.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.synthesizeScrollGesture")]
pub struct SynthesizeScrollGestureParams<'a> {
    /// X coordinate of the start of the gesture in CSS pixels.
    pub x: f64,
    /// Y coordinate of the start of the gesture in CSS pixels.
    pub y: f64,
    /// The distance to scroll along the X axis (positive to scroll left).
    #[serde(skip_serializing_if = "Option::is_none", rename = "xDistance")]
    pub x_distance: Option<f64>,
    /// The distance to scroll along the Y axis (positive to scroll up).
    #[serde(skip_serializing_if = "Option::is_none", rename = "yDistance")]
    pub y_distance: Option<f64>,
    /// The number of additional pixels to scroll back along the X axis, in addition to the given
    /// distance.
    #[serde(skip_serializing_if = "Option::is_none", rename = "xOverscroll")]
    pub x_overscroll: Option<f64>,
    /// The number of additional pixels to scroll back along the Y axis, in addition to the given
    /// distance.
    #[serde(skip_serializing_if = "Option::is_none", rename = "yOverscroll")]
    pub y_overscroll: Option<f64>,
    /// Prevent fling (default: true).
    #[serde(skip_serializing_if = "Option::is_none", rename = "preventFling")]
    pub prevent_fling: Option<bool>,
    /// Swipe speed in pixels per second (default: 800).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gestureSourceType")]
    pub gesture_source_type: Option<GestureSourceType>,
    /// The number of times to repeat the gesture (default: 0).
    #[serde(skip_serializing_if = "Option::is_none", rename = "repeatCount")]
    pub repeat_count: Option<u64>,
    /// The number of milliseconds delay between each repeat. (default: 250).
    #[serde(skip_serializing_if = "Option::is_none", rename = "repeatDelayMs")]
    pub repeat_delay_ms: Option<i64>,
    /// The name of the interaction markers to generate, if not empty (default: "").
    #[serde(skip_serializing_if = "Option::is_none", rename = "interactionMarkerName")]
    pub interaction_marker_name: Option<Cow<'a, str>>,
}
/// Synthesizes a tap gesture over a time period by issuing appropriate touch events.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.synthesizeTapGesture")]
pub struct SynthesizeTapGestureParams {
    /// X coordinate of the start of the gesture in CSS pixels.
    pub x: f64,
    /// Y coordinate of the start of the gesture in CSS pixels.
    pub y: f64,
    /// Duration between touchdown and touchup events in ms (default: 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Number of times to perform the tap (e.g. 2 for double tap, default: 1).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tapCount")]
    pub tap_count: Option<u64>,
    /// Which type of input events to be generated (default: 'default', which queries the platform
    /// for the preferred input type).
    #[serde(skip_serializing_if = "Option::is_none", rename = "gestureSourceType")]
    pub gesture_source_type: Option<GestureSourceType>,
}
/// Emitted only when 'Input.setInterceptDrags' is enabled. Use this data with 'Input.dispatchDragEvent' to
/// restore normal drag and drop behavior.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Input.dragIntercepted")]
pub struct DragIntercepted<'a> {
    pub data: DragData<'a>,
}