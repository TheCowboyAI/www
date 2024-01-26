use leptos::*;

#[component]
pub fn NewFunctionality() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"Enhancing Business Systems with AI & Event Modeling"}
          </h1>
          <p class="mt-5">
              {"AI and Event Modeling are revolutionizing the way we enhance our business systems, enabling the addition of new functionalities without the complexity of monolithic applications. They streamline the process of creating interactive dashboards and insightful reports through efficient data handling and analysis."}
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
                          {"Modular System Enhancement"}
                      </summary>
                      <p>
                          {"AI and Event Modeling allow for the modular addition of new functionalities to business systems. This means you can integrate new features or improve existing ones without having to overhaul the entire system, avoiding the pitfalls of monolithic application structures."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Efficient Data Handling and Analysis"}
                      </summary>
                      <p>
                          {"Event Modeling structures data as a series of events, making it easier for AI algorithms to analyze and derive meaningful insights. This structured approach ensures that data is handled efficiently, and insights are generated in a more coherent and context-aware manner."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Dynamic Dashboards and Reports"}
                      </summary>
                      <p>
                          {"Projections from events can be used to create dynamic dashboards and reports, offering real-time insights into business operations. AI enhances this by providing predictive analytics and personalized reports, ensuring that decision-makers have access to the most relevant and actionable information."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"The combination of AI and Event Modeling not only facilitates the seamless integration of new functionalities but also transforms the way businesses interpret data, making systems more adaptable, insights more accessible, and decision-making processes more informed."}
          </p>
      </div>
  </div>
  
    }
}
