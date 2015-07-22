
extern crate winapi;
extern crate kernel32;
extern crate user32;

use generic_backend::GenericWindowBackend;
use super::super::{Visibility, WindowBorderStyle};
use std::ptr;
use std::ffi::OsStr;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::sync::{Once, ONCE_INIT};
use self::winapi::*;
use self::kernel32::{GetModuleHandleW};
use self::user32::{
    CreateWindowExW,
    DefWindowProcW,
    DestroyWindow,
    GetWindowLongW,
    RegisterClassExW,
    SetWindowTextW,
    ShowWindow,
};
use dynamin2d::{Point, Size};

const WINDOW_CLASS_NAME: &'static str = "DynaminWindowRust";

static REGISTER_WINDOW_CLASS: Once = ONCE_INIT;

pub struct WindowBackend {
    visibility: Visibility,
	handle: HWND,
	owner: HWND,
	text: String,
	location: Point,
	size: Size,
	border_style: WindowBorderStyle,
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

unsafe extern "system"
fn windowProc(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    DefWindowProcW(hwnd, uMsg, wParam, lParam)
}

impl WindowBackend {
	fn delete_handle(&mut self) {
	    if self.handle != ptr::null_mut() {
	        unsafe { DestroyWindow(self.handle); }
	        self.handle = ptr::null_mut();
	    }
	}

	fn window_styles(&self) -> (DWORD, DWORD) {
	    let (mut style, mut ex_style) = (0, 0);
		if(self.handle_created()) {
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
            //set_if(WS_THICKFRAME,
            //	resizable && self.border_style != WindowBorderStyle::None);
            set_if(WS_MINIMIZEBOX, self.border_style == WindowBorderStyle::Normal);
            //set_if(WS_MAXIMIZEBOX,
            //	self.border_style == WindowBorderStyle::Normal && resizable &&
            //	content.max_width == 0 && content.max_height == 0);
            set_if(WS_SYSMENU, self.border_style != WindowBorderStyle::None);
            if(self.border_style == WindowBorderStyle::Tool) {
                ex_style |= WS_EX_TOOLWINDOW;
            } else {
                ex_style &= !WS_EX_TOOLWINDOW;
            }
		}
        (style, ex_style)
	}

	fn recreate_handle(&mut self) {
	    REGISTER_WINDOW_CLASS.call_once(|| {
            unsafe {
                let mut class_name: Vec<u16> = OsStr::new(WINDOW_CLASS_NAME).encode_wide().collect();
                class_name.push(0);

                let mut wc: WNDCLASSEXW = mem::zeroed();
                wc.cbSize = mem::size_of::<WNDCLASSEXW>() as u32;
                wc.style = CS_DBLCLKS;
                wc.lpfnWndProc = Some(windowProc);
                wc.hInstance = GetModuleHandleW(ptr::null());
                wc.lpszClassName = class_name.as_ptr();

                assert!(RegisterClassExW(&wc) != 0);
            }
	    });

	    let (style, ex_style) = self.window_styles();
	    self.delete_handle();

	    let mut class_name: Vec<u16> = OsStr::new(WINDOW_CLASS_NAME).encode_wide().collect();
	    class_name.push(0);
	    let mut text: Vec<u16> = OsStr::new(&self.text).encode_wide().collect();
	    text.push(0);
	    unsafe {
            self.handle = CreateWindowExW(ex_style,
                                          class_name.as_ptr(),
                                          text.as_ptr(),
                                          style,
                                          self.location.x() as c_int,
                                          self.location.y() as c_int,
                                          self.size.width() as c_int,
                                          self.size.height() as c_int,
                                          self.owner,
                                          ptr::null_mut(),
                                          ptr::null_mut(),
                                          ptr::null_mut());
            assert!(self.handle != ptr::null_mut());
	    }
	}

	fn handle_created(&self) -> bool {
	    self.handle != ptr::null_mut()
	}

	fn handle(&mut self) -> HWND {
	    if self.handle_created() {
	        self.handle
	    } else {
	        self.recreate_handle();
	        self.handle
	    }
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
		    location: Point(0.0, 0.0),
		    size: Size(400.0, 300.0),
		    border_style: WindowBorderStyle::Normal,
		}
	}

    fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
        if self.handle_created() {
            unsafe {
                let wide = self.text.to_wide();
                SetWindowTextW(self.handle, wide.as_ptr());
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
            if self.handle_created() {
                unsafe { ShowWindow(self.handle, SW_HIDE); }
            }
        }
    }
    // enabling and disabling the close button can be done dynamically by enabling or disabling
    // the close menu item: http://blogs.msdn.com/b/oldnewthing/archive/2010/06/04/10019758.aspx
}
