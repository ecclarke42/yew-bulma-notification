mod consumer;
mod notification;
mod properties;
mod service;

pub use consumer::NotificationConsumer;
pub use notification::{Notification, NotificationProps};
pub use properties::{Color, Position, Size};
pub use service::{NotificationService, NotificationServiceInput, NotificationServiceOutput};
