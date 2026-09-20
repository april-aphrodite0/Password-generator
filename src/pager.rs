// Copyright (C) 2026 april-aphrodite0, vubrixx, ghmaxx1k

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

// DEPENDENCIES
use std::io::Write;
use std::process::{Command, Stdio};
const CONDITIONS: &str = include_str!("GPL-2.0.txt");
const WARRANTY: &str = include_str!("GPL-nowarranty.txt");

// WARRANTY
pub fn warranty() {
    let mut pager = Command::new("less")
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute less")
    ;
    pager
        .stdin
        .as_mut()
        .unwrap()
        .write_all(WARRANTY.as_bytes())
        .expect("failed to write to less")
    ;
    pager.wait().expect("failed to wait for less");
}

// WARRANTY
pub fn conditions() {
    let mut pager = Command::new("less")
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute less")
    ;
    pager
        .stdin
        .as_mut()
        .unwrap()
        .write_all(CONDITIONS.as_bytes())
        .expect("failed to write to less")
    ;
    pager.wait().expect("failed to wait for less");
}