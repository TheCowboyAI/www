use leptos::*;

#[component]
pub fn ExistingApps() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"AI-Powered API Extraction and Code Testing"}
          </h1>
          <p class="mt-5">
              {"AI significantly streamlines the process of extracting APIs and generating fully tested code, even from legacy systems or applications where the internals are not directly accessible. This enables businesses to swiftly integrate and innovate with AI, unlocking new capabilities and efficiencies."}
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
                          {"Efficient API Extraction"}
                      </summary>
                      <p>
                          {"AI tools can analyze application structures and data flows to identify and extract APIs quickly. This allows your business to connect and automate various systems, even if they were not originally designed for integration."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Automated Code Testing"}
                      </summary>
                      <p>
                          {"AI not only generates code but also ensures it is fully tested. It simulates various scenarios and use cases, rigorously testing the code to ensure it performs correctly and reliably under different conditions."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Seamless Integration with Existing Applications"}
                      </summary>
                      <p>
                          {"The ability of AI to understand and interact with existing applications, even without internal access, enables your business to integrate advanced AI capabilities seamlessly. This opens up opportunities for enhancing existing applications and creating new, innovative solutions."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"With AI's ability to quickly extract APIs and produce fully tested code, businesses can accelerate their digital transformation, integrate diverse systems more efficiently, and harness the full potential of their existing applications in conjunction with AI-driven innovations."}
          </p>
      </div>
  </div>
  
    }
}
