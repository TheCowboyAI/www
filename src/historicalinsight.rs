use leptos::*;

#[component]
pub fn HistoricalInsight() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"Historical Insight"}
          </h1>
          <p class="mt-5">
              {"Event Sourcing is not just a method for recording events; it's a strategic tool that provides deep historical insights, enabling businesses to understand past behaviors, make informed decisions, and evolve their strategies and operations over time."}
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
                          {"Understanding Past Behaviors"}
                      </summary>
                      <p>
                          {"Event Sourcing captures every change and interaction, offering an in-depth view of past behaviors and trends. This comprehensive historical record allows businesses to analyze patterns and understand the outcomes of past decisions."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Informed Decision-Making"}
                      </summary>
                      <p>
                          {"The insights gained from Event Sourcing equip businesses with the knowledge needed for informed decision-making. It provides a factual basis for predicting future trends, optimizing operations, and planning strategic initiatives."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Evolving Business Strategies"}
                      </summary>
                      <p>
                          {"Event Sourcing allows businesses to evolve by learning from the past. By understanding the impact of previous actions, businesses can refine their strategies, innovate processes, and adapt to changes more effectively, ensuring continuous improvement and growth."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"Through the detailed historical insights provided by Event Sourcing, businesses can embark on a path of continuous evolution, transforming data into actionable intelligence and fostering an environment of informed strategy and dynamic growth."}
          </p>
      </div>
  </div>
  
    }
}
