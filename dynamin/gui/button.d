
/*
 * Copyright Jordan Miner
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

module dynamin.gui.button;

import dynamin.all_core;
import dynamin.all_gui;
import dynamin.all_painting;
import dynamin.gui.control;
import dynamin.gui.events;
import tango.io.Stdout;
import dynamin.c.cairo; // TODO: remove

// TODO: maybe change to ControlState and add Disabled ?
enum ButtonState {
	Normal, Hot, Pressed
}
// TODO: move to another file
enum TextImageRelation {
	Overlay, TextBeforeImage, ImageBeforeText, TextAboveImage, ImageAboveText
}

/**
 * A button is a control that can be clicked to perform an action.
 *
 * The appearance of a button with Windows Classic:
 *
 * $(IMAGE ../web/example_button.png)
 */
class Button : Control {
protected:
	package bool _isDefault;
	ButtonState _state;
	override void whenFocusGained(EventArgs e) {
		super.whenFocusGained(e);
		auto anc = findAncestor((Container c) {
			return c.defaultButton !is null;
		});
		if(anc) {
			foreach(d; &anc.descendants) {
				if(auto b = cast(Button)d)
					b._isDefault = false;
			}
		}
		_isDefault = true;
	}
	override void whenFocusLost(EventArgs e) {
		super.whenFocusLost(e);
		_isDefault = false;
		auto anc = findAncestor((Container c) {
			return c.defaultButton !is null;
		});
		if(anc)
			anc.defaultButton._isDefault = true;
	}
	override void whenMouseDragged(MouseEventArgs e) {
		if(contains(e.x, e.y))
			state = ButtonState.Pressed;
		else
			state = ButtonState.Normal;
	}
	override void whenMouseEntered(EventArgs e) {
		state = ButtonState.Hot;
	}
	override void whenMouseLeft(EventArgs e) {
		state = ButtonState.Normal;
	}
	override void whenMouseDown(MouseEventArgs e) {
		if(e.button == MouseButton.Left) {
			state = ButtonState.Pressed;
			focus();
		}
	}
	override void whenMouseUp(MouseEventArgs e) {
		if(state != ButtonState.Pressed) return;
		state = ButtonState.Hot;
		clicked(new EventArgs);
	}
	override void whenKeyDown(KeyEventArgs e) {
		if(e.key == Key.Space) {
			state = ButtonState.Pressed;
			e.stopped = true;
		} else if(e.key == Key.Enter) { // being default is implied in keyDown
			scope e2 = new EventArgs;
			clicked(e2);
			e.stopped = true;
		}
	}
	override void whenKeyUp(KeyEventArgs e) {
		if(e.key == Key.Space) {
			if(state != ButtonState.Pressed) return;
			state = ButtonState.Normal;
			scope e2 = new EventArgs;
			clicked(e2);
			e.stopped = true;
		}
	}
	override void whenPainting(PaintingEventArgs e) {
		Theme.current.Button_paint(this, e.graphics);
		return;
		// TODO: move to a theme or delete
		with(e.graphics) {
			if(_state == ButtonState.Hot)
				source = Color(200, 0, 0);
			else if(_state == ButtonState.Pressed)
				source = Color(110, 0, 0);
			else
				source = Color(220, 80, 80);
			moveTo(3, 0);
			lineTo(width-3, 0);
			lineTo(width, 3);
			lineTo(width, height-3);
			lineTo(width-3, height);
			lineTo(3, height);
			lineTo(0, height-3);
			lineTo(0, 3);
			closePath();
			fill();
			auto grad = cairo_pattern_create_linear(0, 0, 0, height/2+3);
			cairo_pattern_add_color_stop_rgba(grad, 0, 1, 1, 1, .4);
			cairo_pattern_add_color_stop_rgba(grad, 1, 1, 1, 1, .05);
			cairo_set_source(handle, grad);
			cairo_pattern_destroy(grad);
			moveTo(3, 0);
			lineTo(width-3, 0);
			lineTo(width, 3);
			lineTo(width, cast(int)height/2-3);
			lineTo(width-3, cast(int)height/2);
			lineTo(3, cast(int)height/2);
			lineTo(0, cast(int)height/2+3);
			lineTo(0, 3);
			closePath();
			fill();

			source = _state == ButtonState.Pressed ? Color.White : Color.Black;
			fontSize = 13;
			drawText(text, 5, height/2);
		}
		return;

	}
	public void paintFore(Graphics g) {
		auto extents = g.getTextExtents(text);
		g.drawText(text, (width-extents.width)/2, (height-extents.height)/2);
	}

public:
	/// Override this method in a subclass to handle the clicked event.
	protected void whenClicked(EventArgs e) { }
	/// This event occurs after the button has been clicked.
	Event!(whenClicked) clicked;

	///
	this() {
		clicked.setUp(&whenClicked);
		_focusable = true;
	}
	///
	this(string text) {
		this();
		this.text = text;
	}
	override Size bestSize() { return Theme.current.Button_bestSize(this); }
	///
	ButtonState state() { return _state; }
	///
	void state(ButtonState s) { _state = s; repaint(); }
	/**
	  * Returns true if this button should be painted as being default.
	  * Mainly for themes.
	  */
	bool isDefault() { return _isDefault; }
}
