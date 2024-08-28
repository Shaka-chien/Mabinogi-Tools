use rdev::{
    Key as rdev_key,  // https://docs.rs/rdev/latest/rdev/enum.Key.html
    // Event as rdev_event,
};

#[allow(dead_code)]
fn sleep(millis: u64) {
    use std::{thread, time};

    let ten_millis = time::Duration::from_millis(millis);
    thread::sleep(ten_millis);
}

#[allow(dead_code)]
fn past_text<S: AsRef<str>>(msg: S) {
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

#[allow(dead_code)]
fn type_kb(key: rdev_key) {
    use rdev::{simulate, EventType};

    simulate(&EventType::KeyPress(key)).unwrap();
    simulate(&EventType::KeyRelease(key)).unwrap();
}

#[allow(dead_code)]
fn press_kb(key: rdev_key) {
    use rdev::{simulate, EventType};

    simulate(&EventType::KeyPress(key)).unwrap();
}

#[allow(dead_code)]
fn release_kb(key: rdev_key) {
    use rdev::{simulate, EventType};

    simulate(&EventType::KeyRelease(key)).unwrap();
}

#[allow(dead_code)]
fn move_ms(x: f64, y: f64) {
    use rdev::{simulate, EventType};

    simulate(&EventType::MouseMove { x: x, y: y }).unwrap();
}

#[allow(dead_code)]
fn elapsed_fn<F: Fn() -> ()>(cb: F) {
    use std::time::Instant;
    let start = Instant::now();
    cb();
    println!("耗時: {:?}", start.elapsed());
}

#[allow(dead_code)]
fn start_event01() {
    #[allow(dead_code)]
    trait State {
        fn enter(&self);
        fn out(&self);
        fn key_press(&self, key: rdev_key);
        fn key_release(&self, key: rdev_key);
        fn mouse_move(&self, x: f64, y: f64);
        fn mouse_button_press(&self, key: rdev_key);
        fn mouse_button_release(&self, key: rdev_key);
    }
    #[allow(dead_code)]
    struct WaitingState {}
    impl State for WaitingState {
        fn enter(&self) {}
        fn out(&self) {}
        #[allow(unused_variables)]
        fn key_press(&self, key: rdev_key) {
            match key {
                rdev_key::ControlRight => {
                    type_kb(rdev_key::Return);
                    past_text("請輸入項目: 0: 測試");
                },
                rdev_key::Num1 => {
                }
                _ => {}
            }
        }
        #[allow(unused_variables)]
        fn key_release(&self, key: rdev_key) {}
        #[allow(unused_variables)]
        fn mouse_move(&self, x: f64, y: f64) {}
        #[allow(unused_variables)]
        fn mouse_button_press(&self, key: rdev_key) {}
        #[allow(unused_variables)]
        fn mouse_button_release(&self, key: rdev_key) {}
    }
}

fn main() {
    start_event01();
}
