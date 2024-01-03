use device_query::{DeviceQuery, DeviceState, Keycode};
use std::process::Command;

fn main() {

    const FIRST_KEY: Keycode = Keycode::Minus;
    const SECOND_KEY: Keycode = Keycode::Equal;
    const THIRD_KEY: Keycode = Keycode::Enter;

    const PENDING_CLOSING_APP: [&str; 2] = ["java.exe", "javaw.exe"];

    let device_state = DeviceState::new();

    println!("Pending... The keys is {}, {}, {}", FIRST_KEY, SECOND_KEY, THIRD_KEY);
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if !keys.is_empty() {
            if keys.contains(&FIRST_KEY) && keys.contains(&SECOND_KEY) && keys.contains(&THIRD_KEY) {
                break
            }
        }
    }

    for app in PENDING_CLOSING_APP {
        let output = {
            Command::new("taskkill").arg("/f").arg("/im").arg(app).output().expect("Something wrong")
        };
        output.stdout;
    }
}