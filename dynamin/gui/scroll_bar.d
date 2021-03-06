
/*
 * Copyright Jordan Miner
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

module dynamin.gui.scroll_bar;

import dynamin.all_core;
import dynamin.all_gui;
import dynamin.all_painting;
import tango.io.Stdout;

enum ArrowDirection {
	Left, Right, Up, Down
}
class ArrowButton : Button {
	ArrowDirection _direction;
	ArrowDirection direction() { return _direction; }
	void direction(ArrowDirection dir) { _direction = dir; }
	override void whenPainting(PaintingEventArgs e) {
		Theme.current.ArrowButton_paint(this, e.graphics);
	}
}
class ScrollBarTrack : Button {
	override void whenPainting(PaintingEventArgs e) {
		Theme.current.ScrollBarTrack_paint(this, e.graphics);
	}
}
class ScrollBarThumb : Button {
	override void whenMouseDown(MouseEventArgs e) {
	}
	override void whenMouseDragged(MouseEventArgs e) {
	}
	override void whenPainting(PaintingEventArgs e) {
		Theme.current.ScrollBarThumb_paint(this, e.graphics);
	}
}
//debug=ScrollBar;

///
abstract class ScrollBar : Container {
protected:
	ScrollBarTrack _track1, _track2;
	ScrollBarThumb _thumb;
	ArrowButton _button1, _button2;
	int _value = 0, _maxValue = 100;
	double _visibleValue = 0.5;
	double _thumbDragLoc = -1;
	// stores the location of the thumb as a percentage of the track
	double _thumbPos;
	this() {
		valueChanged.setUp(&whenValueChanged);

		_track1 = new ScrollBarTrack;
		_track2 = new ScrollBarTrack;
		_thumb = new ScrollBarThumb;
		_button1 = new ArrowButton;
		_button2 = new ArrowButton;
		_track1.mouseDown += &whenTrack1MouseDown;
		_track2.mouseDown += &whenTrack2MouseDown;
		_thumb.mouseDown += &whenThumbMouseDown;
		_thumb.mouseUp += &whenThumbMouseUp;
		_thumb.mouseDragged += &whenThumbMouseDragged;
		_button1.mouseDown += &whenButton1MouseDown;
		_button2.mouseDown += &whenButton2MouseDown;
		add(_track1);
		add(_track2);
		add(_thumb);
		add(_button1);
		add(_button2);
		size = _size;
	}
	void whenThumbMouseDown(MouseEventArgs e);
	void whenThumbMouseDragged(MouseEventArgs e);
	void whenThumbMouseUp(MouseEventArgs e) {
		// This gives the behavior of the thumb jumping when MaxValue
		// is smaller than the track to exactly represent where
		// Value is, as jEdit and some of Windows' controls do.
		_thumbPos = 0;
		updateControls();
	}
	void whenTrack1MouseDown(MouseEventArgs e) {
		value = value - 100;
	}
	void whenTrack2MouseDown(MouseEventArgs e) {
		value = value + 100;
	}
	void whenButton1MouseDown(MouseEventArgs e) {
		value = value - 10;
	}
	void whenButton2MouseDown(MouseEventArgs e) {
		value = value + 10;
	}
	void putControl(Control c, double location, double size);
	double breadth();
	double length();
	override void whenResized(EventArgs e) {
		updateControls();
	}
	void layoutControls(Control[] controls, double[] sizes) {
		assert(controls.length == sizes.length);
		double loc = 0;
		for(int i = 0; i < controls.length; ++i) {
			putControl(controls[i], loc, sizes[i]);
			loc += sizes[i];
		}
	}
	// updates controls based on what value, visible value, and max value are
	void updateControls() {
		if(breadth*2 > length) {
			return;
			// no track or thumb
		}
		// if thumbPos does not represent the current value
		if(cast(int)(_thumbPos * _maxValue) != _value)
			_thumbPos = _value / cast(double)_maxValue;
		auto totalSz = length;
		auto buttonSz = breadth;
		auto totalTrackSz = totalSz - buttonSz*2;
		auto thumbSz = round(totalTrackSz*_visibleValue);
		auto thumbLoc = buttonSz+(totalTrackSz-thumbSz)*_thumbPos;
		if(thumbSz < 8)
			thumbSz = 8;
		auto track1Sz = thumbLoc-buttonSz;
		auto track2Sz = totalTrackSz-track1Sz-thumbSz;
		if(track1Sz < 0) track1Sz = 0;
		if(track2Sz < 0) track2Sz = 0;
debug(ScrollBar) {
		Stdout.format("value={}", value).newline;
		Stdout.format("visibleValue={}", visibleValue).newline;
		Stdout.format("maxValue={}", maxValue).newline;
		Stdout.format("totalSz={}", totalSz).newline;
		Stdout.format("buttonSz={}", buttonSz).newline;
		Stdout.format("totalTrackSz={}", totalTrackSz).newline;
		Stdout.format("thumbSz={}", thumbSz).newline;
		Stdout.format("track1Sz={}", track1Sz).newline;
		Stdout.format("track2Sz={}", track2Sz).newline;
		Stdout("********").newline;
		assert(2*buttonSz+track1Sz+thumbSz+track2Sz <= totalSz + 0.01);
}
		layoutControls([cast(Control)
		               _button1, _track1,  _thumb,  _track2,  _button2],
		               [buttonSz, track1Sz, thumbSz, track2Sz, buttonSz]);
	}
public:
	/// Override this method in a subclass to handle the ValueChanged event.
	protected void whenValueChanged(EventArgs e) { }
	/// This event occurs after Value has been changed.
	Event!(whenValueChanged) valueChanged;

	override Size bestSize() {
		if(cast(VScrollBar)this)
			return Size(Theme.current.ScrollBar_size(), 100);
		else
			return Size(100, Theme.current.ScrollBar_size());
	}
	///
	double thumbLocation();
	/// ditto
	void thumbLocation(double loc) {
		// TODO: return if no thumb (too small for one)
		if(loc < trackStart)
			loc = trackStart;
		if(loc > trackEnd - thumbSize)
			loc = trackEnd - thumbSize;
		if(floatsEqual(loc, thumbLocation, 0.1))
			return;

		_thumbPos = (loc - trackStart) / (trackSize - thumbSize);
		value = cast(int)(_thumbPos * _maxValue);
		updateControls();
	}
	///
	double thumbSize();
	///
	double trackStart();
	///
	double trackEnd();
	///
	double trackSize() { return trackEnd-trackStart; }
	///
	int value() { return _value; }
	/// ditto
	void value(int val) {
		if(val < 0)
			val = 0;
		else if(val > _maxValue)
			val = _maxValue;
		if(val == _value)
			return;
		_value = val;
		updateControls();
		valueChanged(new EventArgs);
	}
	///
	int maxValue() { return _maxValue; }
	/// ditto
	void maxValue(int val) {
		if(val < 1)
			val = 1;
		if(val == _maxValue)
			return;
		_maxValue = val;
		if(_value > _maxValue)
			_value = _maxValue;
		if(_visibleValue > _maxValue)
			_visibleValue = _maxValue;
		updateControls();
	}
	/**
	 * A floating-point number between 0 and 1 that specifies how large the
	 * thumb should be compared to the track. A value of 0 makes the thumb its
	 * minimum size, and a value of 1 makes the thumb take up all of the track.
	 * The default is 0.5.
	 */
	double visibleValue() { return _visibleValue; }
	/// ditto
	void visibleValue(double val) {
		if(val < 0)
			val = 0;
		else if(val > 1)
			val = 1;
		if(val == _visibleValue)
			return;
		_visibleValue = val;
		updateControls();
	}
}
///
class HScrollBar : ScrollBar {
	this() {
		_button1.direction = ArrowDirection.Left;
		_button2.direction = ArrowDirection.Right;
	}
protected:
	override void whenThumbMouseDown(MouseEventArgs e) {
		_thumbDragLoc = e.location.x;
	}
	override void whenThumbMouseDragged(MouseEventArgs e) {
		_thumb.state = ButtonState.Pressed;
		thumbLocation = e.location.x + _thumb.location.x - _thumbDragLoc;
	}
	override void putControl(Control c, double location, double size) {
		c.location = [location, 0.0];
		c.size = [size, height];
	}
	override double breadth() { return height; }
	override double length() { return width; }
	alias ScrollBar.thumbLocation thumbLocation;
	override double thumbLocation() { return _thumb.x; }
	override double thumbSize() { return _thumb.width; }
	override double trackStart() { return _track1.x; }
	override double trackEnd() { return _track2.x+_track2.width; }
}
///
class VScrollBar : ScrollBar {
	this() {
		_button1.direction = ArrowDirection.Up;
		_button2.direction = ArrowDirection.Down;
	}
protected:
	override void whenThumbMouseDown(MouseEventArgs e) {
		_thumbDragLoc = e.location.y;
	}
	override void whenThumbMouseDragged(MouseEventArgs e) {
		_thumb.state = ButtonState.Pressed;
		thumbLocation = e.location.y + _thumb.location.y - _thumbDragLoc;
	}
	override void putControl(Control c, double location, double size) {
		c.location = [0.0, location];
		c.size = [width, size];
	}
	override double breadth() { return width; }
	override double length() { return height; }
	alias ScrollBar.thumbLocation thumbLocation;
	override double thumbLocation() { return _thumb.y; }
	override double thumbSize() { return _thumb.height; }
	override double trackStart() { return _track1.y; }
	override double trackEnd() { return _track2.y+_track2.height; }
}

