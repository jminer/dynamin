
/*
 * Copyright Jordan Miner
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

module dynamin.gui.container;

import dynamin.all_core;
import dynamin.all_painting;
import dynamin.all_gui;
import dynamin.gui.control;
import dynamin.gui.events;
import dynamin.gui.button;
import tango.io.Stdout;

alias List!(Control, true) ControlList;

///
class Container : Control {
protected:
	ControlList _children;
	Size _minSize;
	Size _maxSize;
	Button _defaultButton;

	override void whenResized(EventArgs e) {
		layout();
	}
	// If the specified array is large enough to hold the results, no heap
	// allocation will be done.
	Control[] getFocusableDescendants(Control[] des = null) {
		uint cur = 0;
		foreach(d; &descendantsPlus) {
			if(d.focusable) { // TODO: && ch.enabled) {
				if(cur == des.length)
					des.length = des.length + 20;
				des[cur++] = d;
			}
		}
		return des[0..cur];
	}
	unittest {
		class MyControl : Control {
			this() {
				_focusable = true;
			}
		}
		auto container1 = new Container();
		auto container2 = new Container();
		auto container3 = new Container();
		auto container4 = new Container();
		container2._focusable = true;
		container4._focusable = true;
		auto ctrl1 = new MyControl();
		auto ctrl2 = new MyControl();
		auto ctrl3 = new MyControl();

		container1.add(container2);
		container1.add(container3);
		container3.add(container4);
		container2.add(ctrl1);
		container4.add(ctrl2);
		container4.add(ctrl3);
		assert(container1.getFocusableDescendants() ==
			[cast(Control)container2, ctrl1, container4, ctrl2, ctrl3]);
		Control[5] buf;
		assert(container1.getFocusableDescendants(buf).ptr == buf.ptr);
	}

	// not an event
	void whenChildrenChanged(ListChangeType, Control oldChild, Control newChild, word) {
		if(oldChild) {
			oldChild.parent = null;
			repaint();

			scope e = new HierarchyEventArgs(oldChild);
			descendantRemoved(e);
		}

		if(newChild) {
			if(newChild.parent)
				newChild.parent.remove(newChild);
			newChild.parent = this;
			repaint();

			void callAdded(Control ctrl) {
				scope e = new HierarchyEventArgs(ctrl);
				descendantAdded(e);

				if(auto cntr = cast(Container)ctrl) {
					foreach(c; cntr._children)
						callAdded(c);
				}
			}
			callAdded(newChild);
		}

	}

	void dispatchDescendantAdded(HierarchyEventArgs e) {
		if(e.descendant is defaultButton)
			(cast(Button)e.descendant)._isDefault = true;

		descendantAdded.callHandlers(e);
		descendantAdded.callMainHandler(e);
		e.levels = e.levels + 1;
		if(_parent)
			_parent.descendantAdded(e);
	}
	void dispatchDescendantRemoved(HierarchyEventArgs e) {
		descendantRemoved.callHandlers(e);
		descendantRemoved.callMainHandler(e);
		e.levels = e.levels + 1;
		if(_parent)
			_parent.descendantRemoved(e);
	}

	override void whenKeyDown(KeyEventArgs e) {
		if(e.key == Key.Enter && defaultButton) {
			scope e2 = new EventArgs;
			defaultButton.clicked(e2);
			e.stopped = true;
		}
	}

public:
	/// Override this method in a subclass to handle the minSizeChanged event.
	protected void whenMinSizeChanged(EventArgs e) { }
	/// This event occurs after the control's minimum size has been changed.
	Event!(whenMinSizeChanged) minSizeChanged;

	/// Override this method in a subclass to handle the maxSizeChanged event.
	protected void whenMaxSizeChanged(EventArgs e) { }
	/// This event occurs after the control's maximum size has been changed.
	Event!(whenMaxSizeChanged) maxSizeChanged;

	/// Override this method in a subclass to handle the descendantAdded event.
	protected void whenDescendantAdded(HierarchyEventArgs e) { }
	/// This event occurs after a control is added as a descendant of this container.
	Event!(whenDescendantAdded) descendantAdded;

	/// Override this method in a subclass to handle the descendantRemoved event.
	protected void whenDescendantRemoved(HierarchyEventArgs e) { }
	/// This event occurs after a descendant of this container has been removed.
	Event!(whenDescendantRemoved) descendantRemoved;

	this() {
		minSizeChanged   .setUp(&whenMinSizeChanged);
		maxSizeChanged   .setUp(&whenMaxSizeChanged);
		descendantAdded  .setUp(&whenDescendantAdded, &dispatchDescendantAdded);
		descendantRemoved.setUp(&whenDescendantRemoved,
		                        &dispatchDescendantRemoved);

		_children = new ControlList(&whenChildrenChanged);

		elasticX = true;
		elasticY = true;
	}

	/**
	 * Gets or sets the default button in this container.
	 */
	Button defaultButton() { return _defaultButton; }
	/// ditto
	void defaultButton(Button b) {
		_defaultButton = b;
		foreach(d; &descendants) {
			if(b == d)
				b._isDefault = true;
		}
	}

	override void dispatchPainting(PaintingEventArgs e) {
		super.dispatchPainting(e);
		foreach(c; _children) {
			e.graphics.save();
			e.graphics.translate(c.x, c.y);
			c.setupGraphics(e.graphics);
			c.painting(e);
			e.graphics.restore();
		}
	}
	// TODO: make these use common code...get rid of copy and paste
	override void dispatchMouseDown(MouseEventArgs e) {
		auto c = getChildAtPoint(e.location);
		if(c && getCaptorControl() !is this) {
			e.location = e.location - c.location;
			c.mouseDown(e);
			e.location = e.location + c.location;
		} else {
			super.dispatchMouseDown(e);
		}
	}
	override void dispatchMouseUp(MouseEventArgs e) {
		auto c = getChildAtPoint(e.location);
		if(c && getCaptorControl() !is this) {
			e.location = e.location - c.location;
			c.mouseUp(e);
			e.location = e.location + c.location;
		} else {
			super.dispatchMouseUp(e);
		}
	}
	override void dispatchMouseMoved(MouseEventArgs e) {
		auto c = getChildAtPoint(e.location);
		if(c && getCaptorControl() !is this) {
			e.location = e.location - c.location;
			c.mouseMoved(e);
			e.location = e.location + c.location;
		} else {
			super.dispatchMouseMoved(e);
		}
	}
	override void dispatchMouseDragged(MouseEventArgs e) {
		auto c = getChildAtPoint(e.location);
		if(c && getCaptorControl() !is this) {
			e.location = e.location - c.location;
			c.mouseDragged(e);
			e.location = e.location + c.location;
		} else {
			super.dispatchMouseDragged(e);
		}
	}

	/**
	 * Gets the child control at the specified point. If there are
	 * multiple child controls at the point, the topmost control is returned.
	 * If there is no child control at the point, null is returned. The returned
	 * control, if any, is a direct child of this container.
	 */
	Control getChildAtPoint(double[] pt) {
		assert(pt.length == 2, "pt must be just an x and y");
		return getChildAtPoint(Point(pt[0], pt[1]));
	}
	/// ditto
	Control getChildAtPoint(double x, double y) {
		return getChildAtPoint(Point(x, y));
	}
	/// ditto
	Control getChildAtPoint(Point pt) {
		for(int i = _children.count-1; i >= 0; --i) {
			pt = pt - _children[i].location;
			scope(exit) pt = pt + _children[i].location;
			if(_children[i].contains(pt))
				return _children[i];
		}
		return null;
	}

	/**
	 * Never returns null. If there is no descendant at the specified point,
	 * this container will be returned.
	 */
	Control getDescendantAtPoint(double[] pt) {
		assert(pt.length == 2, "pt must be just an x and y");
		return getDescendantAtPoint(Point(pt[0], pt[1]));
	}
	/// ditto
	Control getDescendantAtPoint(double x, double y) {
		return getDescendantAtPoint(Point(x, y));
	}
	/// ditto
	Control getDescendantAtPoint(Point pt) {
		Container des = this;
		while(true) {
			auto child = des.getChildAtPoint(pt);
			if(!child)
				return des;
			auto isContainer = cast(Container)child;
			if(isContainer) {
				des = isContainer;
				pt = pt - des.location;
				// loop around with this container
			} else {
				return child;
			}
		}
	}

	/**
	 * Gets or sets the minimum size of this window. A minimum width or
	 * height of 0 means that there is no minimum width or height.
	 * The default is Size(0, 0).
	 */
	Size minSize() { return _minSize; }
	/// ditto
	void minSize(Size size) {
		if(_minSize == size)
			return;
		_minSize = size;
		minSizeChanged(new EventArgs);
	}
	/// ditto
	void minSize(double[] size) {
		assert(size.length == 2, "size must be just a width and height");
		minSize = Size(size[0], size[1]);
	}
	///
	double minWidth() { return _minSize.width; }
	///
	double minHeight() { return _minSize.height; }

	/**
	 * Gets or sets the maximum size of this window. A maximum width or
	 * height of 0 means that there is no maximum width or height.
	 * The default is Size(0, 0).
	 */
	Size maxSize() { return _maxSize; }
	/// ditto
	void maxSize(Size size) {
		if(_maxSize == size)
			return;
		_maxSize = size;
		minSizeChanged(new EventArgs);
	}
	/// ditto
	void maxSize(double[] size) {
		assert(size.length == 2, "size must be just a width and height");
		maxSize = Size(size[0], size[1]);
	}
	///
	double maxWidth() { return _maxSize.width; }
	///
	double maxHeight() { return _maxSize.height; }

	/**
	 * Causes this container to position its child controls. Called on every
	 * resize. Usually, this function will get each child's best size, and
	 * then set each child's location and height. The definition in Container
	 * is empty, as it is intended for subclasses to override.
	 */
	void layout() {
	}

	protected void add(Control child) {
		_children.add(child);
	}

	protected void remove(Control child) {
		_children.remove(child);
	}

	/**
	 * Calls the specified delegate with each child of this container, for
	 * use with foreach.
	 */
	int opApply(int delegate(ref Control item) dg) {
		for(uint i = 0; i < _children.count; ++i) {
			auto tmp = _children[i];
			if(int result = dg(tmp))
				return result;
		}
		return 0;
	}
	/// ditto
	int opApply(int delegate(ref uint index, ref Control item) dg) {
		for(uint i = 0; i < _children.count; ++i) {
			auto tmp = _children[i];
			if(int result = dg(i, tmp))
				return result;
		}
		return 0;
	}
	/**
	 * Calls the specified delegate with each descendant of this container, for
	 * use with foreach.
	 *
	 * Example:
	 * -----
	 * foreach(c; &container.descendants) {
	 *     // do something with c
	 * }
	 * -----
	 */
	int descendants(int delegate(ref Control item) dg) {
		for(int i = 0; i < _children.count; ++i) {
			auto tmp = _children[i];
			if(int result = dg(tmp))
				return result;
			auto c = cast(Container)_children[i];
			if(c)
				if(int result = c.descendants(dg))
					return result;
		}
		return 0;
	}
	/**
	 * Same as descendants(), but includes this container in addition to
	 * descendants.
	 */
	int descendantsPlus(int delegate(ref Control item) dg) {
		Control tmp = this;
		if(int result = dg(tmp))
			return result;
		if(int result = descendants(dg))
			return result;
		return 0;
	}
}
unittest {
	int a = 0, r = 0;
	auto container1 = new Panel;
	auto container2 = new Panel;
	auto control1 = new Control;
	auto control2 = new Control;
	auto control3 = new Control;
	container1.descendantAdded += (HierarchyEventArgs e) { a++; };
	container1.descendantRemoved += (HierarchyEventArgs e) { r++; };

	container2.add(control2);
	container2.children.add(control3);
	assert(control3.parent == container2);
	container1.add(control1);
	assert(a == 1);  // test descendantAdded
	container1.add(container2);
	assert(a == 4);  // test descendantAdded

	// test Container.descendants
	auto list = new List!(Control);
	foreach(c; &container1.descendants)
		list.add(c);
	assert(list.data == [control1, container2, control2, control3]);

	list.clear();
	foreach(c; &container1.descendantsPlus)
		list.add(c);
	assert(list.data == [cast(Control)container1, control1, container2,
		control2, control3]);
}

///
class Panel : Container {
	///
	ControlList children() { return _children; }
	///
	override void add(Control child) { super.add(child); };
	///
	override void remove(Control child) { super.remove(child); };
	///
	override int opApply(int delegate(ref Control item) dg) {
		return super.opApply(dg);
	}
	///
	override int opApply(int delegate(ref uint index, ref Control item) dg) {
		return super.opApply(dg);
	}
}

