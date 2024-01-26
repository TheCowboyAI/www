use leptos::*;

use crate::cimsvg::CimSvg;

#[component]
pub fn Cim() -> impl IntoView {
  view! {
    <div class="mt-10 mr-5 md:mr-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"CIM: Next Level Computing AI Strategy"}
          </h1>
          <p class="mt-5">
              {"Composable Information Machine (CIM) represents a forward-thinking AI strategy that is set to redefine the landscape of computing. It's an innovative approach that integrates modularity, intelligence, and adaptability, paving the way for more robust, scalable, and efficient systems."}
          </p>
          <ol
              class=" 
                  mr-5 py-2
                  text-black text-xl md:text-2xl lg:text-3xl 
              "
          >
                <li class="mt-5"> 
                    <CimSvg />                   
                </li>
                <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Modularity and Flexibility"}
                      </summary>
                      <p>
                          {"CIM is built on the principle of modularity, allowing systems to be composed of interchangeable and reusable components. This modularity brings unprecedented flexibility and agility, enabling businesses to adapt quickly to changing needs without extensive rework."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Intelligent Automation"}
                      </summary>
                      <p>
                          {"At its core, CIM leverages AI to automate and optimize various processes. From data analysis to decision-making, CIM's intelligent automation ensures that operations are not only faster but also smarter and more reliable."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Scalability and Efficiency"}
                      </summary>
                      <p>
                          {"CIM's design inherently supports scalability. As business demands grow, CIM systems can scale accordingly, efficiently handling increased workloads and complexities without compromising performance or stability."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"CIM is not just an evolution in computing; it's a revolution that's poised to set new benchmarks in efficiency, adaptability, and intelligence. By embracing CIM, businesses are equipping themselves to thrive in the dynamic, data-driven landscape of tomorrow."}
          </p>
      </div>
  </div>
  }
}
