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
    NewTagged(NotificationProps, String),
    CloseTagged(String),
}

pub enum NotificationAgentOutput {
    New(NotificationProps),
    NewTagged(NotificationProps, String),
    CloseTagged(String),
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
        use NotificationAgentInput::*;
        match msg {
            RegisterConsumer => {
                self.consumers.insert(id);
            }
            New(props) => {
                let mut props: NotificationProps = props; // TODO: type?
                props.standalone = false;
                self.consumers.iter().for_each(|&id| {
                    self.link
                        .respond(id, NotificationAgentOutput::New(props.clone()))
                })
            }
            NewTagged(props, tag) => {
                let mut props: NotificationProps = props;
                props.standalone = false;
                self.consumers.iter().for_each(|&id| {
                    self.link.respond(
                        id,
                        NotificationAgentOutput::NewTagged(props.clone(), tag.clone()),
                    )
                })
            }
            CloseTagged(tag) => self.consumers.iter().for_each(|&id| {
                self.link
                    .respond(id, NotificationAgentOutput::CloseTagged(tag.clone()))
            }),
        };
    }

    // TODO: register here instead?
    // fn connected(&mut self, _id: HandlerId) {

    // }

    fn disconnected(&mut self, id: HandlerId) {
        self.consumers.remove(&id);
    }
}
