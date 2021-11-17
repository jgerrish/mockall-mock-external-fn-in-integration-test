use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use mockall::*;
#[cfg(test)]
use std::sync::Mutex;

use mockall_double::double;

// When #[double] isn't commented out, this generates an error:
// "no mock_event in `crossterm`
#[double]
pub use mockall_mock_external_fn_in_integration_test::crossterm::event;

use mockall_mock_external_fn_in_integration_test::MyWidget;

lazy_static! {
    static ref EVENT_READ_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
// when #[automock] is used in lib.rs instead of cfg_attr(test, automock),
// the #[double] attribute above works, and the read_context below works,
// but the test fails because the event::read in MyWidget.handle_event
// calls the real version, not the mocked / context version
#[ignore]
fn handle_event_handles_key_event() {
    let _m = EVENT_READ_MUTEX.lock().unwrap();

    // When #[double] is commented out, this returns an error:
    // "cannot find function `read_context` in module `event`
    let context = event::read_context();

    let event = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    context.expect().with().returning(move || {
        return ::crossterm::Result::Ok(event);
    });

    let mut widget = MyWidget;

    let event_res = widget.handle_event();

    assert_eq!(event_res, Ok(()));
}
