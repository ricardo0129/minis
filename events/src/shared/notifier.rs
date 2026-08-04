pub trait IntoNotification {
    fn format_notification(&self) -> String;
}

pub trait Notifier {
    async fn post_notification<T>(&self, notification: T)
    where
        T: IntoNotification;
}
