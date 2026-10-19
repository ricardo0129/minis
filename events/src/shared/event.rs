use crate::shared::notifier::IntoNotification;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum EventSource {
    Twitch,
    Kick,
}

#[derive(Clone, Debug)]
pub enum EventPayload {
    Online {
        source: EventSource,
        channel_info: ChannelInfo,
    },
    Offline {
        source: EventSource,
        channel_info: ChannelInfo,
    },
    Update {
        #[allow(dead_code)]
        source: EventSource,
        channel_info: ChannelInfo,
        title_info: TitleInfo,
    },
}

#[derive(Clone, Debug)]
pub struct InternalEvent {
    pub payload: EventPayload,
}

#[derive(Clone, Debug)]
pub struct ChannelInfo {
    pub user_name: String,
}

#[derive(Clone, Debug)]
pub struct TitleInfo {
    pub title: String,
    pub category_name: String,
}

fn stream_url(channel_info: &ChannelInfo, source: &EventSource) -> String {
    match source {
        EventSource::Twitch => format!("https://twitch.com/{}", channel_info.user_name),
        EventSource::Kick => format!("https://kick.com/{}", channel_info.user_name),
    }
}

impl IntoNotification for EventPayload {
    fn format_notification(&self) -> String {
        match &self {
            EventPayload::Online {
                channel_info,
                source,
            } => {
                let url = stream_url(channel_info, source);
                format!("{} Went Live! {}", channel_info.user_name, url)
            }
            EventPayload::Offline {
                channel_info,
                source,
            } => {
                let url = stream_url(channel_info, source);
                format!("{} Went Offline! {}", channel_info.user_name, url)
            }
            EventPayload::Update {
                source: _,
                channel_info,
                title_info,
            } => {
                format!(
                    "{} {} {}",
                    channel_info.user_name, title_info.title, title_info.category_name
                )
            }
        }
    }
}
