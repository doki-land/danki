use dioxus::prelude::*;
use dioxus_i18n::use_i18n;

#[component]
pub fn PostEditor(topic_id: Option<String>, edit_post_id: Option<String>) -> Element {
    
    let title = use_signal(String::new);
    let content = use_signal(String::new);
    
    rsx! {
        div { class: "post-editor",
            h2 { class: "editor-title", 
                if edit_post_id.is_some() {
                    "编辑帖子"
                } else {
                    "发布新帖子"
                }
            }
            div { class: "editor-form",
                div { class: "form-group",
                    label { r#for: "post-title", "标题" }
                    input { 
                        id: "post-title",
                        r#type: "text", 
                        placeholder: "请输入标题",
                        value: title.read().clone(),
                        oninput: move |evt| title.set(evt.value.clone())
                    }
                }
                div { class: "form-group",
                    label { r#for: "post-content", "内容" }
                    textarea { 
                        id: "post-content",
                        placeholder: "请输入内容",
                        rows: "10",
                        value: content.read().clone(),
                    }
                }
                div { class: "form-actions",
                    button { class: "cancel-btn", "取消" }
                    button { 
                        class: "submit-btn", 
                        disabled: title.read().is_empty() || content.read().is_empty(),
                        "发布"
                    }
                }
            }
        }
    }
}