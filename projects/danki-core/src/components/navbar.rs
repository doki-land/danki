use dioxus::prelude::*;
use dioxus_i18n::{t, use_i18n};

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar",
            div { class: "navbar-logo",
                a { href: "/",
                    img { src: asset!("/assets/icons/logo.svg"), alt: "DankiTopic Logo" }
                    span { "DankiTopic" }
                }
            }
            div { class: "navbar-search",
                input { 
                    r#type: "text", 
                    placeholder: t!("search"),
                }
                button { class: "search-btn", {t!("search")} }
            }
            div { class: "navbar-actions",
                if true { // 替换为实际的登录状态检查
                    a { class: "login-btn", href: "/login", {t!("login")} }
                    a { class: "register-btn", href: "/register", {t!("register")} }
                } else {
                    a { class: "post-btn", href: "/post/new", {t!("post")} }
                    div { class: "user-menu",
                        img { class: "avatar", src: asset!("/assets/icons/avatar.svg"), alt: "User Avatar" }
                        div { class: "dropdown-menu",
                            a { href: "/u/current-user", {t!("profile")} }
                            a { href: "/settings", {t!("settings")} }
                            a { href: "/logout", {t!("logout")} }
                        }
                    }
                }
            }
        }
    }
}