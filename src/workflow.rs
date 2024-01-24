use leptos::*;

#[component]
pub fn Workflow() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h2 class="font-bold text-white text-4xl md:text-6xl">
                {"Talk Shop"}
                </h2>
                <blockquote class="mt-10 text-xl italic font-bold">{"Your business has a language"}</blockquote>
                <div class="text-black flex-row mt-10">
                    <ul class="text-2xl">
                    <li>{"Ask it Questions"}</li>
                    <li>{"Generate Code"}</li>
                    <li>{"Use existing Apps"}</li>
                    <li>{"Integrate New Functionality"}</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
