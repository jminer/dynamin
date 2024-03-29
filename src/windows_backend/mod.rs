/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

pub use self::cursor_backend::CursorBackend;
pub use self::window_backend::WindowBackend;

pub mod cursor_backend;
pub mod window_backend;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use smallvec::SmallVec;

pub fn str_to_wide_vec<'a: 'b, 'b, A>(s: &'a str, buf: &'b mut SmallVec<A>) -> *const u16
where A: ::smallvec::Array<Item=u16>
{
    buf.extend(OsStr::new(s).encode_wide().map(|c| if c == 0 { b'?' as u16 } else { c }));
    buf.push(0);
    (&buf[..]).as_ptr() as *const u16
}
