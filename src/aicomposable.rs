use leptos::*;

#[component]
pub fn AIComposable() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"AI Composable Insights"}
          </h1>
          <p class="mt-5">
              {"AI Composable Insights represent a modular approach in AI, where insights are generated through the assembly and reuse of various AI components. This method promotes flexibility, adaptability, and efficiency in AI-driven solutions."}
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
                          {"Modularity"}
                      </summary>
                      <p>
                          {"Modularity refers to the construction of complex systems from smaller, interchangeable components. In AI, this means creating insights from smaller, reusable AI models or services that can be easily combined or modified to suit different needs."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Flexibility & Adaptability"}
                      </summary>
                      <p>
                          {"The composable nature of these insights allows for flexibility and adaptability. Businesses can quickly adapt to changes or new data by reassembling or tweaking the AI components without having to rebuild from scratch."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Efficiency & Scalability"}
                      </summary>
                      <p>
                          {"Composable insights promote efficiency and scalability. Reusing and recombining AI components for different purposes reduces redundancy and accelerates the development of AI solutions, allowing for scaling as business needs grow."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"AI Composable Insights revolutionize the way businesses harness AI, offering a more dynamic, efficient, and adaptable approach to deriving meaningful and actionable insights from data."}
          </p>
      </div>
  </div>  
    }
}
