use yew::prelude::*;
use yew_services::timeout::{TimeoutService, TimeoutTask};

use super::properties::{Color, Position, Size};

#[derive(Properties, Clone, PartialEq)]
pub struct NotificationProps {
    #[prop_or_default]
    pub header: Option<String>,

    pub children: Children,

    #[prop_or(true)]
    pub can_close: bool,

    #[prop_or_default]
    pub timeout: Option<std::time::Duration>,
    #[prop_or_default]
    pub on_timeout: Option<Callback<()>>,
    #[prop_or_default]
    pub on_closed: Option<Callback<()>>,

    #[prop_or(Color::Default)]
    pub color: Color,
    #[prop_or(Size::Normal)]
    pub size: Size,
    #[prop_or(Position::BottomRight)]
    pub position: Position,

    // pub margin: Option<
    /// This should be set (false) by the notification service to handle
    /// positioning of multiple notifications. By default
    #[prop_or(true)]
    pub(crate) standalone: bool,
}

/// A bulma [message](https://bulma.io/documentation/components/message/)
/// that will be displayed as a notification
pub struct Notification {
    closed: bool,
    timed_out: bool,

    props: NotificationProps,
    link: ComponentLink<Self>,
    _timeout: Option<TimeoutTask>,
}

pub enum Msg {
    TimedOut,
    Closed,
    Rendered,
    TimeoutAnimated,
    CloseAnimated,
    DisplayAnimated,
}

impl Component for Notification {
    type Message = Msg;
    type Properties = NotificationProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _timeout = props
            .timeout
            .map(|d| TimeoutService::spawn(d, link.callback(|_| Msg::TimedOut)));
        Self {
            closed: false,
            timed_out: false,
            props,
            _timeout,
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Rendered => false,
            Msg::TimedOut => {
                self.timed_out = true;
                true
            }
            Msg::TimeoutAnimated => {
                if let Some(ref on_timeout) = self.props.on_timeout {
                    on_timeout.emit(())
                }
                false
            }
            Msg::Closed => {
                self.closed = true;
                true
            }
            Msg::CloseAnimated => {
                if let Some(ref on_closed) = self.props.on_closed {
                    on_closed.emit(())
                }
                false
            }
            Msg::DisplayAnimated => false,
        }
    }

    fn view(&self) -> Html {
        // TODO: better way of putting this in the html?
        // TODO: margin property?
        // let style = if self.props.standalone {
        //     self.props.position.style("1em", "1em")
        // } else {
        //     String::new()
        // };

        // Figure out which classes to render
        let mut msg_cls = Classes::from("message ybn-body");
        let mut del_cls = Classes::from("delete");
        if let Some(cls) = self.props.color.class() {
            msg_cls.push(cls);
        }
        if let Some(cls) = self.props.size.class() {
            msg_cls.push(cls);
            del_cls.push(cls);
        }
        if self.props.standalone {
            msg_cls.push(self.props.position.style());
        }

        // Animation class
        let animation_callback = self.add_animation_classes(&mut msg_cls);

        // Show the header if there is a header string, or if "can-close" is true
        let header = if self.props.header.is_some() || self.props.can_close {
            let header = self.props.header.clone().unwrap_or_default();
            let button = if self.props.can_close {
                html! {<button class={del_cls} aria-label="delete" onclick={self.link.callback(|_| Msg::Closed)}></button>}
            } else {
                html! {}
            };
            html! {
                <div class="message-header">
                    <p>{header}</p>
                    {button}
                </div>
            }
        } else {
            html! {}
        };

        html! {
            <article class={msg_cls} onanimationend={animation_callback}>
                {header}
                <div class="message-body">
                    {self.props.children.clone()}
                </div>
            </article>
        }
    }
}

impl Notification {
    fn add_animation_classes(
        &self,
        classes: &mut Classes,
    ) -> Callback<yew::web_sys::AnimationEvent> {
        if self.closed {
            classes.push(self.props.position.animate_out_style());
            self.link.callback(|_| Msg::CloseAnimated)
        } else if self.timed_out {
            classes.push(self.props.position.animate_out_style());
            self.link.callback(|_| Msg::TimeoutAnimated)
        } else {
            classes.push(self.props.position.animate_in_style());
            self.link.callback(|_| Msg::DisplayAnimated)
        }
    }
}
