extern crate arrayvec;
#[macro_use]
extern crate failure;
extern crate winapi;

use {
    std::io,
    winapi::{
        shared::{
            minwindef::DWORD,
            ntdef::HANDLE,
        },
        um::{
            consoleapi::{
                GetConsoleMode,
                ReadConsoleInputW,
                SetConsoleMode,
            },
            handleapi::INVALID_HANDLE_VALUE,
            processenv::GetStdHandle,
            winbase::STD_INPUT_HANDLE,
            wincon::{
                CAPSLOCK_ON,
                COORD,
                DOUBLE_CLICK,
                ENABLE_EXTENDED_FLAGS,
                ENABLE_MOUSE_INPUT,
                ENABLE_WINDOW_INPUT,
                FOCUS_EVENT,
                FOCUS_EVENT_RECORD,
                FROM_LEFT_1ST_BUTTON_PRESSED,
                FROM_LEFT_2ND_BUTTON_PRESSED,
                FROM_LEFT_3RD_BUTTON_PRESSED,
                FROM_LEFT_4TH_BUTTON_PRESSED,
                INPUT_RECORD,
                INPUT_RECORD_Event,
                KEY_EVENT,
                KEY_EVENT_RECORD,
                LEFT_ALT_PRESSED,
                LEFT_CTRL_PRESSED,
                MENU_EVENT,
                MENU_EVENT_RECORD,
                MOUSE_EVENT,
                MOUSE_EVENT_RECORD,
                MOUSE_HWHEELED,
                MOUSE_MOVED,
                MOUSE_WHEELED,
                NUMLOCK_ON,
                RIGHTMOST_BUTTON_PRESSED,
                RIGHT_ALT_PRESSED,
                RIGHT_CTRL_PRESSED,
                SCROLLLOCK_ON,
                SHIFT_PRESSED,
                WINDOW_BUFFER_SIZE_EVENT,
                WINDOW_BUFFER_SIZE_RECORD,
            },
            winuser::{
                VK_ACCEPT,
                VK_ADD,
                VK_APPS,
                VK_ATTN,
                VK_BACK,
                VK_BROWSER_BACK,
                VK_BROWSER_FAVORITES,
                VK_BROWSER_FORWARD,
                VK_BROWSER_HOME,
                VK_BROWSER_REFRESH,
                VK_BROWSER_SEARCH,
                VK_BROWSER_STOP,
                VK_CANCEL,
                VK_CAPITAL,
                VK_CLEAR,
                VK_CONTROL,
                VK_CONVERT,
                VK_CRSEL,
                VK_DECIMAL,
                VK_DELETE,
                VK_DIVIDE,
                VK_DOWN,
                VK_END,
                VK_EREOF,
                VK_ESCAPE,
                VK_EXECUTE,
                VK_EXSEL,
                VK_F1,
                VK_F10,
                VK_F11,
                VK_F12,
                VK_F13,
                VK_F14,
                VK_F15,
                VK_F16,
                VK_F17,
                VK_F18,
                VK_F19,
                VK_F2,
                VK_F20,
                VK_F21,
                VK_F22,
                VK_F23,
                VK_F24,
                VK_F3,
                VK_F4,
                VK_F5,
                VK_F6,
                VK_F7,
                VK_F8,
                VK_F9,
                VK_FINAL,
                VK_HELP,
                VK_HOME,
                VK_INSERT,
                VK_JUNJA,
                VK_KANA,
                VK_KANJI,
                VK_LAUNCH_APP1,
                VK_LAUNCH_APP2,
                VK_LAUNCH_MAIL,
                VK_LAUNCH_MEDIA_SELECT,
                VK_LBUTTON,
                VK_LCONTROL,
                VK_LEFT,
                VK_LMENU,
                VK_LSHIFT,
                VK_LWIN,
                VK_MBUTTON,
                VK_MEDIA_NEXT_TRACK,
                VK_MEDIA_PLAY_PAUSE,
                VK_MEDIA_PREV_TRACK,
                VK_MEDIA_STOP,
                VK_MENU,
                VK_MODECHANGE,
                VK_MULTIPLY,
                VK_NEXT,
                VK_NONCONVERT,
                VK_NUMLOCK,
                VK_NUMPAD0,
                VK_NUMPAD1,
                VK_NUMPAD2,
                VK_NUMPAD3,
                VK_NUMPAD4,
                VK_NUMPAD5,
                VK_NUMPAD6,
                VK_NUMPAD7,
                VK_NUMPAD8,
                VK_NUMPAD9,
                VK_OEM_CLEAR,
                VK_OEM_COMMA,
                VK_OEM_MINUS,
                VK_OEM_PERIOD,
                VK_OEM_PLUS,
                VK_PA1,
                VK_PAUSE,
                VK_PLAY,
                VK_PRINT,
                VK_PRIOR,
                VK_PROCESSKEY,
                VK_RBUTTON,
                VK_RCONTROL,
                VK_RETURN,
                VK_RIGHT,
                VK_RMENU,
                VK_RSHIFT,
                VK_RWIN,
                VK_SCROLL,
                VK_SELECT,
                VK_SEPARATOR,
                VK_SHIFT,
                VK_SLEEP,
                VK_SNAPSHOT,
                VK_SPACE,
                VK_SUBTRACT,
                VK_TAB,
                VK_UP,
                VK_VOLUME_DOWN,
                VK_VOLUME_MUTE,
                VK_VOLUME_UP,
                VK_XBUTTON1,
                VK_XBUTTON2,
                VK_ZOOM,
            },
        },
    },
};

