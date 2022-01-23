/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use smallvec::SmallVec;

// Copy-on-write is used on the vector in the `Rc` so that if a callback is added or removed inside
// a callback, it can make the change to a copy of the vector. The in-progress notification can
// continue iterating over the original vector. To make the vector `Clone`, each function is wrapped
// in an `Rc`. Iterating over the vector only requires read access (even to reentrantly iterate over
// the vector because you can have multiple immutable references), and for write access in
// add/remove_handler, `make_mut` is used.

pub struct EventHandlerVec(RefCell<Rc<Vec<Rc<RefCell<dyn EventHandlerFn>>>>>);

trait EventHandlerFn = for<'a> FnMut(&'a mut EventRoute);

#[non_exhaustive]
pub struct EventRoute<'a> {
    pub event: &'a mut dyn Any,
    pub handled: bool,
    pub self_events: SmallVec<[Box<dyn Any>; 1]>,
}

impl EventHandlerVec {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn add<F>(&self, handler: F)
    where
        // I can't use EventHandler<T> here because the for<'a> doesn't work then.
        F: for<'a> FnMut(&'a mut EventRoute) + 'static,
    {
        let mut event_handlers_rc = self.0.borrow_mut();
        let event_handlers = Rc::make_mut(&mut event_handlers_rc);
        event_handlers.push(Rc::new(RefCell::new(handler)));
    }

    // If this function is called from an event handler on the same control, then it would cause the
    // handler to be called recursively (indirectly). However calling an FnMut recursively is
    // impossible in Rust because it would require multiple mutable references to the same closure.
    // In this situation, the handler will be skipped and all the other handlers will be called. To
    // send an event to the current control without skipping the handler, add the event to the
    // `EventRoute::self_events` list.
    pub fn send(&self, event: &mut dyn Any) {
        let event_handlers = self.0.borrow().clone();
        let mut route = EventRoute { event, handled: false, self_events: SmallVec::new() };
        // Call more recently added handlers first so that they can override the behavior of those
        // added earlier.
        for handler in event_handlers.iter().rev() {
            // If this function is being called by the handler, then we can't borrow the handler
            // to call it again. Let's skip it.
            let borrow_result = handler.try_borrow_mut();
            if let Ok(mut borrow) = borrow_result {
                let handler: &mut dyn FnMut(&mut EventRoute) = &mut *borrow;
                handler(&mut route);
                if route.handled {
                    break;
                }
            }
        }
        for mut event in route.self_events.into_iter() {
            self.send(&mut event);
        }
    }
}

impl Default for EventHandlerVec {
    fn default() -> Self {
        EventHandlerVec::new()
    }
}

pub trait EventHandler {
    fn on_event(&self, route: &mut EventRoute);
}