use glutin;

///! These constants come from and are inspired by the pyprocessing project
///! (https://code.google.com/p/pyprocessing/source/browse/trunk/pyprocessing/constants.py)
///! with some slight adjustments

// shapes
// pub const POINTS = GL_POINTS;
// pub const LINES = GL_LINES;
// pub const TRIANGLES = GL_TRIANGLES;
// pub const TRIANGLE_FAN = GL_TRIANGLE_FAN;
// pub const TRIANGLE_STRIP = GL_TRIANGLE_STRIP;
// pub const QUADS = GL_QUADS;
// pub const QUAD_STRIP = GL_QUAD_STRIP;
pub const CLOSE: isize = 1;

/// Perlin noise table. Use it as a precomputed source of noise that emulates the
/// randomness found in natural processes.
pub const PERLIN: [isize; 512] = [
    151,
    160,
    137,
    91,
    90,
    15,
    131,
    13,
    201,
    95,
    96,
    53,
    194,
    233,
    7,
    225,
    140,
    36,
    103,
    30,
    69,
    142,
    8,
    99,
    37,
    240,
    21,
    10,
    23,
    190,
    6,
    148,
    247,
    120,
    234,
    75,
    0,
    26,
    197,
    62,
    94,
    252,
    219,
    203,
    117,
    35,
    11,
    32,
    57,
    177,
    33,
    88,
    237,
    149,
    56,
    87,
    174,
    20,
    125,
    136,
    171,
    168,
    68,
    175,
    74,
    165,
    71,
    134,
    139,
    48,
    27,
    166,
    77,
    146,
    158,
    231,
    83,
    111,
    229,
    122,
    60,
    211,
    133,
    230,
    220,
    105,
    92,
    41,
    55,
    46,
    245,
    40,
    244,
    102,
    143,
    54,
    65,
    25,
    63,
    161,
    1,
    216,
    80,
    73,
    209,
    76,
    132,
    187,
    208,
    89,
    18,
    169,
    200,
    196,
    135,
    130,
    116,
    188,
    159,
    86,
    164,
    100,
    109,
    198,
    173,
    186,
    3,
    64,
    52,
    217,
    226,
    250,
    124,
    123,
    5,
    202,
    38,
    147,
    118,
    126,
    255,
    82,
    85,
    212,
    207,
    206,
    59,
    227,
    47,
    16,
    58,
    17,
    182,
    189,
    28,
    42,
    223,
    183,
    170,
    213,
    119,
    248,
    152,
    2,
    44,
    154,
    163,
    70,
    221,
    153,
    101,
    155,
    167,
    43,
    172,
    9,
    129,
    22,
    39,
    253,
    19,
    98,
    108,
    110,
    79,
    113,
    224,
    232,
    178,
    185,
    112,
    104,
    218,
    246,
    97,
    228,
    251,
    34,
    242,
    193,
    238,
    210,
    144,
    12,
    191,
    179,
    162,
    241,
    81,
    51,
    145,
    235,
    249,
    14,
    239,
    107,
    49,
    192,
    214,
    31,
    181,
    199,
    106,
    157,
    184,
    84,
    204,
    176,
    115,
    121,
    50,
    45,
    127,
    4,
    150,
    254,
    138,
    236,
    205,
    93,
    222,
    114,
    67,
    29,
    24,
    72,
    243,
    141,
    128,
    195,
    78,
    66,
    215,
    61,
    156,
    180,
    151,
    160,
    137,
    91,
    90,
    15,
    131,
    13,
    201,
    95,
    96,
    53,
    194,
    233,
    7,
    225,
    140,
    36,
    103,
    30,
    69,
    142,
    8,
    99,
    37,
    240,
    21,
    10,
    23,
    190,
    6,
    148,
    247,
    120,
    234,
    75,
    0,
    26,
    197,
    62,
    94,
    252,
    219,
    203,
    117,
    35,
    11,
    32,
    57,
    177,
    33,
    88,
    237,
    149,
    56,
    87,
    174,
    20,
    125,
    136,
    171,
    168,
    68,
    175,
    74,
    165,
    71,
    134,
    139,
    48,
    27,
    166,
    77,
    146,
    158,
    231,
    83,
    111,
    229,
    122,
    60,
    211,
    133,
    230,
    220,
    105,
    92,
    41,
    55,
    46,
    245,
    40,
    244,
    102,
    143,
    54,
    65,
    25,
    63,
    161,
    1,
    216,
    80,
    73,
    209,
    76,
    132,
    187,
    208,
    89,
    18,
    169,
    200,
    196,
    135,
    130,
    116,
    188,
    159,
    86,
    164,
    100,
    109,
    198,
    173,
    186,
    3,
    64,
    52,
    217,
    226,
    250,
    124,
    123,
    5,
    202,
    38,
    147,
    118,
    126,
    255,
    82,
    85,
    212,
    207,
    206,
    59,
    227,
    47,
    16,
    58,
    17,
    182,
    189,
    28,
    42,
    223,
    183,
    170,
    213,
    119,
    248,
    152,
    2,
    44,
    154,
    163,
    70,
    221,
    153,
    101,
    155,
    167,
    43,
    172,
    9,
    129,
    22,
    39,
    253,
    19,
    98,
    108,
    110,
    79,
    113,
    224,
    232,
    178,
    185,
    112,
    104,
    218,
    246,
    97,
    228,
    251,
    34,
    242,
    193,
    238,
    210,
    144,
    12,
    191,
    179,
    162,
    241,
    81,
    51,
    145,
    235,
    249,
    14,
    239,
    107,
    49,
    192,
    214,
    31,
    181,
    199,
    106,
    157,
    184,
    84,
    204,
    176,
    115,
    121,
    50,
    45,
    127,
    4,
    150,
    254,
    138,
    236,
    205,
    93,
    222,
    114,
    67,
    29,
    24,
    72,
    243,
    141,
    128,
    195,
    78,
    66,
    215,
    61,
    156,
    180,
];