#[derive(Debug)]
pub struct ConsoleEventReader {
    handle: HANDLE,
    console_mode_to_restore: DWORD,
}

#[derive(Debug)]
pub struct ConsoleEventOptions {
    enable_window_events: bool,
    enable_mouse_events: bool,
}

#[derive(Debug)]
pub enum ConsoleEvent {
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
    WindowResize(i16, i16),
    Focus(bool),
    Menu(u32),
}

#[derive(Debug)]
pub struct KeyboardEvent {
    key_down: bool,
    repeat_count: u16,
    key_code: KeyCode,
    character: char,
    control_key_state: ControlKeyState,
}

#[derive(Debug)]
pub struct ControlKeyState(u32);

impl ControlKeyState {
    pub fn capslock_enabled(&self) -> bool {
        self.0 & CAPSLOCK_ON > 0
    }

    pub fn left_control_pressed(&self) -> bool {
        self.0 & LEFT_CTRL_PRESSED > 0
    }

    pub fn left_alt_pressed(&self) -> bool {
        self.0 & LEFT_ALT_PRESSED > 0
    }

    pub fn num_lock_enabled(&self) -> bool {
        self.0 & NUMLOCK_ON > 0
    }

    pub fn right_alt_pressed(&self) -> bool {
        self.0 & RIGHT_ALT_PRESSED > 0
    }

    pub fn right_ctrl_pressed(&self) -> bool {
        self.0 & RIGHT_CTRL_PRESSED > 0
    }

    pub fn scroll_lock_enabled(&self) -> bool {
        self.0 & SCROLLLOCK_ON > 0
    }

    pub fn shift_pressed(&self) -> bool {
        self.0 & SHIFT_PRESSED > 0
    }
}

#[derive(Debug)]
pub struct MouseEvent {
    pub coordinates: (i16, i16), // XXX: Maybe make u16?
    pub mouse_state: MouseState,
    pub control_key_state: ControlKeyState,
}

#[derive(Debug)]
pub struct MouseState {
    button_state: u32,
    event_flags: u32,
}

impl MouseState {
    pub fn from_left_1st_button_pressed(&self) -> bool {
        self.button_state & FROM_LEFT_1ST_BUTTON_PRESSED > 0
    }

    pub fn from_left_2nd_button_pressed(&self) -> bool {
        self.button_state & FROM_LEFT_2ND_BUTTON_PRESSED > 0
    }

    pub fn from_left_3rd_button_pressed(&self) -> bool {
        self.button_state & FROM_LEFT_3RD_BUTTON_PRESSED > 0
    }

