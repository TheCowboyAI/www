use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="rounded-t p-2 text-white bg-black">
        {"Copyright 2024 - Cowboy AI"}
        </div>
    }
}