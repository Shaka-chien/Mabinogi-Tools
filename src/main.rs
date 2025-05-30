// keyboard mouse api library
mod km_libs {
    extern crate winapi;
    use winapi::um::winuser::{
        INPUT, INPUT_KEYBOARD, INPUT_MOUSE, SendInput, 
        KEYEVENTF_KEYUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_ABSOLUTE, 
        MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_WHEEL,
        GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
        SetWindowsHookExW, CallNextHookEx, UnhookWindowsHookEx, GetMessageW, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP, KBDLLHOOKSTRUCT,
        WH_MOUSE_LL, WM_MOUSEMOVE, WM_LBUTTONDOWN, WM_RBUTTONDOWN, WM_LBUTTONUP,
        WM_RBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEWHEEL, MSLLHOOKSTRUCT,
        GetCursorPos,
    };
    use winapi::shared::minwindef::{DWORD, LRESULT, WPARAM, LPARAM};
    use std::mem::size_of;

    use std::ptr::null_mut;
    //use std::ffi::c_void;
    use winapi::shared::windef::{HHOOK, POINT};
    use winapi::um::libloaderapi::GetModuleHandleW;

    // --- get system info ---
    fn get_mouse_position() -> (i32, i32) {
        unsafe {
            let mut point: POINT = POINT { x: 0, y: 0 };
            if GetCursorPos(&mut point) != 0 {
                (point.x, point.y)
            } else {
                (-1, -1)
            }
        }
    }

    pub fn convert_to_absolute(x: i32, y: i32) -> (i32, i32) {
        let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

        let fx = x as f64;
        let fy = y as f64;
        let fscreen_width = screen_width as f64;
        let fscreen_height = screen_height as f64;
        
        let dx = (fx * 65535.0 / fscreen_width).round() as i32;
        let dy = (fy * 65535.0 / fscreen_height).round() as i32;
        
        (dx, dy)
    }

