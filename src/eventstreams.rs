use leptos::*;

#[component]
pub fn EventStreams() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1
                    class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                    id="architecture"
                >
                    {"Rewind, Undo, and Replay"}
                </h1>
                <p>
                    {"Event Sourcing offers the unique capability to rewind, undo, and replay event streams, providing businesses with powerful tools for in-depth what-if analysis and detailed workflow inspection, ensuring thorough understanding and optimization of business processes."}
                </p>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Rewinding and Understanding History"}
                    </summary>
                    <p>
                        {"Event Sourcing allows businesses to rewind their systems to any previous state. This capability is invaluable for understanding the sequence of events that led to a particular outcome, providing clear insights into historical workflows and decisions."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Undoing Actions"}
                    </summary>
                    <p>
                        {"The ability to undo actions through Event Sourcing is pivotal for correcting errors and refining processes. It enables businesses to revert to a prior state and rectify any issues, ensuring continuous improvement and accuracy in operations."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Replaying and What-If Analysis"}
                    </summary>
                    <p>
                        {"Event Sourcing's replay functionality allows businesses to test different scenarios by replaying event streams with modified parameters. This what-if analysis is crucial for predicting outcomes, optimizing strategies, and enhancing decision-making processes."}
                    </p>
                </details>

                <p>
                    {"The rewind, undo, and replay capabilities of Event Sourcing provide a dynamic and flexible approach to analyzing and optimizing business processes. These features ensure that businesses can operate with the highest levels of insight, precision, and adaptability."}
                </p>
            </div>
        </div>
    }
}
