pub struct Publication {
    title: &'static str,
    venue: &'static str,
    kind: &'static str,
    authors: &'static [&'static str],
    year: u32,
    links: &'static [(&'static str, &'static str)],
}

include!(concat!(env!("OUT_DIR"), "/content_generated.rs"));