    pub fn from_left_4th_button_pressed(&self) -> bool {
        self.button_state & FROM_LEFT_4TH_BUTTON_PRESSED > 0
    }

    pub fn rightmost_button_pressed(&self) -> bool {
        self.button_state & RIGHTMOST_BUTTON_PRESSED > 0
    }

    pub fn double_click(&self) -> bool {
        self.event_flags & DOUBLE_CLICK > 0
    }

    pub fn mouse_moved(&self) -> bool {
        self.event_flags & MOUSE_MOVED > 0
    }

    pub fn mouse_scrolled_horizontally(&self) -> Option<HorizontalMouseWheelDirection> {
        use HorizontalMouseWheelDirection::*;
        if self.event_flags & MOUSE_HWHEELED > 0 {
            Some(
                if self.high_word_of_button_state_is_positive() {
                    Right
                } else {
                    Left
                }
            )
        } else {
            None
        }
    }

    pub fn mouse_scrolled_vertically(&self) -> Option<VerticalMouseWheelDirection> {
        use VerticalMouseWheelDirection::*;

        if self.event_flags & MOUSE_WHEELED > 0 {
            Some(
                if self.high_word_of_button_state_is_positive() {
                    Up
                } else {
                    Down
                }
            )
        } else {
            None
        }
    }

    fn high_word_of_button_state_is_positive(&self) -> bool {
        self.button_state & 0xFFFF0000 > 0
    }
}

#[derive(Debug)]
pub enum HorizontalMouseWheelDirection {
    Right,
    Left,
}

#[derive(Debug)]
pub enum VerticalMouseWheelDirection {
    Up,
    Down,
}

#[derive(Debug)]
pub enum KeyCode {
    LeftMouse,
    RightMouse,
    MiddleMouse,
    Break,
    X1Mouse,
    X2Mouse,
    Backspace,
    Tab,
    Clear,
    Enter,
    Shift,
    Control,
    Alt,
    Pause,
    CapsLock,
    ImeKanaMode,
    ImeJunjaMode,
    ImeFinalMode,
    ImeKanjiMode,
    Escape,
    ImeConvert,
    ImeNonConvert,
    ImeAccept,
    ImeModeChange,
    Space,
    PageUp,
    PageDown,
    End,
    Home,
    Left,
    Up,
    Right,
    Down,
    Select,
    Print,
    Execute,
    PrintScreen,
    Insert,
    Delete,
    Help,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftWindows,
    RightWindows,
    Applications,
    Sleep,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Multiply,
    Add,
    Separator,
    Subtract,
    Decimal,
    Divide,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    NumLock,
    ScrollLock,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftMenu,
    RightMenu,
    BrowserBack,
    BrowserForward,
    BrowserStop,
    BrowserSearch,
    BrowserFavorites,
    BrowserRefresh,
    BrowserHome,
    Mute,
    VolumeDown,
    VolumeUp,
    MediaNextTrack,
    MediaPreviousTrack,
    MediaStop,
    MediaPlayPause,
    LaunchMail,
    LaunchMediaSelect,
    LaunchApp1,
    LaunchApp2,
    OemPlus,
    OemComma,
    OemMinus,
    OemPeriod,
    Process,
    Packet,
    Attention,
    CrSel,
    ExSel,
    EraseEof,
    Play,
    Zoom,
    Pa1,
    OemClear,
}

