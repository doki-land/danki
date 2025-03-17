use dioxus::prelude::*;
use dioxus_i18n::use_i18n;

#[component]
pub fn CommentItem(comment_id: String, content: String, author: String, comment_time: String, like_count: i32) -> Element {

    
    rsx! {
        div { class: "comment-item",
            div { class: "comment-header",
                // 用户头像
                a { href: format!("/u/{}", author),
                    img { class: "avatar", src: asset!("/assets/icons/avatar.svg"), alt: "User Avatar" }
                }
                // 用户名和评论时间
                div { class: "comment-meta",
                    a { class: "comment-author", href: format!("/u/{}", author), author }
                    span { class: "comment-time", comment_time }
                }
            }
            // 评论内容
            div { class: "comment-content", content }
            // 评论操作
            div { class: "comment-actions",
                button { class: "like-btn", 
                    img { src: asset!("/assets/icons/hot.svg"), alt: "Like" }
                    span { "{like_count}" }
                }
                button { class: "reply-btn", {t!("post")} }
            }
        }
    }
}