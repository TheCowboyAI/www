use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="m-2 rounded-lg p-2 text-black border-black border border-solid">
            <p class="text-[8px]">{"Copyright 2024 - Cowboy AI"}</p>
        </div>
    }
}