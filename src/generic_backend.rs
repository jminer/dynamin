/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

use std::rc::Weak;

use zaffre::{Point2, Size2};

use crate::Window;
use crate::window::WindowData;
use super::Visibility;

//pub trait GenericCursorBackend {

//}

pub trait GenericWindowBackend {
    fn new() -> Self;

    fn set_window(&self, window: Weak<WindowData>);

    fn window(&self) -> Window;

    //fn location(&self) -> Point2<f64>;
    //fn set_location(&mut self, location: &Point2<f64>);
    //fn size(&self) -> Size2<f64>;
    //fn set_size(&mut self, size: &Size2<f64>);
    fn set_text(&self, text: &str);

    fn visibility(&self) -> Visibility;

    fn set_visibility(&self, visibility: Visibility);

    fn set_location(&self, location: &Point2<f64>);

    fn resizable(&self) -> bool;

    fn set_resizable(&self, resizable: bool);
}
