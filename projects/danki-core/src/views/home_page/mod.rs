use super::*;
use crate::components::{Navbar, PostItem, Sidebar, Footer};
use dioxus_i18n::use_i18n;

#[component]
pub fn DankiHome() -> Element {
    rsx! {
        div { class: "danki-app",
            // 导航栏
            Navbar {}
            
            // 主要内容区
            div { class: "main-container",
                // 左侧边栏
                Sidebar {}
                
                // 中间内容区
                div { class: "content-area",
                    // 顶部横幅
                    div { class: "home-banner",
                        h1 { "欢迎来到 DankiTopic" }
                        p { "发现有趣的话题，分享你的想法" }
                    }
                    
                    // 帖子分类导航
                    div { class: "post-categories",
                        a { class: "category-item active", href: "#", "全部" }
                        a { class: "category-item", href: "#", "热门" }
                        a { class: "category-item", href: "#", "最新" }
                        a { class: "category-item", href: "#", "精华" }
                    }
                    
                    // 帖子列表
                    div { class: "post-list",
                        // 置顶帖子
                        PostItem {
                            post_id: "1".to_string(),
                            title: "欢迎来到DankiTopic论坛".to_string(),
                            author: "admin".to_string(),
                            post_time: "2024-05-01".to_string(),
                            view_count: 1024,
                            reply_count: 42,
                            is_hot: true,
                            is_top: true
                        }
                        
                        // 普通帖子
                        PostItem {
                            post_id: "2".to_string(),
                            title: "如何使用DankiTopic的各项功能？".to_string(),
                            author: "moderator".to_string(),
                            post_time: "2024-05-02".to_string(),
                            view_count: 512,
                            reply_count: 18,
                            is_hot: true,
                            is_top: false
                        }
                        
                        PostItem {
                            post_id: "3".to_string(),
                            title: "分享一下我的学习经验".to_string(),
                            author: "user123".to_string(),
                            post_time: "2024-05-03".to_string(),
                            view_count: 256,
                            reply_count: 8,
                            is_hot: false,
                            is_top: false
                        }
                        
                        PostItem {
                            post_id: "4".to_string(),
                            title: "求助：这个问题如何解决？".to_string(),
                            author: "newbie".to_string(),
                            post_time: "2024-05-04".to_string(),
                            view_count: 128,
                            reply_count: 5,
                            is_hot: false,
                            is_top: false
                        }
                    }
                    
                    // 分页控件
                    div { class: "pagination",
                        a { class: "page-item active", "1" }
                        a { class: "page-item", href: "#", "2" }
                        a { class: "page-item", href: "#", "3" }
                        a { class: "page-item", href: "#", "4" }
                        a { class: "page-item", href: "#", "5" }
                        a { class: "page-item next", href: "#", "下一页 >" }
                    }
                }
                
                // 右侧边栏（可选）
                div { class: "right-sidebar",
                    // 热门用户
                    div { class: "sidebar-section hot-users",
                        h3 { class: "section-title", "活跃用户" }
                        ul { class: "user-list",
                            li { class: "user-item",
                                a { href: "/u/user1",
                                    img { class: "avatar", src: asset!("/assets/icons/avatar.svg"), alt: "User Avatar" }
                                    span { "用户名1" }
                                }
                            }
                            li { class: "user-item",
                                a { href: "/u/user2",
                                    img { class: "avatar", src: asset!("/assets/icons/avatar.svg"), alt: "User Avatar" }
                                    span { "用户名2" }
                                }
                            }
                            li { class: "user-item",
                                a { href: "/u/user3",
                                    img { class: "avatar", src: asset!("/assets/icons/avatar.svg"), alt: "User Avatar" }
                                    span { "用户名3" }
                                }
                            }
                        }
                    }
                }
            }
            
            // 页脚
            Footer {}
        }
    }
}