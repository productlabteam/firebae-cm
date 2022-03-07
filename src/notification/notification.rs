use crate::{
    Color
};

#[derive(serde::Serialize, Debug)]
pub enum NotificationPriority {
    PriorityUnspecified,
    PriorityMin,
    PriorityLow,
    PriorityDefault,
    PriorityHigh,
    PriorityMax,
}

#[derive(serde::Serialize, Debug)]
pub enum Visibility {
    VisibilityUnspecified,
    Private,
    Public,
    Secret,
}

#[derive(serde::Serialize, Debug, Default)]
pub struct LightSettings {
    color: Option<Color>,
    light_on_duration: Option<String>,
    light_off_duration: Option<String>,
}

impl LightSettings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn light_on_duration(&mut self, seconds: usize) -> &mut Self {
        self.light_on_duration = Some(format!("{}s", seconds));
        self
    }

    pub fn light_off_duration(&mut self, seconds: usize) -> &mut Self {
        self.light_off_duration = Some(format!("{}s", seconds));
        self
    }
}

#[derive(serde::Serialize, Debug)]
pub struct Notification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}
