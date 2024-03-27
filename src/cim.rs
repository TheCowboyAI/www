use leptos::*;

use crate::cimsvg::CimSvg;

#[component]
pub fn Cim() -> impl IntoView {
  view! {
      <div class="mt-5 animate-fadeIn">
          <div class="flex flex-col gap-4 theme_bg-1-bg-3 rounded-xl p-3 shadow-xl shadow-black">
              <h1
                  class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                  id="architecture"
              >
                  {"CIM: Next Level AI Strategy"}
              </h1>
              <p class="">
                  {"Composable Information Machine (CIM) represents a forward-thinking strategy that is set to redefine the landscape of computing. It's an innovative approach that integrates modularity, intelligence, and adaptability, paving the way for more robust, scalable, and efficient systems."}
              </p>
              <p class="">
                  {"There are mountains of data and places to get it, how will we utilize that best?"}
              </p>
              <CimSvg/>

              <details class="text-black text-sm">
                  <summary class="theme_bg-1-color-5 text-base font-bold">
                      {"Modularity and Flexibility"}
                  </summary>
                  <p>
                      {"CIM is built on the principle of modularity, allowing systems to be composed of interchangeable and reusable components. This modularity brings unprecedented flexibility and agility, enabling businesses to adapt quickly to changing needs without extensive rework."}
                  </p>
              </details>

              <details class="text-black text-sm">
                  <summary class="theme_bg-1-color-5 text-base font-bold">
                      {"Intelligent Automation"}
                  </summary>
                  <p>
                      {"At its core, CIM leverages AI to automate and optimize various processes. From data analysis to decision-making, CIM's intelligent automation ensures that operations are not only faster but also smarter and more reliable."}
                  </p>
              </details>

              <details class="text-black text-sm">
                  <summary class="theme_bg-1-color-5 text-base font-bold">
                      {"Scalability and Efficiency"}
                  </summary>
                  <p>
                      {"CIM's design inherently supports scalability. As business demands grow, CIM systems can scale accordingly, efficiently handling increased workloads and complexities without compromising performance or stability."}
                  </p>
              </details>

              <p class="">
                  {"CIM is not just an evolution in computing; it's a revolution that's poised to set new benchmarks in efficiency, adaptability, and intelligence. By embracing CIM, businesses are equipping themselves to thrive in the dynamic, data-driven landscape of tomorrow."}
              </p>
          </div>
      </div>
  }
}