    // --- keyboard, mouse simulater core ---
    fn simulate_key_press(code: u32, flags_ext: u32, scan_code: u16) {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let ki = input.u.ki_mut();
            ki.wVk = code as u16;
            ki.wScan = scan_code;
            ki.dwFlags = 0; // 按下鍵
            ki.dwFlags = ki.dwFlags | flags_ext; // https://stackoverflow.com/questions/44924962/sendinput-on-c-doesnt-take-ctrl-and-shift-in-account
        }
        unsafe { SendInput(1, &mut input, size_of::<INPUT>() as i32) };
        // unsafe { let ki = input.u.ki(); keybd_event(ki.wVk as u8, 0, ki.dwFlags, 0) };
    }

    fn simulate_key_release(code: u32, flags_ext: u32, scan_code: u16) {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: unsafe { std::mem::zeroed() },
        };
        unsafe {
            let ki = input.u.ki_mut();
            ki.wVk = code as u16;
            ki.wScan = scan_code;
            ki.dwFlags = KEYEVENTF_KEYUP; // 放開鍵
            ki.dwFlags = ki.dwFlags | flags_ext; // https://stackoverflow.com/questions/44924962/sendinput-on-c-doesnt-take-ctrl-and-shift-in-account
        }
        unsafe { SendInput(1, &mut input, size_of::<INPUT>() as i32) };
        // unsafe { let ki = input.u.ki(); keybd_event(ki.wVk as u8, 0, ki.dwFlags, 0) };
    }

    fn simulate_mouse_move(dx: i32, dy: i32) {
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

    #[allow(dead_code)]
    fn simulate_mouse_lbtn_press(dx: i32, dy: i32) {
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

    #[allow(dead_code)]
    fn simulate_mouse_lbtn_release(dx: i32, dy: i32) {
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

    #[allow(dead_code)]
    fn simulate_mouse_rbtn_press(dx: i32, dy: i32) {
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

    #[allow(dead_code)]
    fn simulate_mouse_rbtn_release(dx: i32, dy: i32) {
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

    #[allow(dead_code)]
    fn simulate_mouse_mbtn_press(dx: i32, dy: i32) {
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

    #[allow(dead_code)]
    fn simulate_mouse_mbtn_release(dx: i32, dy: i32) {
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

    #[allow(dead_code)]
    fn simulate_mouse_whell(dx: i32, dy: i32, delta: i16) {
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

    pub static mut EVENT_CALLBACK: Option<Box<dyn FnMut(Event) -> Option<isize>>> = None;

    pub fn listen_keyboard_event() {
        static mut HOOK: HHOOK = null_mut();
        extern "system" fn raw_callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
            if code >= 0 {
                // https://learn.microsoft.com/zh-tw/windows/win32/inputdev/keyboard-input
                // println!("DWORD: {}", w_param as DWORD);
                match w_param as DWORD {
                    // --- keyboard event ---
                    WM_KEYDOWN | WM_SYSKEYDOWN => {
                        let kb_struct = unsafe { &*(l_param as *const KBDLLHOOKSTRUCT) };
                        let key_code = KeyCode::from_code(kb_struct.vkCode);
                        let detail = KeyCodeDetail { code: kb_struct.vkCode, flags: kb_struct.flags, scan_code: kb_struct.scanCode};
                        #[allow(static_mut_refs)]
                        unsafe {
                            if let Some(cb) = &mut EVENT_CALLBACK {
                                let cb_rtn = cb(Event::Keyboard(key_code, ButtonAction::Down, detail));
                                if let Some(rtn_code) = cb_rtn {
                                    return rtn_code; // 這裡回傳 1 會欄截系統事件, 不向程式發送
                                }
                            } else {
                                //println!("Key pressed :: code: {}, flags: {}, scan_code: {}, time: {}, extra_info: {}", kb_struct.vkCode, kb_struct.flags, kb_struct.scanCode, kb_struct.time, kb_struct.dwExtraInfo);
                                println!("{:?} {:?} {:?}", key_code, detail, ButtonAction::Down);
                            }
                        }
                    }
                    WM_KEYUP | WM_SYSKEYUP => {
                        let kb_struct = unsafe { &*(l_param as *const KBDLLHOOKSTRUCT) };
                        let key_code = KeyCode::from_code(kb_struct.vkCode);
                        let detail = KeyCodeDetail { code: kb_struct.vkCode, flags: kb_struct.flags, scan_code: kb_struct.scanCode};
                        #[allow(static_mut_refs)]
                        unsafe {
                            if let Some(cb) = &mut EVENT_CALLBACK {
                                let cb_rtn = cb(Event::Keyboard(key_code, ButtonAction::Up, detail));
                                if let Some(rtn_code) = cb_rtn {
                                    return rtn_code; // 這裡回傳 1 會欄截系統事件, 不向程式發送
                                }
                            } else {
                                //println!("Key released :: code: {}, flags: {}, scan_code: {}, time: {}, extra_info: {}", kb_struct.vkCode, kb_struct.flags, kb_struct.scanCode, kb_struct.time, kb_struct.dwExtraInfo);
                                println!("{:?} {:?} {:?}", key_code, detail, ButtonAction::Up);
                            }
                        }
                    }
                    _ => {}
                }
            }
            unsafe { CallNextHookEx(HOOK, code, w_param, l_param) }
        }
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
        static mut HOOK: HHOOK = null_mut();
        extern "system" fn raw_callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
            if code >= 0 {
                match w_param as DWORD {
                    // --- mouse event ---
                    WM_MOUSEMOVE => {
                        let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                        let event = MouseEvent::Move{x: mouse_info.pt.x, y: mouse_info.pt.y};
                        let detail = MouseCodeDetail { mouse_data: mouse_info.mouseData, flags: mouse_info.flags };
                        #[allow(static_mut_refs)]
                        unsafe {
                            if let Some(cb) = &mut EVENT_CALLBACK {
                                let cb_rtn = cb(Event::Mouse(event, detail));
                                if let Some(rtn_code) = cb_rtn {
                                    return rtn_code; // 這裡回傳 1 會欄截系統事件, 不向程式發送
                                }
                            } else {
                                //println!("Mouse moved to: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                                println!("{:?} {:?}", event, detail);
                            }
                        }
                    }
                    WM_LBUTTONDOWN => {
                        let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                        let event = MouseEvent::LBtnDown{x: mouse_info.pt.x, y: mouse_info.pt.y};
                        let detail = MouseCodeDetail { mouse_data: mouse_info.mouseData, flags: mouse_info.flags };
                        #[allow(static_mut_refs)]
                        unsafe {
                            if let Some(cb) = &mut EVENT_CALLBACK {
                                let cb_rtn = cb(Event::Mouse(event, detail));
                                if let Some(rtn_code) = cb_rtn {
                                    return rtn_code; // 這裡回傳 1 會欄截系統事件, 不向程式發送
                                }
                            } else {
                                //println!("Left button down at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                                println!("{:?} {:?}", event, detail);
                            }
                        }
                    }
                    WM_LBUTTONUP => {
                        let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                        let event = MouseEvent::LBtnUp{x: mouse_info.pt.x, y: mouse_info.pt.y};
                        let detail = MouseCodeDetail { mouse_data: mouse_info.mouseData, flags: mouse_info.flags };
                        #[allow(static_mut_refs)]
                        unsafe {
                            if let Some(cb) = &mut EVENT_CALLBACK {
                                let cb_rtn = cb(Event::Mouse(event, detail));
                                if let Some(rtn_code) = cb_rtn {
                                    return rtn_code; // 這裡回傳 1 會欄截系統事件, 不向程式發送
                                }
                            } else {
                                //println!("Left button up at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                                println!("{:?} {:?}", event, detail);
                            }
                        }
                    }
                    WM_RBUTTONDOWN => {
                        let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                        let event = MouseEvent::RBtnDown{x: mouse_info.pt.x, y: mouse_info.pt.y};
                        let detail = MouseCodeDetail { mouse_data: mouse_info.mouseData, flags: mouse_info.flags };
                        #[allow(static_mut_refs)]
                        unsafe {
                            if let Some(cb) = &mut EVENT_CALLBACK {
                                let cb_rtn = cb(Event::Mouse(event, detail));
                                if let Some(rtn_code) = cb_rtn {
                                    return rtn_code; // 這裡回傳 1 會欄截系統事件, 不向程式發送
                                }
                            } else {
                                //println!("Right button down at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                                println!("{:?} {:?}", event, detail);
                            }
                        }
                    }
                    WM_RBUTTONUP => {
                        let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                        let event = MouseEvent::RBtnUp{x: mouse_info.pt.x, y: mouse_info.pt.y};
                        let detail = MouseCodeDetail { mouse_data: mouse_info.mouseData, flags: mouse_info.flags };
                        #[allow(static_mut_refs)]
                        unsafe {
                            if let Some(cb) = &mut EVENT_CALLBACK {
                                let cb_rtn = cb(Event::Mouse(event, detail));
                                if let Some(rtn_code) = cb_rtn {
                                    return rtn_code; // 這裡回傳 1 會欄截系統事件, 不向程式發送
                                }
                            } else {
                                //println!("Right button up at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                                println!("{:?} {:?}", event, detail);
                            }
                        }
                    }
                    WM_MBUTTONDOWN => {
                        let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                        let event = MouseEvent::MBtnDown{x: mouse_info.pt.x, y: mouse_info.pt.y};
                        let detail = MouseCodeDetail { mouse_data: mouse_info.mouseData, flags: mouse_info.flags };
                        #[allow(static_mut_refs)]
                        unsafe {
                            if let Some(cb) = &mut EVENT_CALLBACK {
                                let cb_rtn = cb(Event::Mouse(event, detail));
                                if let Some(rtn_code) = cb_rtn {
                                    return rtn_code; // 這裡回傳 1 會欄截系統事件, 不向程式發送
                                }
                            } else {
                                //println!("Middle button down at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                                println!("{:?} {:?}", event, detail);
                            }
                        }
                    }
                    WM_MBUTTONUP => {
                        let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                        let event = MouseEvent::MBtnUp{x: mouse_info.pt.x, y: mouse_info.pt.y};
                        let detail = MouseCodeDetail { mouse_data: mouse_info.mouseData, flags: mouse_info.flags };
                        #[allow(static_mut_refs)]
                        unsafe {
                            if let Some(cb) = &mut EVENT_CALLBACK {
                                let cb_rtn = cb(Event::Mouse(event, detail));
                                if let Some(rtn_code) = cb_rtn {
                                    return rtn_code; // 這裡回傳 1 會欄截系統事件, 不向程式發送
                                }
                            } else {
                                //println!("Middle button up at: ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                                println!("{:?} {:?}", event, detail);
                            }
                        }
                    }
                    WM_MOUSEWHEEL => {
                        let mouse_info = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                        let delta = (mouse_info.mouseData >> 16) as i16;
                        let event = MouseEvent::Whell{x: mouse_info.pt.x, y: mouse_info.pt.y, delta};
                        let detail = MouseCodeDetail { mouse_data: mouse_info.mouseData, flags: mouse_info.flags };
                        #[allow(static_mut_refs)]
                        unsafe {
                            if let Some(cb) = &mut EVENT_CALLBACK {
                                let cb_rtn = cb(Event::Mouse(event, detail));
                                if let Some(rtn_code) = cb_rtn {
                                    return rtn_code; // 這裡回傳 1 會欄截系統事件, 不向程式發送
                                }
                            } else {
                                //let up_down = if delta > 0 { "up" } else { "down" };
                                //println!("Mouse wheel scrolled {} at: ({}, {}), delta: {}", up_down, mouse_info.pt.x, mouse_info.pt.y, delta);
                                println!("{:?} {:?}", event, detail);
                            }
                        }
                    }
                    _ => {}
                }
            }
            unsafe { CallNextHookEx(HOOK, code, w_param, l_param) }
        }
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

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum MouseEvent {
        Move { x: i32, y: i32 },
        Whell{ x: i32, y: i32, delta: i16 },
        LBtnDown { x: i32, y: i32 },
        RBtnDown { x: i32, y: i32 },
        MBtnDown { x: i32, y: i32 },
        LBtnUp { x: i32, y: i32 },
        RBtnUp { x: i32, y: i32 },
        MBtnUp { x: i32, y: i32 },
    }
    impl MouseEvent {
        // position
        pub fn get_mouse_position0() -> (i32, i32) { get_mouse_position() }
        // dx, dy
        pub fn get_mouse_position1() -> (i32, i32) {
            let (x, y) = get_mouse_position();
            convert_to_absolute(x, y)
        }
        pub fn do_it(&self) {
            match self {
                MouseEvent::Move { x, y } => { simulate_mouse_move(*x, *y); }
                MouseEvent::Whell{ x, y, delta } => { simulate_mouse_whell(*x, *y, *delta); }
                MouseEvent::LBtnDown { x, y } => { simulate_mouse_lbtn_press(*x, *y); }
                MouseEvent::RBtnDown { x, y } => { simulate_mouse_rbtn_press(*x, *y); }
                MouseEvent::MBtnDown { x, y } => { simulate_mouse_mbtn_press(*x, *y); }
                MouseEvent::LBtnUp { x, y } => { simulate_mouse_lbtn_release(*x, *y); }
                MouseEvent::RBtnUp { x, y } => { simulate_mouse_rbtn_release(*x, *y); }
                MouseEvent::MBtnUp { x, y } => { simulate_mouse_mbtn_release(*x, *y); }
            }
        }
        pub fn click(dx: i32, dy: i32) {
            simulate_mouse_lbtn_press(dx, dy);
            sleep(20);
            simulate_mouse_lbtn_release(dx, dy);
        }
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum ButtonAction {
        Down, Up
    }

    #[derive(Debug)]
    pub enum KeyCode {
        Alt,   // left  alt
        AltGr, // right alt
        Backspace,
        CapsLock,
        ControlLeft,
        ControlRight,
        Delete,
        DownArrow,
        End,
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
        F16,
        F17,
        F18,
        F19,
        F20,
        F21,
        F22,
        F23,
        F24,
        F25,
        F26,
        F27,
        F28,
        F29,
        F30,
        F31,
        F32,
        FN,
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

        VolumeMute,
        VolumeUp,
        VolumeDown,
        MediaPlayPause,

        BackQuote, // `
        // 數字1~0
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
        //數字鍵盤
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
    // key code list - https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    impl KeyCode {
        pub fn from_code(code: u32) -> KeyCode {
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
                113     => { KeyCode::F2 }
                114     => { KeyCode::F3 }
                115     => { KeyCode::F4 }
                116     => { KeyCode::F5 }
                117     => { KeyCode::F6 }
                118     => { KeyCode::F7 }
                119     => { KeyCode::F8 }
                120     => { KeyCode::F9 }
                121     => { KeyCode::F10 }
                122     => { KeyCode::F11 }
                123     => { KeyCode::F12 }
                124     => { KeyCode::F13 }
                125     => { KeyCode::F14 }
                126     => { KeyCode::F15 }
                127     => { KeyCode::F16 }
                128     => { KeyCode::F17 }
                129     => { KeyCode::F18 }
                130     => { KeyCode::F19 }
                131     => { KeyCode::F20 }
                132     => { KeyCode::F21 }
                133     => { KeyCode::F22 }
                134     => { KeyCode::F23 }
                135     => { KeyCode::F24 }
                136     => { KeyCode::F25 }
                137     => { KeyCode::F26 }
                138     => { KeyCode::F27 }
                139     => { KeyCode::F28 }
                140     => { KeyCode::F29 }
                141     => { KeyCode::F30 }
                142     => { KeyCode::F31 }
                143     => { KeyCode::F32 }
                255     => { KeyCode::FN  }
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
                173     => { KeyCode::VolumeMute }
                174     => { KeyCode::VolumeDown }
                175     => { KeyCode::VolumeUp }
                179     => { KeyCode::MediaPlayPause }
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
        pub fn to_code(&self) -> u32 {
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
                KeyCode::F2             => { 113    }
                KeyCode::F3             => { 114    }
                KeyCode::F4             => { 115    }
                KeyCode::F5             => { 116    }
                KeyCode::F6             => { 117    }
                KeyCode::F7             => { 118    }
                KeyCode::F8             => { 119    }
                KeyCode::F9             => { 120    }
                KeyCode::F10            => { 121    }
                KeyCode::F11            => { 122    }
                KeyCode::F12            => { 123    }
                KeyCode::F13            => { 124    }
                KeyCode::F14            => { 125    }
                KeyCode::F15            => { 126    }
                KeyCode::F16            => { 127    }
                KeyCode::F17            => { 128    }
                KeyCode::F18            => { 129    }
                KeyCode::F19            => { 130    }
                KeyCode::F20            => { 131    }
                KeyCode::F21            => { 132    }
                KeyCode::F22            => { 133    }
                KeyCode::F23            => { 134    }
                KeyCode::F24            => { 135    }
                KeyCode::F25            => { 136    }
                KeyCode::F26            => { 137    }
                KeyCode::F27            => { 138    }
                KeyCode::F28            => { 139    }
                KeyCode::F29            => { 140    }
                KeyCode::F30            => { 141    }
                KeyCode::F31            => { 142    }
                KeyCode::F32            => { 143    }
                KeyCode::FN             => { 255    }
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
                KeyCode::VolumeMute     => { 173    }
                KeyCode::VolumeDown     => { 174    }
                KeyCode::VolumeUp       => { 175    }
                KeyCode::MediaPlayPause => { 179    }
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
        pub fn flags_ext(&self) -> u32 {
            match self {
                KeyCode::ControlRight   => { 1 }
                KeyCode::Insert         => { 1 }
                KeyCode::Delete         => { 1 }
                KeyCode::Home           => { 1 }
                KeyCode::End            => { 1 }
                KeyCode::PageUp         => { 1 }
                KeyCode::PageDown       => { 1 }
                KeyCode::UpArrow        => { 1 }
                KeyCode::DownArrow      => { 1 }
                KeyCode::LeftArrow      => { 1 }
                KeyCode::RightArrow     => { 1 }
                KeyCode::NumLock        => { 1 }
                KeyCode::KpMinus        => { 1 }
                KeyCode::KpPlus         => { 1 }
                KeyCode::KpMultiply     => { 1 }
                KeyCode::KpDivide       => { 1 }
                KeyCode::Kp0            => { 1 }
                KeyCode::Kp1            => { 1 }
                KeyCode::Kp2            => { 1 }
                KeyCode::Kp3            => { 1 }
                KeyCode::Kp4            => { 1 }
                KeyCode::Kp5            => { 1 }
                KeyCode::Kp6            => { 1 }
                KeyCode::Kp7            => { 1 }
                KeyCode::Kp8            => { 1 }
                KeyCode::Kp9            => { 1 }
                KeyCode::KpDelete       => { 1 }
                _ => { 0 }
            }
        }
        pub fn scan_code(&self) -> u16 {
            match self {
                KeyCode::Alt            => { 56 }
                KeyCode::AltGr          => { 56 }
                KeyCode::BackQuote      => { 41 }
                KeyCode::BackSlash      => { 43 }
                KeyCode::Backspace      => { 14 }
                KeyCode::CapsLock       => { 58 }
                KeyCode::Comma          => { 51 }
                KeyCode::ControlLeft    => { 29 }
                KeyCode::ControlRight   => { 29 }
                KeyCode::Delete         => { 83 }
                KeyCode::Dot            => { 52 }
                KeyCode::DownArrow      => { 80 }
                KeyCode::End            => { 79 }
                KeyCode::Equal          => { 13 }
                KeyCode::Escape         => { 1 }
                KeyCode::F1             => { 59 }
                KeyCode::F2             => { 60 }
                KeyCode::F3             => { 61 }
                KeyCode::F4             => { 62 }
                KeyCode::F5             => { 63 }
                KeyCode::F6             => { 64 }
                KeyCode::F7             => { 65 }
                KeyCode::F8             => { 66 }
                KeyCode::F9             => { 67 }
                KeyCode::F10            => { 68 }
                KeyCode::F11            => { 87 }
                KeyCode::F12            => { 88 }
                KeyCode::Home           => { 71 }
                KeyCode::Insert         => { 82 }
                KeyCode::KeyA           => { 30 }
                KeyCode::KeyB           => { 48 }
                KeyCode::KeyC           => { 46 }
                KeyCode::KeyD           => { 32 }
                KeyCode::KeyE           => { 18 }
                KeyCode::KeyF           => { 33 }
                KeyCode::KeyG           => { 34 }
                KeyCode::KeyH           => { 35 }
                KeyCode::KeyI           => { 23 }
                KeyCode::KeyJ           => { 36 }
                KeyCode::KeyK           => { 37 }
                KeyCode::KeyL           => { 38 }
                KeyCode::KeyM           => { 50 }
                KeyCode::KeyN           => { 49 }
                KeyCode::KeyO           => { 24 }
                KeyCode::KeyP           => { 25 }
                KeyCode::KeyQ           => { 16 }
                KeyCode::KeyR           => { 19 }
                KeyCode::KeyS           => { 31 }
                KeyCode::KeyT           => { 20 }
                KeyCode::KeyU           => { 22 }
                KeyCode::KeyV           => { 47 }
                KeyCode::KeyW           => { 17 }
                KeyCode::KeyX           => { 45 }
                KeyCode::KeyY           => { 21 }
                KeyCode::KeyZ           => { 44 }
                KeyCode::Kp0            => { 82 }
                KeyCode::Kp1            => { 79 }
                KeyCode::Kp2            => { 80 }
                KeyCode::Kp3            => { 81 }
                KeyCode::Kp4            => { 75 }
                KeyCode::Kp5            => { 76 }
                KeyCode::Kp6            => { 77 }
                KeyCode::Kp7            => { 71 }
                KeyCode::Kp8            => { 72 }
                KeyCode::Kp9            => { 73 }
                KeyCode::KpDelete       => { 83 }
                KeyCode::KpDivide       => { 53 }
                KeyCode::KpMinus        => { 74 }
                KeyCode::KpMultiply     => { 55 }
                KeyCode::KpPlus         => { 78 }
                KeyCode::LeftArrow      => { 75 }
                KeyCode::LeftBracket    => { 26 }
                KeyCode::MetaLeft       => { 91 }
                KeyCode::Minus          => { 12 }
                KeyCode::Num0           => { 11 }
                KeyCode::Num1           => { 2 }
                KeyCode::Num2           => { 3 }
                KeyCode::Num3           => { 4 }
                KeyCode::Num4           => { 5 }
                KeyCode::Num5           => { 6 }
                KeyCode::Num6           => { 7 }
                KeyCode::Num7           => { 8 }
                KeyCode::Num8           => { 9 }
                KeyCode::Num9           => { 10 }
                KeyCode::NumLock        => { 69 }
                KeyCode::PageDown       => { 81 }
                KeyCode::PageUp         => { 73 }
                KeyCode::Quote          => { 40 }
                KeyCode::Return         => { 28 }
                KeyCode::RightArrow     => { 77 }
                KeyCode::RightBracket   => { 27 }
                KeyCode::SemiColon      => { 39 }
                KeyCode::ShiftLeft      => { 42 }
                KeyCode::ShiftRight     => { 54 }
                KeyCode::Slash          => { 53 }
                KeyCode::Space          => { 0 }
                KeyCode::Tab            => { 15 }
                KeyCode::UpArrow        => { 72 }
                _ => { 0 }
            }
        }
        pub fn down(&self) {
            let code = self.to_code();
            let flags_ext = self.flags_ext();
            let scan_code = self.scan_code();
            simulate_key_press(code, flags_ext, scan_code);
        }
        pub fn up(&self) {
            let code = self.to_code();
            let flags_ext = self.flags_ext();
            let scan_code = self.scan_code();
            simulate_key_release(code, flags_ext, scan_code);
        }
        pub fn click(&self) {
            let code = self.to_code();
            let flags_ext = self.flags_ext();
            let scan_code = self.scan_code();
            simulate_key_press(code, flags_ext, scan_code);
            sleep(20);
            simulate_key_release(code, flags_ext, scan_code);
        }
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct KeyCodeDetail { pub code: u32, pub scan_code: u32, pub flags: u32 }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct MouseCodeDetail { pub mouse_data: u32, pub flags: u32 }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum Event {
        Mouse(MouseEvent, MouseCodeDetail),
        Keyboard(KeyCode, ButtonAction, KeyCodeDetail),
    }

    // --- 剪貼簿 ---
    #[allow(dead_code)]
    pub fn cp_text_line_to_end() -> String {
        // 剪貼簿 library
        extern crate clipboard;
        use clipboard::ClipboardProvider;
        use clipboard::ClipboardContext;

        // 模擬按下 Shift + End, Ctrl + C
        KeyCode::ShiftLeft.down();
        sleep(20);
        KeyCode::End.click();
        sleep(20);
        KeyCode::ShiftLeft.up();
        sleep(20);
        KeyCode::ControlLeft.down();
        sleep(20);
        KeyCode::KeyC.click();
        sleep(20);
        KeyCode::ControlLeft.up();
        sleep(20);
        KeyCode::LeftArrow.click();

        // 將文字從 剪貼簿中 copy 出來
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        match ctx.get_contents() {
            Ok(s) => {s},
            Err(_e) => {String::from("")},
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
        KeyCode::ControlLeft.down();
        sleep(20);
        KeyCode::KeyV.click();
        sleep(20);
        KeyCode::ControlLeft.up();
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

// screen api library
mod sc_libs {
    use xcap::{
        Monitor,
        image::Pixel,
    };

    #[allow(dead_code)]
    fn normalized(filename: &str) -> String {
        filename
            .replace("|", "")
            .replace("\\", "")
            .replace(":", "")
            .replace("/", "")
    }

    pub fn get_rgb(x: u32, y: u32) -> Option<(u8, u8, u8)> {
        let monitors = Monitor::all().unwrap();
        for monitor in monitors {
            let image = monitor.capture_image().unwrap(); // image::RgbaImage
            let (width, height) = image.dimensions();

            // x, y 為絕對座標, 此處還原為 monitor 的相對座標
            let x = (x as i32) - monitor.x();
            let y = (y as i32) - monitor.y();
            if x > (width as i32) || y > (height as i32) {
                // 超過範圍, 找下一個 monitor
                continue;
            }

            // println!("{}: dimensions: {:?}, x: {}, y: {}", normalized(monitor.name()), (width, height), monitor.x(), monitor.y());
            let rgb = image.get_pixel(x as u32, y as u32).to_rgb(); // image::Rgba -> image::Rgb
            return Some((rgb[0], rgb[1], rgb[2]));
        }
        return None;
    }
}

mod ctrl {
    use crate::km_libs;
    use crate::sc_libs;
    use std::{
        cell::Cell,
        sync,
        thread,
        sync::atomic::{
            AtomicBool, AtomicI32, Ordering
        },
        sync::Arc,
        sync::mpsc,
        collections::HashMap,
    };

    pub struct Tools {}
    impl Tools {
        pub fn line_text_clear() {
            km_libs::KeyCode::End.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.down();
            km_libs::sleep(100);
            km_libs::KeyCode::Home.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.up();
            km_libs::sleep(100);
            km_libs::KeyCode::Backspace.click();
        }
    }

    #[allow(dead_code)]
    pub enum EventHandleReturn {
        CONTINUE,  // 事件泡泡繼續傳遞
        INTERCEPT, // 欄截系統事件, 不向程式發送
    }

    // --- 狀態 ---
    #[allow(dead_code)]
    pub trait State {
        #[allow(unused_variables)]
        fn enter(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {}
        #[allow(unused_variables)]
        fn out(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {}

        // 遇到程式摸擬的 鍵盤/滑鼠 事件, 是否要靜聲(不處理事件)
        fn mute_event_when_simulate(self: Arc<Self>) -> bool { true }
        // 靜聲時的 return value hook
        fn do_event_when_mute(self: Arc<Self>) -> (Arc<dyn State>, EventHandleReturn);

        #[allow(unused_variables)]
        fn do_event(self: Arc<Self>, event: km_libs::Event, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) -> (Arc<dyn State>, EventHandleReturn) {
            match event {
                km_libs::Event::Mouse(event, detail) => {
                    let mouse_simulate_mask = 1;
                    if self.clone().mute_event_when_simulate() && (detail.flags & mouse_simulate_mask) == mouse_simulate_mask {
                        return self.do_event_when_mute();
                    }
                    self.do_mouse_event(event, tx)
                }
                km_libs::Event::Keyboard(event, act, detail) => {
                    let keyboard_simulate_mask = 0x10;
                    if self.clone().mute_event_when_simulate() && (detail.flags & keyboard_simulate_mask) == keyboard_simulate_mask {
                        return self.do_event_when_mute();
                    }
                    match act {
                        km_libs::ButtonAction::Down => { self.do_keyboard_down(event, tx) }
                        km_libs::ButtonAction::Up => { self.do_keyboard_up(event, tx) }
                    }
                }
            }
        }

        #[allow(unused_variables)]
        fn do_mouse_event(self: Arc<Self>, event: km_libs::MouseEvent, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) -> (Arc<dyn State>, EventHandleReturn) { self.do_event_when_mute() }

        #[allow(unused_variables)]
        fn do_keyboard_down(self: Arc<Self>, event: km_libs::KeyCode, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) -> (Arc<dyn State>, EventHandleReturn) { self.do_event_when_mute() }

        #[allow(unused_variables)]
        fn do_keyboard_up(self: Arc<Self>, event: km_libs::KeyCode, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) -> (Arc<dyn State>, EventHandleReturn) { self.do_event_when_mute() }
    }

    // --- 等待(入口) ---
    #[allow(dead_code)]
    pub struct WaitingState { flag: Cell<bool> }
    impl WaitingState {
        fn new() -> WaitingState {
            WaitingState { flag: Cell::new(false) }
        }
    }
    impl State for WaitingState {
        #[allow(unused_variables)]
        fn enter(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {
            println!("等待模式開始... Right Ctrl 開啟功能");
        }
        #[allow(unused_variables)]
        fn out(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {
            km_libs::sleep(200);
            km_libs::KeyCode::End.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.down();
            km_libs::sleep(100);
            km_libs::KeyCode::Home.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.up();
            km_libs::sleep(100);
            km_libs::KeyCode::Backspace.click();
            km_libs::sleep(100);
        }
        fn do_event_when_mute(self: Arc<Self>) -> (Arc<dyn State>, EventHandleReturn) { (self.clone(), EventHandleReturn::CONTINUE) }
        #[allow(unused_variables)]
        fn do_keyboard_up(self: Arc<Self>, event: km_libs::KeyCode, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) -> (Arc<dyn State>, EventHandleReturn) {
            match event {
                km_libs::KeyCode::ControlRight => {
                    if !self.flag.get() {
                        self.flag.set(true);
                        km_libs::sleep(50);
                        km_libs::KeyCode::Return.click();
                        km_libs::sleep(50);
                        km_libs::past_text("請選擇 - c: 滑鼠連點, m: 取得滑鼠位置, t: 道具移動, b: 戰鬥模式, x: 返回待命狀態, q:退出 :: ");
                    }
                }
                km_libs::KeyCode::KeyM => {
                    if self.flag.get() {
                        return (Arc::new(MousePositionState::new()), EventHandleReturn::CONTINUE);
                    }
                }
                km_libs::KeyCode::KeyT => {
                    if self.flag.get() {
                        return (Arc::new(ItemMoveToState::new()), EventHandleReturn::CONTINUE);
                    }
                }
                km_libs::KeyCode::KeyC => {
                    if self.flag.get() {
                        return (Arc::new(MouseClicksState::new()), EventHandleReturn::CONTINUE);
                    }
                }
                km_libs::KeyCode::KeyB => {
                    if self.flag.get() {
                        return (Arc::new(FingingState::new(false)), EventHandleReturn::CONTINUE);
                    }
                }
                km_libs::KeyCode::KeyQ => {
                    if self.flag.get() {
                        return (Arc::new(ExitState::new()), EventHandleReturn::CONTINUE);
                    }
                }
                km_libs::KeyCode::KeyX => {
                    if self.flag.get() {
                        self.flag.set(false);
                        km_libs::sleep(200);
                        km_libs::KeyCode::End.click();
                        km_libs::sleep(100);
                        km_libs::KeyCode::ShiftLeft.down();
                        km_libs::sleep(100);
                        km_libs::KeyCode::Home.click();
                        km_libs::sleep(100);
                        km_libs::KeyCode::ShiftLeft.up();
                        km_libs::sleep(100);
                        km_libs::KeyCode::Backspace.click();
                        km_libs::sleep(100);
                        km_libs::KeyCode::Return.click();
                    }
                }
                _ => { }
            }
            (self.clone(), EventHandleReturn::CONTINUE)
        }
    }

    // --- 取得滑鼠位置 ---
    pub struct MousePositionState {}
    impl MousePositionState {
        fn new() -> MousePositionState { MousePositionState {} }
    }
    impl State for MousePositionState {
        #[allow(unused_variables)]
        fn enter(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {
            println!("取得滑鼠位置資訊開始...");
            let (x, y) = km_libs::MouseEvent::get_mouse_position0();
            match sc_libs::get_rgb(x as u32, y as u32) {
                Some((r, g, b)) => {
                    //println!("座標 x: {}, y: {}, rgb: ({}, {}, {})", x, y, r, g, b);
                    km_libs::past_text(format!("滑鼠座標: ({}, {}), rgb: ({}, {}, {}), esc 回到 WaitingState", x, y, r, g, b));
                }
                _ => {}
            }
        }
        #[allow(unused_variables)]
        fn out(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {
            km_libs::KeyCode::End.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.down();
            km_libs::sleep(100);
            km_libs::KeyCode::Home.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.up();
            km_libs::sleep(100);
            km_libs::KeyCode::Backspace.click();
            km_libs::sleep(100);
            km_libs::KeyCode::Return.click();
        }
        fn do_event_when_mute(self: Arc<Self>) -> (Arc<dyn State>, EventHandleReturn) { (self.clone(), EventHandleReturn::CONTINUE) }
        #[allow(unused_variables)]
        fn do_keyboard_up(self: Arc<Self>, event: km_libs::KeyCode, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) -> (Arc<dyn State>, EventHandleReturn) {
            match event {
                km_libs::KeyCode::Escape => {
                    return (Arc::new(WaitingState::new()), EventHandleReturn::CONTINUE);
                }
                _ => { }
            }
            (self.clone(), EventHandleReturn::CONTINUE)
        }
    }

    // --- 滑鼠連點 ---
    pub struct MouseClicksState {
        handle: Cell<Option<thread::JoinHandle<()>>>,
        alive: sync::Arc<AtomicBool>,
        r_mouse_btn: sync::Arc<AtomicBool>,
        dx: sync::Arc<AtomicI32>,
        dy: sync::Arc<AtomicI32>,
    }
    impl MouseClicksState {
        fn new() -> MouseClicksState {
            MouseClicksState {
                handle: Cell::new(None),
                alive: sync::Arc::new(AtomicBool::new(false)),
                r_mouse_btn: sync::Arc::new(AtomicBool::new(false)),
                dx: sync::Arc::new(AtomicI32::new(-1)),
                dy: sync::Arc::new(AtomicI32::new(-1)),
            }
        }
    }
    impl State for MouseClicksState {
        #[allow(unused_variables)]
        fn enter(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {
            println!("滑鼠連點開始...");
            let alive = self.alive.clone();
            let r_mouse_btn = self.r_mouse_btn.clone();
            let dx = self.dx.clone();
            let dy = self.dy.clone();

            let (_dx, _dy) = km_libs::MouseEvent::get_mouse_position1();
            if dx.load(Ordering::Relaxed) == -1 { dx.store(_dx, Ordering::Relaxed); }
            if dy.load(Ordering::Relaxed) == -1 { dy.store(_dy, Ordering::Relaxed); }

            if !alive.load(Ordering::Relaxed) {
                km_libs::past_text(format!("滑鼠連點啟動中, esc 回到 WaitingState"));
                alive.store(true, Ordering::Relaxed);
                self.handle.set(Some(thread::spawn(move || {
                    while alive.load(Ordering::Relaxed) {
                        if !r_mouse_btn.load(Ordering::Relaxed) {
                            km_libs::MouseEvent::click(dx.load(Ordering::Relaxed), dy.load(Ordering::Relaxed));
                        }
                        km_libs::sleep(200);
                    }
                })));
            }
        }
        #[allow(unused_variables)]
        fn out(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {
            km_libs::sleep(50);
            km_libs::KeyCode::Return.click();
            km_libs::sleep(100);
            km_libs::KeyCode::End.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.down();
            km_libs::sleep(100);
            km_libs::KeyCode::Home.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.up();
            km_libs::sleep(100);
            km_libs::KeyCode::Backspace.click();
            km_libs::sleep(100);
            km_libs::KeyCode::Return.click();
        }
        fn do_event_when_mute(self: Arc<Self>) -> (Arc<dyn State>, EventHandleReturn) { (self.clone(), EventHandleReturn::CONTINUE) }
        #[allow(unused_variables)]
        fn do_mouse_event(self: Arc<Self>, event: km_libs::MouseEvent, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) -> (Arc<dyn State>, EventHandleReturn) {
            let dx = self.dx.clone();
            let dy = self.dy.clone();
            match event {
                km_libs::MouseEvent::Move { x, y } => {
                    //let (_dx, _dy) = km_libs::convert_to_absolute(x, y);
                    let (_dx, _dy) = km_libs::MouseEvent::get_mouse_position1();
                    dx.store(_dx, Ordering::Relaxed);
                    dy.store(_dy, Ordering::Relaxed);
                }
                km_libs::MouseEvent::RBtnDown { x, y } => {
                    self.r_mouse_btn.store(true, Ordering::Relaxed);
                }
                km_libs::MouseEvent::RBtnUp { x, y } => {
                    self.r_mouse_btn.store(false, Ordering::Relaxed);
                }
                _ => {}
            }
            (self.clone(), EventHandleReturn::CONTINUE)
        }
        #[allow(unused_variables)]
        fn do_keyboard_up(self: Arc<Self>, event: km_libs::KeyCode, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) -> (Arc<dyn State>, EventHandleReturn) {
            match event {
                km_libs::KeyCode::Escape => {
                    self.alive.store(false, Ordering::Relaxed);
                    self.handle
                        .take().expect("Called stop on non-running thread")
                        .join().expect("Could not join spawned thread");
                    return (Arc::new(WaitingState::new()), EventHandleReturn::CONTINUE);
                }
                km_libs::KeyCode::ShiftLeft => {
                    if self.alive.load(Ordering::Relaxed) {
                        km_libs::sleep(20);
                        km_libs::KeyCode::Alt.down();
                    }
                }
                _ => { }
            }
            (self.clone(), EventHandleReturn::CONTINUE)
        }
    }

    // --- 批次道具移動 ---
    pub struct ItemMoveToState {
        is_end: sync::Arc<AtomicBool>,
        step: sync::Arc<AtomicI32>,
        item_dbase1: sync::Arc<AtomicI32>,
        res_x: sync::Arc<AtomicI32>,
        res_y: sync::Arc<AtomicI32>,
        dest_x: sync::Arc<AtomicI32>,
        dest_y: sync::Arc<AtomicI32>,
        item_w: sync::Arc<AtomicI32>,
        item_h: sync::Arc<AtomicI32>,
        item_cnt_w: sync::Arc<AtomicI32>,
        item_cnt: sync::Arc<AtomicI32>,
        dest_cnt_w: sync::Arc<AtomicI32>,
    }
    impl ItemMoveToState {
        fn new() -> ItemMoveToState {
            ItemMoveToState {
                is_end: sync::Arc::new(AtomicBool::new(false)),
                step: sync::Arc::new(AtomicI32::new(1)),
                item_dbase1: sync::Arc::new(AtomicI32::new(24)),
                res_x: sync::Arc::new(AtomicI32::new(-1)),
                res_y: sync::Arc::new(AtomicI32::new(-1)),
                dest_x: sync::Arc::new(AtomicI32::new(-1)),
                dest_y: sync::Arc::new(AtomicI32::new(-1)),
                item_w: sync::Arc::new(AtomicI32::new(-1)),
                item_h: sync::Arc::new(AtomicI32::new(-1)),
                item_cnt_w: sync::Arc::new(AtomicI32::new(-1)),
                item_cnt: sync::Arc::new(AtomicI32::new(-1)),
                dest_cnt_w: sync::Arc::new(AtomicI32::new(-1)),
            }
        }
        fn error_rtn(is_end: sync::Arc<AtomicBool>) -> (Arc<dyn State>, EventHandleReturn) {
            is_end.store(true, Ordering::Relaxed);
            Tools::line_text_clear();
            km_libs::past_text("發生錯誤... 3 秒後退出 批次道具移動...");
            km_libs::sleep(3000);
            Tools::line_text_clear();
            km_libs::KeyCode::Return.click();
            return (Arc::new(WaitingState::new()), EventHandleReturn::CONTINUE);
        }
    }
    impl State for ItemMoveToState {
        #[allow(unused_variables)]
        fn enter(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {
            println!("批次道具移動開始...");
            km_libs::past_text(format!("批次道具移動, 請移動到道具來源處, 並按下 right shift 進行下一步"));
        }
        #[allow(unused_variables)]
        fn out(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {
            km_libs::KeyCode::Return.click();
            km_libs::sleep(100);
            Tools::line_text_clear();
            km_libs::sleep(100);
            km_libs::KeyCode::Return.click();
        }
        fn do_event_when_mute(self: Arc<Self>) -> (Arc<dyn State>, EventHandleReturn) { (self.clone(), EventHandleReturn::CONTINUE) }
        #[allow(unused_variables)]
        fn do_keyboard_up(self: Arc<Self>, event: km_libs::KeyCode, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) -> (Arc<dyn State>, EventHandleReturn) {
            let is_end      = self.is_end.clone();
            let step        = self.step.clone();
            let item_dbase1 = self.item_dbase1.clone();
            let res_x       = self.res_x.clone();
            let res_y       = self.res_y.clone();
            let dest_x      = self.dest_x.clone();
            let dest_y      = self.dest_y.clone();
            let item_w      = self.item_w.clone();
            let item_h      = self.item_h.clone();
            let item_cnt_w  = self.item_cnt_w.clone();
            let item_cnt    = self.item_cnt.clone();
            let dest_cnt_w  = self.dest_cnt_w.clone();
            if !is_end.load(Ordering::Relaxed) {
                match event {
                    km_libs::KeyCode::ShiftRight => {
                        match step.load(Ordering::Relaxed) {
                            1 => {
                                // 道具來源處
                                let (x, y) = km_libs::MouseEvent::get_mouse_position0();
                                res_x.store(x, Ordering::Relaxed);
                                res_y.store(y, Ordering::Relaxed);

                                km_libs::sleep(100);
                                Tools::line_text_clear();
                                km_libs::sleep(100);
                                km_libs::past_text(format!("道具位置為 x: {}, y: {}, 請移動到道具目標處, 並按下 right shift 進行下一步 :: ", x, y));
                                step.store(2, Ordering::Relaxed);
                            }
                            2 => {
                                // 道具目標處
                                let (x, y) = km_libs::MouseEvent::get_mouse_position0();
                                dest_x.store(x, Ordering::Relaxed);
                                dest_y.store(y, Ordering::Relaxed);

                                km_libs::sleep(100);
                                Tools::line_text_clear();
                                km_libs::sleep(100);
                                km_libs::past_text(format!("道具目標為 x: {}, y: {}, 輸入道具寬 (cursor 在數字前, right shift 下一步) :: ", x, y));
                                step.store(3, Ordering::Relaxed);
                            }
                            3 => {
                                // 道具寬
                                km_libs::sleep(100);
                                let _item_w = km_libs::cp_text_line_to_end();

                                match _item_w.parse::<i32>() {
                                    Ok(_int_item_w) => {
                                        item_w.store(_int_item_w, Ordering::Relaxed);
                                        km_libs::sleep(100);
                                        Tools::line_text_clear();
                                        km_libs::sleep(100);
                                        km_libs::past_text(format!("道具寬為 {}, 輸入道具高 (cursor 在數字前, right shift 下一步) :: ", _item_w));

                                        step.store(4, Ordering::Relaxed);
                                    }
                                    _ => {
                                        return ItemMoveToState::error_rtn(is_end.clone());
                                    }
                                }
                            }
                            4 => {
                                // 道具高
                                km_libs::sleep(100);
                                let _item_h = km_libs::cp_text_line_to_end();

                                match _item_h.parse::<i32>() {
                                    Ok(_int_item_h) => {
                                        item_h.store(_int_item_h, Ordering::Relaxed);
                                        km_libs::sleep(100);
                                        Tools::line_text_clear();
                                        km_libs::sleep(100);
                                        km_libs::past_text(format!("道具高為 {}, 請輸入道具數 (cursor 在數字前, right shift 下一步) :: ", _item_h));

                                        step.store(5, Ordering::Relaxed);
                                    }
                                    _ => {
                                        return ItemMoveToState::error_rtn(is_end.clone());
                                    }
                                }
                            }
                            5 => {
                                // 道具數
                                km_libs::sleep(100);
                                let _item_cnt = km_libs::cp_text_line_to_end();

                                match _item_cnt.parse::<i32>() {
                                    Ok(_int_item_cnt) => {
                                        item_cnt.store(_int_item_cnt, Ordering::Relaxed);
                                        km_libs::sleep(100);
                                        Tools::line_text_clear();
                                        km_libs::sleep(100);
                                        km_libs::past_text(format!("道具數為 {}, 請輸入道具寬數 (cursor 在數字前, right shift 下一步) :: ", _item_cnt));

                                        step.store(6, Ordering::Relaxed);
                                    }
                                    _ => {
                                        return ItemMoveToState::error_rtn(is_end.clone());
                                    }
                                }
                            }
                            6 => {
                                // 道具寬數
                                km_libs::sleep(100);
                                let _item_cnt_w = km_libs::cp_text_line_to_end();

                                match _item_cnt_w.parse::<i32>() {
                                    Ok(_int_item_cnt_w) => {
                                        item_cnt_w.store(_int_item_cnt_w, Ordering::Relaxed);
                                        km_libs::sleep(100);
                                        Tools::line_text_clear();
                                        km_libs::sleep(100);
                                        km_libs::past_text(format!("道具寬數為 {}, 請輸入目標寬數 (cursor 在數字前, right shift 下一步) :: ", _item_cnt_w));

                                        step.store(7, Ordering::Relaxed);
                                    }
                                    _ => {
                                        return ItemMoveToState::error_rtn(is_end.clone());
                                    }
                                }
                            }
                            7 => {
                                // 目標寬數
                                km_libs::sleep(100);
                                let _dest_cnt_w = km_libs::cp_text_line_to_end();

                                match _dest_cnt_w.parse::<i32>() {
                                    Ok(_int_dest_cnt_w) => {
                                        dest_cnt_w.store(_int_dest_cnt_w, Ordering::Relaxed);
                                        km_libs::sleep(100);
                                        Tools::line_text_clear();
                                        km_libs::sleep(100);
                                        //km_libs::past_text(format!("目標寬數為 {} (right shift 下一步) :: ", _dest_cnt_w));
                                        km_libs::past_text(format!("來源 {}, {}, 目標 {}, {}, 數量: {}, w: {}, h: {}, wc: {}, dwc: {} (right shift 下一步)", 
                                            res_x.load(Ordering::Relaxed),
                                            res_y.load(Ordering::Relaxed),
                                            dest_x.load(Ordering::Relaxed),
                                            dest_y.load(Ordering::Relaxed),
                                            item_cnt.load(Ordering::Relaxed),
                                            item_w.load(Ordering::Relaxed),
                                            item_h.load(Ordering::Relaxed),
                                            item_cnt_w.load(Ordering::Relaxed),
                                            dest_cnt_w.load(Ordering::Relaxed)));

                                        step.store(8, Ordering::Relaxed);
                                    }
                                    _ => {
                                        return ItemMoveToState::error_rtn(is_end.clone());
                                    }
                                }
                            }
                            8 => {
                                is_end.store(true, Ordering::Relaxed);

                                // 開始移動
                                let base = item_dbase1.load(Ordering::Relaxed);
                                let cnt = item_cnt.load(Ordering::Relaxed);
                                let item_w = item_w.load(Ordering::Relaxed);
                                let item_h = item_h.load(Ordering::Relaxed);
                                let item_cnt_w = item_cnt_w.load(Ordering::Relaxed);
                                let dest_cnt_w = dest_cnt_w.load(Ordering::Relaxed);

                                let __res_x = res_x.load(Ordering::Relaxed);
                                let __res_y = res_y.load(Ordering::Relaxed);
                                let __dest_x = dest_x.load(Ordering::Relaxed);
                                let __dest_y = dest_y.load(Ordering::Relaxed);

                                let mut c_res_x = __res_x;
                                let mut c_res_y = __res_y;
                                let mut c_res_row = 0;
                                let mut c_res_col = 0;

                                let mut c_dest_x = __dest_x;
                                let mut c_dest_y = __dest_y;
                                let mut c_dest_row = 0;
                                let mut c_dest_col = 0;

                                for idx in 0..cnt {
                                    km_libs::sleep(350);
                                    //println!("f x: {}, y: {}", c_res_x, c_res_y);
                                    let (dx, dy) = km_libs::convert_to_absolute(c_res_x, c_res_y);
                                    km_libs::MouseEvent::Move{x: dx, y: dy}.do_it();
                                    km_libs::sleep(80);
                                    km_libs::MouseEvent::click(dx, dy);

                                    km_libs::sleep(150);
                                    //println!("t x: {}, y: {}", c_dest_x, c_dest_y);
                                    let (dx, dy) = km_libs::convert_to_absolute(c_dest_x, c_dest_y);
                                    km_libs::MouseEvent::Move{x: dx, y: dy}.do_it();
                                    km_libs::sleep(80);
                                    km_libs::MouseEvent::click(dx, dy);

                                    c_res_col += 1;
                                    c_dest_col += 1;
                                    if c_res_col >= item_cnt_w {
                                        c_res_row += 1;
                                        c_res_col = 0;
                                    }
                                    if c_dest_col >= dest_cnt_w {
                                        c_dest_row += 1;
                                        c_dest_col = 0;
                                    }
                                    c_res_x = __res_x + (c_res_col * base * item_w);
                                    c_res_y = __res_y + (c_res_row * base * item_h);
                                    c_dest_x = __dest_x + (c_dest_col * base * item_w);
                                    c_dest_y = __dest_y + (c_dest_row * base * item_h);
                                }

                                km_libs::sleep(150);
                                return (Arc::new(WaitingState::new()), EventHandleReturn::CONTINUE);

                            }
                            _ => {
                                is_end.store(true, Ordering::Relaxed);
                                Tools::line_text_clear();
                                km_libs::past_text("未知的狀態... 3 秒後退出 批次道具移動...");
                                km_libs::sleep(3000);
                                Tools::line_text_clear();
                                km_libs::KeyCode::Return.click();
                                return (Arc::new(WaitingState::new()), EventHandleReturn::CONTINUE);
                            }
                        }
                    }
                    km_libs::KeyCode::Escape => {
                        is_end.store(true, Ordering::Relaxed);
                        Tools::line_text_clear();
                        km_libs::past_text("3 秒後退出 批次道具移動...");
                        km_libs::sleep(3000);
                        Tools::line_text_clear();
                        km_libs::KeyCode::Return.click();
                        return (Arc::new(WaitingState::new()), EventHandleReturn::CONTINUE);
                    }
                    _ => { }
                }
            }
            (self.clone(), EventHandleReturn::CONTINUE)
        }
    }

    /* --- 戰鬥狀態 ---
     * Alt + 1~5 ==> 6~0
     * Alt + F1 ==> -
     * Alt + F2 ==> =
     * Alt + F3 ==> 3152
     * Alt + F4 ==> 5231
     * Alt + QWER ==> F5~F8
     * Alt + ASDF ==> F9~F12)
     */
    pub struct FingingState {
        skip_enter: sync::Arc<AtomicBool>,
        alt_btn: sync::Arc<AtomicBool>,
        press: sync::Arc<HashMap<String,AtomicBool>>,
    }
    impl FingingState {
        fn new(skip_enter: bool) -> FingingState {
            let mut press: HashMap<String,AtomicBool> = HashMap::new();
            press.insert(String::from("Num1"), AtomicBool::new(false));
            press.insert(String::from("Num2"), AtomicBool::new(false));
            press.insert(String::from("Num3"), AtomicBool::new(false));
            press.insert(String::from("Num4"), AtomicBool::new(false));
            press.insert(String::from("Num5"), AtomicBool::new(false));
            press.insert(String::from("F1"), AtomicBool::new(false));
            press.insert(String::from("F2"), AtomicBool::new(false));
            press.insert(String::from("KeyQ"), AtomicBool::new(false));
            press.insert(String::from("KeyW"), AtomicBool::new(false));
            press.insert(String::from("KeyE"), AtomicBool::new(false));
            press.insert(String::from("KeyR"), AtomicBool::new(false));
            press.insert(String::from("KeyA"), AtomicBool::new(false));
            press.insert(String::from("KeyS"), AtomicBool::new(false));
            press.insert(String::from("KeyD"), AtomicBool::new(false));
            press.insert(String::from("KeyF"), AtomicBool::new(false));
            FingingState {
                skip_enter: sync::Arc::new(AtomicBool::new(skip_enter)),
                alt_btn: sync::Arc::new(AtomicBool::new(false)),
                press: sync::Arc::new(press),
            }
        }
    }
    impl State for FingingState {
        #[allow(unused_variables)]
        fn enter(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {
            println!("戰鬥狀態開始..., Right Ctrl 回到 WaitingState");
            let skip_enter = self.skip_enter.clone();
            if skip_enter.load(Ordering::Relaxed) {
                return;
            }
            // 若下面的對話框開著, 則戰鬥時按下 shift 會沒作用, 故需關閉下方對話框
            km_libs::past_text("戰鬥狀態開始..., 0.5 秒後關閉此對話框, Right Ctrl 回到 WaitingState");
            km_libs::sleep(500);
            km_libs::KeyCode::End.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.down();
            km_libs::sleep(100);
            km_libs::KeyCode::Home.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.up();
            km_libs::sleep(100);
            km_libs::KeyCode::Backspace.click();
            km_libs::sleep(100);
            km_libs::KeyCode::Return.click();
        }
        #[allow(unused_variables)]
        fn out(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {
            km_libs::sleep(50);
            km_libs::KeyCode::Return.click();
            km_libs::sleep(100);
            km_libs::KeyCode::End.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.down();
            km_libs::sleep(100);
            km_libs::KeyCode::Home.click();
            km_libs::sleep(100);
            km_libs::KeyCode::ShiftLeft.up();
            km_libs::sleep(100);
            km_libs::KeyCode::Backspace.click();
            km_libs::sleep(100);
            km_libs::KeyCode::Return.click();
        }
        fn do_event_when_mute(self: Arc<Self>) -> (Arc<dyn State>, EventHandleReturn) { (self.clone(), EventHandleReturn::CONTINUE) }
        #[allow(unused_variables)]
        fn do_keyboard_down(self: Arc<Self>, event: km_libs::KeyCode, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) -> (Arc<dyn State>, EventHandleReturn) {
            let alt_btn = self.alt_btn.clone();
            let press = self.press.clone();
            const SLEEP_MINISEC: u64 = 20;
            match event {
                // Alt
                km_libs::KeyCode::Alt => {
                    if !alt_btn.load(Ordering::Relaxed) {
                        self.alt_btn.store(true, Ordering::Relaxed);
                    }
                    //return (self.clone(), EventHandleReturn::INTERCEPT);
                }
                // Alt + 1 -> 6
                km_libs::KeyCode::Num1 => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("Num1")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num6.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + 2 -> 7
                km_libs::KeyCode::Num2 => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("Num2")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num7.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + 3 -> 8
                km_libs::KeyCode::Num3 => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("Num3")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num8.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + 4 -> 9
                km_libs::KeyCode::Num4 => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("Num4")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num9.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + 5 -> 0
                km_libs::KeyCode::Num5 => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("Num5")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num0.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + F1 -> -
                km_libs::KeyCode::F1 => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("F1")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Minus.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + F2 -> =
                km_libs::KeyCode::F2 => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("F2")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Equal.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + F3 -> 3152
                km_libs::KeyCode::F3 => {
                    if alt_btn.load(Ordering::Relaxed) {
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num3.click();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num1.click();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num5.click();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num2.click();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + F4 -> 5231
                km_libs::KeyCode::F4 => {
                    if alt_btn.load(Ordering::Relaxed) {
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num5.click();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num2.click();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num3.click();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num1.click();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + Q -> F5
                km_libs::KeyCode::KeyQ => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("KeyQ")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F5.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + W -> F6
                km_libs::KeyCode::KeyW => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("KeyW")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F6.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + E -> F7
                km_libs::KeyCode::KeyE => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("KeyE")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F7.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + R -> F8
                km_libs::KeyCode::KeyR => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("KeyR")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F8.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + A -> F9
                km_libs::KeyCode::KeyA => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("KeyA")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F9.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + S -> F10
                km_libs::KeyCode::KeyS => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("KeyS")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F10.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + D -> F11
                km_libs::KeyCode::KeyD => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("KeyD")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F11.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                // Alt + F -> F12
                km_libs::KeyCode::KeyF => {
                    if alt_btn.load(Ordering::Relaxed) {
                        press[&String::from("KeyF")].store(true, Ordering::Relaxed);
                        tx.send(Box::new(move || {
                            km_libs::KeyCode::Alt.up();
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F12.down();
                            km_libs::sleep(SLEEP_MINISEC);
                            if alt_btn.load(Ordering::Relaxed) {
                                km_libs::KeyCode::Alt.down();
                            }
                        })).unwrap();
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                _ => { }
            }
            (self.clone(), EventHandleReturn::CONTINUE)
        }
        #[allow(unused_variables)]
        fn do_keyboard_up(self: Arc<Self>, event: km_libs::KeyCode, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) -> (Arc<dyn State>, EventHandleReturn) {
            let alt_btn = self.alt_btn.clone();
            let press = self.press.clone();
            const SLEEP_MINISEC: u64 = 50;
            match event {
                km_libs::KeyCode::ControlRight => {
                    return (Arc::new(WaitingState::new()), EventHandleReturn::CONTINUE);
                }
                km_libs::KeyCode::Alt => {
                    tx.send(Box::new(move || {
                        alt_btn.store(false, Ordering::Relaxed);
                        km_libs::KeyCode::Alt.up();
                    })).unwrap();
                }
                km_libs::KeyCode::Num1 => {
                    if press[&String::from("Num1")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num6.up();
                        })).unwrap();
                        press[&String::from("Num1")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::Num2 => {
                    if press[&String::from("Num2")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num7.up();
                        })).unwrap();
                        press[&String::from("Num2")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::Num3 => {
                    if press[&String::from("Num3")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num8.up();
                        })).unwrap();
                        press[&String::from("Num3")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::Num4 => {
                    if press[&String::from("Num4")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num9.up();
                        })).unwrap();
                        press[&String::from("Num4")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::Num5 => {
                    if press[&String::from("Num5")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Num0.up();
                        })).unwrap();
                        press[&String::from("Num5")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::F1 => {
                    if press[&String::from("F1")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Minus.up();
                        })).unwrap();
                        press[&String::from("F1")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::F2 => {
                    if press[&String::from("F2")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::Equal.up();
                        })).unwrap();
                        press[&String::from("F2")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::KeyQ => {
                    if press[&String::from("KeyQ")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F5.up();
                        })).unwrap();
                        press[&String::from("KeyQ")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::KeyW => {
                    if press[&String::from("KeyW")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F6.up();
                        })).unwrap();
                        press[&String::from("KeyW")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::KeyE => {
                    if press[&String::from("KeyE")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F7.up();
                        })).unwrap();
                        press[&String::from("KeyE")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::KeyR => {
                    if press[&String::from("KeyR")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F8.up();
                        })).unwrap();
                        press[&String::from("KeyR")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::KeyA => {
                    if press[&String::from("KeyA")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F9.up();
                        })).unwrap();
                        press[&String::from("KeyA")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::KeyS => {
                    if press[&String::from("KeyS")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F10.up();
                        })).unwrap();
                        press[&String::from("KeyS")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::KeyD => {
                    if press[&String::from("KeyD")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F11.up();
                        })).unwrap();
                        press[&String::from("KeyD")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                km_libs::KeyCode::KeyF => {
                    if press[&String::from("KeyF")].load(Ordering::Relaxed) {
                        tx.send(Box::new(|| {
                            km_libs::sleep(SLEEP_MINISEC);
                            km_libs::KeyCode::F12.up();
                        })).unwrap();
                        press[&String::from("KeyF")].store(false, Ordering::Relaxed);
                        return (self.clone(), EventHandleReturn::INTERCEPT);
                    }
                }
                _ => { }
            }
            (self.clone(), EventHandleReturn::CONTINUE)
        }
    }

    // --- 離開 ---
    pub struct ExitState {}
    impl ExitState {
        fn new() -> ExitState { ExitState {} }
    }
    impl State for ExitState {
        #[allow(unused_variables)]
        fn enter(self: Arc<Self>, tx: mpsc::Sender<Box<dyn FnOnce() + Send>>) {
            println!("程式已退出");
            km_libs::past_text("程式已退出");
            km_libs::exit();
        }
        fn do_event_when_mute(self: Arc<Self>) -> (Arc<dyn State>, EventHandleReturn) { (self.clone(), EventHandleReturn::CONTINUE) }
    }

    // --- Context ---
    pub struct Context {
        state: Arc<dyn State>,
        tx: mpsc::Sender<Box<dyn FnOnce() + Send>>,
    }
    impl Context {
        pub fn new() -> Context {
            let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
            thread::spawn(move || {
                loop {
                    let result = rx.recv();
                    match result {
                        Ok(action) => {
                            action();
                        },
                        Err(_err) => {
                            eprintln!("thread error ...");
                        },
                    }
                }
            });

            // 預設的模式
          //let init_state = Arc::new(WaitingState::new()); // 等待模式
            let init_state = Arc::new(FingingState::new(true)); // 戰鬥模式
            init_state.clone().enter(tx.clone());
            Context { state: init_state.clone(), tx: tx }
        }
        
        pub fn event_callback(&mut self, event: km_libs::Event) -> Option<isize> {
            let state = Arc::clone(&self.state);
            let (next_state, evt_hdl_rtn) = state.do_event(event, self.tx.clone());
            self.state_change(next_state);
            match evt_hdl_rtn {
                EventHandleReturn::INTERCEPT => Some(1), // 欄截系統事件, 不向程式發送
                EventHandleReturn::CONTINUE => None,
            }
        }
    
        fn state_change(&mut self, next_state: Arc<dyn State>) {
            if !Arc::ptr_eq(&self.state, &next_state) {
                self.state.clone().out(self.tx.clone());
                self.state = next_state;
                self.state.clone().enter(self.tx.clone());
            }
        }
    }

    // --- 啟動 (listen 系統狀態) ---
    #[allow(dead_code)]
    pub fn listen() {
        let mut ctx = Context::new();
        unsafe {
            km_libs::EVENT_CALLBACK = Some(Box::new(move |event| {
                //println!("{:?}", event);
                ctx.event_callback(event)
            }));
        }
        // 可能不需要 thread
        // 兩個 fn listen_keyboard_event, listen_mouse_event 都在以下地方 停住
        // `while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {}`
        let handle01 = thread::spawn(|| {
            km_libs::listen_keyboard_event();
        });
        let handle02 = thread::spawn(|| {
            km_libs::listen_mouse_event();
        });
        handle01.join().unwrap(); // 等待執行緒結束
        handle02.join().unwrap(); // 等待執行緒結束
    }

    #[allow(dead_code)]
    pub fn test01_copy_to_end() {
        //println!("3秒後copy to end");
        //km_libs::sleep(3000);
        //let s = km_libs::cp_text_line_to_end();
        //println!("{s}");
    }

    #[allow(dead_code)]
    pub fn test02_mask_testing() {
        let simulate = 0x10;
        let flag = 16;
        println!("flag {} - {}", flag, (flag & simulate) == simulate);
        let flag = 144;
        println!("flag {} - {}", flag, (flag & simulate) == simulate);
    }

    #[allow(dead_code)]
    pub fn test02_test_simulate_detail() {
        let handle01 = thread::spawn(|| {
            km_libs::listen_keyboard_event();
        });
        let handle02 = thread::spawn(|| {
            km_libs::listen_mouse_event();
        });

        km_libs::sleep(500);
        km_libs::KeyCode::KeyA.click();
        km_libs::sleep(500);
        km_libs::KeyCode::ControlLeft.click();
        km_libs::sleep(500);
        km_libs::KeyCode::Home.click();
        km_libs::sleep(500);
        let (dx, dy) = km_libs::MouseEvent::get_mouse_position1();
        km_libs::MouseEvent::click(dx, dy);

        handle01.join().unwrap(); // 等待執行緒結束
        handle02.join().unwrap(); // 等待執行緒結束
    }

    #[allow(dead_code)]
    pub fn test03_get_mouse_info() {
        let (x, y) = km_libs::MouseEvent::get_mouse_position0();
        match sc_libs::get_rgb(x as u32, y as u32) {
            Some((r, g, b)) => {
                println!("座標 x: {}, y: {}, rgb: ({}, {}, {})", x, y, r, g, b);
            }
            _ => {}
        }
    }
}

fn main() {
    //檢視鍵盤事件
    //km_libs::listen_keyboard_event();

    //檢視滑鼠事件
    //km_libs::listen_mouse_event();
    
    //啟動監聽模式(for mabinogi)
    ctrl::listen();

    //ctrl::test01_copy_to_end();
    //ctrl::test02_mask_testing();
    //ctrl::test02_test_simulate_detail();
    //ctrl::test03_get_mouse_info();
}
