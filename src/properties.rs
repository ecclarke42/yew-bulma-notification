#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Default,
    Dark,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

impl Color {
    pub(crate) fn class(&self) -> Option<&'static str> {
        match self {
            Color::Default => None,
            Color::Dark => Some("is-dark"),
            Color::Primary => Some("is-primary"),
            Color::Link => Some("is-link"),
            Color::Info => Some("is-info"),
            Color::Success => Some("is-success"),
            Color::Warning => Some("is-warning"),
            Color::Danger => Some("is-danger"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl Size {
    pub(crate) fn class(&self) -> Option<&'static str> {
        match self {
            Size::Small => Some("is-small"),
            Size::Normal => None,
            Size::Medium => Some("is-medium"),
            Size::Large => Some("is-large"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Position {
    TopLeft,
    // TopMiddle,
    TopRight,

    BottomLeft,
    // BottomMiddle,
    BottomRight,
}

impl Position {
    pub(crate) fn style(&self) -> &'static str {
        match self {
            Position::TopLeft => "ybn-top-left",
            Position::TopRight => "ybn-top-right",
            Position::BottomLeft => "ybn-bottom-left",
            Position::BottomRight => "ybn-bottom-right",
        }
    }

    pub(crate) fn animate_in_style(&self) -> &'static str {
        match self {
            Position::TopLeft => "ybn-animate-in-from-left",
            Position::TopRight => "ybn-animate-in-from-right",
            Position::BottomLeft => "ybn-animate-in-from-left",
            Position::BottomRight => "ybn-animate-in-from-right",
        }
    }

    pub(crate) fn animate_out_style(&self) -> &'static str {
        match self {
            Position::TopLeft => "ybn-animate-out-to-left",
            Position::TopRight => "ybn-animate-out-to-right",
            Position::BottomLeft => "ybn-animate-out-to-left",
            Position::BottomRight => "ybn-animate-out-to-right",
        }
    }
}
