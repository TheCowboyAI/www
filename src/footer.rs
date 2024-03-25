use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="rounded-lg leading-6 p-2 text-black border-black border border-solid">
            <p class="text-xs">Copyright 2024</p>
            <p class="text-xs">Cowboy AI</p>
        </div>
    }
}