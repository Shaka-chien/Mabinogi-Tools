mod libs {
    use rdev::{
        Key as rdev_key,  // https://docs.rs/rdev/latest/rdev/enum.Key.html
    };

    // 休息
    #[allow(dead_code)]
    pub fn sleep(millis: u64) {
        use std::{thread, time};

        let ten_millis = time::Duration::from_millis(millis);
        thread::sleep(ten_millis);
    }

    // 貼上
    #[allow(dead_code)]
    pub fn past_text<S: AsRef<str>>(msg: S) {
        let msg = String::from(msg.as_ref());
        // 剪貼簿 library
        extern crate clipboard;
        use clipboard::ClipboardProvider;
        use clipboard::ClipboardContext;

        // 滑鼠+鍵盤 事件&控制 library
        use rdev::{simulate, EventType};

        // 將文字放倒剪貼簿中
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(msg).unwrap();

        // 模擬按下 Ctrl + V (貼上)
        simulate(&EventType::KeyPress(rdev_key::ControlLeft)).unwrap();
        simulate(&EventType::KeyPress(rdev_key::KeyV)).unwrap();
        simulate(&EventType::KeyRelease(rdev_key::KeyV)).unwrap();
        simulate(&EventType::KeyRelease(rdev_key::ControlLeft)).unwrap();
    }

    // 打鍵盤
    #[allow(dead_code)]
    pub fn type_kb(key: rdev_key) {
        use rdev::{simulate, EventType};

        simulate(&EventType::KeyPress(key)).unwrap();
        simulate(&EventType::KeyRelease(key)).unwrap();
    }

    // 按下鍵盤
    #[allow(dead_code)]
    pub fn press_kb(key: rdev_key) {
        use rdev::{simulate, EventType};

        simulate(&EventType::KeyPress(key)).unwrap();
    }

    // 放開鍵盤
    #[allow(dead_code)]
    pub fn release_kb(key: rdev_key) {
        use rdev::{simulate, EventType};

        simulate(&EventType::KeyRelease(key)).unwrap();
    }

    // 滑鼠移動
    #[allow(dead_code)]
    pub fn move_ms(x: f64, y: f64) {
        use rdev::{simulate, EventType};

        simulate(&EventType::MouseMove { x: x, y: y }).unwrap();
    }

    // 滑鼠click
    #[allow(dead_code)]
    pub fn click_ms() {
        use rdev::{simulate, Button, EventType};

        simulate(&EventType::ButtonPress(Button::Left)).unwrap();
        simulate(&EventType::ButtonRelease(Button::Left)).unwrap();
    }

    // 滑鼠左鍵按下
    #[allow(dead_code)]
    pub fn left_press_ms() {
        use rdev::{simulate, Button, EventType};

        simulate(&EventType::ButtonPress(Button::Left)).unwrap();
    }

    // 滑鼠左鍵放開
    #[allow(dead_code)]
    pub fn left_release_ms() {
        use rdev::{simulate, Button, EventType};

        simulate(&EventType::ButtonRelease(Button::Left)).unwrap();
    }

    // 滑鼠右鍵
    #[allow(dead_code)]
    pub fn rclick_ms() {
        use rdev::{simulate, Button, EventType};

        simulate(&EventType::ButtonPress(Button::Right)).unwrap();
        simulate(&EventType::ButtonRelease(Button::Right)).unwrap();
    }

    // 滑鼠右鍵按下
    #[allow(dead_code)]
    pub fn right_press_ms() {
        use rdev::{simulate, Button, EventType};

        simulate(&EventType::ButtonPress(Button::Right)).unwrap();
    }

    // 滑鼠右鍵放開
    #[allow(dead_code)]
    pub fn right_release_ms() {
        use rdev::{simulate, Button, EventType};

        simulate(&EventType::ButtonRelease(Button::Right)).unwrap();
    }

    // 查 type
    #[allow(dead_code)]
    pub fn type_of<T>(_: &T) -> String {
        return String::from(std::any::type_name::<T>());
    }

    // 計算耗時
    #[allow(dead_code)]
    pub fn elapsed_fn<F: Fn() -> ()>(cb: F) {
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
    use rdev::{
        Key as rdev_key,    // https://docs.rs/rdev/latest/rdev/enum.Key.html
        Button as rdev_btn, // https://docs.rs/rdev/latest/rdev/enum.Button.html
    };
    use crate::libs;

    //#[derive(Copy, Clone)]
    pub enum State {
        Waiting,
        Hello,
    }

    #[allow(dead_code)]
    trait Action {
        #[allow(unused_variables)]
        fn enter(&mut self) {}
        #[allow(unused_variables)]
        fn out(&mut self) {}

        #[allow(unused_variables)]
        fn key_press(&mut self, key: rdev_key) -> Option<State> { None }
        #[allow(unused_variables)]
        fn key_release(&mut self, key: rdev_key) -> Option<State> { None }
        #[allow(unused_variables)]
        fn mouse_move(&mut self, x: f64, y: f64) -> Option<State> { None }
        #[allow(unused_variables)]
        fn mouse_button_press(&mut self, button: rdev_btn) -> Option<State> { None }
        #[allow(unused_variables)]
        fn mouse_button_release(&mut self, button: rdev_btn) -> Option<State> { None }
    }

    //#[derive(Copy, Clone)]
    struct ActionWaiting { flag1: bool }
    impl Default for ActionWaiting {
        fn default() -> ActionWaiting {
            ActionWaiting {
                flag1: false,
            }
        }
    }
    impl Action for ActionWaiting {
        fn key_press(&mut self, key: rdev_key) -> Option<State> {
            match key {
                rdev_key::ControlRight => {
                    if !self.flag1 {
                        self.flag1 = true;
                        libs::type_kb(rdev_key::Return);
                        libs::past_text("請選擇 - h:hello");
                    }
                }
                rdev_key::KeyH => {
                    if self.flag1 {
                        libs::type_kb(rdev_key::End);
                        libs::press_kb(rdev_key::ShiftLeft);
                        libs::type_kb(rdev_key::Home);
                        libs::release_kb(rdev_key::ShiftLeft);
                        libs::type_kb(rdev_key::Backspace);
                        libs::type_kb(rdev_key::Return);

                        return Some(State::Hello);
                    }
                }
                _ => {}
            }
            None
        }
    }

    #[derive(Copy, Clone)]
    struct ActionHello;
    impl Action for ActionHello {
        #[allow(unused_variables)]
        fn enter(&mut self) {
            libs::type_kb(rdev_key::Return);
            libs::past_text("Hello 測試狀態 !!!");
            libs::exit();
        }
    }

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
        pub fn key_press(&mut self, key: rdev_key) {
            let state = self.action_mapping().key_press(key);
            self.change_state_if(state);
        }
        pub fn key_release(&mut self, key: rdev_key) {
            let state = self.action_mapping().key_release(key);
            self.change_state_if(state);
        }
        pub fn mouse_move(&mut self, x: f64, y: f64) {
            let state = self.action_mapping().mouse_move(x, y);
            self.change_state_if(state);
        }
        pub fn mouse_button_press(&mut self, button: rdev_btn) {
            let state = self.action_mapping().mouse_button_press(button);
            self.change_state_if(state);
        }
        pub fn mouse_button_release(&mut self, button: rdev_btn) {
            let state = self.action_mapping().mouse_button_press(button);
            self.change_state_if(state);
        }
    }
}

#[allow(dead_code)]
fn start_event01() {
    use rdev::{listen, Event};

    let mut ctx: pc_ctrl::Context = Default::default();
    let callback = move |event: Event| {
        match event.event_type {
            rdev::EventType::KeyPress(key) => {
                ctx.key_press(key);
            }
            rdev::EventType::KeyRelease(key) => {
                ctx.key_release(key);
            }
            rdev::EventType::MouseMove { x, y } => {
                ctx.mouse_move(x, y);
            }
            rdev::EventType::ButtonPress(button) => {
                ctx.mouse_button_press(button);
            }
            rdev::EventType::ButtonRelease(button) => {
                ctx.mouse_button_release(button);
            }
            _ => {}
        }
    };
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn main() {
    start_event01();
}
