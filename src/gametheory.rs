use leptos::*;

#[component]
pub fn GameTheory() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"Evolutionary Models with Game Theory"}
          </h1>
          <p class="mt-5">
              {"Evolutionary Models with Game Theory combine the principles of evolutionary biology with the strategic decision-making framework of game theory, offering insights into how competitive strategies evolve and stabilize within populations over time."}
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
                          {"Strategic Adaptation and Survival"}
                      </summary>
                      <p>
                          {"These models simulate how individuals or strategies adapt and survive within competitive environments. They provide a dynamic view of how interactions and decisions lead to the natural selection of the most effective strategies over time."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Equilibrium and Stability Analysis"}
                      </summary>
                      <p>
                          {"Game theory concepts are used to analyze and predict the points of equilibrium and stability within a system. This helps in understanding how various strategies will perform and stabilize in the long run, given the nature of their interactions."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Insights into Behavioral Dynamics"}
                      </summary>
                      <p>
                          {"These models shed light on the behavioral dynamics of populations, illustrating how individual decisions impact group behavior and vice versa. This is crucial for predicting the outcomes of strategic interactions in complex systems like markets or ecosystems."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"Evolutionary Models with Game Theory provide a robust framework for analyzing and predicting the development and stability of competitive strategies, offering invaluable insights into a wide array of fields from economics to ecology."}
          </p>
      </div>
  </div>
  
    }
}
