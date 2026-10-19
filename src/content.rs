#[derive(Clone, PartialEq)]
pub struct Publication {
    pub title: &'static str,
    pub venue: &'static str,
    pub kind: &'static str,
    pub authors: &'static [&'static str],
    pub year: u32,
    pub links: &'static [(&'static str, &'static str)],
}

#[derive(Clone, Copy, PartialEq)]
pub enum Status {
    Active,
    Wip,
    Archived,
}

impl Status {
    pub fn label(&self) -> &'static str {
        match self {
            Status::Active => "active",
            Status::Wip => "wip",
            Status::Archived => "archived",
        }
    }

    pub fn dot_class(&self) -> &'static str {
        match self {
            Status::Active => "bg-primary",
            Status::Wip => "bg-secondary",
            Status::Archived => "bg-neutral-400 dark:bg-neutral-500",
        }
    }
}

#[derive(PartialEq)]
pub struct Project {
    pub name: &'static str,
    pub summary: &'static str,
    pub year: u32,
    pub status: Status,
    pub tags: &'static [&'static str],
    pub links: &'static [(&'static str, &'static str)],
    pub post: Option<&'static str>,
}

include!(concat!(env!("OUT_DIR"), "/content_generated.rs"));
