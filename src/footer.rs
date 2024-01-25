use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="rounded-lg leading-6 p-2 text-white border-black border-2 border-solid rounded-lg">
            <p class="text-xs">Copyright 2024</p>
            <p class="text-xs">Cowboy AI</p>
        </div>
    }
}