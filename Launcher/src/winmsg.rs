use std::iter;
use windows::core::{w, PCWSTR};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, IDYES, MB_ICONERROR, MB_ICONQUESTION, MB_OK, MB_YESNO};

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

pub fn ask_yes_no(msg: &str) -> bool {
    unsafe {
        let msg: Vec<u16> = msg.encode_utf16()
            .chain(iter::once(0))
            .collect();

        let title: Vec<u16> = "Confirm".encode_utf16()
            .chain(iter::once(0))
            .collect();

        let result = MessageBoxW(
            None,
            PCWSTR(msg.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_YESNO | MB_ICONQUESTION,
        );

        result == IDYES
    }
}