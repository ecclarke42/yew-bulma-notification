use std::collections::HashSet;

use yew::worker::*;

use crate::NotificationProps;

pub struct NotificationAgent {
    link: AgentLink<Self>,
    consumers: HashSet<HandlerId>,
}

pub enum NotificationAgentInput {
    RegisterConsumer,
    New(NotificationProps),
}

pub enum NotificationAgentOutput {
    New(NotificationProps),
}

impl Agent for NotificationAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = NotificationAgentInput;
    type Output = NotificationAgentOutput;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            consumers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            NotificationAgentInput::RegisterConsumer => {
                self.consumers.insert(id);
            }
            NotificationAgentInput::New(props) => {
                let mut props: NotificationProps = props; // TODO: type?
                props.standalone = false;
                self.consumers.iter().for_each(|&id| {
                    self.link
                        .respond(id, NotificationAgentOutput::New(props.clone()))
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
