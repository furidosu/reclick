use std::time::Duration;

use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use tfc::{Context, MouseButton, MouseContext};

fn main() {
    let d = DeviceState::new();
    let mut ml = vec![];
    println!("Press Q to start recording mouse clicks.");
    println!("Press Q again to stop recording.");
    println!("Qを押したらマウスクリックをレコードはじめます。");
    println!("Qをもう一度押したらレコード終了します。");
    while !d.get_keys().contains(&Keycode::Q) {
        ::std::thread::sleep(Duration::from_millis(1000 / 20));
    }
    ::std::thread::sleep(Duration::from_millis(1000 / 2));
    loop {
        if d.get_keys().contains(&Keycode::Q) {
            break;
        }
        let m = d.get_mouse();
        println!("{:?}", m);
        ml.push(m);
        ::std::thread::sleep(Duration::from_millis(1000 / 50));
    }
    ::std::thread::sleep(Duration::from_millis(1000 / 2));
    let mut ctx = Context::new().unwrap();
    // For OS-specific reasons, it's necessary to wait a moment after
    // creating the context before generating events.
    ::std::thread::sleep(Duration::from_millis(10));
    loop {
        println!("Press Q to replay the mouse recording.");
        println!("Qを押したらマウスクリックレコーディングをリプレイします。");
        println!("Press Ctrl+C or close the window to exit.");
        println!("Ctrl+Cを押すかウィンドウ閉じたら終了です。");
        while !d.get_keys().contains(&Keycode::Q) {
            ::std::thread::sleep(Duration::from_millis(1000 / 20));
        }
        let mut is_down = false;
        for MouseState {
            coords: (x, y),
            button_pressed,
        } in ml.iter()
        {
            ctx.mouse_move_abs(*x, *y).unwrap();
            if button_pressed[1] && !is_down {
                ctx.mouse_down(MouseButton::Left).unwrap();
                is_down = true;
            } else if !button_pressed[1] && is_down {
                ctx.mouse_up(MouseButton::Left).unwrap();
                is_down = false;
            }
            ::std::thread::sleep(Duration::from_millis(1000 / 50));
        }
    }
}
