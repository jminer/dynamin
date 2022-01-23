/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

#![feature(non_exhaustive)]
#![feature(trait_alias)]

#![deny(unreachable_pub)]

#[allow(dead_code)]

pub use button::{Button, ClickEvent};
pub use control::{ChildrenVec, Control, MouseButton, MouseEnteredEvent, MouseLeftEvent, MouseDownEvent, MouseUpEvent, MouseDraggedEvent, MouseMovedEvent, PaintingEvent, set_tab_order, SubControl, Visibility};
pub use cursor::{Cursor, CursorData};
pub use event_vec::EventHandlerVec;
pub use window::{Window, WindowData, WindowBorderStyle, WindowEvent};
use zaffre::RenderingBackend;

mod bitfield;
mod button;
mod control;
mod cursor;
mod event_vec;
mod window;

mod generic_backend;

#[cfg(windows)]
#[path = "windows_backend/mod.rs"]
pub mod backend;
#[cfg(unix)]
#[path = "gtk_backend/mod.rs"]
pub mod backend;

pub fn set_rendering_preference(pref: RenderingBackend) {

}
