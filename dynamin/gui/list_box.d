
/*
 * Copyright Jordan Miner
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

module dynamin.gui.list_box;

import dynamin.all_painting;
import dynamin.all_core;
import dynamin.all_gui;

/**
 * A control that shows a list of items that can be selected.
 *
 * The appearance of a list box with Windows Classic:
 *
 * $(IMAGE ../web/example_list_box.png)
 */
class ListBox : Scrollable {
protected:
	List!(mstring, true) _items;
	int _selectedIndex = -1;

	override void whenKeyDown(KeyEventArgs e) {
			if(e.key == Key.Down) {
				if(selectedIndex + 1 < _items.count)
					selectedIndex = selectedIndex + 1;
			} else if(e.key == Key.Up) {
				if(selectedIndex - 1 >= 0)
					selectedIndex = selectedIndex - 1;
			}
		}
	override Size bestSize() {
		return Size(0, 13*_items.count);
	}
	override void whenContentPainting(PaintingEventArgs e) {
		with(e.graphics) {
			source = backColor;
			paint();
			auto clip = getClipExtents();
			int start = cast(int)clip.y/13, end = cast(int)clip.bottom/13+1;
			for(int i = start; i < _items.count && i < end; ++i) {
				source = Color.Black;
				if(i == _selectedIndex) {
					// TODO: hack
					//Source = WindowsTheme.getColor(dynamin.c.windows.COLOR_HIGHLIGHT);
					rectangle(0, i*13, width, 13);
					fill();
					//Source = WindowsTheme.getColor(dynamin.c.windows.COLOR_HIGHLIGHTTEXT);
				}
				drawText(_items[i], 3, i*13);
			}
		}
	}
	override void whenContentMouseDown(MouseEventArgs e) {
		focus();
		selectedIndex = cast(int)e.y/13;
	}
public:

	/// Override this method in a subclass to handle the selectionChanged event.
	protected void whenSelectionChanged(EventArgs e) { }
	/// This event occurs after the selection has changed.
	Event!(whenSelectionChanged) selectionChanged;

	void whenListItemsChanged(ListChangeType, mstring, mstring, word) {
		super.layout();
		repaint();
	}

	///
	this() {
		selectionChanged.setUp(&whenSelectionChanged);
		_items = new List!(mstring, true)(&whenListItemsChanged);

		super();
		_focusable = true;
		// TODO: hack
		backColor = WindowsTheme.getColor(5);
	}
	///
	List!(mstring, true) items() {
		return _items;
	}
	///
	int selectedIndex() { return _selectedIndex; }
	/// ditto
	void selectedIndex(int i) {
		if(i == _selectedIndex)
			return;
		_selectedIndex = i;
		repaint();
		selectionChanged(new EventArgs);
	}
}

