//! Run with:
//!
//! ```sh
//! dx serve --platform web
//! ```

use dioxus::prelude::*;
use dioxus_i18n::{
    unic_langid::langid,
    use_i18n::{use_init_i18n, I18nConfig, Locale},
};
use dora_core::views::DankiRouter;

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only!(
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(std::env::current_exe().unwrap().parent().unwrap().join("public"))
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        ))
        .launch(app);
}

pub fn app() -> Element {
    let _ = use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale(Locale::new_static(langid!("en-US"), include_str!("../i18n/en-US.ftl")))
            .with_locale(Locale::new_static(langid!("zh-Hans"), include_str!("../i18n/zh-Hans.ftl")))
    });
    rsx! {
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/styles/index.scss")
        }
        Router::<DankiRouter> {}
    }
}
