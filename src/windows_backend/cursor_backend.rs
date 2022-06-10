use std::cell::Cell;
use std::ptr;

use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::UI::WindowsAndMessaging::{HCURSOR, IMAGE_CURSOR, LR_SHARED, LR_DEFAULTSIZE, LoadImageW};
use windows::core::{PWSTR, PCWSTR};

use crate::cursor::Cursor;
use crate::generic_backend::GenericCursorBackend;

fn MAKEINTRESOURCE(i: u32) -> *mut u16 {
    i as u16 as usize as *mut u16
}

pub struct CursorBackend {
    handle: Cell<HCURSOR>,
}

impl CursorBackend {
    unsafe fn load_cursor(cur_resource: u32) -> Self {
        let hcur = HCURSOR(
            LoadImageW(
                HINSTANCE(0), PCWSTR(MAKEINTRESOURCE(cur_resource)),
                IMAGE_CURSOR,
                0, 0, LR_SHARED | LR_DEFAULTSIZE
            ).expect("LoadImageW() failed").0);
        Self {
            handle: Cell::new(hcur),
        }
    }
}

const OCR_NORMAL: u32 = 32512;
const OCR_IBEAM: u32 = 32513;

impl GenericCursorBackend for CursorBackend {
    fn none() -> Self {
        unsafe { Self { handle: Cell::new(HCURSOR(0)) } }
    }

    fn arrow() -> Self {
        unsafe { Self::load_cursor(OCR_NORMAL) }
    }

    fn text() -> Self {
        unsafe { Self::load_cursor(OCR_IBEAM) }
    }
}
