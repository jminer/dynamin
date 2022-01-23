/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ptr;
use std::ffi::OsStr;
use std::mem;
use std::os::raw::{c_int, c_short, c_ushort};
use std::os::windows::ffi::OsStrExt;
use std::rc::{Rc, Weak};
use std::sync::{Once, ONCE_INIT};

use crate::control::{PaintingEvent, set_hot_control};
use crate::{Control, Visibility, Window, WindowBorderStyle, MouseDownEvent, MouseUpEvent, MouseMovedEvent, MouseDraggedEvent};
use crate::generic_backend::GenericWindowBackend;
use crate::{WindowData, WindowEvent};

use smallvec::SmallVec;
use windows::Win32::Foundation::{HWND, WPARAM, LPARAM, LRESULT, PWSTR, HINSTANCE};
use windows::Win32::Graphics::Gdi::{PAINTSTRUCT, BeginPaint, EndPaint};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::WM_MOUSELEAVE;
use windows::Win32::UI::Input::KeyboardAndMouse::{SetCapture, ReleaseCapture, TRACKMOUSEEVENT, TME_LEAVE, TrackMouseEvent};
use windows::Win32::UI::WindowsAndMessaging::{WM_LBUTTONDOWN, WM_MBUTTONDOWN, WM_RBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONUP, WM_RBUTTONUP, MK_LBUTTON, MK_MBUTTON, MK_RBUTTON, MK_XBUTTON1, MK_XBUTTON2, DefWindowProcW, DestroyWindow, GetWindowLongW, GWL_STYLE, GWL_EXSTYLE, WS_DLGFRAME, WS_BORDER, WS_THICKFRAME, WS_MINIMIZEBOX, WS_SYSMENU, WS_EX_TOOLWINDOW, SetWindowLongW, SetWindowPos, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SWP_FRAMECHANGED, WNDCLASSEXW, CS_DBLCLKS, RegisterClassExW, CreateWindowExW, HMENU, SetWindowTextW, ShowWindow, SW_SHOW, SW_HIDE, WM_CLOSE, WM_PAINT, WM_MOUSEMOVE};
use zaffre::{Brush, Color, PainterExt, PathBuf, Point2, RenderingBackend, Size2, StrokeStyle, SwapchainSurface};
use zaffre::AsPathIter;

use super::str_to_wide_vec;

// Copied from winapi temporarily until hopefully the windows crate adds them.

// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
#[allow(non_snake_case)]
#[inline]
pub fn LOWORD(l: u32) -> u16 {
    (l & 0xffff) as u16
}
#[allow(non_snake_case)]
#[inline]
pub fn HIWORD(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}
#[allow(non_snake_case)]
#[inline]
pub fn GET_X_LPARAM(lp: LPARAM) -> c_int {
    LOWORD(lp.0 as u32) as c_short as c_int
}
#[allow(non_snake_case)]
#[inline]
pub fn GET_Y_LPARAM(lp: LPARAM) -> c_int {
    HIWORD(lp.0 as u32) as c_short as c_int
}

const WINDOW_CLASS_NAME: &'static str = "DynaminWindowRust";

static REGISTER_WINDOW_CLASS: Once = ONCE_INIT;

pub struct WindowBackend {
    window: Cell<Option<Weak<WindowData>>>,
    visibility: Cell<Visibility>,
    handle: Cell<HWND>,
    owner: Cell<HWND>,
    surface: Cell<Option<SwapchainSurface>>,
    text: Cell<String>,
    location: Cell<Point2<f64>>,
    size: Cell<Size2<f64>>,
    border_style: Cell<WindowBorderStyle>,
    resizable: Cell<bool>,
    tracking_mouse_leave: Cell<bool>,
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
    static WINDOWS: RefCell<HashMap<isize, Weak<WindowData>>> = RefCell::new(HashMap::new());
}

fn get_window(hwnd: HWND) -> Rc<WindowData> {
    // The Rust side object should exist as long as the native window because when the Rust object
    // is dropped, it destroys the native window. Thus, the unwrap() should be safe.
    WINDOWS.with(|windows| windows.borrow()[&hwnd.0].upgrade()).unwrap()
}

