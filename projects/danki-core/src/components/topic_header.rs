use dioxus::prelude::*;
use dioxus_i18n::use_i18n;

#[component]
pub fn TopicHeader(topic_id: String, topic_name: String, description: String, follower_count: i32, post_count: i32) -> Element {

    rsx! {
        div { class: "topic-header",
            div { class: "topic-header-banner",
                // 贴吧背景图
                div { class: "topic-banner-image" }
                // 贴吧图标和信息
                div { class: "topic-info",
                    img { class: "topic-avatar", src: asset!("/assets/icons/logo.svg"), alt: "Topic Logo" }
                    div { class: "topic-details",
                        h1 { class: "topic-name", "{topic_name}" }
                        p { class: "topic-description", "{description}" }
                        div { class: "topic-stats",
                            span { class: "follower-count", "关注: {follower_count}" }
                            span { class: "post-count", "帖子: {post_count}" }
                        }
                    }
                }
                // 关注按钮
                button { class: "follow-btn", "+ 关注" }
            }
            // 导航菜单
            div { class: "topic-nav",
                a { class: "nav-item active", href: format!("/r/{}", topic_id), "看帖" }
                a { class: "nav-item", href: format!("/r/{}/images", topic_id), "图片" }
                a { class: "nav-item", href: format!("/r/{}/videos", topic_id), "视频" }
            }
        }
    }
}