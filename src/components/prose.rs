use dioxus::prelude::*;

#[component]
pub fn Prose(html: String) -> Element {
    rsx! {
        div {
            class: "prose prose-lg prose-site dark:prose-invert max-w-none \
                            prose-a:font-normal prose-a:decoration-primary prose-a:decoration-2 prose-a:underline-offset-4 \
                            prose-a:hover:text-primary-deep dark:prose-a:hover:text-primary \
                            prose-code:before:content-none prose-code:after:content-none prose-code:font-normal \
                            [&_:not(pre)>code]:px-1 [&_:not(pre)>code]:py-0.5 [&_:not(pre)>code]:rounded-sm \
                            prose-h2:mt-12 prose-h2:mb-4 prose-h2:text-2xl prose-h2:font-semibold prose-h2:tracking-tight \
                            prose-h3:mt-8 prose-h3:mb-3 prose-h3:text-xl prose-h3:font-semibold \
                            prose-h4:mt-6 prose-h4:mb-2 prose-h4:text-lg prose-h4:font-semibold",
            dangerous_inner_html: html,
        }
    }
}
