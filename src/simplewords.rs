use leptos::*;

#[component]
pub fn SimpleWords() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1
                    class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                    id="architecture"
                >
                    {"Evolution from Simple Words to Knowledge Graphs"}
                </h1>
                <p>
                    {"Simple words, when structured and related systematically, can evolve into complex systems of knowledge. This evolution typically progresses from a taxonomy, to an ontology, and finally to a knowledge graph."}
                </p>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">{"Taxonomy"}</summary>
                    <p>
                        {"A taxonomy is a hierarchical classification or framework that organizes concepts or words into groups or categories based on their similarities or relationships. It simplifies the understanding and retrieval of information by categorizing individual elements into a structured form."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">{"Ontology"}</summary>
                    <p>
                        {"An ontology is a more advanced form of classification. It not only groups similar concepts but also defines the relationships between different concepts, providing a richer representation of knowledge by describing the nature of the relationships between those concepts."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Knowledge Graph"}
                    </summary>
                    <p>
                        {"A knowledge graph is a network of interconnected entities and their interrelations. Knowledge graphs are used to integrate information from different sources and represent them in a way that is understandable and usable for advanced analysis and decision-making."}
                    </p>
                </details>

                <p class="">
                    {"In essence, this progression represents the evolution of data into knowledge, enabling deeper insights, more informed decision-making, and more intelligent systems."}
                </p>
            </div>
        </div>
    }
}
