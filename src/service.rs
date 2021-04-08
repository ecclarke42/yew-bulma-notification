use yew::prelude::*;

use crate::agent::{NotificationAgent, NotificationAgentInput};
use crate::notification::NotificationProps;

pub struct NotificationService {
    dispatcher: yew::agent::Dispatcher<NotificationAgent>,
}

impl Default for NotificationService {
    fn default() -> Self {
        Self {
            dispatcher: NotificationAgent::dispatcher(),
        }
    }
}

impl NotificationService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self, props: NotificationProps) {
        self.dispatcher.send(NotificationAgentInput::New(props))
    }

    /// Spawn a notification with a specific id, so that it can be manually closed.
    /// It is the user's responsibility to use unique id's.
    /// TODO: don't force responsibility onto the user?
    pub fn spawn_with_id(&mut self, props: NotificationProps, id: String) {
        self.dispatcher
            .send(NotificationAgentInput::NewTagged(props, id))
    }

    pub fn close_id(&mut self, id: String) {
        self.dispatcher
            .send(NotificationAgentInput::CloseTagged(id))
    }

    // Helper methods
}
