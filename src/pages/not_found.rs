use dioxus::prelude::*;

#[inline_props]
pub fn NotFound(cx: Scope,segments: Vec<String> ) -> Element {
    render!(rsx!(
        div{
            "kord said page not found"
        }
    ))
}
