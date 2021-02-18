use yew::prelude::*;

use yew_bulma_notification::Notification;

fn main() {
    yew::start_app::<App>();
}

/// Simple yew application that spawns a notification
pub struct App {
    notify: bool,

    link: ComponentLink<Self>,
}

pub enum Msg {
    OpenNotification,
    CloseNotification,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            notify: false,
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenNotification => {
                self.notify = true;
                true
            }
            Msg::CloseNotification => {
                self.notify = false;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let close_notification = self.link.callback(|_| Msg::CloseNotification);
        html! {
            <main class="ybn-parent">
                <h1>{"Single Notification Example"}</h1>
                <button onclick={self.link.callback(|_| Msg::OpenNotification)}>{"Notify Me"}</button>

                {if self.notify {
                    html! {
                        <Notification
                            header="a message"
                            timeout={std::time::Duration::from_secs(5)}
                            on_timeout={close_notification.clone()}
                            on_closed={close_notification.clone()}
                        >
                            <p>{"body"}</p>
                            <p>{"body"}</p>
                            <p>{"body"}</p>
                        </Notification>
                    }
                } else {
                    html!{}
                }}
            </main>
        }
    }
}
