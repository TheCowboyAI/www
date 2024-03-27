use leptos::*;

#[component]
pub fn EventSourcing() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1
                    class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                    id="architecture"
                >
                    {"Event Sourcing"}
                </h1>
                <p>
                    {"Event Sourcing is an architectural pattern where each change to the state of an application is stored as a sequence of events. This allows for an accurate audit trail and the ability to reconstruct past states of the system."}
                </p>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Immutable Event Log"}
                    </summary>
                    <p>
                        {"In Event Sourcing, each event represents a state change. The events are stored in a log, which is append-only and immutable. This ensures that the history of changes is preserved and traceable."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Audit Trail and Debugging"}
                    </summary>
                    <p>
                        {"The event log serves as a comprehensive audit trail, making it easier to debug issues or conduct analyses. You can trace what actions were performed and in what order, aiding in understanding and diagnosing system behavior."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"State Reconstruction"}
                    </summary>
                    <p>
                        {"Event Sourcing allows for the reconstruction of the system's state at any point in time. By replaying the events from the log, you can view the state of the system as it was after any given event, enhancing system resilience and data recovery capabilities."}
                    </p>
                </details>

                <p>
                    {"Event Sourcing offers significant benefits for system integrity, auditability, and historical analysis, making it a powerful pattern for complex systems where understanding and maintaining a detailed history of changes is crucial."}
                </p>
            </div>
        </div>
    }
}
