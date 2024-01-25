use leptos::*;

#[component]
pub fn Workflow() -> impl IntoView {
    view! {
        <div class="flex items-center animate-fadeIn">
            <div class="mt-10 flex-row"> 
                <h2 class="font-bold text-white text-4xl md:text-6xl">
                {"Workflows"}
                </h2>
                <blockquote class="mt-10 text-xl italic font-bold">
                {"Your business;"}
                </blockquote>
                <blockquote class="text-xl italic font-bold">
                {"Your language"}
                </blockquote>
                <div class="text-black flex-row mt-10">
                    <ul class="text-2xl">
                    <li>{"Use Natural Language"}</li>
                    <li class="mt-2">{"Ask it Questions"}</li>
                    <li class="mt-2">{"Generate Code"}</li>
                    <li class="mt-2">{"Use existing Apps"}</li>
                    <li class="mt-2">{"Integrate New Functionality"}</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
