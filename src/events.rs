use leptos::*;

#[component]
pub fn Events() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                    {"It's all Events..."}
                </h1>
                <ul 
                  class=" 
                    mt-10 mr-5 py-2
                    text-black text-xl md:text-2xl lg:text-3xl 
                  "
                >                
                    <li>{"Audit Trail"}</li>
                    <li class="mt-5">{"Historical Insight"}</li>
                    <li class="mt-5">{"Spot inefficiencies"}</li>
                    <li class="mt-5">{"Unlimited Projections"}</li>
                    <li class="mt-5">{"Undo and Replay"}</li>
                    <li class="mt-5">{"Evolving Business Processes"}</li>
                    <li class="mt-5">{"Synchronization"}</li>
                    <li class="mt-5">{"Decision Making"}</li>
                    <li class="mt-5">{"Predictions"}</li>
                </ul>
                // <Next />
            </div>
        </div>
    }
}
