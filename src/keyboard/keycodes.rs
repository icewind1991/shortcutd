use num_enum::TryFromPrimitive;
use parse_display::{Display, FromStr};

#[derive(Copy, Clone, Debug, TryFromPrimitive, PartialEq, Eq, Hash, Display, FromStr)]
#[repr(u16)]
pub enum Key {
    KeyReserved = 0,
    KeyEsc = 1,
    Key1 = 2,
    Key2 = 3,
    Key3 = 4,
    Key4 = 5,
    Key5 = 6,
    Key6 = 7,
    Key7 = 8,
    Key8 = 9,
    Key9 = 10,
    Key0 = 11,
    KeyMinus = 12,
    KeyEqual = 13,
    KeyBackspace = 14,
    KeyTab = 15,
    KeyQ = 16,
    KeyW = 17,
    KeyE = 18,
    KeyR = 19,
    KeyT = 20,
    KeyY = 21,
    KeyU = 22,
    KeyI = 23,
    KeyO = 24,
    KeyP = 25,
    KeyLeftBrace = 26,
    KeyRightBrace = 27,
    KeyEnter = 28,
    KeyLeftCtrl = 29,
    KeyA = 30,
    KeyS = 31,
    KeyD = 32,
    KeyF = 33,
    KeyG = 34,
    KeyH = 35,
    KeyJ = 36,
    KeyK = 37,
    KeyL = 38,
    KeySemicolon = 39,
    KeyApostrophe = 40,
    KeyGrave = 41,
    KeyLeftShift = 42,
    KeyBackslash = 43,
    KeyZ = 44,
    KeyX = 45,
    KeyC = 46,
    KeyV = 47,
    KeyB = 48,
    KeyN = 49,
    KeyM = 50,
    KeyComma = 51,
    KeyDot = 52,
    KeySlash = 53,
    KeyRightShift = 54,
    KeyKpAsterisk = 55,
    KeyLeftAlt = 56,
    KeySpace = 57,
    KeyCapsLock = 58,
    KeyF1 = 59,
    KeyF2 = 60,
    KeyF3 = 61,
    KeyF4 = 62,
    KeyF5 = 63,
    KeyF6 = 64,
    KeyF7 = 65,
    KeyF8 = 66,
    KeyF9 = 67,
    KeyF10 = 68,
    KeyNumLock = 69,
    KeyScrollLock = 70,
    KeyKp7 = 71,
    KeyKp8 = 72,
    KeyKp9 = 73,
    KeyKpMinus = 74,
    KeyKp4 = 75,
    KeyKp5 = 76,
    KeyKp6 = 77,
    KeyKpPlus = 78,
    KeyKp1 = 79,
    KeyKp2 = 80,
    KeyKp3 = 81,
    KeyKp0 = 82,
    KeyKpDot = 83,
    KeyZenkakuhankaku = 85,
    Key102nd = 86,
    KeyF11 = 87,
    KeyF12 = 88,
    KeyRo = 89,
    KeyKatakana = 90,
    KeyHiragana = 91,
    KeyHenkan = 92,
    KeyKatakanahiragana = 93,
    KeyMuhenkan = 94,
    KeyKpJpComma = 95,
    KeyKpEnter = 96,
    KeyRightCtrl = 97,
    KeyKpSlash = 98,
    KeySysRQ = 99,
    KeyRightAlt = 100,
    KeyLinefeed = 101,
    KeyHome = 102,
    KeyUp = 103,
    KeyPageup = 104,
    KeyLeft = 105,
    KeyRight = 106,
    KeyEnd = 107,
    KeyDown = 108,
    KeyPagedown = 109,
    KeyInsert = 110,
    KeyDelete = 111,
    KeyMacro = 112,
    KeyMute = 113,
    KeyVolumeDown = 114,
    KeyVolumeUp = 115,
    KeyPower = 116, /* SC System Power Down */
    KeyKpEqual = 117,
    KeyKpPlusMinus = 118,
    KeyPause = 119,
    KeyScale = 120, /* AL Compiz Scale (Expose) */
    KeyKpComma = 121,
    KeyHangeul = 122,
    KeyHanja = 123,
    KeyYen = 124,
    KeyLeftMeta = 125,
    KeyRightMeta = 126,
    KeyCompose = 127,
    KeyStop = 128, /* AC Stop */
    KeyAgain = 129,
    KeyProps = 130, /* AC Properties */
    KeyUndo = 131,  /* AC Undo */
    KeyFront = 132,
    KeyCopy = 133,  /* AC Copy */
    KeyOpen = 134,  /* AC Open */
    KeyPaste = 135, /* AC Paste */
    KeyFind = 136,  /* AC Search */
    KeyCut = 137,   /* AC Cut */
    KeyHelp = 138,  /* AL Integrated Help Center */
    KeyMenu = 139,  /* Menu (show menu) */
    KeyCalc = 140,  /* AL Calculator */
    KeySetup = 141,
    KeySleep = 142,  /* SC System Sleep */
    KeyWakeup = 143, /* System Wake Up */
    KeyFile = 144,   /* AL Local Machine Browser */
    KeySendfile = 145,
    KeyDeleteFile = 146,
    KeyXfer = 147,
    KeyProg1 = 148,
    KeyProg2 = 149,
    KeyWww = 150, /* AL Internet Browser */
    KeyMsDos = 151,
    KeyCoffee = 152, /* AL Terminal Lock/Screensaver */
    KeyDirection = 153,
    KeyCycleWindows = 154,
    KeyMail = 155,
    KeyBookmarks = 156, /* AC Bookmarks */
    KeyComputer = 157,
    KeyBack = 158,    /* AC Back */
    KeyForward = 159, /* AC Forward */
    KeyCloseCd = 160,
    KeyEjectCd = 161,
    KeyEjectCloseCd = 162,
    KeyNextSong = 163,
    KeyPlayPause = 164,
    KeyPreviousSong = 165,
    KeyStopCd = 166,
    KeyRecord = 167,
    KeyRewind = 168,
    KeyPhone = 169, /* Media Select Telephone */
    KeyIso = 170,
    KeyConfig = 171,   /* AL Consumer Control Configuration */
    KeyHomepage = 172, /* AC Home */
    KeyRefresh = 173,  /* AC Refresh */
    KeyExit = 174,     /* AC Exit */
    KeyMove = 175,
    KeyEdit = 176,
    KeyScrollUp = 177,
    KeyScrollDown = 178,
    KeyKpLeftParen = 179,
    KeyKpRightParen = 180,
    KeyNew = 181,  /* AC New */
    KeyRedo = 182, /* AC Redo/Repeat */
    KeyF13 = 183,
    KeyF14 = 184,
    KeyF15 = 185,
    KeyF16 = 186,
    KeyF17 = 187,
    KeyF18 = 188,
    KeyF19 = 189,
    KeyF20 = 190,
    KeyF21 = 191,
    KeyF22 = 192,
    KeyF23 = 193,
    KeyF24 = 194,
    KeyPlayCd = 200,
    KeyPauseCd = 201,
    KeyProg3 = 202,
    KeyProg4 = 203,
    KeyDashboard = 204, /* AL Dashboard */
    KeySuspend = 205,
    KeyClose = 206, /* AC Close */
    KeyPlay = 207,
    KeyFastForward = 208,
    KeyBassBoost = 209,
    KeyPrint = 210, /* AC Print */
    KeyHp = 211,
    KeyCamera = 212,
    KeySound = 213,
    KeyQuestion = 214,
    KeyEmail = 215,
    KeyChat = 216,
    KeySearch = 217,
    KeyConnect = 218,
    KeyFinance = 219,
    KeySport = 220,
    KeyShop = 221,
    KeyAltErase = 222,
    KeyCancel = 223,
    KeyBrightnessDown = 224,
    KeyBrightnessUp = 225,
    KeyMedia = 226,
    KeySwitchVideoMode = 227,
    KeyKbDillumToggle = 228,
    KeyKbDillumDown = 229,
    KeyKbDillumUp = 230,
    KeySend = 231,
    KeyReply = 232,
    KeyForwardMail = 233,
    KeySave = 234,
    KeyDocuments = 235,
    KeyBattery = 236,
    KeyBluetooth = 237,
    KeyWlan = 238,
    KeyUwb = 239,
    KeyUnknown = 240,
    KeyVideoNext = 241,
    KeyVideoPrev = 242,
    KeyBrightnessCycle = 243,
    KeyBrightnessAuto = 244,
    KeyDisplayOff = 245,
    KeyWwan = 246,
    KeyRfKill = 247,
    KeyMicMute = 248,
    Btn0 = 0x100,
    Btn1 = 0x101,
    Btn2 = 0x102,
    Btn3 = 0x103,
    Btn4 = 0x104,
    Btn5 = 0x105,
    Btn6 = 0x106,
    Btn7 = 0x107,
    Btn8 = 0x108,
    Btn9 = 0x109,
    BtnLeft = 0x110,
    BtnRight = 0x111,
    BtnMiddle = 0x112,
    BtnSide = 0x113,
    BtnExtra = 0x114,
    BtnForward = 0x115,
    BtnBack = 0x116,
    BtnTask = 0x117,
    BtnTrigger = 0x120,
    BtnThumb = 0x121,
    BtnThumb2 = 0x122,
    BtnTop = 0x123,
    BtnTop2 = 0x124,
    BtnPinkie = 0x125,
    BtnBase = 0x126,
    BtnBase2 = 0x127,
    BtnBase3 = 0x128,
    BtnBase4 = 0x129,
    BtnBase5 = 0x12a,
    BtnBase6 = 0x12b,
    BtnDead = 0x12f,
    BtnSouth = 0x130,
    BtnEast = 0x131,
    BtnC = 0x132,
    BtnNorth = 0x133,
    BtnWest = 0x134,
    BtnZ = 0x135,
    BtnTl = 0x136,
    BtnTr = 0x137,
    BtnTl2 = 0x138,
    BtnTr2 = 0x139,
    BtnSelect = 0x13a,
    BtnStart = 0x13b,
    BtnMode = 0x13c,
    BtnThumbL = 0x13d,
    BtnThumbR = 0x13e,
    BtnToolPen = 0x140,
    BtnToolRubber = 0x141,
    BtnToolBrush = 0x142,
    BtnToolPencil = 0x143,
    BtnToolAirbrush = 0x144,
    BtnToolFinger = 0x145,
    BtnToolMouse = 0x146,
    BtnToolLens = 0x147,
    BtnToolQuintTap = 0x148, /* Five fingers on trackpad */
    BtnTouch = 0x14a,
    BtnStylus = 0x14b,
    BtnStylus2 = 0x14c,
    BtnToolDoubleTap = 0x14d,
    BtnToolTripleTap = 0x14e,
    BtnToolQuadTap = 0x14f, /* Four fingers on trackpad */
    BtnGearDown = 0x150,
    BtnGearUp = 0x151,
    KeyOk = 0x160,
    KeySelect = 0x161,
    KeyGoto = 0x162,
    KeyClear = 0x163,
    KeyPower2 = 0x164,
    KeyOption = 0x165,
    KeyInfo = 0x166, /* AL OEM Features/Tips/Tutorial */
    KeyTime = 0x167,
    KeyVendor = 0x168,
    KeyArchive = 0x169,
    KeyProgram = 0x16a, /* Media Select Program Guide */
    KeyChannel = 0x16b,
    KeyFavorites = 0x16c,
    KeyEpg = 0x16d,
    KeyPvr = 0x16e, /* Media Select Home */
    KeyMhp = 0x16f,
    KeyLanguage = 0x170,
    KeyTitle = 0x171,
    KeySubtitle = 0x172,
    KeyAngle = 0x173,
    KeyZoom = 0x174,
    KeyMode = 0x175,
    KeyKeyboard = 0x176,
    KeyScreen = 0x177,
    KeyPc = 0x178,   /* Media Select Computer */
    KeyTv = 0x179,   /* Media Select TV */
    KeyTv2 = 0x17a,  /* Media Select Cable */
    KeyVcr = 0x17b,  /* Media Select VCR */
    KeyVcr2 = 0x17c, /* VCR Plus */
    KeySat = 0x17d,  /* Media Select Satellite */
    KeySat2 = 0x17e,
    KeyCd = 0x17f,   /* Media Select CD */
    KeyTape = 0x180, /* Media Select Tape */
    KeyRadio = 0x181,
    KeyTuner = 0x182, /* Media Select Tuner */
    KeyPlayer = 0x183,
    KeyText = 0x184,
    KeyDvd = 0x185, /* Media Select DVD */
    KeyAux = 0x186,
    KeyMp3 = 0x187,
    KeyAudio = 0x188, /* AL Audio Browser */
    KeyVideo = 0x189, /* AL Movie Browser */
    KeyDirectory = 0x18a,
    KeyList = 0x18b,
    KeyMemo = 0x18c, /* Media Select Messages */
    KeyCalendar = 0x18d,
    KeyRed = 0x18e,
    KeyGreen = 0x18f,
    KeyYellow = 0x190,
    KeyBlue = 0x191,
    KeyChannelUp = 0x192,   /* Channel Increment */
    KeyChannelDown = 0x193, /* Channel Decrement */
    KeyFirst = 0x194,
    KeyLast = 0x195, /* Recall Last */
    KeyAb = 0x196,
    KeyNext = 0x197,
    KeyRestart = 0x198,
    KeySlow = 0x199,
    KeyShuffle = 0x19a,
    KeyBreak = 0x19b,
    KeyPrevious = 0x19c,
    KeyDigits = 0x19d,
    KeyTeen = 0x19e,
    KeyTwen = 0x19f,
    KeyVideophone = 0x1a0,     /* Media Select Video Phone */
    KeyGames = 0x1a1,          /* Media Select Games */
    KeyZoomIn = 0x1a2,         /* AC Zoom In */
    KeyZoomOut = 0x1a3,        /* AC Zoom Out */
    KeyZoomReset = 0x1a4,      /* AC Zoom */
    KeyWordProcessor = 0x1a5,  /* AL Word Processor */
    KeyEditor = 0x1a6,         /* AL Text Editor */
    KeySpreadsheet = 0x1a7,    /* AL Spreadsheet */
    KeyGraphicsEditor = 0x1a8, /* AL Graphics Editor */
    KeyPresentation = 0x1a9,   /* AL Presentation App */
    KeyDatabase = 0x1aa,       /* AL Database App */
    KeyNews = 0x1ab,           /* AL Newsreader */
    KeyVoiceMail = 0x1ac,      /* AL Voicemail */
    KeyAddressBook = 0x1ad,    /* AL Contacts/Address Book */
    KeyMessenger = 0x1ae,      /* AL Instant Messaging */
    KeyDisplayToggle = 0x1af,  /* Turn display (LCD) on and off */
    KeySpellcheck = 0x1b0,     /* AL Spell Check */
    KeyLogoff = 0x1b1,         /* AL Logoff */
    KeyDollar = 0x1b2,
    KeyEuro = 0x1b3,
    KeyFrameBack = 0x1b4, /* Consumer - transport controls */
    KeyFrameForward = 0x1b5,
    KeyContextMenu = 0x1b6,    /* GenDesc - system context menu */
    KeyMediaRepeat = 0x1b7,    /* Consumer - transport control */
    Key10ChannelsUp = 0x1b8,   /* 10 channels up (10+) */
    Key10ChannelsDown = 0x1b9, /* 10 channels down (10-) */
    KeyImages = 0x1ba,         /* AL Image Browser */
    KeyDelEol = 0x1c0,
    KeyDelEos = 0x1c1,
    KeyInsLine = 0x1c2,
    KeyDelLine = 0x1c3,
    KeyFn = 0x1d0,
    KeyFnEsc = 0x1d1,
    KeyFnF1 = 0x1d2,
    KeyFnF2 = 0x1d3,
    KeyFnF3 = 0x1d4,
    KeyFnF4 = 0x1d5,
    KeyFnF5 = 0x1d6,
    KeyFnF6 = 0x1d7,
    KeyFnF7 = 0x1d8,
    KeyFnF8 = 0x1d9,
    KeyFnF9 = 0x1da,
    KeyFnF10 = 0x1db,
    KeyFnF11 = 0x1dc,
    KeyFnF12 = 0x1dd,
    KeyFn1 = 0x1de,
    KeyFn2 = 0x1df,
    KeyFnD = 0x1e0,
    KeyFnE = 0x1e1,
    KeyFnF = 0x1e2,
    KeyFnS = 0x1e3,
    KeyFnB = 0x1e4,
    KeyBrlDot1 = 0x1f1,
    KeyBrlDot2 = 0x1f2,
    KeyBrlDot3 = 0x1f3,
    KeyBrlDot4 = 0x1f4,
    KeyBrlDot5 = 0x1f5,
    KeyBrlDot6 = 0x1f6,
    KeyBrlDot7 = 0x1f7,
    KeyBrlDot8 = 0x1f8,
    KeyBrlDot9 = 0x1f9,
    KeyBrlDot10 = 0x1fa,
    KeyNumeric0 = 0x200, /* used by phones, remote controls, */
    KeyNumeric1 = 0x201, /* and other keypads */
    KeyNumeric2 = 0x202,
    KeyNumeric3 = 0x203,
    KeyNumeric4 = 0x204,
    KeyNumeric5 = 0x205,
    KeyNumeric6 = 0x206,
    KeyNumeric7 = 0x207,
    KeyNumeric8 = 0x208,
    KeyNumeric9 = 0x209,
    KeyNumericStar = 0x20a,
    KeyNumericPound = 0x20b,
    KeyCameraFocus = 0x210,
    KeyWpsButton = 0x211,      /* WiFi Protected Setup key */
    KeyTouchpadToggle = 0x212, /* Request switch touchpad on or off */
    KeyTouchpadOn = 0x213,
    KeyTouchpadOff = 0x214,
    KeyCameraZoomIn = 0x215,
    KeyCameraZoomOut = 0x216,
    KeyCameraUp = 0x217,
    KeyCameraDown = 0x218,
    KeyCameraLeft = 0x219,
    KeyCameraRight = 0x21a,
    KeyAttendantOn = 0x21b,
    KeyAttendantOff = 0x21c,
    KeyAttendantToggle = 0x21d, /* Attendant call on or off */
    KeyLightsToggle = 0x21e,    /* Reading light on or off */
    BtnDpadUp = 0x220,
    BtnDpadDown = 0x221,
    BtnDpadLeft = 0x222,
    BtnDpadRight = 0x223,
    KeyAlsToggle = 0x230,     /* Ambient light sensor */
    KeyButtonConfig = 0x240,  /* AL Button Configuration */
    KeyTaskManager = 0x241,   /* AL Task/Project Manager */
    KeyJournal = 0x242,       /* AL Log/Journal/Timecard */
    KeyControlPanel = 0x243,  /* AL Control Panel */
    KeyAppSelect = 0x244,     /* AL Select Task/Application */
    KeyScreenSaver = 0x245,   /* AL Screen Saver */
    KeyVoiceCommand = 0x246,  /* Listening Voice Command */
    KeyBrightnessMin = 0x250, /* Set Brightness to Minimum */
    KeyBrightnessMax = 0x251, /* Set Brightness to Maximum */
    KeyKbdinputassistPrev = 0x260,
    KeyKbdinputassistNext = 0x261,
    KeyKbdinputassistPrevgroup = 0x262,
    KeyKbdinputassistNextgroup = 0x263,
    KeyKbdinputassistAccept = 0x264,
    KeyKbdinputassistCancel = 0x265,
    BtnTriggerHappy1 = 0x2c0,
    BtnTriggerHappy2 = 0x2c1,
    BtnTriggerHappy3 = 0x2c2,
    BtnTriggerHappy4 = 0x2c3,
    BtnTriggerHappy5 = 0x2c4,
    BtnTriggerHappy6 = 0x2c5,
    BtnTriggerHappy7 = 0x2c6,
    BtnTriggerHappy8 = 0x2c7,
    BtnTriggerHappy9 = 0x2c8,
    BtnTriggerHappy10 = 0x2c9,
    BtnTriggerHappy11 = 0x2ca,
    BtnTriggerHappy12 = 0x2cb,
    BtnTriggerHappy13 = 0x2cc,
    BtnTriggerHappy14 = 0x2cd,
    BtnTriggerHappy15 = 0x2ce,
    BtnTriggerHappy16 = 0x2cf,
    BtnTriggerHappy17 = 0x2d0,
    BtnTriggerHappy18 = 0x2d1,
    BtnTriggerHappy19 = 0x2d2,
    BtnTriggerHappy20 = 0x2d3,
    BtnTriggerHappy21 = 0x2d4,
    BtnTriggerHappy22 = 0x2d5,
    BtnTriggerHappy23 = 0x2d6,
    BtnTriggerHappy24 = 0x2d7,
    BtnTriggerHappy25 = 0x2d8,
    BtnTriggerHappy26 = 0x2d9,
    BtnTriggerHappy27 = 0x2da,
    BtnTriggerHappy28 = 0x2db,
    BtnTriggerHappy29 = 0x2dc,
    BtnTriggerHappy30 = 0x2dd,
    BtnTriggerHappy31 = 0x2de,
    BtnTriggerHappy32 = 0x2df,
    BtnTriggerHappy33 = 0x2e0,
    BtnTriggerHappy34 = 0x2e1,
    BtnTriggerHappy35 = 0x2e2,
    BtnTriggerHappy36 = 0x2e3,
    BtnTriggerHappy37 = 0x2e4,
    BtnTriggerHappy38 = 0x2e5,
    BtnTriggerHappy39 = 0x2e6,
    BtnTriggerHappy40 = 0x2e7,
    KeyMax = 0x2ff,
}

impl Key {
    pub fn is_ascii_alpha_numeric(&self) -> bool {
        match self {
            Key::Key0 => true,
            Key::Key1 => true,
            Key::Key2 => true,
            Key::Key3 => true,
            Key::Key4 => true,
            Key::Key5 => true,
            Key::Key6 => true,
            Key::Key7 => true,
            Key::Key8 => true,
            Key::Key9 => true,
            Key::KeyA => true,
            Key::KeyB => true,
            Key::KeyC => true,
            Key::KeyD => true,
            Key::KeyE => true,
            Key::KeyF => true,
            Key::KeyG => true,
            Key::KeyH => true,
            Key::KeyI => true,
            Key::KeyJ => true,
            Key::KeyK => true,
            Key::KeyL => true,
            Key::KeyM => true,
            Key::KeyN => true,
            Key::KeyO => true,
            Key::KeyP => true,
            Key::KeyQ => true,
            Key::KeyR => true,
            Key::KeyS => true,
            Key::KeyT => true,
            Key::KeyU => true,
            Key::KeyV => true,
            Key::KeyW => true,
            Key::KeyX => true,
            Key::KeyY => true,
            Key::KeyZ => true,
            _ => false,
        }
    }
}
