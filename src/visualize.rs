use leptos::*;

#[component]
pub fn Visualize() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"Visualize your Model"}
          </h1>
          <p class="mt-5">
              {"Event sourcing, when coupled with taxonomies, ontologies, and knowledge graphs, forms a powerful synergy, providing a multidimensional view of a business model. This integration offers a realistic and dynamic way to visualize and understand the intricate relationships and processes within a business, enhancing strategic planning and operational efficiency."}
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
                          {"Data Richness"}
                      </summary>
                      <p>
                          {"Event sourcing captures a detailed record of all business activities and changes, providing a rich dataset that reflects the real-time state and history of business operations, forming the backbone for a comprehensive analytical framework."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Structuring Knowledge"}
                      </summary>
                      <p>
                          {"Taxonomies and ontologies structure and define the relationships within the data captured by event sourcing. Taxonomies categorize and organize the data, while ontologies define the relationships between different data entities, enhancing understanding and navigability."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Visualizing with Knowledge Graphs"}
                      </summary>
                      <p>
                          {"Knowledge graphs visualize the interconnections and relationships defined by taxonomies and ontologies, offering an interactive and intuitive representation of the business model. This visualization aids in identifying patterns, dependencies, and opportunities for innovation and optimization."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"The seamless integration of event sourcing with taxonomies, ontologies, and knowledge graphs provides businesses with a realistic and detailed representation of their operations and relationships. This comprehensive view is instrumental for strategic decision-making, operational efficiency, and fostering a deeper understanding of the business ecosystem."}
          </p>
      </div>
  </div>
  
    }
}
