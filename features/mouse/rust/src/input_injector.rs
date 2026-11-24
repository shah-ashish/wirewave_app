#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_MIDDLEDOWN,
    MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_WHEEL, MOUSEINPUT, MOUSE_EVENT_FLAGS,
};

pub const BUTTON_LEFT: u8 = 1;
pub const BUTTON_RIGHT: u8 = 2;
pub const BUTTON_MIDDLE: u8 = 3;

#[cfg(windows)]
fn send_mouse_event(flags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, data: u32) {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx,
                dy,
                mouseData: data,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn inject_relative_move(dx: i16, dy: i16) {
    #[cfg(windows)]
    send_mouse_event(MOUSEEVENTF_MOVE, dx as i32, dy as i32, 0);
    #[cfg(not(windows))]
    let _ = (dx, dy);
}

pub fn inject_button_down(button: u8) {
    #[cfg(windows)]
    {
        let flag = match button {
            BUTTON_LEFT => MOUSEEVENTF_LEFTDOWN,
            BUTTON_RIGHT => MOUSEEVENTF_RIGHTDOWN,
            BUTTON_MIDDLE => MOUSEEVENTF_MIDDLEDOWN,
            _ => return,
        };
        send_mouse_event(flag, 0, 0, 0);
    }
    #[cfg(not(windows))]
    let _ = button;
}

pub fn inject_button_up(button: u8) {
    #[cfg(windows)]
    {
        let flag = match button {
            BUTTON_LEFT => MOUSEEVENTF_LEFTUP,
            BUTTON_RIGHT => MOUSEEVENTF_RIGHTUP,
            BUTTON_MIDDLE => MOUSEEVENTF_MIDDLEUP,
            _ => return,
        };
        send_mouse_event(flag, 0, 0, 0);
    }
    #[cfg(not(windows))]
    let _ = button;
}

pub fn inject_click(button: u8) {
    inject_button_down(button);
    inject_button_up(button);
}

pub fn inject_scroll(delta: i16) {
    #[cfg(windows)]
    {
        // On Windows, WHEEL_DELTA is typically 120 per notch.
        // We multiply the incoming normalized scroll value by a standard factor.
        let wheel_delta = (delta as i32) * 12;
        send_mouse_event(MOUSEEVENTF_WHEEL, 0, 0, wheel_delta as u32);
    }
    #[cfg(not(windows))]
    let _ = delta;
}
