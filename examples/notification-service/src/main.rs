use yew::prelude::*;

use yew_bulma_notification::{
    Color, NotificationConsumer, NotificationProps, NotificationService, Position,
};

fn main() {
    yew::start_app::<App>();
}

/// Simple yew application that spawns a notification
pub struct App {
    notification_service: NotificationService,
    link: ComponentLink<Self>,
}

pub enum Msg {
    NotifyTL,
    NotifyTR,
    NotifyBL,
    NotifyBR,

    BridgeMsg,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            notification_service: NotificationService::new(),
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NotifyTL => {
                self.notification_service.spawn(
                    NotificationProps::builder()
                        .position(Position::TopLeft)
                        .header(Some(String::from("Notification Top Left")))
                        .color(Color::Success)
                        .timeout(Some(std::time::Duration::from_secs(5)))
                        .children(yew::html::ChildrenRenderer::new(vec![html! {
                            <>
                                <p>{"Hello from the top left of the page!"}</p>
                                <p>{"I use the \"success\" color and time out after 5 seconds"}</p>
                            </>
                        }]))
                        .build(),
                );
                false
            }
            Msg::NotifyTR => {
                self.notification_service.spawn(
                    NotificationProps::builder()
                        .position(Position::TopRight)
                        .header(Some(String::from("Notification Top Right")))
                        .children(yew::html::ChildrenRenderer::new(vec![html! {
                            <>
                                <p>{"This is the body of the notification"}</p>
                                <p>{"Any `html!` output can be placed here"}</p>
                                <p></p>
                                <p>{"Hello from the top right of the page!"}</p>
                            </>
                        }]))
                        .build(),
                );
                false
            }
            Msg::NotifyBL => {
                self.notification_service.spawn(
                    NotificationProps::builder()
                        .position(Position::BottomLeft)
                        .header(Some(String::from("Notification Bottom Left")))
                        .children(yew::html::ChildrenRenderer::new(vec![html! {
                            <>
                                <p>{"This is the body of the notification"}</p>
                                <p>{"Any `html!` output can be placed here"}</p>
                                <p></p>
                                <p>{"Hello from the bottom left of the page!"}</p>
                            </>
                        }]))
                        .build(),
                );
                false
            }
            Msg::NotifyBR => {
                self.notification_service.spawn(
                    NotificationProps::builder()
                        .position(Position::BottomRight)
                        .header(Some(String::from("Notification Bottom Right")))
                        .children(yew::html::ChildrenRenderer::new(vec![html! {
                            <>
                                <p>{"This is the body of the notification"}</p>
                                <p>{"Any `html!` output can be placed here"}</p>
                                <p></p>
                                <p>{"Hello from the bottom right of the page!"}</p>
                            </>
                        }]))
                        .build(),
                );
                false
            }

            Msg::BridgeMsg => false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <main class="ybn-parent">
                <h1>{"Notification Service Example"}</h1>
                <p>{"Click the below buttons to spawn a notification at the given location"}</p>
                <table>
                    <tr>
                        <td><button onclick={self.link.callback(|_| Msg::NotifyTL)}>{"Notify Top Left"}</button></td>
                        <td><button onclick={self.link.callback(|_| Msg::NotifyTR)}>{"Notify Top Right"}</button></td>
                    </tr>
                    <tr>
                        <td><button onclick={self.link.callback(|_| Msg::NotifyBL)}>{"Notify Bottom Left"}</button></td>
                        <td><button onclick={self.link.callback(|_| Msg::NotifyBR)}>{"Notify Bottom Right"}</button></td>
                    </tr>
                </table>

                <NotificationConsumer />
            </main>
        }
    }
}
