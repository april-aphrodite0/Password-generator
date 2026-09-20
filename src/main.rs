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
use std::io;
use std::io::Write;
mod pager;
const NOTICE: &str = include_str!("GPL-notice-initprompt.txt");

// MAIN
fn main() {
    
    // VARIABLES
    let mut user_prompt: String = String::new();

    // NOTICE AND INITIAL PROMPT
    print!("{}\n\nuser: ", NOTICE);

    // USER INPUT
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_prompt).expect("failed to read line");
    let user_prompt_string: &str = user_prompt.trim();

    // USER PROMPT MATCHER
    match user_prompt_string {

        // WARRANTY
        "show warranty" => {
            pager::warranty();
        }

        
        // CONDITIONS
        "show conditions" => {
            pager::conditions();
        }

        // NO INPUT
        _ => {}

    }

}