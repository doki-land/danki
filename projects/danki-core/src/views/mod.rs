use dioxus::prelude::*;

mod home_page;
mod category_page;
mod post_page;
mod user_page;

pub use self::home_page::DankiHome;
pub use self::category_page::DankiTopic;
pub use self::post_page::DankiPost;
pub use self::user_page::DankiUser;

#[derive(Clone, Routable, PartialEq)]
pub enum DankiRouter {
    #[route("/")]
    DankiHome {},
    #[route("/r/:topic_id")]
    DankiTopic { topic_id: String },
    #[route("/p/:post_id")]
    DankiPost { post_id: String },
    #[route("/u/:user_id")]
    DankiUser { user_id: String}
}