impl KeyCode {
    fn from_u16(x: u16) -> Option<KeyCode> {
        use KeyCode::*;

        Some(match x as i32 {
            VK_ACCEPT => ImeAccept,
            VK_ADD => Add,
            VK_APPS => Applications,
            VK_ATTN => Attention,
            VK_BACK => Backspace,
            VK_BROWSER_BACK => BrowserBack,
            VK_BROWSER_FAVORITES => BrowserFavorites,
            VK_BROWSER_FORWARD => BrowserForward,
            VK_BROWSER_HOME => BrowserHome,
            VK_BROWSER_REFRESH => BrowserRefresh,
            VK_BROWSER_SEARCH => BrowserSearch,
            VK_BROWSER_STOP => BrowserStop,
            VK_CANCEL => Break,
            VK_CAPITAL => CapsLock,
            VK_CLEAR => Clear,
            VK_CONTROL => Control,
            VK_CONVERT => ImeConvert,
            VK_CRSEL => CrSel,
            VK_DECIMAL => Decimal,
            VK_DELETE => Delete,
            VK_DIVIDE => Divide,
            VK_DOWN => Down,
            VK_END => End,
            VK_EREOF => EraseEof,
            VK_ESCAPE => Escape,
            VK_EXECUTE => Execute,
            VK_EXSEL => ExSel,
            VK_F1 => F1,
            VK_F10 => F10,
            VK_F11 => F11,
            VK_F12 => F12,
            VK_F13 => F13,
            VK_F14 => F14,
            VK_F15 => F15,
            VK_F16 => F16,
            VK_F17 => F17,
            VK_F18 => F18,
            VK_F19 => F19,
            VK_F2 => F2,
            VK_F20 => F20,
            VK_F21 => F21,
            VK_F22 => F22,
            VK_F23 => F23,
            VK_F24 => F24,
            VK_F3 => F3,
            VK_F4 => F4,
            VK_F5 => F5,
            VK_F6 => F6,
            VK_F7 => F7,
            VK_F8 => F8,
            VK_F9 => F9,
            VK_FINAL => ImeFinalMode,
            VK_HELP => Help,
            VK_HOME => Help,
            VK_INSERT => Insert,
            VK_JUNJA => ImeJunjaMode,
            VK_KANA => ImeKanaMode,
            VK_KANJI => ImeKanjiMode,
            VK_LAUNCH_APP1 => LaunchApp1,
            VK_LAUNCH_APP2 => LaunchApp2,
            VK_LAUNCH_MAIL => LaunchMail,
            VK_LAUNCH_MEDIA_SELECT => LaunchMediaSelect,
            VK_LBUTTON => LeftMouse,
            VK_LCONTROL => LeftControl,
            VK_LEFT => Left,
            VK_LMENU => LeftMenu,
            VK_LSHIFT => LeftShift,
            VK_LWIN => LeftWindows,
            VK_MBUTTON => MiddleMouse,
            VK_MEDIA_NEXT_TRACK => MediaNextTrack,
            VK_MEDIA_PLAY_PAUSE => MediaPlayPause,
            VK_MEDIA_PREV_TRACK => MediaPreviousTrack,
            VK_MEDIA_STOP => MediaStop,
            VK_MENU => Alt,
            VK_MODECHANGE => ImeModeChange,
            VK_MULTIPLY => Multiply,
            VK_NEXT => PageDown,
            VK_NONCONVERT => ImeNonConvert,
            VK_NUMLOCK => NumLock,
            VK_NUMPAD0 => Numpad0,
            VK_NUMPAD1 => Numpad1,
            VK_NUMPAD2 => Numpad2,
            VK_NUMPAD3 => Numpad3,
            VK_NUMPAD4 => Numpad4,
            VK_NUMPAD5 => Numpad5,
            VK_NUMPAD6 => Numpad6,
            VK_NUMPAD7 => Numpad7,
            VK_NUMPAD8 => Numpad8,
            VK_NUMPAD9 => Numpad9,
            VK_OEM_CLEAR => OemClear,
            VK_OEM_COMMA => OemComma,
            VK_OEM_MINUS => OemMinus,
            VK_OEM_PERIOD => OemPeriod,
            VK_OEM_PLUS => OemPlus,
            VK_PA1 => Pa1,
            VK_PAUSE => Pause,
            VK_PLAY => Play,
            VK_PRINT => Print,
            VK_PRIOR => PageUp,
            VK_PROCESSKEY => Process,
            VK_RBUTTON => RightMouse,
            VK_RCONTROL => RightControl,
            VK_RETURN => Enter,
            VK_RIGHT => Right,
            VK_RMENU => RightMenu,
            VK_RSHIFT => RightShift,
            VK_RWIN => RightWindows,
            VK_SCROLL => ScrollLock,
            VK_SELECT => Select,
            VK_SEPARATOR => Separator,
            VK_SHIFT => Shift,
            VK_SLEEP => Sleep,
            VK_SNAPSHOT => PrintScreen,
            VK_SPACE => Space,
            VK_SUBTRACT => Subtract,
            VK_TAB => Tab,
            VK_UP => Up,
            VK_VOLUME_DOWN => VolumeDown,
            VK_VOLUME_MUTE => Mute,
            VK_VOLUME_UP => VolumeUp,
            VK_XBUTTON1 => X1Mouse,
            VK_XBUTTON2 => X2Mouse,
            VK_ZOOM => Zoom,
            0x30 => Zero,
            0x31 => One,
            0x32 => Two,
            0x33 => Three,
            0x34 => Four,
            0x35 => Five,
            0x36 => Six,
            0x37 => Seven,
            0x38 => Eight,
            0x39 => Nine,
            0x41 => A,
            0x42 => B,
            0x43 => C,
            0x44 => D,
            0x45 => E,
            0x46 => F,
            0x47 => G,
            0x48 => H,
            0x49 => I,
            0x4A => J,
            0x4B => K,
            0x4C => L,
            0x4D => M,
            0x4E => N,
            0x4F => O,
            0x50 => P,
            0x51 => Q,
            0x52 => R,
            0x53 => S,
            0x54 => T,
            0x55 => U,
            0x56 => V,
            0x57 => W,
            0x58 => X,
            0x59 => Y,
            0x5A => Z,
            _ => return None,
        })
    }
}

