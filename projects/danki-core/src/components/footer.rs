use dioxus::prelude::*;
use dioxus_i18n::use_i18n;

#[component]
pub fn Footer() -> Element {

    let current_year = 2024; // 实际应用中可以使用动态获取的年份
    
    rsx! {
        footer { class: "site-footer",
            div { class: "footer-content",
                div { class: "footer-logo",
                    img { src: asset!("/assets/icons/logo.svg"), alt: "DankiTopic Logo" }
                    span { "DankiTopic" }
                }
                div { class: "footer-links",
                    div { class: "footer-section",
                        h4 { "关于我们" }
                        ul {
                            li { a { href: "/about", "关于DankiTopic" } }
                            li { a { href: "/terms", "服务条款" } }
                            li { a { href: "/privacy", "隐私政策" } }
                        }
                    }
                    div { class: "footer-section",
                        h4 { "帮助中心" }
                        ul {
                            li { a { href: "/help", "常见问题" } }
                            li { a { href: "/feedback", "意见反馈" } }
                            li { a { href: "/report", "举报中心" } }
                        }
                    }
                }
            }
            div { class: "footer-bottom",
                p { class: "copyright", "© {current_year} DankiTopic 版权所有" }
            }
        }
    }
}