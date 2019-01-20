
extern crate winapi;

use generic_backend::GenericWindowBackend;
use super::super::{Visibility, WindowBorderStyle};
use std::ptr;
use std::ffi::OsStr;
use std::mem;
use std::os::raw::c_int;
use std::os::windows::ffi::OsStrExt;
use std::sync::{Once, ONCE_INIT};
use self::winapi::shared::{
    minwindef::*,
    ntdef::*,
    windef::*,
};
use self::winapi::um::{
    libloaderapi::*,
    winuser::*,
};

use smallvec::SmallVec;
use zaffre::{Point2, Size2};

use super::str_to_wide_vec;

const WINDOW_CLASS_NAME: &'static str = "DynaminWindowRust";

static REGISTER_WINDOW_CLASS: Once = ONCE_INIT;

pub struct WindowBackend {
    visibility: Visibility,
    handle: HWND,
    owner: HWND,
    text: String,
    location: Point2<f64>,
    size: Size2<f64>,
    border_style: WindowBorderStyle,
    resizable: bool,
}

trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
}

impl ToWide for str {
    fn to_wide(&self) -> Vec<u16> {
        let mut wide: Vec<u16> = OsStr::new(self).encode_wide().collect();
        wide.push(0);
        wide
    }
}

#[allow(non_snake_case)]
unsafe extern "system"
fn windowProc(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    DefWindowProcW(hwnd, uMsg, wParam, lParam)
}

impl WindowBackend {
    fn delete_handle(&mut self) {
        if !self.handle.is_null() {
            unsafe { DestroyWindow(self.handle); }
            self.handle = ptr::null_mut();
        }
    }

    fn window_styles(&self) -> (DWORD, DWORD) {
        let (mut style, mut ex_style) = (0, 0);
        if self.is_handle_created() {
            unsafe {
                style = GetWindowLongW(self.handle, GWL_STYLE) as DWORD;
                ex_style = GetWindowLongW(self.handle, GWL_EXSTYLE) as DWORD;
            }
        }
        {
            let mut set_if = |s: DWORD, b: bool| {
                // if condition satisfied, add style, otherwise clear style
                if b { (style |= s) } else { (style &= !s) }
            };
            set_if(WS_DLGFRAME, self.border_style != WindowBorderStyle::None);
            set_if(WS_BORDER, self.border_style != WindowBorderStyle::None);
            set_if(WS_THICKFRAME,
                self.resizable && self.border_style != WindowBorderStyle::None);
            set_if(WS_MINIMIZEBOX, self.border_style == WindowBorderStyle::Normal);
            //set_if(WS_MAXIMIZEBOX,
            //    self.border_style == WindowBorderStyle::Normal && self.resizable &&
            //    content.max_width == 0 && content.max_height == 0);
            set_if(WS_SYSMENU, self.border_style != WindowBorderStyle::None);
            if self.border_style == WindowBorderStyle::Tool {
                ex_style |= WS_EX_TOOLWINDOW;
            } else {
                ex_style &= !WS_EX_TOOLWINDOW;
            }
        }
        (style, ex_style)
    }

    fn update_window_styles(&self) {
        if !self.is_handle_created() {
            return;
        }
        let (mut style, mut ex_style) = self.window_styles();
        unsafe {
            SetWindowLongW(self.handle, GWL_STYLE, style as LONG);
            SetWindowLongW(self.handle, GWL_EXSTYLE, ex_style as LONG);
            SetWindowPos(self.handle, ptr::null_mut(), 0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
        }
    }

    fn recreate_handle(&mut self) {
        REGISTER_WINDOW_CLASS.call_once(|| {
            unsafe {
                let mut class_name_buf = SmallVec::<[u16; 32]>::new();
                let wide_class_name = str_to_wide_vec(WINDOW_CLASS_NAME, &mut class_name_buf);

                let mut wc: WNDCLASSEXW = mem::zeroed();
                wc.cbSize = mem::size_of::<WNDCLASSEXW>() as u32;
                wc.style = CS_DBLCLKS;
                wc.lpfnWndProc = Some(windowProc);
                wc.hInstance = GetModuleHandleW(ptr::null());
                wc.lpszClassName = wide_class_name;

                assert!(RegisterClassExW(&wc) != 0);
            }
        });

        let (style, ex_style) = self.window_styles();
        self.delete_handle();

        let mut class_name_buf = SmallVec::<[u16; 32]>::new();
        let wide_class_name = str_to_wide_vec(WINDOW_CLASS_NAME, &mut class_name_buf);
        let mut text_buf = SmallVec::<[u16; 64]>::new();
        let wide_text = str_to_wide_vec(&self.text, &mut text_buf);
        unsafe {
            self.handle = CreateWindowExW(
                ex_style,
                wide_class_name,
                wide_text,
                style,
                self.location.x as c_int,
                self.location.y as c_int,
                self.size.width as c_int,
                self.size.height as c_int,
                self.owner,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            assert!(self.handle != ptr::null_mut());
        }
    }

    fn is_handle_created(&self) -> bool {
        !self.handle.is_null()
    }

    fn handle(&mut self) -> HWND {
        if !self.is_handle_created() {
            self.recreate_handle();
        }
        self.handle
    }
}

impl Drop for WindowBackend {
    fn drop(&mut self) {
        self.delete_handle();
    }
}

impl GenericWindowBackend for WindowBackend {
    fn new() -> Self {
        WindowBackend {
            visibility: Visibility::Gone,
            handle: ptr::null_mut(),
            owner: ptr::null_mut(),
            text: "".to_string(),
            location: Point2::new(0.0, 0.0),
            size: Size2::new(400.0, 300.0),
            border_style: WindowBorderStyle::Normal,
            resizable: true,
        }
    }

    fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
        if self.is_handle_created() {
            unsafe {
                let mut text_buf = SmallVec::<[u16; 64]>::new();
                let wide_text = str_to_wide_vec(text, &mut text_buf);
                SetWindowTextW(self.handle, wide_text);
            }
        }
    }

    fn visibility(&self) -> Visibility {
        self.visibility
    }

    fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
        if self.visibility == Visibility::Visible {
            // TODO: this isn't how I did it in D
            unsafe { ShowWindow(self.handle(), SW_SHOW); }
        } else {
            if self.is_handle_created() {
                unsafe { ShowWindow(self.handle, SW_HIDE); }
            }
        }
    }

    fn resizable(&self) -> bool {
        self.resizable
    }

    fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
        self.update_window_styles();
    }
    // enabling and disabling the close button can be done dynamically by enabling or disabling
    // the close menu item: http://blogs.msdn.com/b/oldnewthing/archive/2010/06/04/10019758.aspx
}
