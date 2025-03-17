use dioxus::prelude::*;
use dioxus_i18n::use_i18n;

#[component]
pub fn PostItem(post_id: String, title: String, author: String, post_time: String, view_count: i32, reply_count: i32, is_hot: bool, is_top: bool) -> Element {

    
    rsx! {
        div { class: "post-item",
            div { class: "post-item-left",
                // 用户头像
                a { href: format!("/u/{}", author),
                    img { class: "avatar", src: asset!("/assets/icons/avatar.svg"), alt: "User Avatar" }
                }
            }
            div { class: "post-item-right",
                div { class: "post-item-header",
                    // 帖子标签（置顶、热门等）
                    if is_top {
                        span { class: "post-tag post-tag-top", "置顶" }
                    }
                    if is_hot {
                        span { class: "post-tag post-tag-hot", "热门" }
                    }
                    // 帖子标题
                    a { class: "post-title", href: format!("/p/{}", post_id), "{title}" }
                }
                div { class: "post-item-meta",
                    // 作者信息
                    a { class: "post-author", href: format!("/u/{}", author), "{author}" }
                    // 发布时间
                    span { class: "post-time", "{post_time}" }
                    // 浏览量和回复数
                    span { class: "post-view-count", "浏览: {view_count}" }
                    span { class: "post-reply-count", "回复: {reply_count}" }
                }
            }
        }
    }
}