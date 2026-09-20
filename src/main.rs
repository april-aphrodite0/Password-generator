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
mod generator;
const NOTICE: &str = include_str!("GPL-notice-initprompt.txt");

// MAIN
fn main() {

    // PRINT NOTICE ONCE AT STARTUP
    println!("{}", NOTICE);

    // MAIN LOOP
    loop {

        // PROMPT
        print!("\nuser: ");
        io::stdout().flush().unwrap();

        // VARIABLES
        let mut user_prompt: String = String::new();

        // USER INPUT
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

            // GENERATE PASSWORD
            "generate" => {
                generator::createpassword();
            }

            // HELP
            "help" => {
                println!(
                    "available commands:\n\
                     \x20 generate         - generate a password\n\
                     \x20 show warranty    - show the warranty notice\n\
                     \x20 show conditions  - show the license conditions\n\
                     \x20 help             - show this message\n\
                     \x20 exit / quit      - exit the program"
                );
            }

            // EXIT
            "exit" | "quit" => {
                return();
            }
            
            // NO INPUT
            "" => {}

            // UNKNOWN COMMAND
            _ => {
                println!("unknown command, type 'help' for a list");
            }

        }

    }

}