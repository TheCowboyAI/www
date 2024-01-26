use leptos::*;

#[component]
pub fn Events() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                    <a href="/eventsourcing">
                    {"It's all Events..."}
                    </a>
                </h1>
                <ul 
                  class=" 
                    mt-10 mr-5 py-2
                    text-black text-xl md:text-2xl lg:text-3xl 
                  "
                >                
                    <li>
                        <a href="/audittrail">
                        {"Audit Trail"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <a href="/historicalinsight">
                        {"Historical Insight"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <a href="/projections">
                        {"Projections"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <a href="/eventstreams">
                        {"Undo and Replay"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <a href="predictions">
                        {"Predictions"}
                        </a>
                    </li>
                </ul>
                // <Next />
            </div>
        </div>
    }
}
