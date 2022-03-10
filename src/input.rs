/// keyboard codes to keys
/// FIXME: just copied from here, but some keys not working
/// Some sort of Union-like bitwise-OR functionality might be intended
/// https://docs.microsoft.com/en-us/dotnet/api/system.windows.forms.keys?view=net-5.0
#[derive(Eq, PartialEq, Debug)]
pub enum KeyCode {
    A = 65,
    Add = 107,
    Alt = 262144,
    Apps = 93,
    B = 66,
    Back = 8,
    C = 67,
    CapsLock = 20,
    Clear = 12,
    ControlKey = 17,
    D = 68,
    D0 = 48,
    D1 = 49,
    D2 = 50,
    D3 = 51,
    D4 = 52,
    D5 = 53,
    D6 = 54,
    D7 = 55,
    D8 = 56,
    D9 = 57,
    Decimal = 110,
    Delete = 46,
    Divide = 111,
    Down = 40,
    E = 69,
    End = 35,
    Enter = 13,
    Escape = 27,
    F = 70,
    F1 = 112,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    G = 71,
    H = 72,
    Help = 47,
    Home = 36,
    I = 73,
    Insert = 45,
    J = 74,
    JunjaMode = 23,
    K = 75,
    KeyCode = 65535,
    L = 76,
    LaunchApplication1 = 182,
    LaunchApplication2 = 183,
    LaunchMail = 180,
    LButton = 1,
    LControlKey = 162,
    Left = 37,
    LineFeed = 10,
    LMenu = 164,
    LShiftKey = 160,
    LWin = 91,
    M = 77,
    MButton = 4,
    MediaNextTrack = 176,
    MediaPlayPause = 179,
    MediaPreviousTrack = 177,
    MediaStop = 178,
    Menu = 18,
    Multiply = 106,
    N = 78,
    NumLock = 144,
    NumPad0 = 96,
    NumPad1 = 97,
    NumPad2 = 98,
    NumPad3 = 99,
    NumPad4 = 100,
    NumPad5 = 101,
    NumPad6 = 102,
    NumPad7 = 103,
    NumPad8 = 104,
    NumPad9 = 105,
    O = 79,
    P = 80,
    Pa1 = 253,
    Packet = 231,
    PageDown = 34,
    PageUp = 33,
    Pause = 19,
    Play = 250,
    Print = 42,
    PrintScreen = 44,
    Q = 81,
    R = 82,
    RButton = 2,
    RControlKey = 163,
    Right = 39,
    RMenu = 165,
    RShiftKey = 161,
    RWin = 92,
    S = 83,
    Scroll = 145,
    Select = 41,
    SelectMedia = 181,
    Separator = 108,
    Shift = 65536,
    ShiftKey = 16,
    Space = 32,
    Subtract = 109,
    T = 84,
    Tab = 9,
    U = 85,
    Up = 38,
    V = 86,
    VolumeDown = 174,
    VolumeMute = 173,
    VolumeUp = 175,
    W = 87,
    X = 88,
    XButton1 = 5,
    XButton2 = 6,
    Y = 89,
    Z = 90,
    Zoom = 251,
}

