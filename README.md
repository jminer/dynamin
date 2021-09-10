
Dynamin is a GUI library written in Rust. It doesn't use native controls but has a priority to look and feel native.

## Goals

- The default UI should look and behave natively. Not just similar to native, but almost indistinguishable.

  Actually using native controls isn't necessarily a requirement and has many drawbacks. As long as developers pay attention to details, it should be possible to make a UI that doesn't use native controls but still looks and feels native. A good example is Qt Widgets on Windows (non-native, but almost perfect), and a bad example is Java Swing (looks close, but lots of small differences).

  Other custom themes may be supported, and advanced graphics should be possible. However, a native looking UI is a top priority.

- The UI should be efficient. It should be optimized to use little system memory and not excessive GPU memory. It should be very responsive. The UI should paint in a few milliseconds, fast enough to render at 120Hz.

  As an example, WPF is a modern GUI rendered using Direct3D. It should be as fast or faster than old Win32 controls. However, in practice, it is usually slower in typical apps even though it's "hardware accelerated." Similarly, the Windows 10 UI feels much slower than Windows XP did even though computers are easily an order of magnitude faster than 20 years ago.

- It should be possible to do everything in Rust. No other scripting language should be necessary. There may a DSL for defining layouts or other things, but it will be optional.

## Origin

I started work on Dynamin 10/1/2014 after working on a version in D previously.
