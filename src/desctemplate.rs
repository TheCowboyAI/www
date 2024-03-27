use leptos::*;

#[component]
pub fn DescTemplate() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1 class="shadow-text-black font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {""}
                </h1>
                <p class="">{""}</p>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">{""}</summary>
                    <p>{""}</p>
                </details>

                <p class="">{""}</p>
            </div>
        </div>
    }
}
