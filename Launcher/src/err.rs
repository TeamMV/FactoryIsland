use std::iter;
use windows::core::{w, PCWSTR};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK};

pub fn error(msg: &str) {
    unsafe {
        let msg: Vec<u16> = msg.encode_utf16()
            .chain(iter::once(0))
            .collect();

        MessageBoxW(
            None,
            PCWSTR(msg.as_ptr()),
            w!("Error"),
            MB_OK | MB_ICONERROR
        );
    }
}