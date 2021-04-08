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

                    self.notifications.insert(id, None, position, props);
                    true
                }

                NotificationAgentOutput::NewTagged(mut props, tag) => {
                    let position = props.position;
                    let id = self.notifications.next_id(position);
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
                    self.notifications.insert(id, Some(tag), position, props);
                    true
                }

                NotificationAgentOutput::CloseTagged(tag) => {
                    self.notifications.remove_tag(&tag);
                    true
                }
            },
            Msg::Closed(position, id, callback) => {
                yew_services::ConsoleService::log("closed");
                self.notifications.remove_id(position, id);
                if let Some(callback) = callback {
                    callback.emit(());
                }
                true
            }
            Msg::TimedOut(position, id, callback) => {
                yew_services::ConsoleService::log("timed out");
                self.notifications.remove_id(position, id);
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

    fn insert(
        &mut self,
        id: usize,
        tag: Option<String>,
        position: Position,
        props: NotificationProps,
    ) {
        match position {
            Position::TopLeft => self.tl.insert(id, tag, props),
            Position::TopRight => self.tr.insert(id, tag, props),
            Position::BottomLeft => self.bl.insert(id, tag, props),
            Position::BottomRight => self.br.insert(id, tag, props),
        }
    }

    fn remove_id(&mut self, position: Position, id: usize) {
        match position {
            Position::TopLeft => self.tl.remove_id(id),
            Position::TopRight => self.tr.remove_id(id),
            Position::BottomLeft => self.bl.remove_id(id),
            Position::BottomRight => self.br.remove_id(id),
        }
    }

    fn remove_tag(&mut self, tag: &str) {
        self.tl.remove_tag(tag);
        self.tr.remove_tag(tag);
        self.bl.remove_tag(tag);
        self.br.remove_tag(tag);
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
    items: Vec<(usize, Option<String>, NotificationProps)>,
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

    fn insert(&mut self, id: usize, tag: Option<String>, props: NotificationProps) {
        // TODO: ensure tag uniqueness?
        self.items.push((id, tag, props));
    }

    fn remove_id(&mut self, id: usize) {
        for i in 0..self.items.len() {
            if self.items[i].0 == id {
                // self.items.swap_remove(i); // do we need to preserve order?
                self.items.remove(i);
                return;
            }
        }
    }

    /// Exhaustive (unlike remove_id, which removes the first match)
    fn remove_tag(&mut self, tag: &str) {
        for i in 0..self.items.len() {
            if let Some(ref t) = self.items[i].1 {
                if t == tag {
                    self.items.remove(i);
                }
            }
        }
    }

    fn view(&self, position: Position) -> Html {
        if self.items.is_empty() {
            html! {}
        } else {
            html! {
                <div class={position.style()}>
                    { self.items.iter().map(|(id, _tag, props)| {
                        let props = props.clone();
                        html! { <Notification key={*id} with props /> }
                    } ).collect::<Html>() }
                </div>
            }
        }
    }
}
