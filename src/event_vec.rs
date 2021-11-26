/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// Copy-on-write is used on the vector in the `Rc` so that if a callback is added or removed inside
// a callback, it can make the change to a copy of the vector. The in-progress notification can
// continue iterating over the original vector. To make the vector `Clone`, each function is wrapped
// in an `Rc`. Iterating over the vector only requires read access (even to reentrantly iterate over
// the vector because you can have multiple immutable references), and for write access in
// add/remove_handler, `make_mut` is used.

pub struct EventHandlerVec(RefCell<Rc<Vec<Rc<RefCell<dyn EventHandlerFn>>>>>);

pub trait EventHandlerFn = for<'a> FnMut(&'a mut dyn Any);

impl EventHandlerVec {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn add<F>(&self, handler: F)
    where
        // I can't use EventHandler<T> here because the for<'a> doesn't work then.
        F: for<'a> FnMut(&'a mut dyn Any) + 'static,
    {
        let mut event_handlers_rc = self.0.borrow_mut();
        let event_handlers = Rc::make_mut(&mut event_handlers_rc);
        event_handlers.push(Rc::new(RefCell::new(handler)));
    }

    pub fn send(&self, event: &mut dyn Any) {
        let event_handlers = self.0.borrow().clone();
        for handler in event_handlers.iter().rev() {
            let handler: &mut dyn EventHandlerFn = &mut *handler.borrow_mut();
            handler(event);
        }
    }
}

impl Default for EventHandlerVec {
    fn default() -> Self {
        EventHandlerVec::new()
    }
}

pub trait EventHandler {
    fn on_event(&self, event: &mut dyn Any);
}