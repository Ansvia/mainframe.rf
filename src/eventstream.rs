//! Event stream and listener implementation for $name_pascal_case$

extern crate event_stream;

use diesel::prelude::*;

use self::event_stream::{EventDispatcher, EventDispatcherBuilder, EventListener};
use crate::{chrono, db};

use std::{env, sync::Arc, thread::sleep, time::Duration};

/// Detax internal events
#[derive(Debug, Clone)]
pub enum Event {
    /// Event emited when service run on startup.
    Startup(),

    // @TODO(*): Add more events here
}

/// $name_pascal_case$ event listener implemetation
#[derive(Clone)]
struct $name_pascal_case$EventListener {
    db: db::DbConnMan,
}

impl_event_listener!($name_pascal_case$EventListener);

impl EventListener<Event> for $name_pascal_case$EventListener {
    fn dispatch(&self, event: &Event) {
        use self::Event::*;

        debug!("{:?} got event: {:?}", self, event);

        match event {
            Startup() => {
                debug!("on startup called");
            }
            // _ => (),
        }
    }
}

impl std::fmt::Debug for $name_pascal_case$EventListener {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(out, "<$name_pascal_case$EventListener>")
    }
}


lazy_static! {

    /// Event dispatcher global var
    pub static ref EVENT_DISPATCHER:EventDispatcher<Event> = {
        let event_dispatcher = EventDispatcherBuilder::new()
        .add_listener($name_pascal_case$EventListener::new())
        .build();

        event_dispatcher.start();
        event_dispatcher
    };
}

/// Emit event to the stream
pub fn emit(event: Event) {
    EVENT_DISPATCHER.emit(event)
}
