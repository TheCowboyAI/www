use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="rounded-lg leading-6 p-2 text-white bg-black opacity-50">
            <p class="text-sm">Copyright 2024</p>
            <p class="text-sm">Cowboy AI</p>
        </div>
    }
}