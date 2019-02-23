
extern crate winapi;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ptr;
use std::ffi::OsStr;
use std::mem;
use std::os::raw::c_int;
use std::os::windows::ffi::OsStrExt;
use std::rc::{Rc, Weak};
use std::sync::{Once, ONCE_INIT};

use generic_backend::GenericWindowBackend;
use window::{WindowData, WindowEvent};
use super::super::{Visibility, WindowBorderStyle};

use smallvec::SmallVec;
use zaffre::{Point2, Size2};
use self::winapi::shared::{
    minwindef::*,
    ntdef::*,
    windef::*,
};
use self::winapi::um::{
    libloaderapi::*,
    winuser::*,
};


use super::str_to_wide_vec;

const WINDOW_CLASS_NAME: &'static str = "DynaminWindowRust";

static REGISTER_WINDOW_CLASS: Once = ONCE_INIT;

pub struct WindowBackend {
    window: Cell<Option<Weak<WindowData>>>,
    visibility: Cell<Visibility>,
    handle: Cell<HWND>,
    owner: Cell<HWND>,
    text: Cell<String>,
    location: Cell<Point2<f64>>,
    size: Cell<Size2<f64>>,
    border_style: Cell<WindowBorderStyle>,
    resizable: Cell<bool>,
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

// I considered using SetWindowLongPtr() to store window handles, but a HashMap is far faster
// than a system call.
thread_local! {
    static WINDOWS: RefCell<HashMap<HWND, Weak<WindowData>>> = RefCell::new(HashMap::new());
}

fn get_window(hwnd: HWND) -> Rc<WindowData> {
    // The Rust side object should exist as long as the native window because when the Rust object
    // is dropped, it destroys the native window. Thus, the unwrap() should be safe.
    WINDOWS.with(|windows| windows.borrow()[&hwnd].upgrade()).unwrap()
}

#[allow(non_snake_case)]
unsafe extern "system"
fn windowProc(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    match uMsg {
        WM_CLOSE => {
            let window = get_window(hwnd);
            let mut event = WindowEvent::Closing;
            window.event_handlers().send(&mut event);
            // TODO: get handle to window and send Closing event
            0
        }
        _ => DefWindowProcW(hwnd, uMsg, wParam, lParam)
    }
}

impl WindowBackend {
    fn delete_handle(&self) {
        if !self.handle.get().is_null() {
            WINDOWS.with(|windows| {
                let mut windows = windows.borrow_mut();
                windows.remove(&self.handle.get());
            });
            unsafe { DestroyWindow(self.handle.get()); }
            self.handle.set(ptr::null_mut());
        }
    }

    fn window_styles(&self) -> (DWORD, DWORD) {
        let (mut style, mut ex_style) = (0, 0);
        if self.is_handle_created() {
            unsafe {
                style = GetWindowLongW(self.handle.get(), GWL_STYLE) as DWORD;
                ex_style = GetWindowLongW(self.handle.get(), GWL_EXSTYLE) as DWORD;
            }
        }
        {
            let mut set_if = |s: DWORD, b: bool| {
                // if condition satisfied, add style, otherwise clear style
                if b { (style |= s) } else { (style &= !s) }
            };
            set_if(WS_DLGFRAME, self.border_style.get() != WindowBorderStyle::None);
            set_if(WS_BORDER, self.border_style.get() != WindowBorderStyle::None);
            set_if(WS_THICKFRAME,
                self.resizable.get() && self.border_style.get() != WindowBorderStyle::None);
            set_if(WS_MINIMIZEBOX, self.border_style.get() == WindowBorderStyle::Normal);
            //set_if(WS_MAXIMIZEBOX,
            //    self.border_style == WindowBorderStyle::Normal && self.resizable &&
            //    content.max_width == 0 && content.max_height == 0);
            set_if(WS_SYSMENU, self.border_style.get() != WindowBorderStyle::None);
            if self.border_style.get() == WindowBorderStyle::Tool {
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
            SetWindowLongW(self.handle.get(), GWL_STYLE, style as LONG);
            SetWindowLongW(self.handle.get(), GWL_EXSTYLE, ex_style as LONG);
            SetWindowPos(self.handle.get(), ptr::null_mut(), 0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
        }
    }

    fn recreate_handle(&self) {
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
        let text_temp = self.text.take();
        let wide_text = str_to_wide_vec(&text_temp, &mut text_buf);
        self.text.set(text_temp);
        unsafe {
            self.handle.set(CreateWindowExW(
                ex_style,
                wide_class_name,
                wide_text,
                style,
                self.location.get().x as c_int,
                self.location.get().y as c_int,
                self.size.get().width as c_int,
                self.size.get().height as c_int,
                self.owner.get(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            ));
            assert!(self.handle.get() != ptr::null_mut());
            WINDOWS.with(|windows| {
                let mut windows = windows.borrow_mut();
                let window = self.window.take();
                windows.insert(self.handle.get(), window.clone().unwrap());
                self.window.set(window);
            });
        }
    }

    fn is_handle_created(&self) -> bool {
        !self.handle.get().is_null()
    }

    fn handle(&self) -> HWND {
        if !self.is_handle_created() {
            self.recreate_handle();
        }
        self.handle.get()
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
            window: Cell::new(None),
            visibility: Cell::new(Visibility::Gone),
            handle: Cell::new(ptr::null_mut()),
            owner: Cell::new(ptr::null_mut()),
            text: Cell::new("".to_string()),
            location: Cell::new(Point2::new(0.0, 0.0)),
            size: Cell::new(Size2::new(400.0, 300.0)),
            border_style: Cell::new(WindowBorderStyle::Normal),
            resizable: Cell::new(true),
        }
    }

    fn set_window(&self, window: Weak<WindowData>) {
        self.window.set(Some(window));
    }

    fn set_text(&self, text: &str) {
        self.text.set(text.to_owned());
        if self.is_handle_created() {
            unsafe {
                let mut text_buf = SmallVec::<[u16; 64]>::new();
                let wide_text = str_to_wide_vec(text, &mut text_buf);
                SetWindowTextW(self.handle.get(), wide_text);
            }
        }
    }

    fn visibility(&self) -> Visibility {
        self.visibility.get()
    }

    fn set_visibility(&self, visibility: Visibility) {
        self.visibility.set(visibility);
        if self.visibility.get() == Visibility::Visible {
            // TODO: this isn't how I did it in D
            unsafe { ShowWindow(self.handle(), SW_SHOW); }
        } else {
            if self.is_handle_created() {
                unsafe { ShowWindow(self.handle.get(), SW_HIDE); }
            }
        }
    }

    fn set_location(&self, location: &Point2<f64>) {
        // Don't set the backend fields, only the native window location.
        // The struct fields should be updated by the window procedure.
    }

    fn resizable(&self) -> bool {
        self.resizable.get()
    }

    fn set_resizable(&self, resizable: bool) {
        self.resizable.set(resizable);
        self.update_window_styles();
    }
    // enabling and disabling the close button can be done dynamically by enabling or disabling
    // the close menu item: http://blogs.msdn.com/b/oldnewthing/archive/2010/06/04/10019758.aspx
}
