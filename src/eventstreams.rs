use leptos::*;

#[component]
pub fn EventStreams() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
        <div class="flex-row"> 
            <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Rewind, Undo, and Replay"}
            </h1>
            <p class="mt-5">
                {"Event Sourcing offers the unique capability to rewind, undo, and replay event streams, providing businesses with powerful tools for in-depth what-if analysis and detailed workflow inspection, ensuring thorough understanding and optimization of business processes."}
            </p>
            <ol
                class=" 
                    mr-5 py-2
                    text-black text-xl md:text-2xl lg:text-3xl 
                "
            >
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Rewinding and Understanding History"}
                        </summary>
                        <p>
                            {"Event Sourcing allows businesses to rewind their systems to any previous state. This capability is invaluable for understanding the sequence of events that led to a particular outcome, providing clear insights into historical workflows and decisions."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Undoing Actions"}
                        </summary>
                        <p>
                            {"The ability to undo actions through Event Sourcing is pivotal for correcting errors and refining processes. It enables businesses to revert to a prior state and rectify any issues, ensuring continuous improvement and accuracy in operations."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Replaying and What-If Analysis"}
                        </summary>
                        <p>
                            {"Event Sourcing's replay functionality allows businesses to test different scenarios by replaying event streams with modified parameters. This what-if analysis is crucial for predicting outcomes, optimizing strategies, and enhancing decision-making processes."}
                        </p>
                    </details>
                </li>
            </ol>
            <p class="mt-5">
                {"The rewind, undo, and replay capabilities of Event Sourcing provide a dynamic and flexible approach to analyzing and optimizing business processes. These features ensure that businesses can operate with the highest levels of insight, precision, and adaptability."}
            </p>
        </div>
    </div>
    
    }
}
