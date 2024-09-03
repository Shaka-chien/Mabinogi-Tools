mod libs {
    extern crate winapi;
    use winapi::um::winuser::{
        INPUT, INPUT_KEYBOARD, INPUT_MOUSE, SendInput, 
        KEYEVENTF_KEYUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_ABSOLUTE, 
        MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_WHEEL,
        GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
        SetWindowsHookExW, CallNextHookEx, UnhookWindowsHookEx, GetMessageW, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, KBDLLHOOKSTRUCT,
        WH_MOUSE_LL, WM_MOUSEMOVE, WM_LBUTTONDOWN, WM_RBUTTONDOWN, WM_LBUTTONUP,
        WM_RBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEWHEEL, MSLLHOOKSTRUCT,
    }; // KEYBDINPUT, MOUSEINPUT, KEYEVENTF_SCANCODE
    use winapi::shared::minwindef::{DWORD, LRESULT, WPARAM, LPARAM}; // UINT
    use std::mem::size_of;

    use std::ptr::null_mut;
    //use std::ffi::c_void;
    use winapi::shared::windef::{HHOOK, POINT};
    use winapi::um::libloaderapi::GetModuleHandleW;

    // --- keyboard, mouse simulater core ---
    fn simulate_key_press(vk: u16) {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let ki = input.u.ki_mut();
            ki.wVk = vk;
            ki.dwFlags = 0; // 按下鍵
            ki.dwFlags = ki.dwFlags | 1; // https://stackoverflow.com/questions/44924962/sendinput-on-c-doesnt-take-ctrl-and-shift-in-account
        }
        unsafe { SendInput(1, &mut input, size_of::<INPUT>() as i32) };
    }

    fn simulate_key_release(vk: u16) {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let ki = input.u.ki_mut();
            ki.wVk = vk;
            ki.dwFlags = KEYEVENTF_KEYUP; // 放開鍵
            ki.dwFlags = ki.dwFlags | 1; // https://stackoverflow.com/questions/44924962/sendinput-on-c-doesnt-take-ctrl-and-shift-in-account
        }
        unsafe { SendInput(1, &mut input, size_of::<INPUT>() as i32) };
    }

    fn convert_to_absolute(x: i32, y: i32) -> (i32, i32) {
        let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };
        
        let dx = (x * 65535 / screen_width) as i32;
        let dy = (y * 65535 / screen_height) as i32;
        
        (dx, dy)
    }

    fn simulate_mouse_move(x: i32, y: i32) {
        let (dx, dy) = convert_to_absolute(x, y);

        let mut input = INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let mi = input.u.mi_mut();
            mi.dx = dx;
            mi.dy = dy;
            mi.dwFlags = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
        }
        unsafe { SendInput(1, &mut input, size_of::<INPUT>() as i32) };
    }

    fn simulate_mouse_lbtn_press(x: i32, y: i32) {
        let (dx, dy) = convert_to_absolute(x, y);

        let mut input_down = INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let mi = input_down.u.mi_mut();
            mi.dx = dx;
            mi.dy = dy;
            mi.dwFlags = MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
        }
        unsafe {
            SendInput(1, &mut input_down, size_of::<INPUT>() as i32);
        }
    }

    fn simulate_mouse_lbtn_release(x: i32, y: i32) {
        let (dx, dy) = convert_to_absolute(x, y);

        let mut input_up = INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let mi = input_up.u.mi_mut();
            mi.dx = dx;
            mi.dy = dy;
            mi.dwFlags = MOUSEEVENTF_LEFTUP | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
        }
        unsafe {
            SendInput(1, &mut input_up, size_of::<INPUT>() as i32);
        }
    }

    fn simulate_mouse_rbtn_press(x: i32, y: i32) {
        let (dx, dy) = convert_to_absolute(x, y);

        let mut input_down = INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let mi = input_down.u.mi_mut();
            mi.dx = dx;
            mi.dy = dy;
            mi.dwFlags = MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
        }
        unsafe {
            SendInput(1, &mut input_down, size_of::<INPUT>() as i32);
        }
    }

    fn simulate_mouse_rbtn_release(x: i32, y: i32) {
        let (dx, dy) = convert_to_absolute(x, y);

        let mut input_up = INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let mi = input_up.u.mi_mut();
            mi.dx = dx;
            mi.dy = dy;
            mi.dwFlags = MOUSEEVENTF_RIGHTUP | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
        }
        unsafe {
            SendInput(1, &mut input_up, size_of::<INPUT>() as i32);
        }
    }

    fn simulate_mouse_mbtn_press(x: i32, y: i32) {
        let (dx, dy) = convert_to_absolute(x, y);

        let mut input_down = INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let mi = input_down.u.mi_mut();
            mi.dx = dx;
            mi.dy = dy;
            mi.dwFlags = MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
        }
        unsafe {
            SendInput(1, &mut input_down, size_of::<INPUT>() as i32);
        }
    }

    fn simulate_mouse_mbtn_release(x: i32, y: i32) {
        let (dx, dy) = convert_to_absolute(x, y);

        let mut input_up = INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let mi = input_up.u.mi_mut();
            mi.dx = dx;
            mi.dy = dy;
            mi.dwFlags = MOUSEEVENTF_MIDDLEUP | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
        }
        unsafe {
            SendInput(1, &mut input_up, size_of::<INPUT>() as i32);
        }
    }

    fn simulate_mouse_whell(x: i32, y: i32, delta: i16) {
        let (dx, dy) = convert_to_absolute(x, y);

        let mut input_up = INPUT {
            type_: INPUT_MOUSE,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let mi = input_up.u.mi_mut();
            mi.mouseData = delta as u32;
            mi.dx = dx;
            mi.dy = dy;
            mi.dwFlags = MOUSEEVENTF_WHEEL | MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
        }
        unsafe {
            SendInput(1, &mut input_up, size_of::<INPUT>() as i32);
        }
    }

    // --- keyboard, mouse public method ---
    #[derive(Debug)]
    pub enum ButtonState {
        Press, Release
    }

    #[derive(Debug)]
    pub enum KeyCode {
        Alt,
        AltGr,
        Backspace,
        CapsLock,
        ControlLeft,
        ControlRight,
        Delete,
        DownArrow,
        End,
        Escape,
        F1,
        F10,
        F11,
        F12,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        Home,
        LeftArrow,
        MetaLeft,
        PageDown,
        PageUp,
        Return,
        RightArrow,
        ShiftLeft,
        ShiftRight,
        Space,
        Tab,
        UpArrow,
        PrintScreen,
        ScrollLock,
        Pause,
        NumLock,
        BackQuote,
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
        Minus,
        Equal,
        KeyQ,
        KeyW,
        KeyE,
        KeyR,
        KeyT,
        KeyY,
        KeyU,
        KeyI,
        KeyO,
        KeyP,
        LeftBracket,
        RightBracket,
        KeyA,
        KeyS,
        KeyD,
        KeyF,
        KeyG,
        KeyH,
        KeyJ,
        KeyK,
        KeyL,
        SemiColon,
        Quote,
        BackSlash,
        IntlBackslash,
        KeyZ,
        KeyX,
        KeyC,
        KeyV,
        KeyB,
        KeyN,
        KeyM,
        Comma,
        Dot,
        Slash,
        Insert,
        KpMinus,
        KpPlus,
        KpMultiply,
        KpDivide,
        Kp0,
        Kp1,
        Kp2,
        Kp3,
        Kp4,
        Kp5,
        Kp6,
        Kp7,
        Kp8,
        Kp9,
        KpDelete,
        UnicodePrefix, // windows 字母前綴
        Unknow,
    }
    impl KeyCode {
        pub fn from_int(code: u32) -> KeyCode {
            match code {
                164     => { KeyCode::Alt }
                165     => { KeyCode::AltGr }
                0x08    => { KeyCode::Backspace }
                20      => { KeyCode::CapsLock }
                162     => { KeyCode::ControlLeft }
                163     => { KeyCode::ControlRight }
                46      => { KeyCode::Delete }
                40      => { KeyCode::DownArrow }
                35      => { KeyCode::End }
                27      => { KeyCode::Escape }
                112     => { KeyCode::F1 }
                121     => { KeyCode::F10 }
                122     => { KeyCode::F11 }
                123     => { KeyCode::F12 }
                113     => { KeyCode::F2 }
                114     => { KeyCode::F3 }
                115     => { KeyCode::F4 }
                116     => { KeyCode::F5 }
                117     => { KeyCode::F6 }
                118     => { KeyCode::F7 }
                119     => { KeyCode::F8 }
                120     => { KeyCode::F9 }
                36      => { KeyCode::Home }
                37      => { KeyCode::LeftArrow }
                91      => { KeyCode::MetaLeft }
                34      => { KeyCode::PageDown }
                33      => { KeyCode::PageUp }
                0x0D    => { KeyCode::Return } // KP_RETURN, 13,
                39      => { KeyCode::RightArrow }
                160     => { KeyCode::ShiftLeft }
                161     => { KeyCode::ShiftRight }
                32      => { KeyCode::Space }
                0x09    => { KeyCode::Tab }
                38      => { KeyCode::UpArrow }
                44      => { KeyCode::PrintScreen }
                145     => { KeyCode::ScrollLock }
                19      => { KeyCode::Pause }
                144     => { KeyCode::NumLock }
                192     => { KeyCode::BackQuote }
                49      => { KeyCode::Num1 }
                50      => { KeyCode::Num2 }
                51      => { KeyCode::Num3 }
                52      => { KeyCode::Num4 }
                53      => { KeyCode::Num5 }
                54      => { KeyCode::Num6 }
                55      => { KeyCode::Num7 }
                56      => { KeyCode::Num8 }
                57      => { KeyCode::Num9 }
                48      => { KeyCode::Num0 }
                189     => { KeyCode::Minus }
                187     => { KeyCode::Equal }
                81      => { KeyCode::KeyQ }
                87      => { KeyCode::KeyW }
                69      => { KeyCode::KeyE }
                82      => { KeyCode::KeyR }
                84      => { KeyCode::KeyT }
                89      => { KeyCode::KeyY }
                85      => { KeyCode::KeyU }
                73      => { KeyCode::KeyI }
                79      => { KeyCode::KeyO }
                80      => { KeyCode::KeyP }
                219     => { KeyCode::LeftBracket }
                221     => { KeyCode::RightBracket }
                65      => { KeyCode::KeyA }
                83      => { KeyCode::KeyS }
                68      => { KeyCode::KeyD }
                70      => { KeyCode::KeyF }
                71      => { KeyCode::KeyG }
                72      => { KeyCode::KeyH }
                74      => { KeyCode::KeyJ }
                75      => { KeyCode::KeyK }
                76      => { KeyCode::KeyL }
                186     => { KeyCode::SemiColon }
                222     => { KeyCode::Quote }
                220     => { KeyCode::BackSlash }
                226     => { KeyCode::IntlBackslash }
                90      => { KeyCode::KeyZ }
                88      => { KeyCode::KeyX }
                67      => { KeyCode::KeyC }
                86      => { KeyCode::KeyV }
                66      => { KeyCode::KeyB }
                78      => { KeyCode::KeyN }
                77      => { KeyCode::KeyM }
                188     => { KeyCode::Comma }
                190     => { KeyCode::Dot }
                191     => { KeyCode::Slash }
                45      => { KeyCode::Insert }
                109     => { KeyCode::KpMinus }
                107     => { KeyCode::KpPlus }
                106     => { KeyCode::KpMultiply }
                111     => { KeyCode::KpDivide }
                96      => { KeyCode::Kp0 }
                97      => { KeyCode::Kp1 }
                98      => { KeyCode::Kp2 }
                99      => { KeyCode::Kp3 }
                100     => { KeyCode::Kp4 }
                101     => { KeyCode::Kp5 }
                102     => { KeyCode::Kp6 }
                103     => { KeyCode::Kp7 }
                104     => { KeyCode::Kp8 }
                105     => { KeyCode::Kp9 }
                110     => { KeyCode::KpDelete }
                231     => { KeyCode::UnicodePrefix }
                _       => { println!("Unknow: {code}"); KeyCode::Unknow }
            }
        }
        pub fn to_int(&self) -> u32 {
            match self {
                KeyCode::Alt            => { 164    }
                KeyCode::AltGr          => { 165    }
                KeyCode::Backspace      => { 0x08   }
                KeyCode::CapsLock       => { 20     }
                KeyCode::ControlLeft    => { 162    }
                KeyCode::ControlRight   => { 163    }
                KeyCode::Delete         => { 46     }
                KeyCode::DownArrow      => { 40     }
                KeyCode::End            => { 35     }
                KeyCode::Escape         => { 27     }
                KeyCode::F1             => { 112    }
                KeyCode::F10            => { 121    }
                KeyCode::F11            => { 122    }
                KeyCode::F12            => { 123    }
                KeyCode::F2             => { 113    }
                KeyCode::F3             => { 114    }
                KeyCode::F4             => { 115    }
                KeyCode::F5             => { 116    }
                KeyCode::F6             => { 117    }
                KeyCode::F7             => { 118    }
                KeyCode::F8             => { 119    }
                KeyCode::F9             => { 120    }
                KeyCode::Home           => { 36     }
                KeyCode::LeftArrow      => { 37     }
                KeyCode::MetaLeft       => { 91     }
                KeyCode::PageDown       => { 34     }
                KeyCode::PageUp         => { 33     }
                KeyCode::Return         => { 0x0D   }
                KeyCode::RightArrow     => { 39     }
                KeyCode::ShiftLeft      => { 160    }
                KeyCode::ShiftRight     => { 161    }
                KeyCode::Space          => { 32     }
                KeyCode::Tab            => { 0x09   }
                KeyCode::UpArrow        => { 38     }
                KeyCode::PrintScreen    => { 44     }
                KeyCode::ScrollLock     => { 145    }
                KeyCode::Pause          => { 19     }
                KeyCode::NumLock        => { 144    }
                KeyCode::BackQuote      => { 192    }
                KeyCode::Num1           => { 49     }
                KeyCode::Num2           => { 50     }
                KeyCode::Num3           => { 51     }
                KeyCode::Num4           => { 52     }
                KeyCode::Num5           => { 53     }
                KeyCode::Num6           => { 54     }
                KeyCode::Num7           => { 55     }
                KeyCode::Num8           => { 56     }
                KeyCode::Num9           => { 57     }
                KeyCode::Num0           => { 48     }
                KeyCode::Minus          => { 189    }
                KeyCode::Equal          => { 187    }
                KeyCode::KeyQ           => { 81     }
                KeyCode::KeyW           => { 87     }
                KeyCode::KeyE           => { 69     }
                KeyCode::KeyR           => { 82     }
                KeyCode::KeyT           => { 84     }
                KeyCode::KeyY           => { 89     }
                KeyCode::KeyU           => { 85     }
                KeyCode::KeyI           => { 73     }
                KeyCode::KeyO           => { 79     }
                KeyCode::KeyP           => { 80     }
                KeyCode::LeftBracket    => { 219    }
                KeyCode::RightBracket   => { 221    }
                KeyCode::KeyA           => { 65     }
                KeyCode::KeyS           => { 83     }
                KeyCode::KeyD           => { 68     }
                KeyCode::KeyF           => { 70     }
                KeyCode::KeyG           => { 71     }
                KeyCode::KeyH           => { 72     }
                KeyCode::KeyJ           => { 74     }
                KeyCode::KeyK           => { 75     }
                KeyCode::KeyL           => { 76     }
                KeyCode::SemiColon      => { 186    }
                KeyCode::Quote          => { 222    }
                KeyCode::BackSlash      => { 220    }
                KeyCode::IntlBackslash  => { 226    }
                KeyCode::KeyZ           => { 90     }
                KeyCode::KeyX           => { 88     }
                KeyCode::KeyC           => { 67     }
                KeyCode::KeyV           => { 86     }
                KeyCode::KeyB           => { 66     }
                KeyCode::KeyN           => { 78     }
                KeyCode::KeyM           => { 77     }
                KeyCode::Comma          => { 188    }
                KeyCode::Dot            => { 190    }
                KeyCode::Slash          => { 191    }
                KeyCode::Insert         => { 45     }
                KeyCode::KpMinus        => { 109    }
                KeyCode::KpPlus         => { 107    }
                KeyCode::KpMultiply     => { 106    }
                KeyCode::KpDivide       => { 111    }
                KeyCode::Kp0            => { 96     }
                KeyCode::Kp1            => { 97     }
                KeyCode::Kp2            => { 98     }
                KeyCode::Kp3            => { 99     }
                KeyCode::Kp4            => { 100    }
                KeyCode::Kp5            => { 101    }
                KeyCode::Kp6            => { 102    }
                KeyCode::Kp7            => { 103    }
                KeyCode::Kp8            => { 104    }
                KeyCode::Kp9            => { 105    }
                KeyCode::KpDelete       => { 110    }
                KeyCode::UnicodePrefix  => { 231    }
                KeyCode::Unknow         => { 0      }
            }
        }

        pub fn press(&self) {
            let vk = self.to_int();
            simulate_key_press((vk as u32).try_into().unwrap());
        }

        pub fn release(&self) {
            let vk = self.to_int();
            simulate_key_release((vk as u32).try_into().unwrap());
        }

        pub fn click(&self) {
            let vk = self.to_int();
            simulate_key_press((vk as u32).try_into().unwrap());
            simulate_key_release((vk as u32).try_into().unwrap());
        }
    }

    #[derive(Debug)]
    pub enum MouseAction {
        Move(i32, i32), // x, y
        Whell(i32, i32, i16), // x, y, delta
    }
    impl MouseAction {
        pub fn do_it(&self) {
            match self {
                MouseAction::Move(x, y)           => { simulate_mouse_move(*x, *y) }
                MouseAction::Whell(x, y, delta)   => { simulate_mouse_whell(*x, *y, *delta) }
            }
        }
    }
    #[derive(Debug)]
    pub enum MouseButton {
        LBtn(i32, i32), // x, y
        RBtn(i32, i32), // x, y
        MBtn(i32, i32), // x, y
    }
    impl MouseButton {
        pub fn press(&self) {
            match self {
                MouseButton::LBtn(x, y)     => {
                    simulate_mouse_lbtn_press(*x, *y)
                }
                MouseButton::RBtn(x, y)     => {
                    simulate_mouse_rbtn_press(*x, *y)
                }
                MouseButton::MBtn(x, y)     => {
                    simulate_mouse_mbtn_press(*x, *y)
                }
            }
        }
        pub fn release(&self) {
            match self {
                MouseButton::LBtn(x, y)     => {
                    simulate_mouse_lbtn_release(*x, *y)
                }
                MouseButton::RBtn(x, y)     => {
                    simulate_mouse_rbtn_release(*x, *y)
                }
                MouseButton::MBtn(x, y)     => {
                    simulate_mouse_mbtn_release(*x, *y)
                }
            }
        }
    }

    static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(SystemInputEvent)>> = None;
    static mut HOOK: HHOOK = null_mut();

    extern "system" fn raw_callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if code >= 0 {
            match w_param as DWORD {
                // --- keyboard event ---
                WM_KEYDOWN => {
                    let kb_struct = unsafe { &*(l_param as *const KBDLLHOOKSTRUCT) };
                    //println!("Key pressed: {}", kb_struct.vkCode);
                    let key_code = KeyCode::from_int(kb_struct.vkCode);
                    match key_code {
                        KeyCode::UnicodePrefix => {}
                        _ => {
                            #[allow(static_mut_refs)]
                            unsafe {
                                if let Some(cb) = &mut GLOBAL_CALLBACK {
                                    cb(SystemInputEvent::KeyCode(key_code, ButtonState::Press));
                                } else {
                                    println!("Pressed: {:?}", key_code);
                                }
                            }
                        }
                    }
                }
                WM_KEYUP => {
                    let kb_struct = unsafe { &*(l_param as *const KBDLLHOOKSTRUCT) };
                    //println!("Key released: {}", kb_struct.vkCode);
                    let key_code = KeyCode::from_int(kb_struct.vkCode);
                    match key_code {
                        KeyCode::UnicodePrefix => {}
                        _ => {
                            #[allow(static_mut_refs)]
                            unsafe {
                                if let Some(cb) = &mut GLOBAL_CALLBACK {
                                    cb(SystemInputEvent::KeyCode(key_code, ButtonState::Release));
                                } else {
                                    println!("Release: {:?}", key_code);
                                }
                            }
                        }
                    }
                }
                // --- mouse event ---
                WM_MOUSEMOVE => {
                    let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                    let POINT { x, y } = mouse_info.pt;
                    // println!("Mouse moved to: ({}, {})", x, y);
                    let move_event = MouseAction::Move(x, y);
                    #[allow(static_mut_refs)]
                    unsafe {
                        if let Some(cb) = &mut GLOBAL_CALLBACK {
                            cb(SystemInputEvent::MouseAction(move_event));
                        } else {
                            println!("Mouse moved to: ({:?})", move_event);
                        }
                    }
                }
                WM_LBUTTONDOWN => {
                    let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                    //println!("Left button down at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                    let lbtn_press = MouseButton::LBtn(mouse_info.pt.x, mouse_info.pt.y);
                    #[allow(static_mut_refs)]
                    unsafe {
                        if let Some(cb) = &mut GLOBAL_CALLBACK {
                            cb(SystemInputEvent::MouseButton(lbtn_press, ButtonState::Press));
                        } else {
                            println!("Pressed: {:?}", lbtn_press);
                        }
                    }
                }
                WM_LBUTTONUP => {
                    let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                    //println!("Left button up at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                    let lbtn_release = MouseButton::LBtn(mouse_info.pt.x, mouse_info.pt.y);
                    #[allow(static_mut_refs)]
                    unsafe {
                        if let Some(cb) = &mut GLOBAL_CALLBACK {
                            cb(SystemInputEvent::MouseButton(lbtn_release, ButtonState::Release));
                        } else {
                            println!("Releaseed: {:?}", lbtn_release);
                        }
                    }
                }
                WM_RBUTTONDOWN => {
                    let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                    //println!("Right button down at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                    let rbtn_press = MouseButton::RBtn(mouse_info.pt.x, mouse_info.pt.y);
                    #[allow(static_mut_refs)]
                    unsafe {
                        if let Some(cb) = &mut GLOBAL_CALLBACK {
                            cb(SystemInputEvent::MouseButton(rbtn_press, ButtonState::Press));
                        } else {
                            println!("Pressed: {:?}", rbtn_press);
                        }
                    }
                }
                WM_RBUTTONUP => {
                    let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                    //println!("Right button up at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                    let rbtn_release = MouseButton::RBtn(mouse_info.pt.x, mouse_info.pt.y);
                    #[allow(static_mut_refs)]
                    unsafe {
                        if let Some(cb) = &mut GLOBAL_CALLBACK {
                            cb(SystemInputEvent::MouseButton(rbtn_release, ButtonState::Release));
                        } else {
                            println!("Releaseed: {:?}", rbtn_release);
                        }
                    }
                }
                WM_MBUTTONDOWN => {
                    let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                    //println!("Middle button down at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                    let mbtn_press = MouseButton::MBtn(mouse_info.pt.x, mouse_info.pt.y);
                    #[allow(static_mut_refs)]
                    unsafe {
                        if let Some(cb) = &mut GLOBAL_CALLBACK {
                            cb(SystemInputEvent::MouseButton(mbtn_press, ButtonState::Press));
                        } else {
                            println!("Pressed: {:?}", mbtn_press);
                        }
                    }
                }
                WM_MBUTTONUP => {
                    let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                    //println!("Middle button up at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                    let mbtn_release = MouseButton::MBtn(mouse_info.pt.x, mouse_info.pt.y);
                    #[allow(static_mut_refs)]
                    unsafe {
                        if let Some(cb) = &mut GLOBAL_CALLBACK {
                            cb(SystemInputEvent::MouseButton(mbtn_release, ButtonState::Release));
                        } else {
                            println!("Releaseed: {:?}", mbtn_release);
                        }
                    }
                }
                WM_MOUSEWHEEL => {
                    let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                    let delta = (mouse_info.mouseData >> 16) as i16;
                    //if delta > 0 {
                    //    println!("Mouse wheel scrolled up at: ({}, {}), delta: {}", mouse_info.pt.x, mouse_info.pt.y, delta);
                    //} else {
                    //    println!("Mouse wheel scrolled down at: ({}, {}), delta: {}", mouse_info.pt.x, mouse_info.pt.y, delta);
                    //}
                    let whell = MouseAction::Whell(mouse_info.pt.x, mouse_info.pt.y, delta);
                    #[allow(static_mut_refs)]
                    unsafe {
                        if let Some(cb) = &mut GLOBAL_CALLBACK {
                            cb(SystemInputEvent::MouseAction(whell));
                        } else {
                            println!("{:?}", whell);
                        }
                    }
                }
                _ => {}
            }
        }
        unsafe { CallNextHookEx(HOOK, code, w_param, l_param) }
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum SystemInputEvent {
        KeyCode(KeyCode, ButtonState),
        MouseAction(MouseAction),
        MouseButton(MouseButton, ButtonState),
        Delay,
    }
    impl SystemInputEvent {
        pub fn listen_keyboard_event() {
            unsafe {
                let h_instance = GetModuleHandleW(null_mut());
                HOOK = SetWindowsHookExW(
                    WH_KEYBOARD_LL,
                    Some(raw_callback),
                    h_instance,
                    0,
                );

                if HOOK.is_null() {
                    println!("Failed to set hook");
                    return;
                }

                let mut msg = std::mem::zeroed();
                while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {}
                UnhookWindowsHookEx(HOOK);
            }
        }
        pub fn listen_mouse_event() {
            unsafe {
                let h_instance = GetModuleHandleW(null_mut());
                HOOK = SetWindowsHookExW(
                    WH_MOUSE_LL,
                    Some(raw_callback),
                    h_instance,
                    0,
                );

                if HOOK.is_null() {
                    println!("Failed to set hook");
                    return;
                }

                let mut msg = std::mem::zeroed();
                while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {}
                UnhookWindowsHookEx(HOOK);
            }
        }
        pub fn listen_all<T>(mut cb: T) where T: FnMut(SystemInputEvent) + 'static {
            use std::thread;

            unsafe {
                GLOBAL_CALLBACK = Some(Box::new(move |event| {
                    cb(event);
                }));
            }

            let handle01 = thread::spawn(|| {
                SystemInputEvent::listen_keyboard_event();
            });
            let handle02 = thread::spawn(|| {
                SystemInputEvent::listen_mouse_event();
            });
            handle01.join().unwrap(); // 等待執行緒結束
            handle02.join().unwrap(); // 等待執行緒結束
        }
    }

    // 貼上
    #[allow(dead_code)]
    pub fn past_text<S: AsRef<str>>(msg: S) {
        let msg = String::from(msg.as_ref());
        // 剪貼簿 library
        extern crate clipboard;
        use clipboard::ClipboardProvider;
        use clipboard::ClipboardContext;

        // 將文字放倒剪貼簿中
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(msg).unwrap();

        // 模擬按下 Ctrl + V (貼上)
        KeyCode::ControlLeft.press();
        KeyCode::KeyV.press();
        KeyCode::KeyV.release();
        KeyCode::ControlLeft.release();
    }

    // --- commons ---
    // 休息
    #[allow(dead_code)]
    pub fn sleep(millis: u64) {
        use std::{thread, time};

        let ten_millis = time::Duration::from_millis(millis);
        thread::sleep(ten_millis);
    }

    // 查 type
    #[allow(dead_code)]
    pub fn type_of<T>(_: &T) -> String {
        return String::from(std::any::type_name::<T>());
    }

    // 計算耗時
    #[allow(dead_code)]
    pub fn elapsed_fn<F: Fn()>(cb: F) {
        use std::time::Instant;
        let start = Instant::now();
        cb();
        println!("耗時: {:?}", start.elapsed());
    }

    // 退出程式
    #[allow(dead_code)]
    pub fn exit() {
        use std::process;
        process::exit(0);
    }
}