#[allow(non_snake_case)]
unsafe extern "system"
fn windowProc(hwnd: HWND, uMsg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    match uMsg {
        WM_CLOSE => {
            let window = get_window(hwnd);
            let mut event = WindowEvent::Closing;
            window.event_handlers().send(&mut event);
            // TODO: get handle to window and send Closing event
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::zeroed();
            BeginPaint(hwnd, &mut ps);

            let window = get_window(hwnd);
            let backend = &window.backend;
            let mut surface = backend.surface.take().unwrap();

            let mut painter = surface.start_painting(ps.hdc);

            let mut path = PathBuf::new();
            path.move_to(Point2::new(5.0f32, 5.0f32));
            path.line_to(Point2::new(60.0f32, 25.0f32));
            painter.clear(Color::from_rgba(200, 200, 200, 255));
            painter.stroke_path(&mut path.path_iter(),
                &Brush::Solid(Color::from_rgba(0, 0, 255, 255)),
                &StrokeStyle::with_width(3.0f32));
            // painter.stroke(&path.as_path(),
            //     &Brush::Solid(Color::from_rgba(0, 0, 255, 128)),
            //     &StrokeStyle::with_width(2.0f32));
            let mut event = PaintingEvent {
                painter,
            };
            if let Some(child) = window.children().borrow().first() {
                child.dispatch_painting(&mut event);
            }

            surface.end_painting(ps.hdc);
            backend.surface.set(Some(surface));

            EndPaint(hwnd, &mut ps);
            LRESULT(0)
        }
        WM_LBUTTONDOWN | WM_MBUTTONDOWN | WM_RBUTTONDOWN => {
            SetCapture(hwnd);
            let (x, y) = (GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam));

            let window = get_window(hwnd);

            if let Some(child) = window.children().borrow().first() {
                let descendant = child.descendant_at_point(x as f64, y as f64)
                    .unwrap_or_else(|| child.clone());
                descendant.event_handlers().send(&mut MouseDownEvent {
                });
            }

            LRESULT(0)
        }
        WM_LBUTTONUP | WM_MBUTTONUP | WM_RBUTTONUP => {
            ReleaseCapture();

            let (x, y) = (GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam));

            let window = get_window(hwnd);

            if let Some(child) = window.children().borrow().first() {
                let descendant = child.descendant_at_point(x as f64, y as f64)
                    .unwrap_or_else(|| child.clone());
                descendant.event_handlers().send(&mut MouseUpEvent {
                });
            }

            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            let (x, y) = (GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam));

            let window = get_window(hwnd);
            let backend = &window.backend;

            if !backend.tracking_mouse_leave.get() {
                backend.tracking_mouse_leave.set(true);

                let mut tme: TRACKMOUSEEVENT = mem::zeroed();
                tme.cbSize = mem::size_of::<TRACKMOUSEEVENT>() as u32;
                tme.dwFlags = TME_LEAVE;
                tme.hwndTrack = hwnd;
                TrackMouseEvent(&mut tme);
            }

            if let Some(child) = window.children().borrow().first() {
                let descendant = child.descendant_at_point(x as f64, y as f64)
                    .unwrap_or_else(|| child.clone());
                set_hot_control(Some(&descendant));
                let wParam = wParam.0 as u32;
                if wParam & (MK_LBUTTON | MK_MBUTTON | MK_RBUTTON | MK_XBUTTON1 | MK_XBUTTON2) != 0
                {
                    descendant.event_handlers().send(&mut MouseDraggedEvent {
                    });
                } else {
                    descendant.event_handlers().send(&mut MouseMovedEvent {
                    });
                }
            }

            LRESULT(0)
        }
        WM_MOUSELEAVE => {
            let window = get_window(hwnd);
            let backend = &window.backend;

            backend.tracking_mouse_leave.set(false);

            set_hot_control(None);

            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, uMsg, wParam, lParam)
    }
}

