# mockall-mock-external-fn-in-integration-test #

This application tests mocking external functions in both unit and
integration tests.

Currently, we can create a _context wrapper in the unit tests, but not
in the integration tests.

The unit test code is in src/lib.rs
The integration test code is in tests/mywidget_handle_event.rs

Trying to mock the crossterm read function, found in the namespace:

crossterm::event::read

We created a wrapper in src/lib.rs for read in the crossterm and event modules.


The errors we're getting are either `no mock_event` in `crossterm`
or
cannot find function `read_context` in module `event`
