use leptos::*;

#[component]
pub fn Collaborate() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"Fostering Collaboration"}
                </h1>

                <p>
                    {"A system built with event modeling and domain-driven design (DDD) inherently promotes more effective collaboration. This approach aligns technical development with business needs, ensuring that all team members have a shared understanding of the system’s functionality and the business context, fostering a cohesive and productive working environment."}
                </p>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Shared Understanding with Event Modeling"}
                    </summary>
                    <p>
                        {"Event modeling visualizes the flow of events and interactions within the system, providing a clear and shared understanding of processes and workflows. This shared perspective helps align technical and business teams, ensuring that everyone is on the same page regarding system behavior and expectations."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Domain-Driven Design for Business Alignment"}
                    </summary>
                    <p>
                        {"Domain-driven design focuses on creating a model that reflects the business domain, ensuring that the system’s structure and language are aligned with business concepts and terminology. This alignment helps bridge the gap between technical and non-technical team members, enhancing mutual understanding and collaboration."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Enhanced Collaboration and Iteration"}
                    </summary>
                    <p>
                        {"By using event modeling and DDD, teams can more effectively collaborate on identifying requirements, designing solutions, and iterating on the system. The clarity and shared language provided by these methodologies ensure that contributions from all team members are constructive, relevant, and directly tied to business objectives."}
                    </p>
                </details>

                <p>
                    {"Incorporating event modeling and domain-driven design into system development cultivates an environment where collaboration is natural, effective, and oriented towards delivering solutions that truly resonate with the business’s needs and goals."}
                </p>
            </div>
        </div>
    }
}
