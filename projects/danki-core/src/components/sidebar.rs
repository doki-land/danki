use dioxus::prelude::*;
use dioxus_i18n::use_i18n;

#[component]
pub fn Sidebar() -> Element {
    
    
    rsx! {
        div { class: "sidebar",
            // 用户信息卡片（如果已登录）
            div { class: "sidebar-section user-info-card",
                h3 { class: "section-title", "个人信息" }
                div { class: "user-info",
                    img { class: "avatar", src: asset!("/assets/icons/avatar.svg"), alt: "User Avatar" }
                    div { class: "user-details",
                        p { class: "username", "用户名" }
                        div { class: "user-stats",
                            span { class: "post-count", "帖子: 0" }
                            span { class: "follow-count", "关注: 0" }
                        }
                    }
                }
                div { class: "user-actions",
                    a { class: "profile-btn", href: "/u/current-user", "个人主页" }
                    a { class: "settings-btn", href: "/settings", "设置" }
                }
            }
            
            // 热门话题
            div { class: "sidebar-section hot-topics",
                h3 { class: "section-title", "热门话题" }
                ul { class: "topic-list",
                    li { class: "topic-item",
                        a { href: "/r/programming", "编程" }
                        span { class: "topic-count", "1.2k帖子" }
                    }
                    li { class: "topic-item",
                        a { href: "/r/design", "设计" }
                        span { class: "topic-count", "856帖子" }
                    }
                    li { class: "topic-item",
                        a { href: "/r/gaming", "游戏" }
                        span { class: "topic-count", "2.3k帖子" }
                    }
                }
            }
            
            // 推荐内容
            div { class: "sidebar-section recommended",
                h3 { class: "section-title", "推荐内容" }
                div { class: "recommended-posts",
                    div { class: "recommended-post",
                        a { href: "/p/123", "推荐帖子标题1" }
                        p { class: "post-excerpt", "帖子摘要内容..." }
                    }
                    div { class: "recommended-post",
                        a { href: "/p/456", "推荐帖子标题2" }
                        p { class: "post-excerpt", "帖子摘要内容..." }
                    }
                }
            }
        }
    }
}