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

    // Helper methods
}
