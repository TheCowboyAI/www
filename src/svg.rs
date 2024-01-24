use leptos::*

#[component]
pub fn SvgImage() -> impl IntoView {
    // Include the contents of the SVG file as a &str
    let svg_content = include_str!("../assets/logo.svg");

    view! {
        <svg class="svg-container" inner_html={svg_content} />
    }
}
