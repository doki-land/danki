use dioxus::prelude::*;
use dioxus_i18n::use_i18n;

#[component]
pub fn UserCard(user_id: String, username: String, avatar: Option<String>, post_count: i32, follow_count: i32) -> Element {

    let avatar_src = avatar.unwrap_or_else(|| asset!("/assets/icons/avatar.svg").to_string());
    
    rsx! {
        div { class: "user-card",
            div { class: "user-card-header",
                img { class: "user-avatar", src: avatar_src, alt: "用户头像" }
                div { class: "user-info",
                    h3 { class: "username", "{username}" }
                    div { class: "user-stats",
                        span { class: "post-count", "帖子: {post_count}" }
                        span { class: "follow-count", "关注: {follow_count}" }
                    }
                }
            }
            div { class: "user-actions",
                button { class: "follow-btn", "关注" }
                a { class: "message-btn", href: format!("/message/{}", user_id), "私信" }
            }
        }
    }
}