use leptos::*;

#[component]
pub fn  ModeledIntelligence() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1
                    class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                    id="architecture"
                >
                    {"Modeled Intelligence"}
                </h1>
                <p>
                    {"Modeled Intelligence refers to the creation of artificial systems that can simulate human intelligence, understanding, and thought processes through predefined models and algorithms. This approach to AI aims to replicate or surpass human cognitive abilities in specific domains."}
                </p>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Cognitive Simulation"}
                    </summary>
                    <p>
                        {"Modeled Intelligence involves the creation of models that simulate human cognitive processes. These models are designed to understand, learn from, and interact with the world in a way that mimics human thought and decision-making patterns."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Advanced Problem-Solving"}
                    </summary>
                    <p>
                        {"This form of intelligence is adept at solving complex problems by analyzing vast amounts of data, recognizing patterns, and making decisions based on the synthesized information, often outperforming human capabilities in specific tasks."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Domain-Specific Expertise"}
                    </summary>
                    <p>
                        {"Modeled Intelligence can be tailored to specific domains, acquiring expertise and providing insights at a level that matches or exceeds human experts. This makes it invaluable in fields such as healthcare, finance, and engineering."}
                    </p>
                </details>

                <p class="">
                    {"Modeled Intelligence represents a significant stride in the field of artificial intelligence, offering the potential to revolutionize various industries by providing deep, domain-specific insights and solutions."}
                </p>
            </div>
        </div>
    }
}
