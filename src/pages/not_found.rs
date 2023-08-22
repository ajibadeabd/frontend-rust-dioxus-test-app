use dioxus::prelude::*;

#[inline_props]
pub fn NotFound(cx: Scope,segments: Vec<String> ) -> Element {
    println!("{:?}",segments);
    render!(rsx!(
        div{
            "not found"
        }
    ))
}
