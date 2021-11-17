#[warn(missing_docs)]
#[warn(unsafe_code)]

use std::process::exit;
use log::error;
use env_logger;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use mockall_mock_external_fn_in_integration_test::MyWidget;

fn main() {
    // initialize the logger
    match env_logger::try_init() {
        Ok(_e) => { },
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        },
    }

    // enable raw mode so we can get single key events
    match enable_raw_mode() {
        Ok(_e) => { },
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        },
    }

    let mut widget = MyWidget;

    println!("Press a key:");

    let res = widget.handle_event();
    
    match res {
        Ok(e) => {
            println!("Handled event: {:?}", e);
        },
        Err(e) => {
            error!("Invalid or unhandled event: {}", e);
        }
    }

    // return the terminal to normal mode
    match disable_raw_mode() {
        Ok(_e) => { },
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        },
    }

    exit(0);
}
