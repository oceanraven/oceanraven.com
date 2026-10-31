use crate::components::{PageTitle, Prose, META_LINK_CLASS};
use crate::content::CV_HTML;
use dioxus::prelude::*;

const CV_PDF: Asset = asset!("/assets/cv.pdf");

#[component]
pub fn Cv() -> Element {
    rsx! {
        div { class: "space-y-8",
            div { class: "flex items-baseline justify-between gap-4",
                PageTitle { title: "CV" }
                a {
                    href: "{CV_PDF}",
                    download: "Seokyoung-Yoon-CV.pdf",
                    class: "{META_LINK_CLASS} shrink-0 font-mono text-sm",
                    "[pdf]"
                }
            }

            div { class: "[&_h3+p]:mt-1 [&_h3+p]:text-sm [&_h3+p]:text-neutral-600 dark:[&_h3+p]:text-neutral-400",
                Prose { html: CV_HTML }
            }
        }
    }
}