/// The different mouse events that can be the result of user input.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
}

impl From<MouseButton> for glutin::MouseButton {
    fn from(btn: MouseButton) -> Self {
        match btn {
            MouseButton::Left => glutin::MouseButton::Left,
            MouseButton::Right => glutin::MouseButton::Right,
            MouseButton::Middle => glutin::MouseButton::Middle,
            MouseButton::Button4 => glutin::MouseButton::Other(4),
            MouseButton::Button5 => glutin::MouseButton::Other(5),
            MouseButton::Button6 => glutin::MouseButton::Other(6),
            MouseButton::Button7 => glutin::MouseButton::Other(7),
            MouseButton::Button8 => glutin::MouseButton::Other(8),
        }
    }
}

/// The different key events that can be the result of user input.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Key {
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
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
    Escape,
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
    Snapshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Compose,
    Numlock,
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
    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    //LMenu,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    //RMenu,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
}

impl From<Key> for glutin::VirtualKeyCode {
    fn from(btn: Key) -> Self {
        match btn {
            Key::Num1 => glutin::VirtualKeyCode::Key1,
            Key::Num2 => glutin::VirtualKeyCode::Key2,
            Key::Num3 => glutin::VirtualKeyCode::Key3,
            Key::Num4 => glutin::VirtualKeyCode::Key4,
            Key::Num5 => glutin::VirtualKeyCode::Key5,
            Key::Num6 => glutin::VirtualKeyCode::Key6,
            Key::Num7 => glutin::VirtualKeyCode::Key7,
            Key::Num8 => glutin::VirtualKeyCode::Key8,
            Key::Num9 => glutin::VirtualKeyCode::Key9,
            Key::Num0 => glutin::VirtualKeyCode::Key0,
            Key::A => glutin::VirtualKeyCode::A,
            Key::B => glutin::VirtualKeyCode::B,
            Key::C => glutin::VirtualKeyCode::C,
            Key::D => glutin::VirtualKeyCode::D,
            Key::E => glutin::VirtualKeyCode::E,
            Key::F => glutin::VirtualKeyCode::F,
            Key::G => glutin::VirtualKeyCode::G,
            Key::H => glutin::VirtualKeyCode::H,
            Key::I => glutin::VirtualKeyCode::I,
            Key::J => glutin::VirtualKeyCode::J,
            Key::K => glutin::VirtualKeyCode::K,
            Key::L => glutin::VirtualKeyCode::L,
            Key::M => glutin::VirtualKeyCode::M,
            Key::N => glutin::VirtualKeyCode::N,
            Key::O => glutin::VirtualKeyCode::O,
            Key::P => glutin::VirtualKeyCode::P,
            Key::Q => glutin::VirtualKeyCode::Q,
            Key::R => glutin::VirtualKeyCode::R,
            Key::S => glutin::VirtualKeyCode::S,
            Key::T => glutin::VirtualKeyCode::T,
            Key::U => glutin::VirtualKeyCode::U,
            Key::V => glutin::VirtualKeyCode::V,
            Key::W => glutin::VirtualKeyCode::W,
            Key::X => glutin::VirtualKeyCode::X,
            Key::Y => glutin::VirtualKeyCode::Y,
            Key::Z => glutin::VirtualKeyCode::Z,
            Key::Escape => glutin::VirtualKeyCode::Escape,
            Key::F1 => glutin::VirtualKeyCode::F1,
            Key::F2 => glutin::VirtualKeyCode::F2,
            Key::F3 => glutin::VirtualKeyCode::F3,
            Key::F4 => glutin::VirtualKeyCode::F4,
            Key::F5 => glutin::VirtualKeyCode::F5,
            Key::F6 => glutin::VirtualKeyCode::F6,
            Key::F7 => glutin::VirtualKeyCode::F7,
            Key::F8 => glutin::VirtualKeyCode::F8,
            Key::F9 => glutin::VirtualKeyCode::F9,
            Key::F10 => glutin::VirtualKeyCode::F10,
            Key::F11 => glutin::VirtualKeyCode::F11,
            Key::F12 => glutin::VirtualKeyCode::F12,
            Key::F13 => glutin::VirtualKeyCode::F13,
            Key::F14 => glutin::VirtualKeyCode::F14,
            Key::F15 => glutin::VirtualKeyCode::F15,
            Key::Snapshot => glutin::VirtualKeyCode::Snapshot,
            Key::Scroll => glutin::VirtualKeyCode::Scroll,
            Key::Pause => glutin::VirtualKeyCode::Pause,
            Key::Insert => glutin::VirtualKeyCode::Insert,
            Key::Home => glutin::VirtualKeyCode::Home,
            Key::Delete => glutin::VirtualKeyCode::Delete,
            Key::End => glutin::VirtualKeyCode::End,
            Key::PageDown => glutin::VirtualKeyCode::PageDown,
            Key::PageUp => glutin::VirtualKeyCode::PageUp,
            Key::Left => glutin::VirtualKeyCode::Left,
            Key::Up => glutin::VirtualKeyCode::Up,
            Key::Right => glutin::VirtualKeyCode::Right,
            Key::Down => glutin::VirtualKeyCode::Down,
            Key::Back => glutin::VirtualKeyCode::Back,
            Key::Return => glutin::VirtualKeyCode::Return,
            Key::Space => glutin::VirtualKeyCode::Space,
            Key::Compose => glutin::VirtualKeyCode::Compose,
            Key::Numlock => glutin::VirtualKeyCode::Numlock,
            Key::Numpad0 => glutin::VirtualKeyCode::Numpad0,
            Key::Numpad1 => glutin::VirtualKeyCode::Numpad1,
            Key::Numpad2 => glutin::VirtualKeyCode::Numpad2,
            Key::Numpad3 => glutin::VirtualKeyCode::Numpad3,
            Key::Numpad4 => glutin::VirtualKeyCode::Numpad4,
            Key::Numpad5 => glutin::VirtualKeyCode::Numpad5,
            Key::Numpad6 => glutin::VirtualKeyCode::Numpad6,
            Key::Numpad7 => glutin::VirtualKeyCode::Numpad7,
            Key::Numpad8 => glutin::VirtualKeyCode::Numpad8,
            Key::Numpad9 => glutin::VirtualKeyCode::Numpad9,
            Key::AbntC1 => glutin::VirtualKeyCode::AbntC1,
            Key::AbntC2 => glutin::VirtualKeyCode::AbntC2,
            Key::Add => glutin::VirtualKeyCode::Add,
            Key::Apostrophe => glutin::VirtualKeyCode::Apostrophe,
            Key::Apps => glutin::VirtualKeyCode::Apps,
            Key::At => glutin::VirtualKeyCode::At,
            Key::Ax => glutin::VirtualKeyCode::Ax,
            Key::Backslash => glutin::VirtualKeyCode::Backslash,
            Key::Calculator => glutin::VirtualKeyCode::Calculator,
            Key::Capital => glutin::VirtualKeyCode::Capital,
            Key::Colon => glutin::VirtualKeyCode::Colon,
            Key::Comma => glutin::VirtualKeyCode::Comma,
            Key::Convert => glutin::VirtualKeyCode::Convert,
            Key::Decimal => glutin::VirtualKeyCode::Decimal,
            Key::Divide => glutin::VirtualKeyCode::Divide,
            Key::Equals => glutin::VirtualKeyCode::Equals,
            Key::Grave => glutin::VirtualKeyCode::Grave,
            Key::Kana => glutin::VirtualKeyCode::Kana,
            Key::Kanji => glutin::VirtualKeyCode::Kanji,
            Key::LAlt => glutin::VirtualKeyCode::LAlt,
            Key::LBracket => glutin::VirtualKeyCode::LBracket,
            Key::LControl => glutin::VirtualKeyCode::LControl,
            //Key::LMenu => glutin::VirtualKeyCode::LMenu,
            Key::LShift => glutin::VirtualKeyCode::LShift,
            Key::LWin => glutin::VirtualKeyCode::LWin,
            Key::Mail => glutin::VirtualKeyCode::Mail,
            Key::MediaSelect => glutin::VirtualKeyCode::MediaSelect,
            Key::MediaStop => glutin::VirtualKeyCode::MediaStop,
            Key::Minus => glutin::VirtualKeyCode::Minus,
            Key::Multiply => glutin::VirtualKeyCode::Multiply,
            Key::Mute => glutin::VirtualKeyCode::Mute,
            Key::MyComputer => glutin::VirtualKeyCode::MyComputer,
            Key::NavigateForward => glutin::VirtualKeyCode::NavigateForward,
            Key::NavigateBackward => glutin::VirtualKeyCode::NavigateBackward,
            Key::NextTrack => glutin::VirtualKeyCode::NextTrack,
            Key::NoConvert => glutin::VirtualKeyCode::NoConvert,
            Key::NumpadComma => glutin::VirtualKeyCode::NumpadComma,
            Key::NumpadEnter => glutin::VirtualKeyCode::NumpadEnter,
            Key::NumpadEquals => glutin::VirtualKeyCode::NumpadEquals,
            Key::OEM102 => glutin::VirtualKeyCode::OEM102,
            Key::Period => glutin::VirtualKeyCode::Period,
            Key::PlayPause => glutin::VirtualKeyCode::PlayPause,
            Key::Power => glutin::VirtualKeyCode::Power,
            Key::PrevTrack => glutin::VirtualKeyCode::PrevTrack,
            Key::RAlt => glutin::VirtualKeyCode::RAlt,
            Key::RBracket => glutin::VirtualKeyCode::RBracket,
            Key::RControl => glutin::VirtualKeyCode::RControl,
            //Key::RMenu => glutin::VirtualKeyCode::RMenu,
            Key::RShift => glutin::VirtualKeyCode::RShift,
            Key::RWin => glutin::VirtualKeyCode::RWin,
            Key::Semicolon => glutin::VirtualKeyCode::Semicolon,
            Key::Slash => glutin::VirtualKeyCode::Slash,
            Key::Sleep => glutin::VirtualKeyCode::Sleep,
            Key::Stop => glutin::VirtualKeyCode::Stop,
            Key::Subtract => glutin::VirtualKeyCode::Subtract,
            Key::Sysrq => glutin::VirtualKeyCode::Sysrq,
            Key::Tab => glutin::VirtualKeyCode::Tab,
            Key::Underline => glutin::VirtualKeyCode::Underline,
            Key::Unlabeled => glutin::VirtualKeyCode::Unlabeled,
            Key::VolumeDown => glutin::VirtualKeyCode::VolumeDown,
            Key::VolumeUp => glutin::VirtualKeyCode::VolumeUp,
            Key::Wake => glutin::VirtualKeyCode::Wake,
            Key::WebBack => glutin::VirtualKeyCode::WebBack,
            Key::WebFavorites => glutin::VirtualKeyCode::WebFavorites,
            Key::WebForward => glutin::VirtualKeyCode::WebForward,
            Key::WebHome => glutin::VirtualKeyCode::WebHome,
            Key::WebRefresh => glutin::VirtualKeyCode::WebRefresh,
            Key::WebSearch => glutin::VirtualKeyCode::WebSearch,
            Key::WebStop => glutin::VirtualKeyCode::WebStop,
            Key::Yen => glutin::VirtualKeyCode::Yen,
        }
    }
}
