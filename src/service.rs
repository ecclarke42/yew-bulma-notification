use std::collections::HashSet;

use yew::worker::*;

use crate::NotificationProps;

// TODO: rename to agent, create a service wrapper type for easy interaction

pub struct NotificationService {
    link: AgentLink<NotificationService>,
    consumers: HashSet<HandlerId>,
}

pub enum NotificationServiceInput {
    RegisterConsumer,
    New(NotificationProps),
}

pub enum NotificationServiceOutput {
    New(NotificationProps),
}

impl Agent for NotificationService {
    type Reach = Context<Self>;
    type Message = ();
    type Input = NotificationServiceInput;
    type Output = NotificationServiceOutput;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            consumers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            NotificationServiceInput::RegisterConsumer => {
                self.consumers.insert(id);
            }
            NotificationServiceInput::New(props) => {
                let mut props: NotificationProps = props; // TODO: type?
                props.standalone = false;
                self.consumers.iter().for_each(|&id| {
                    self.link
                        .respond(id, NotificationServiceOutput::New(props.clone()))
                })
            }
        };
    }

    // TODO: register here instead?
    // fn connected(&mut self, _id: HandlerId) {

    // }

    fn disconnected(&mut self, id: HandlerId) {
        self.consumers.remove(&id);
    }
}
