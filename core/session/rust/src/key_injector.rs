#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS,
    KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, VIRTUAL_KEY, VK_BACK, VK_CONTROL, VK_DOWN,
    VK_ESCAPE, VK_LEFT, VK_RETURN, VK_RIGHT, VK_SHIFT, VK_SPACE, VK_TAB, VK_UP,
};

#[cfg(windows)]
pub fn inject_text(text: &str) {
    for ch in text.chars() {
        let mut utf16_buf = [0u16; 2];
        let encoded = ch.encode_utf16(&mut utf16_buf);
        for &mut code_unit in encoded {
            let down = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: code_unit,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            let up = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: code_unit,
                        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            unsafe {
                SendInput(&[down, up], std::mem::size_of::<INPUT>() as i32);
            }
        }
    }
}

#[cfg(windows)]
fn send_vk(vk: VIRTUAL_KEY) {
    let down = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    let up = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe {
        SendInput(&[down, up], std::mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(windows)]
fn send_chord(mod_vk: VIRTUAL_KEY, key_vk: VIRTUAL_KEY) {
    let mod_down = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: mod_vk,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    let key_down = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key_vk,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    let key_up = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key_vk,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    let mod_up = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: mod_vk,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe {
        SendInput(&[mod_down, key_down, key_up, mod_up], std::mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(windows)]
pub fn inject_key_code(code: &str) {
    match code {
        "Backspace" => send_vk(VK_BACK),
        "Enter" => send_vk(VK_RETURN),
        "Space" => send_vk(VK_SPACE),
        "Tab" => send_vk(VK_TAB),
        "Escape" | "Esc" => send_vk(VK_ESCAPE),
        "Up" => send_vk(VK_UP),
        "Down" => send_vk(VK_DOWN),
        "Left" => send_vk(VK_LEFT),
        "Right" => send_vk(VK_RIGHT),
        "Ctrl" => send_vk(VK_CONTROL),
        "Shift" => send_vk(VK_SHIFT),
        "Ctrl+A" => send_chord(VK_CONTROL, VIRTUAL_KEY(0x41)),
        "Ctrl+C" => send_chord(VK_CONTROL, VIRTUAL_KEY(0x43)),
        "Ctrl+V" => send_chord(VK_CONTROL, VIRTUAL_KEY(0x56)),
        "Ctrl+X" => send_chord(VK_CONTROL, VIRTUAL_KEY(0x58)),
        _ => {
            inject_text(code);
        }
    }
}

pub fn inject_key_input(text: Option<&str>, key_code: Option<&str>) {
    #[cfg(windows)]
    {
        if let Some(code) = key_code {
            inject_key_code(code);
        } else if let Some(txt) = text {
            inject_text(txt);
        }
    }
    #[cfg(not(windows))]
    {
        let _ = (text, key_code);
    }
}