impl KeyCode {
    pub fn from_raw(val: usize) -> Option<Self> {
        match val {
            65 => Some(Self::A),
            107 => Some(Self::Add),
            262144 => Some(Self::Alt),
            93 => Some(Self::Apps),
            66 => Some(Self::B),
            8 => Some(Self::Back),
            67 => Some(Self::C),
            20 => Some(Self::CapsLock),
            12 => Some(Self::Clear),
            17 => Some(Self::ControlKey),
            68 => Some(Self::D),
            48 => Some(Self::D0),
            49 => Some(Self::D1),
            50 => Some(Self::D2),
            51 => Some(Self::D3),
            52 => Some(Self::D4),
            53 => Some(Self::D5),
            54 => Some(Self::D6),
            55 => Some(Self::D7),
            56 => Some(Self::D8),
            57 => Some(Self::D9),
            110 => Some(Self::Decimal),
            46 => Some(Self::Delete),
            111 => Some(Self::Divide),
            40 => Some(Self::Down),
            69 => Some(Self::E),
            35 => Some(Self::End),
            13 => Some(Self::Enter),
            27 => Some(Self::Escape),
            70 => Some(Self::F),
            112 => Some(Self::F1),
            121 => Some(Self::F10),
            122 => Some(Self::F11),
            123 => Some(Self::F12),
            113 => Some(Self::F2),
            114 => Some(Self::F3),
            115 => Some(Self::F4),
            116 => Some(Self::F5),
            117 => Some(Self::F6),
            118 => Some(Self::F7),
            119 => Some(Self::F8),
            120 => Some(Self::F9),
            71 => Some(Self::G),
            72 => Some(Self::H),
            47 => Some(Self::Help),
            36 => Some(Self::Home),
            73 => Some(Self::I),
            45 => Some(Self::Insert),
            74 => Some(Self::J),
            23 => Some(Self::JunjaMode),
            75 => Some(Self::K),
            65535 => Some(Self::KeyCode),
            76 => Some(Self::L),
            182 => Some(Self::LaunchApplication1),
            183 => Some(Self::LaunchApplication2),
            180 => Some(Self::LaunchMail),
            1 => Some(Self::LButton),
            162 => Some(Self::LControlKey),
            37 => Some(Self::Left),
            10 => Some(Self::LineFeed),
            164 => Some(Self::LMenu),
            160 => Some(Self::LShiftKey),
            91 => Some(Self::LWin),
            77 => Some(Self::M),
            4 => Some(Self::MButton),
            176 => Some(Self::MediaNextTrack),
            179 => Some(Self::MediaPlayPause),
            177 => Some(Self::MediaPreviousTrack),
            178 => Some(Self::MediaStop),
            18 => Some(Self::Menu),
            106 => Some(Self::Multiply),
            78 => Some(Self::N),
            144 => Some(Self::NumLock),
            96 => Some(Self::NumPad0),
            97 => Some(Self::NumPad1),
            98 => Some(Self::NumPad2),
            99 => Some(Self::NumPad3),
            100 => Some(Self::NumPad4),
            101 => Some(Self::NumPad5),
            102 => Some(Self::NumPad6),
            103 => Some(Self::NumPad7),
            104 => Some(Self::NumPad8),
            105 => Some(Self::NumPad9),
            79 => Some(Self::O),
            80 => Some(Self::P),
            253 => Some(Self::Pa1),
            231 => Some(Self::Packet),
            34 => Some(Self::PageDown),
            33 => Some(Self::PageUp),
            19 => Some(Self::Pause),
            250 => Some(Self::Play),
            42 => Some(Self::Print),
            44 => Some(Self::PrintScreen),
            81 => Some(Self::Q),
            82 => Some(Self::R),
            2 => Some(Self::RButton),
            163 => Some(Self::RControlKey),
            39 => Some(Self::Right),
            165 => Some(Self::RMenu),
            161 => Some(Self::RShiftKey),
            92 => Some(Self::RWin),
            83 => Some(Self::S),
            145 => Some(Self::Scroll),
            41 => Some(Self::Select),
            181 => Some(Self::SelectMedia),
            108 => Some(Self::Separator),
            65536 => Some(Self::Shift),
            16 => Some(Self::ShiftKey),
            32 => Some(Self::Space),
            109 => Some(Self::Subtract),
            84 => Some(Self::T),
            9 => Some(Self::Tab),
            85 => Some(Self::U),
            38 => Some(Self::Up),
            86 => Some(Self::V),
            174 => Some(Self::VolumeDown),
            173 => Some(Self::VolumeMute),
            175 => Some(Self::VolumeUp),
            87 => Some(Self::W),
            88 => Some(Self::X),
            5 => Some(Self::XButton1),
            6 => Some(Self::XButton2),
            89 => Some(Self::Y),
            90 => Some(Self::Z),
            251 => Some(Self::Zoom),
            _ => None,
        }
    }
}
