mod not_found;
mod home;
mod projects;
mod research;
mod cv;
mod blog;

pub use not_found::NotFound;
pub use home::Home;
pub use projects::Projects;
pub use research::Research;
pub use cv::Cv;
pub use blog::{BlogIndex, BlogPost};