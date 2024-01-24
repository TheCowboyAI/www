use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h2 class="font-bold text-white text-4xl md:text-6xl">
                    {"Business Evolution"}
                </h2>
                <h3 class="text-3xl md:text-4xl mt-12 font-bold">{"Ready to level up?"}</h3>
                <ul class="border-dashed text-2xl md:text-3xl mt-10 px-5 py-2 rounded-lg text-white">
                    <li>{"Understand context"}</li>
                    <li>{"Generate content"}</li>
                    <li>{"Be deterministic"}</li>
                </ul>
                // <Next />
            </div>
        </div>
    }
}
