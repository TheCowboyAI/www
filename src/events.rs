use leptos::*;

#[component]
pub fn Events() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h2 class="font-bold text-white text-4xl md:text-6xl">
                    {"It's all Events..."}
                </h2>
                <ul class="border-dashed text-2xl md:text-3xl mt-10 px-5 py-2 rounded-lg text-black font-bold">
                    <li>{"Audit Trail"}</li>
                    <li>{"Historical Insight"}</li>
                    <li>{"Spot inefficiencies"}</li>
                    <li>{"Unlimited Projections"}</li>
                    <li>{"Undo and Replay"}</li>
                    <li>{"Evolving Business Processes"}</li>
                    <li>{"Synchronization"}</li>
                    <li>{"Decision Making"}</li>
                    <li>{"Predictions"}</li>
                </ul>
                // <Next />
            </div>
        </div>

    }
}
