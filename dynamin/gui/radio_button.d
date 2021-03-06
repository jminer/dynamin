
/*
 * Copyright Jordan Miner
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

module dynamin.gui.radio_button;

import dynamin.all_core;
import dynamin.all_gui;
import dynamin.all_painting;
import tango.io.Stdout;

/**
 * A control that can be checked only if other radio buttons are unchecked.
 *
 * The appearance of a radio button with Windows Classic:
 *
 * $(IMAGE ../web/example_radio_button.png)
 */
class RadioButton : CheckBox {
protected:
	int _group = 1;
	RadioButton[] collectGroup(ref int checkedIndex) {
		Window topLevel = cast(Window)getTopLevel();
		if(!topLevel)
			return null;
		RadioButton[] radios;
		void collectFromContainer(Container container) {
			foreach(control; container) {
				if(auto r = cast(RadioButton)control) {
					if(r.group != group)
						continue;
					radios.length = radios.length + 1;
					radios[$-1] = r;
					if(r.checked)
						checkedIndex = radios.length-1;
				} else if(auto c = cast(Container)control)
					collectFromContainer(c);
			}
		}
		checkedIndex = -1;
		collectFromContainer(topLevel.content);
		return radios;
	}
	override void whenKeyDown(KeyEventArgs e) {
		// TODO: when getTopLevel() is changed to return NativeControl,
		// update this
		int index;
		if(e.key == Key.Down || e.key == Key.Right) {
			RadioButton[] radios = collectGroup(index);
			if(radios is null)
				return;
			if(++index >= radios.length)
				index = 0;
			radios[index].clicked(new EventArgs);
			e.stopped = true;
		} else if(e.key == Key.Up || e.key == Key.Left) {
			RadioButton[] radios = collectGroup(index);
			if(radios is null)
				return;
			if(--index < 0)
				index = radios.length-1;
			radios[index].clicked(new EventArgs);
			e.stopped = true;
		}
	}
	override void whenPainting(PaintingEventArgs e) {
		Theme.current.RadioButton_paint(this, e.graphics);
	}
	override void whenClicked(EventArgs e) {
		int index;
		RadioButton[] radios = collectGroup(index);
		foreach(r; radios)
			r.checked = false;
		checked = true;
		focus();

	}
public:
	this() {
		_focusable = false;
	}
	this(string text) {
		this();
		this.text = text;
	}
	override Size bestSize() {
		return Size(70, 15);
	}
	alias CheckBox.checked checked;
	override void checked(bool b) {
		focusable = b;
		super.checked(b);
	}
	/**
	 * Gets or sets what group this radio button is a part of. The default is 1.
	 */
	int group() { return _group; }
	/// ditto
	void group(int i) { _group = i; }
}

/**
 * Simply sets every radio button to have the specified group.
 *
 * Example:
 * -----
 * auto radioButton1 = new RadioButton;
 * auto radioButton2 = new RadioButton;
 * setGroup(7, radioButton1, radioButton2);
 * -----
 */
void setGroup(int group, RadioButton[] radios...) {
	foreach(r; radios)
		r.group = group;
}

