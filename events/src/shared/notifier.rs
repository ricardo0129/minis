pub trait Notifier {
    async fn post_notification(&self, notification: impl IntoNotification);
}

pub trait IntoNotification {
    fn format_notification(&self) -> String;
}
