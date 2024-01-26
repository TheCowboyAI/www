use leptos::*;

#[component]
pub fn Collaborate() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"Fostering Collaboration"}
          </h1>
          <p class="mt-5">
              {"A system built with event modeling and domain-driven design (DDD) inherently promotes more effective collaboration. This approach aligns technical development with business needs, ensuring that all team members have a shared understanding of the system’s functionality and the business context, fostering a cohesive and productive working environment."}
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
                          {"Shared Understanding with Event Modeling"}
                      </summary>
                      <p>
                          {"Event modeling visualizes the flow of events and interactions within the system, providing a clear and shared understanding of processes and workflows. This shared perspective helps align technical and business teams, ensuring that everyone is on the same page regarding system behavior and expectations."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Domain-Driven Design for Business Alignment"}
                      </summary>
                      <p>
                          {"Domain-driven design focuses on creating a model that reflects the business domain, ensuring that the system’s structure and language are aligned with business concepts and terminology. This alignment helps bridge the gap between technical and non-technical team members, enhancing mutual understanding and collaboration."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Enhanced Collaboration and Iteration"}
                      </summary>
                      <p>
                          {"By using event modeling and DDD, teams can more effectively collaborate on identifying requirements, designing solutions, and iterating on the system. The clarity and shared language provided by these methodologies ensure that contributions from all team members are constructive, relevant, and directly tied to business objectives."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"Incorporating event modeling and domain-driven design into system development cultivates an environment where collaboration is natural, effective, and oriented towards delivering solutions that truly resonate with the business’s needs and goals."}
          </p>
      </div>
  </div>
  
    }
}
