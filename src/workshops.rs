use leptos::*;

use crate::workshopform::WorkshopForm;

#[component]
pub fn Workshops() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row">
                <h1 class="font-bold text-white text-4xl md:text-6xl">{"Knowledge Workshops"}</h1>
                <p class="mt-5">
                    {"Cowboy AI offers Knowledge Workshops designed to empower your business with the tools and knowledge needed for strategic planning and system integration. These workshops cover a range of crucial topics, including the generation of Business and Value Proposition Canvases, understanding and utilizing Knowledge Graphs, and mastering Event Sourced systems, all tailored specifically to your business needs."}
                </p>
                <ol class=" 
                mr-5 py-2
                text-black text-xl md:text-2xl lg:text-3xl 
                ">
                    <li class="mt-5">
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                                {"Comprehensive Workshop Content"}
                            </summary>
                            <p>
                                {"Cowboy AI's Knowledge Workshops are meticulously designed to cover essential aspects of business modeling and system design. Participants will receive guided instruction in creating a Business Model Canvas, crafting a compelling Value Proposition Canvas, understanding the intricacies of Knowledge Graphs for business, and learning how to create and use Event Sourced systems effectively."}
                            </p>
                        </details>
                    </li>
                    <li class="mt-5">
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                                {"Structured Learning Experience"}
                            </summary>
                            <p>
                                {"Each workshop includes a 4-hour instruction seminar packed with valuable insights and practical knowledge. Participants will also receive comprehensive training material to reinforce and apply the learning in their specific business context."}
                            </p>
                        </details>
                    </li>
                    <li class="mt-5">
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                                {"Personalized Business Modeling"}
                            </summary>
                            <p>
                                {"Cowboy AI ensures that every workshop participant receives personal attention, allowing for the customization and refinement of models specifically for your business. This personalized approach guarantees that the strategies and systems developed during the workshop are perfectly aligned with your business objectives and operational needs."}
                            </p>
                        </details>
                    </li>
                </ol>
                <p class="mt-5">
                    {"Cowboy AI's Knowledge Workshops are more than just learning sessions; they are transformative experiences designed to equip your team with the knowledge, tools, and confidence to drive your business forward with strategic insights and innovative solutions."}
                </p>
                <div class="mt-10">
                    <WorkshopForm/>
                </div>
            </div>
        </div>
    }
}
