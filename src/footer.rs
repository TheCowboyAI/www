use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="flex mt-auto rounded-lg p-2 theme_bg-1-bg-1 theme_bg-1-color-5 border-black border border-solid">
            <p class="text-[8px]">{"Copyright 2024 - Cowboy AI"}</p>
        </div>
    }
}