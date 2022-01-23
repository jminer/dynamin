/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use zaffre::{Painter, Point2, Size2, Rect};

use crate::Window;
use crate::bitfield::BitField;
use crate::event_vec::{EventHandler, EventHandlerVec, EventRoute};

/// Whether a control is visible or affects layout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Visibility {
    /// The control is painted and can be interacted with, as long as its parent is visible as well.
    Visible,
    /// The control isn't painted and can't receive focus or be interacted with, but it still takes
    /// up space in the layout.
    Invisible,
    /// The control is invisible and doesn't take up space in the layout. This state is about the
    /// same as removing the control from its parent.
    Gone,
}

// a From impl can't be private, so this is a function
fn visibility_to_u8(v: Visibility) -> u8 {
    match v {
        Visibility::Visible => 0,
        Visibility::Invisible => 1,
        Visibility::Gone => 2,
    }
}

fn u8_to_visibility(v: u8) -> Visibility {
    match v {
        0 => Visibility::Visible,
        1 => Visibility::Invisible,
        2 => Visibility::Gone,
        _ => panic!("invalid Visibility u8"),
    }
}

// Use a separate trait that isn't reexported to hide methods.
pub trait PrivControl {
    fn set_parent(&self, parent: Weak<dyn Control>);
}

pub trait Control : PrivControl {
    fn as_window(&self) -> Option<Window>;

    fn visibility(&self) -> Visibility;
    fn set_visibility(&self, visibility: Visibility);

    fn location(&self) -> Point2<f64>;
    fn set_location(&self, location: &Point2<f64>);

    fn size(&self) -> Size2<f64>;
    fn set_size(&self, size: &Size2<f64>);

    fn tab_index(&self) -> u16;
    fn set_tab_index(&self, tab_index: u16);

    fn children(&self) -> &RefCell<ChildrenVec>;

    fn parent(&self) -> Option<Rc<dyn Control>>;

    fn window(&self) -> Option<Window>;

    fn event_handlers(&self) -> &EventHandlerVec;

    fn repaint_later(&self);

    fn dispatch_painting(&self, event: &mut PaintingEvent);

    fn child_at_point(&self, x: f64, y: f64) -> Option<Rc<dyn Control>> {
        self.children().borrow().iter()
            .find(|child| (child.location() + child.size()).contains_pt(Point2::new(x, y)))
            .map(|c| c.clone())

        // for child in self.children().borrow().iter() {
        //     if (child.location() + child.size()).contains_pt(Point2::new(x, y)) {
        //         return Some(child.clone());
        //     }
        // }
        // None
    }

    fn descendant_at_point(&self, x: f64, y: f64) -> Option<Rc<dyn Control>> {
        self.children().borrow().iter()
            .find(|child| (child.location() + child.size()).contains_pt(Point2::new(x, y)))
            .map(|child| {
                let (cx, cy) = (x - child.location().x, y - child.location().y);
                child.descendant_at_point(cx, cy).unwrap_or_else(|| child.clone())
            })
    }
}

pub struct ChildrenVec {
    pub(crate) control: Option<Weak<dyn Control>>,
    vec: Vec<Rc<dyn Control>>,
}

impl ChildrenVec {
    pub fn new() -> Self {
        ChildrenVec {
            control: None,
            vec: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.vec.clear();
        self.update_control();
    }

    pub fn push<T>(&mut self, new_control: T) where T: Into<Rc<dyn Control>> {
        let new_control = new_control.into();
        new_control.set_parent(self.control.clone()
            .expect("ChildrenVec control not set; need to call register_handle()"));
        self.vec.push(new_control);
        self.update_control();
    }

    fn update_control(&self) {
        // TODO: update `control`
    }
}

impl Deref for ChildrenVec {
    type Target = [Rc<dyn Control>];
    fn deref(&self) -> &[Rc<dyn Control>] {
        &self.vec
    }
}

#[derive(Clone)]
pub struct SubControl(Rc<SubControlData>);

impl SubControl {
    pub fn new() -> Self {
        SubControl::register_handle(SubControl(Rc::new(SubControlData::new())))
    }

