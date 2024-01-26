use leptos::*;

#[component]
pub fn CimSvg() -> impl IntoView {
    // Include the contents of the SVG file as a &str
    let svg_content = include_str!("../assets/CIM.svg");

    view! {
        <div class="mt-5 items-center justify-center">
            <a href="/cim" class="border-black border-solid border-2 rounded-lg">
            <div class="svg-container" inner_html={svg_content} />
            </a>
        </div>
    }
}