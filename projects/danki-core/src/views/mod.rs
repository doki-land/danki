use dioxus::prelude::*;

mod home_page;
mod category_page;
mod post_page;
mod user_page;

pub use self::home_page::DoraHome;
pub use self::category_page::DoraCategory;
pub use self::post_page::DoraPost;
pub use self::user_page::DoraUser;

#[derive(Clone, Routable, PartialEq)]
pub enum DoraRouter {
    #[route("/")]
    DoraHome {},
    #[route("/c/:category_id")]
    DoraCategory { category_id: String },
    #[route("/p/:post_id")]
    DoraPost { post_id: String },
    #[route("/u/:user_id")]
    DoraUser { user_id: String}
}