    pub fn register_handle<T>(handle: T) -> T
    where
        T: Into<Rc<dyn Control>> + Into<Rc<dyn EventHandler>> + Clone,
    {
        let control: &Rc<dyn Control> = &handle.clone().into();
        control.children().borrow_mut().control = Some(Rc::downgrade(control));

        let ev_handler: Rc<dyn EventHandler> = handle.clone().into();
        control.event_handlers().add(move |route| { ev_handler.on_event(route) });

        handle
    }
}

impl Deref for SubControl {
    type Target = Rc<SubControlData>;
    fn deref(&self) -> &Rc<SubControlData> {
        &self.0
    }
}

impl From<SubControl> for Rc<dyn Control> {
    fn from(self_: SubControl) -> Self {
        self_.0 as Rc<dyn Control>
    }
}

impl From<SubControl> for Rc<dyn EventHandler> {
    fn from(self_: SubControl) -> Self {
        self_.0 as Rc<dyn EventHandler>
    }
}


// TODO: I currently think BaseControl is a better name than SubControl.
pub struct SubControlData {
    location: Cell<Point2<f64>>,
    size: Cell<Size2<f64>>,
    children: RefCell<ChildrenVec>,
    parent: Cell<Option<Weak<dyn Control>>>,
    event_handlers: EventHandlerVec,
    //draw_commands: Cell<Vec<DrawCommand>>,
    tab_index: Cell<u16>,
    bit_fields: Cell<u8>,
}

pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
}

#[non_exhaustive]
pub struct MouseDownEvent {
}

#[non_exhaustive]
pub struct MouseUpEvent {
}

#[non_exhaustive]
pub struct MouseMovedEvent {
}

#[non_exhaustive]
pub struct MouseDraggedEvent {
}

#[non_exhaustive]
pub struct MouseEnteredEvent;

#[non_exhaustive]
pub struct MouseLeftEvent;

#[non_exhaustive]
pub struct PaintingEvent {
    pub painter: Box<dyn Painter>,
}

const FOCUSABLE_POS: u8 = 0;
const FOCUSED_POS: u8 = 1;
const ENABLED_POS: u8 = 2;
const VISIBILITY_POS: u8 = 3;
const ELASTIC_X_POS: u8 = 5;
const ELASTIC_Y_POS: u8 = 6;

impl PrivControl for SubControlData {
    fn set_parent(&self, parent: Weak<dyn Control>) {
        self.parent.set(Some(parent));
    }
}

impl Control for SubControlData {
    fn as_window(&self) -> Option<Window> {
        None
    }

    fn visibility(&self) -> Visibility {
        u8_to_visibility(self.bit_fields.get().get_bits(VISIBILITY_POS..ELASTIC_X_POS))
    }
    fn set_visibility(&self, visibility: Visibility) {
        self.bit_fields.set(
            self.bit_fields
                .get()
                .set_bits(VISIBILITY_POS..ELASTIC_X_POS, visibility_to_u8(visibility)),
        );
    }

    fn location(&self) -> Point2<f64> {
        self.location.get()
    }
    fn set_location(&self, location: &Point2<f64>) {
        self.location.set(*location);
        self.repaint_later();
    }

    fn size(&self) -> Size2<f64> {
        self.size.get()
    }
    fn set_size(&self, size: &Size2<f64>) {
        self.size.set(*size);
        self.repaint_later();
    }

    fn tab_index(&self) -> u16 {
        self.tab_index.get()
    }
    fn set_tab_index(&self, tab_index: u16) {
        self.tab_index.set(tab_index)
    }

    fn children(&self) -> &RefCell<ChildrenVec> {
        &self.children
    }

    fn parent(&self) -> Option<Rc<dyn Control>> {
        let parent = self.parent.take();
        let parent_copy = parent.clone();
        self.parent.set(parent);
        parent_copy.and_then(|p| p.upgrade())
    }

    // Returns the top-level window containing this control or `None`.
    fn window(&self) -> Option<Window> {
        self.parent().and_then(|p| p.window())
    }

    fn event_handlers(&self) -> &EventHandlerVec {
        &self.event_handlers
    }

    fn repaint_later(&self) {
    }

    fn dispatch_painting(&self, event: &mut PaintingEvent) {
        self.event_handlers.send(event);
        for child in self.children().borrow().iter() {
            event.painter.save();
            event.painter.translate(child.location().x, child.location().y);
            child.event_handlers().send(event);
            event.painter.restore();
        }
    }
}

impl EventHandler for SubControlData {
    fn on_event(&self, route: &mut EventRoute) {
    }
}

pub trait SubControlRef {
    fn sub_control_ref(&self) -> &SubControlData;
}

impl<T> PrivControl for T where T: SubControlRef {
    fn set_parent(&self, parent: Weak<dyn Control>) {
        self.sub_control_ref().set_parent(parent)
    }
}

