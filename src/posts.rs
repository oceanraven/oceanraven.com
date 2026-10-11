pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub tags: &'static [&'static str],
    pub html: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/posts_generated.rs"));

pub fn all_posts() -> &'static [Post] {
    POSTS
}

pub fn find_post(slug: &str) -> Option<&'static Post> {
    POSTS.iter().find(|post| post.slug == slug)
}