/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

use std::cell::Cell;
use std::ptr;
use std::rc::Weak;

use glib_sys::gboolean;
use gtk_sys::{GTK_WINDOW_TOPLEVEL, GtkWidget, GtkWindow, gtk_widget_destroy, gtk_widget_hide, gtk_widget_show, gtk_window_new, gtk_window_set_resizable, gtk_window_set_title};
use smallvec::SmallVec;

use crate::{Visibility, WindowData};
use crate::generic_backend::GenericWindowBackend;

use super::str_to_c_vec;

pub struct WindowBackend {
    window: Cell<Option<Weak<WindowData>>>,
    handle: Cell<*mut GtkWidget>,
    visibility: Cell<Visibility>,
    text: Cell<String>,
    resizable: Cell<bool>,
}

impl WindowBackend {
    fn delete_handle(&self) {
        if !self.handle.get().is_null() {
            unsafe { gtk_widget_destroy(self.handle.get()); }
            self.handle.set(ptr::null_mut());
        }
    }

    fn recreate_handle(&self) {
        unsafe {
            self.handle.set(gtk_window_new(GTK_WINDOW_TOPLEVEL));
            // TODO: have to set text, resizable, etc.
        }
    }

    fn is_handle_created(&self) -> bool {
        !self.handle.get().is_null()
    }

    fn handle(&self) -> *mut GtkWidget {
        if !self.is_handle_created() {
            self.recreate_handle();
        }
        self.handle.get()
    }
}

impl GenericWindowBackend for WindowBackend {
    fn new() -> WindowBackend {
        WindowBackend {
            window: Cell::new(None),
            handle: Cell::new(ptr::null_mut()),
            visibility: Cell::new(Visibility::Gone),
            text: Cell::new("".to_string()),
            resizable: Cell::new(true),
        }
    }

    fn set_window(&self, window: Weak<crate::WindowData>) {
        self.window.set(Some(window));
    }

    fn set_text(&self, text: &str) {
        self.text.set(text.to_owned());
        if self.is_handle_created() {
            unsafe {
                let mut text_buf = SmallVec::<[u8; 128]>::new();
                let c_text = str_to_c_vec(text, &mut text_buf);
                gtk_window_set_title(self.handle.get() as *mut GtkWindow, c_text);
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
            unsafe { gtk_widget_show(self.handle()); }
        } else {
            if self.is_handle_created() {
                unsafe { gtk_widget_hide(self.handle.get()); }
            }
        }
    }

    fn set_location(&self, location: &Point2<f64>) {
        todo!()
    }

    fn resizable(&self) -> bool {
        self.resizable.get()
    }

    fn set_resizable(&self, resizable: bool) {
        self.resizable.set(resizable);
        if self.is_handle_created() {
            unsafe {
                gtk_window_set_resizable(
                    self.handle.get() as *mut GtkWindow,
                    resizable as gboolean);
            }
        }
    }
}