impl<T> Control for T where T: SubControlRef {
    fn as_window(&self) -> Option<Window> {
        self.sub_control_ref().as_window()
    }

    fn visibility(&self) -> Visibility {
        self.sub_control_ref().visibility()
    }
    fn set_visibility(&self, visibility: Visibility) {
        self.sub_control_ref().set_visibility(visibility)
    }

    fn location(&self) -> Point2<f64> {
        self.sub_control_ref().location()
    }
    fn set_location(&self, location: &Point2<f64>) {
        self.sub_control_ref().set_location(location)
    }

    fn size(&self) -> Size2<f64> {
        self.sub_control_ref().size()
    }
    fn set_size(&self, size: &Size2<f64>) {
        self.sub_control_ref().set_size(size)
    }

    fn tab_index(&self) -> u16 {
        self.sub_control_ref().tab_index()
    }
    fn set_tab_index(&self, tab_index: u16) {
        self.sub_control_ref().set_tab_index(tab_index)
    }

    fn children(&self) -> &RefCell<ChildrenVec> {
        self.sub_control_ref().children()
    }

    fn parent(&self) -> Option<Rc<dyn Control>> {
        self.sub_control_ref().parent()
    }

    fn window(&self) -> Option<Window> {
        self.sub_control_ref().window()
    }

    fn event_handlers(&self) -> &EventHandlerVec {
        self.sub_control_ref().event_handlers()
    }

    fn repaint_later(&self) {
        self.sub_control_ref().repaint_later()
    }

    fn dispatch_painting(&self, event: &mut PaintingEvent) {
        self.sub_control_ref().dispatch_painting(event)
    }

}

impl SubControlData {
    pub fn new() -> Self {
        SubControlData {
            location: Cell::new(Point2::new(0.0, 0.0)),
            size: Cell::new(Size2::new(50.0, 50.0)),
            children: RefCell::new(ChildrenVec::new()),
            parent: Cell::new(None),
            event_handlers: EventHandlerVec::new(),
            tab_index: Cell::new(0),
            bit_fields: Cell::new(
                0.set_bits(
                    VISIBILITY_POS..ELASTIC_X_POS,
                    visibility_to_u8(Visibility::Visible),
                )
                .set_bit(ENABLED_POS, true),
            ),
        }
    }

    pub fn event_handlers(&self) -> &EventHandlerVec {
        &self.event_handlers
    }

    pub fn focusable(&self) -> bool {
        self.bit_fields.get().get_bit(FOCUSABLE_POS)
    }

    pub fn set_focusable(&self, focusable: bool) {
        self.bit_fields.set(self.bit_fields.get().set_bit(FOCUSABLE_POS, focusable));
    }
}

impl Default for SubControlData {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    static HOT_CONTROL: RefCell<Option<Weak<dyn Control>>> = Default::default();
}

pub(crate) fn get_hot_control() -> Option<Weak<dyn Control>> {
    HOT_CONTROL.with(|hot_control| hot_control.borrow().clone())
}

// Sets which control the mouse is over.
pub(crate) fn set_hot_control(control: Option<&Rc<dyn Control>>) {
    HOT_CONTROL.with(|hot_control| {
        let mut hot_control_ref = hot_control.borrow_mut();
        let old = hot_control_ref.as_ref().map(|c| c.as_ptr());
        let new = control.map(Rc::as_ptr);
        if new != old {
            if let Some(old_rc) = hot_control_ref.as_ref().and_then(|c| c.upgrade()) {
                old_rc.event_handlers().send(&mut MouseLeftEvent);
            }

            *hot_control_ref = control.map(|c| Rc::downgrade(c));

            if let Some(control) = control {
                control.event_handlers().send(&mut MouseEnteredEvent);
            }
        }
    });
}


pub fn set_tab_order<'a, I>(start_index: u16, controls: I)
where
    I: Iterator<Item = &'a Rc<dyn Control>>, // TODO: I'm not sure what the `Item` type should be
{
    let mut index = start_index;
    for c in controls {
        c.set_tab_index(index);
        index += 1;
    }
}

#[test]
fn test_set_tab_order() {
    let parent = SubControl::new();
    let child0 = SubControl::new();
    let child1 = SubControl::new();
    parent.children().borrow_mut().push(child0.clone());
    parent.children().borrow_mut().push(child1.clone());
    set_tab_order(4, parent.children().borrow().iter());
    assert_eq!(child0.tab_index(), 4);
    assert_eq!(child1.tab_index(), 5);
}
