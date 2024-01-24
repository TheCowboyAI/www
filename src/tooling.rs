use leptos::*;

#[component]
pub fn Tooling() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center">
            <div class="flex-row"> 
                <h2 class="text-white text-4xl md:text-6xl">
                {"Modeled Intelligence"}
                </h2>
                <div class="text-black flex-row mt-10">
                    <ul class="text-2xl">
                    <li>{"Business model"}</li>
                    <li>{"+"}</li>
                    <li class="pb-2">{"simple words"}</li>
                    </ul>
                    <hr class="border border-t-2 border-black" />
                    <h2 class="text-2xl mt-2 font-bold italic">
                    {"AI Composable Insights"}
                    </h2>
                </div>
            </div>
        </div>
    }
}