mod pc_ctrl {
    use crate::libs;

    // --- 狀態 ---
    pub enum State {
        Waiting,
        Hello,
    }

    // --- 狀態介面 ---
    #[allow(dead_code)]
    trait Action {
        fn enter(&mut self) {}
        fn out(&mut self) {}

        // --- 系統觸發介面 ---
        // 若動作狀態需要進行狀態轉移時, 則在此回傳新的狀態, 由 Context 接手處理 狀態轉移
        #[allow(unused_variables)]
        fn do_event(&mut self, event: libs::SystemInputEvent) -> Option<State> { None }
    }

    // --- 狀態實作 ---
    // 等待狀態
    struct ActionWaiting { flag1: bool }
    impl Default for ActionWaiting {
        fn default() -> ActionWaiting {
            ActionWaiting {
                flag1: false,
            }
        }
    }
    impl Action for ActionWaiting {
        fn enter(&mut self) {
            self.flag1 = false;
        }

        fn do_event(&mut self, event: libs::SystemInputEvent) -> Option<State> {
            match event {
                libs::SystemInputEvent::KeyCode(key_code, btn_state) => {
                    match btn_state {
                        libs::ButtonState::Release => {
                            match key_code {
                                libs::KeyCode::ControlRight => {
                                    if !self.flag1 {
                                        self.flag1 = true;
                                        libs::KeyCode::Return.click();
                                        libs::past_text("請選擇 - h:hello, q:退出 :: ");
                                    }
                                }
                                libs::KeyCode::KeyH => {
                                    if self.flag1 {
                                        libs::KeyCode::End.click();
                                        libs::sleep(100);
                                        libs::KeyCode::ShiftLeft.press();
                                        libs::sleep(100);
                                        libs::KeyCode::Home.click();
                                        libs::sleep(100);
                                        libs::KeyCode::ShiftLeft.release();
                                        libs::sleep(100);
                                        libs::KeyCode::Backspace.click();
                                        libs::sleep(100);
                                        libs::KeyCode::Return.click();

                                        return Some(State::Hello);
                                    }
                                }
                                libs::KeyCode::KeyQ => {
                                    libs::KeyCode::End.click();
                                    libs::sleep(100);
                                    libs::KeyCode::ShiftLeft.press();
                                    libs::sleep(100);
                                    libs::KeyCode::Home.click();
                                    libs::sleep(100);
                                    libs::KeyCode::ShiftLeft.release();
                                    libs::sleep(100);
                                    libs::KeyCode::Backspace.click();
                                    libs::sleep(100);
                                    libs::past_text("程式已退出");

                                    libs::exit();
                                }
                                _ => {}
                            }
                        }
                        libs::ButtonState::Press => {}
                    }
                }
                _ => {}
            }
            None
        }
    }