impl Default for ConsoleEventOptions {
    fn default() -> ConsoleEventOptions {
        ConsoleEventOptions {
            enable_window_events: true,
            enable_mouse_events: true,
        }
    }
}

// FIXME: Allow only a single instance of this to exist!
impl ConsoleEventReader {
    pub fn from_handle(
        handle: HANDLE,
        options: &ConsoleEventOptions,
    ) -> Result<ConsoleEventReader, ConsoleEventsReaderConstructionError> {
        use ConsoleEventsReaderConstructionError::*;

        let console_mode_to_restore = {
            let mut console_mode_to_restore = 0;

            if unsafe { GetConsoleMode(handle, &mut console_mode_to_restore as *mut DWORD) } == 0 {
                return Err(CouldNotGetConsoleMode(io::Error::last_os_error()));
            }

            console_mode_to_restore
        };

        let mut console_mode = ENABLE_EXTENDED_FLAGS;

        if options.enable_mouse_events {
            console_mode |= ENABLE_MOUSE_INPUT
        }

        if options.enable_window_events {
            console_mode |= ENABLE_WINDOW_INPUT
        }

        if unsafe { SetConsoleMode(handle, console_mode) } == 0 {
            return Err(CouldNotSetConsoleMode(io::Error::last_os_error()));
        }

        Ok(ConsoleEventReader {
            handle,
            console_mode_to_restore,
        })
    }

    pub fn from_stdin(
        options: &ConsoleEventOptions,
    ) -> Result<ConsoleEventReader, ConsoleEventsReaderConstructionError> {
        use ConsoleEventsReaderConstructionError::*;

        let stdin = unsafe { GetStdHandle(STD_INPUT_HANDLE) };

        if stdin == INVALID_HANDLE_VALUE {
            return Err(CouldNotGetStdin(io::Error::last_os_error()));
        }

        Self::from_handle(stdin, options)
    }

