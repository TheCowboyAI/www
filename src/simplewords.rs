use leptos::*;

#[component]
pub fn SimpleWords() -> impl IntoView {
    view! {
    <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"Evolution from Simple Words to Knowledge Graphs"}
          </h1>
          <p class="mt-5">
              {"Simple words, when structured and related systematically, can evolve into complex systems of knowledge. This evolution typically progresses from a taxonomy, to an ontology, and finally to a knowledge graph."}
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
                          {"Taxonomy"}
                      </summary>
                      <p>
                          {"A taxonomy is a hierarchical classification or framework that organizes concepts or words into groups or categories based on their similarities or relationships. It simplifies the understanding and retrieval of information by categorizing individual elements into a structured form."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Ontology"}
                      </summary>
                      <p>
                          {"An ontology is a more advanced form of classification. It not only groups similar concepts but also defines the relationships between different concepts, providing a richer representation of knowledge by describing the nature of the relationships between those concepts."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Knowledge Graph"}
                      </summary>
                      <p>
                          {"A knowledge graph is a network of interconnected entities and their interrelations. Knowledge graphs are used to integrate information from different sources and represent them in a way that is understandable and usable for advanced analysis and decision-making."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"In essence, this progression represents the evolution of data into knowledge, enabling deeper insights, more informed decision-making, and more intelligent systems."}
          </p>
      </div>
  </div>
  }
}