    // 測試狀態
    struct ActionHello;
    impl Action for ActionHello {
        #[allow(unused_variables)]
        fn enter(&mut self) {
            libs::KeyCode::Return.click();
            libs::sleep(50);
            libs::past_text("Hello 測試狀態 !!!");
            libs::exit();
        }
    }

    // --- 事件觸發介面, 保留各種狀態的實體 ---
    pub struct Context {
        current_s: State,

        action_waiting: ActionWaiting,
        action_hello: ActionHello,
    }
    impl Default for Context {
        fn default() -> Context {
            // Action 僅在此初始化一次
            Context {
                current_s: State::Waiting,
                action_waiting: ActionWaiting{..Default::default()},
                action_hello: ActionHello{},
            }
        }
    }
    impl Context {
        fn action_mapping(&mut self) -> &mut dyn Action {
            match self.current_s {
                State::Waiting => {
                    return &mut (self.action_waiting);
                }
                State::Hello => {
                    return &mut (self.action_hello);
                }
            }
        }

        fn change_state_if(&mut self, state_opt: Option<State>) {
            if let Some(state) = state_opt {
                let action1 = self.action_mapping();
                action1.out();
                self.current_s = state;
                let action2 = self.action_mapping();
                action2.enter();
            }
        }

        // --- event 轉發到 Action 中處理 ---
        pub fn do_event(&mut self, event: libs::SystemInputEvent) {
            let state = self.action_mapping().do_event(event);
            self.change_state_if(state);
        }
    }
}

fn start_event01() {
    let mut ctx: pc_ctrl::Context = Default::default();
    libs::SystemInputEvent::listen_all(move |event| {
        //println!("{:?}", event);
        ctx.do_event(event);
    });
}

fn main() {
    start_event01();
}