impl WindowBackend {
    fn delete_handle(&self) {
        if !self.handle.get().is_invalid() {
            WINDOWS.with(|windows| {
                let mut windows = windows.borrow_mut();
                windows.remove(&self.handle.get().0);
            });
            unsafe { DestroyWindow(self.handle.get()); }
            self.handle.set(HWND(0));
        }
    }

    fn window_styles(&self) -> (u32, u32) {
        let (mut style, mut ex_style) = (0, 0);
        if self.is_handle_created() {
            unsafe {
                style = GetWindowLongW(self.handle.get(), GWL_STYLE) as u32;
                ex_style = GetWindowLongW(self.handle.get(), GWL_EXSTYLE) as u32;
            }
        }
        {
            let mut set_if = |s: u32, b: bool| {
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
            SetWindowLongW(self.handle.get(), GWL_STYLE, style as i32);
            SetWindowLongW(self.handle.get(), GWL_EXSTYLE, ex_style as i32);
            SetWindowPos(self.handle.get(), HWND(0), 0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
        }
    }

    fn recreate_handle(&self) {
        REGISTER_WINDOW_CLASS.call_once(|| {
            unsafe {
                let mut class_name_buf = SmallVec::<[u16; 32]>::new();
                let wide_class_name = PWSTR(
                    str_to_wide_vec(WINDOW_CLASS_NAME, &mut class_name_buf) as *mut _);

                let mut wc: WNDCLASSEXW = mem::zeroed();
                wc.cbSize = mem::size_of::<WNDCLASSEXW>() as u32;
                wc.style = CS_DBLCLKS;
                wc.lpfnWndProc = Some(windowProc);
                wc.hInstance = GetModuleHandleW(PWSTR(ptr::null_mut()));
                wc.lpszClassName = wide_class_name;

                assert!(RegisterClassExW(&wc) != 0);
            }
        });

        let (style, ex_style) = self.window_styles();
        self.delete_handle();

        let mut class_name_buf = SmallVec::<[u16; 32]>::new();
        let wide_class_name = PWSTR(
            str_to_wide_vec(WINDOW_CLASS_NAME, &mut class_name_buf) as *mut _);
        let mut text_buf = SmallVec::<[u16; 64]>::new();
        let text_temp = self.text.take();
        let wide_text = PWSTR(str_to_wide_vec(&text_temp, &mut text_buf) as *mut _);
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
                HMENU(0),
                HINSTANCE(0),
                ptr::null_mut(),
            ));
            assert!(!self.handle.get().is_invalid());
            WINDOWS.with(|windows| {
                let mut windows = windows.borrow_mut();
                let window = self.window.take();
                windows.insert(self.handle.get().0, window.clone().unwrap());
                self.window.set(window);
            });

            self.surface.set(Some(
                SwapchainSurface::from_hwnd(self.handle.get(), RenderingBackend::Gpu)
            ));
        }
    }

    fn is_handle_created(&self) -> bool {
        !self.handle.get().is_invalid()
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
            handle: Cell::new(HWND(0)),
            owner: Cell::new(HWND(0)),
            surface: Cell::new(None),
            text: Cell::new("".to_string()),
            location: Cell::new(Point2::new(0.0, 0.0)),
            size: Cell::new(Size2::new(400.0, 300.0)),
            border_style: Cell::new(WindowBorderStyle::Normal),
            resizable: Cell::new(true),
            tracking_mouse_leave: Cell::new(false),
        }
    }

    fn set_window(&self, window: Weak<WindowData>) {
        self.window.set(Some(window));
    }

    fn window(&self) -> Window {
        let window = self.window.take();
        // The unwrap can't fail because `self` is a reference to the window so obviously it hasn't
        // been dropped.
        let window_copy = window.as_ref().expect("backend.window should be set").upgrade().unwrap();
        self.window.set(window);
        Window(window_copy)
    }

    fn set_text(&self, text: &str) {
        self.text.set(text.to_owned());
        if self.is_handle_created() {
            unsafe {
                let mut text_buf = SmallVec::<[u16; 64]>::new();
                let wide_text = PWSTR(str_to_wide_vec(text, &mut text_buf) as *mut _);
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