    pub fn read(
        &mut self,
        records_to_read: u32,
    ) -> Result<(u32, Vec<ConsoleEvent>), ConsoleEventReadError> {
        use ConsoleEventReadError::*;

        let mut records_actually_read: u32 = 0;
        let mut event_buf = Vec::with_capacity(records_to_read as usize);
        if unsafe {
            ReadConsoleInputW(
                self.handle,
                event_buf.as_mut_ptr(),
                records_to_read,
                &mut records_actually_read as *mut DWORD,
            )
        } == 0
        {
            return Err(CouldNotGetEvents(io::Error::last_os_error()));
        }

        unsafe { event_buf.set_len(records_actually_read as usize) };

        let event_buf = event_buf.into_iter().map(|event| {
            let INPUT_RECORD {
                EventType: event_type,
                Event: event,
            } = event;

            use ConsoleEvent::*;
            Ok(match event_type {
                KEY_EVENT => {
                    let KEY_EVENT_RECORD {
                        bKeyDown: key_down,
                        wRepeatCount: repeat_count,
                        wVirtualKeyCode: key_code,
                        wVirtualScanCode: _,
                        uChar: u_char,
                        dwControlKeyState: control_key_state,
                    } = unsafe { event.KeyEvent() };

                    let key_down = *key_down == 0;
                    let repeat_count = *repeat_count;
                    let key_code = KeyCode::from_u16(*key_code)
                        .ok_or_else(|| InvalidKeyCode(*key_code))?;

                    let c16 = unsafe { *u_char.UnicodeChar() };
                    let character = ::std::char::from_u32(c16 as u32)
                        .ok_or_else(|| InvalidUtf16Char(c16))?;
                    let control_key_state = ControlKeyState(*control_key_state);

                    Keyboard(KeyboardEvent {
                        key_down,
                        repeat_count,
                        key_code,
                        character,
                        control_key_state,
                    })
                }
                MOUSE_EVENT => {
                    let MOUSE_EVENT_RECORD {
                        dwMousePosition: COORD {
                            X: x,
                            Y: y,
                        },
                        dwButtonState: button_state,
                        dwControlKeyState: control_key_state,
                        dwEventFlags: event_flags,
                    } = unsafe { event.MouseEvent() };
                    let button_state = *button_state;
                    let event_flags = *event_flags;
                    // TODO: Use a single struct that can get ALL mouse information between event
                    // flags and button state
                    Mouse(MouseEvent {
                        coordinates: (*x, *y),
                        mouse_state: MouseState {
                            button_state,
                            event_flags,
                        },
                        control_key_state: ControlKeyState(*control_key_state),
                    })
                }
                WINDOW_BUFFER_SIZE_EVENT => {
                    let WINDOW_BUFFER_SIZE_RECORD {
                        dwSize: COORD {
                            X: x,
                            Y: y,
                        },
                    } = unsafe { event.WindowBufferSizeEvent() };
                    WindowResize(*x, *y)
                }
                MENU_EVENT => {
                    let MENU_EVENT_RECORD {
                        dwCommandId: command_id,
                    } = unsafe { event.MenuEvent() };
                    Menu(*command_id)
                }
                FOCUS_EVENT => {
                    let FOCUS_EVENT_RECORD {
                        bSetFocus: is_focused,
                    } = unsafe { event.FocusEvent() };
                    let is_focused = *is_focused > 0;
                    Focus(is_focused)
                }
                t @ _ => return Err(UnrecognizedEventType(t)),
            })
        }).collect::<Result<Vec<_>, _>>()?;

        Ok((records_actually_read, event_buf))
    }
}

#[derive(Debug, Fail)]
pub enum ConsoleEventsReaderConstructionError {
    #[fail(display = "unable to get stdin: {}", _0)]
    CouldNotGetStdin(io::Error),
    #[fail(display = "unable to get console mode: {}", _0)]
    CouldNotGetConsoleMode(io::Error),
    #[fail(display = "unable to set console mode: {}", _0)]
    CouldNotSetConsoleMode(io::Error),
}

#[derive(Debug, Fail)]
pub enum ConsoleEventReadError {
    #[fail(display = "unable to get next set of events: {}", _0)]
    CouldNotGetEvents(io::Error),
    #[fail(display = "unrecognized event type {:#X}", _0)]
    UnrecognizedEventType(u16),
    #[fail(display = "invalid UTF-16 character {:#X}", _0)]
    InvalidUtf16Char(u16),
    #[fail(display = "invalid key code {:#X}", _0)]
    InvalidKeyCode(u16),
}
