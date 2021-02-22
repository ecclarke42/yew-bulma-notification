use yew::prelude::*;

use crate::{
    Notification, NotificationAgent, NotificationAgentInput, NotificationAgentOutput,
    NotificationProps, Position,
};

pub struct NotificationConsumer {
    link: ComponentLink<Self>,
    _bridge: Box<dyn Bridge<NotificationAgent>>,
    notifications: NotificationCollection,
}

pub enum Msg {
    ServiceMsg(NotificationAgentOutput),

    Closed(Position, usize, Option<Callback<()>>),
    TimedOut(Position, usize, Option<Callback<()>>),
}

impl Component for NotificationConsumer {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut bridge = NotificationAgent::bridge(link.callback(Msg::ServiceMsg));
        bridge.send(NotificationAgentInput::RegisterConsumer);
        Self {
            link,
            _bridge: bridge,
            notifications: NotificationCollection::new(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ServiceMsg(msg) => match msg {
                NotificationAgentOutput::New(mut props) => {
                    let position = props.position;
                    let id = self.notifications.next_id(position);

                    // Wrap the closed and timeout callbacks in callbacks for
                    // this component, so we can control rendering
                    let callback = props.on_closed.take();
                    props.on_closed = Some(
                        self.link
                            .callback_once(move |_| Msg::Closed(position, id, callback)),
                    );

                    let callback = props.on_timeout.take();
                    props.on_timeout = Some(
                        self.link
                            .callback_once(move |_| Msg::TimedOut(position, id, callback)),
                    );

                    // Add to collection and re-render
                    self.notifications.insert(position, id, props);
                    true
                }
            },
            Msg::Closed(position, id, callback) => {
                yew_services::ConsoleService::log("closed");
                self.notifications.remove(position, id);
                if let Some(callback) = callback {
                    callback.emit(());
                }
                true
            }
            Msg::TimedOut(position, id, callback) => {
                yew_services::ConsoleService::log("timed out");
                self.notifications.remove(position, id);
                if let Some(callback) = callback {
                    callback.emit(());
                }
                true
            }
        }
    }

    fn view(&self) -> Html {
        // TODO: conditionally render position divs

        let (top_left, top_right, bottom_left, bottom_right) = self.notifications.by_position();

        html! {
            <>
                {top_left.view(Position::TopLeft)}
                {top_right.view(Position::TopRight)}
                {bottom_left.view(Position::BottomLeft)}
                {bottom_right.view(Position::BottomRight)}
            </>
        }
    }
}

/// Helper struct for managing the notifications
struct NotificationCollection {
    tl: NotificaitonList,
    tr: NotificaitonList,
    bl: NotificaitonList,
    br: NotificaitonList,
}

impl NotificationCollection {
    fn new() -> Self {
        Self {
            tl: NotificaitonList::new(),
            tr: NotificaitonList::new(),
            bl: NotificaitonList::new(),
            br: NotificaitonList::new(),
        }
    }

    fn next_id(&mut self, position: Position) -> usize {
        match position {
            Position::TopLeft => self.tl.next_id(),
            Position::TopRight => self.tr.next_id(),
            Position::BottomLeft => self.bl.next_id(),
            Position::BottomRight => self.br.next_id(),
        }
    }

    fn insert(&mut self, position: Position, id: usize, props: NotificationProps) {
        match position {
            Position::TopLeft => self.tl.insert(id, props),
            Position::TopRight => self.tr.insert(id, props),
            Position::BottomLeft => self.bl.insert(id, props),
            Position::BottomRight => self.br.insert(id, props),
        }
    }

    fn remove(&mut self, position: Position, id: usize) {
        match position {
            Position::TopLeft => self.tl.remove(id),
            Position::TopRight => self.tr.remove(id),
            Position::BottomLeft => self.bl.remove(id),
            Position::BottomRight => self.br.remove(id),
        }
    }

    /// Vectors of NotificationProps ordered by position
    /// (TopLeft, TopRight, BottomLeft, BottomRight)
    fn by_position(
        &self,
    ) -> (
        &NotificaitonList,
        &NotificaitonList,
        &NotificaitonList,
        &NotificaitonList,
    ) {
        (&self.tl, &self.tr, &self.bl, &self.br)
    }
}

struct NotificaitonList {
    next_id: usize,
    items: Vec<(usize, NotificationProps)>,
}

impl NotificaitonList {
    fn new() -> Self {
        Self {
            next_id: 0,
            items: Vec::new(),
        }
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn insert(&mut self, id: usize, props: NotificationProps) {
        self.items.push((id, props))
    }

    fn remove(&mut self, id: usize) {
        for i in 0..self.items.len() {
            if self.items[i].0 == id {
                // self.items.swap_remove(i); // do we need to preserve order?
                self.items.remove(i);
                return;
            }
        }
    }

    fn view(&self, position: Position) -> Html {
        if self.items.is_empty() {
            html! {}
        } else {
            html! {
                <div class={position.style()}>
                    { self.items.iter().map(|(id, props)| {
                        let props = props.clone();
                        html! { <Notification key={*id} with props /> }
                    } ).collect::<Html>() }
                </div>
            }
        }
    }
}
