#[warn(missing_docs)]
#[warn(unsafe_code)]
use ::crossterm::event::Event;
use mockall_double::double;

// This exists to provide a mock context for read
// For some reason, this only works in the unit tests, not the integration
// tests.
pub mod crossterm {
    #[allow(unused_imports)]
    use mockall::automock;

    #[cfg_attr(test, automock)]
    pub mod event {
        pub fn read() -> ::crossterm::Result<::crossterm::event::Event> {
            return ::crossterm::event::read();
        }

        pub use crossterm::event::Event;
        pub use crossterm::event::KeyCode;
    }
}

#[double]
pub use crate::crossterm::event;

/// Sample widget to test crossterm events
pub struct MyWidget;

impl MyWidget {
    /// Process an event and return a result indicating whether the event was
    /// handled or unhandled
    pub fn handle_event(&mut self) -> Result<(), String> {
        let res = event::read();
        match res {
            Ok(ev) => match ev {
                Event::Key(_code) => Ok(()),
                _ => Err("Unknown Event".to_string()),
            },
            Err(e) => Err(format!("Error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    use mockall::*;
    use mockall_double::double;

    use crate::MyWidget;

    #[double]
    pub use crate::crossterm::event;

    lazy_static! {
        static ref EVENT_READ_MUTEX: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn handle_event_handles_key_event() {
        let _m = EVENT_READ_MUTEX.lock().unwrap();

        let context = event::read_context();

        let event = Event::Key(KeyEvent::new(
            KeyCode::Enter,
            KeyModifiers::NONE,
        ));
        context.expect().with().returning(move || {
            return ::crossterm::Result::Ok(event);
        });

        let mut widget = MyWidget;

        let event_res = widget.handle_event();

        assert_eq!(event_res, Ok(()));
    }
}
