#[derive(PartialEq, Clone)]
pub struct Publication {
    pub title: &'static str,
    pub venue: &'static str,
    pub kind: &'static str,
    pub authors: &'static [&'static str],
    pub year: u32,
    pub links: &'static [(&'static str, &'static str)],
}

include!(concat!(env!("OUT_DIR"), "/content_generated.rs"));