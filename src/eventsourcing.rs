use leptos::*;

#[component]
pub fn EventSourcing() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
        <div class="flex-row"> 
            <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Event Sourcing"}
            </h1>
            <p class="mt-5">
                {"Event Sourcing is an architectural pattern where each change to the state of an application is stored as a sequence of events. This allows for an accurate audit trail and the ability to reconstruct past states of the system."}
            </p>
            <ol
                class=" 
                    mt-10 mr-5 py-2
                    text-black text-xl md:text-2xl lg:text-3xl 
                "
            >
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Immutable Event Log"}
                        </summary>
                        <p>
                            {"In Event Sourcing, each event represents a state change. The events are stored in a log, which is append-only and immutable. This ensures that the history of changes is preserved and traceable."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Audit Trail and Debugging"}
                        </summary>
                        <p>
                            {"The event log serves as a comprehensive audit trail, making it easier to debug issues or conduct analyses. You can trace what actions were performed and in what order, aiding in understanding and diagnosing system behavior."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"State Reconstruction"}
                        </summary>
                        <p>
                            {"Event Sourcing allows for the reconstruction of the system's state at any point in time. By replaying the events from the log, you can view the state of the system as it was after any given event, enhancing system resilience and data recovery capabilities."}
                        </p>
                    </details>
                </li>
            </ol>
            <p class="mt-5">
                {"Event Sourcing offers significant benefits for system integrity, auditability, and historical analysis, making it a powerful pattern for complex systems where understanding and maintaining a detailed history of changes is crucial."}
            </p>
        </div>
    </div>
    
    }
}
