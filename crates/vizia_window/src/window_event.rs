use crate::{CursorIcon, Position, WindowSize};
use morphorm::GeometryChanged;
use vizia_input::{Code, Key, MouseButton};

/// Events generated by the application in response to OS events as well as events that can be used
/// to set properties of the window.
///
/// This type is part of the prelude.
#[derive(Debug, Clone)]
pub enum WindowEvent {
    /// Emitted when a window is closed
    WindowClose,
    /// Emitted when a window is opened
    WindowResize(f32, f32),
    /// Emitted when a mouse button is double clicked
    MouseDoubleClick(MouseButton),
    /// Emitted when a mouse button is triple clicked
    MouseTripleClick(MouseButton),
    /// Emitted when a mouse button is pressed
    MouseDown(MouseButton),
    /// Emitted when a mouse button is released
    MouseUp(MouseButton),
    /// When an interactable element has started to be triggered with a left mouse click or keyboard
    TriggerDown {
        mouse: bool,
    },
    /// When an interactable element has stopped being triggered with a left mouse click or keyboard
    TriggerUp {
        mouse: bool,
    },
    /// Emitted when the mouse cursor is moved
    MouseMove(f32, f32),
    /// Emitted when the mouse scroll wheel is scrolled
    MouseScroll(f32, f32),
    /// Emitted when the mouse cursor enters the bounding box of an entity
    MouseOver,
    /// Emitted when the mouse cursor leaves the bounding box of an entity
    MouseOut,
    /// Emitted when the mouse cursor enters an entity or one of its descendants
    MouseEnter,
    /// Emitted when the mouse cursor leaves an entity or one of its descendants
    MouseLeave,

    FocusIn,

    FocusOut,

    /// Emitted when a character is typed
    CharInput(char),
    /// Emitted when a keyboard key is pressed
    KeyDown(Code, Option<Key>),
    /// Emitted when a keyboard key is released
    KeyUp(Code, Option<Key>),
    /// Sets the mouse cursor icon
    SetCursor(CursorIcon),
    /// Grabs the mouse cursor, preventing it from leaving the window
    GrabCursor(bool),
    /// Sets the (x,y) position of the mouse cursor in window coordinates
    SetCursorPosition(u32, u32),

    SetTitle(String),
    SetSize(WindowSize),
    SetPosition(Position),
    SetMaxSize(Option<WindowSize>),
    SetMinSize(Option<WindowSize>),
    SetResizable(bool),
    SetMinimized(bool),
    SetMaximized(bool),
    SetVisible(bool),
    SetDecorations(bool),
    SetAlwaysOnTop(bool),

    /// Emitted when mouse events have been captured
    MouseCaptureEvent,
    /// Emitted when mouse events have been released
    MouseCaptureOutEvent,
    /// Emitted when an entity changes position or size (TODO: check if this includes margins + borders)
    GeometryChanged(GeometryChanged),
    /// Requests a redraw of the window contents
    Redraw,
    /// Request a restyle
    Restyle,
    /// Requests a relayout
    Relayout,
    /// Prints the debug message to the console
    Debug(String),
}
