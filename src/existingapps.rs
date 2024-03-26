use leptos::*;

#[component]
pub fn ExistingApps() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
        <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
            <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl" id="architecture">
              {"AI Powered Extraction and Generation"}
          </h1>
          <p class="">
              {"AI significantly streamlines the process of extracting APIs and generating fully tested code, even from legacy systems or applications where the internals are not directly accessible. This enables businesses to swiftly integrate and innovate with AI, unlocking new capabilities and efficiencies."}
          </p>
        <details class="text-black text-sm">
            <summary class="theme_bg-1-color-5 text-base font-bold">
                {"Efficient API Extraction"}
            </summary>
            <p>
                {"AI tools can analyze application structures and data flows to identify and extract APIs quickly. This allows your business to connect and automate various systems, even if they were not originally designed for integration."}
            </p>
        </details>


        <details class="text-black text-sm">
            <summary class="theme_bg-1-color-5 text-base font-bold">
                {"Automated Code Testing"}
            </summary>
            <p>
                {"AI not only generates code but also ensures it is fully tested. It simulates various scenarios and use cases, rigorously testing the code to ensure it performs correctly and reliably under different conditions."}
            </p>
        </details>

        <details class="text-black text-sm">
            <summary class="theme_bg-1-color-5 text-base font-bold">
                {"Seamless Integration with Existing Applications"}
            </summary>
            <p>
                {"The ability of AI to understand and interact with existing applications, even without internal access, enables your business to integrate advanced AI capabilities seamlessly. This opens up opportunities for enhancing existing applications and creating new, innovative solutions."}
            </p>
        </details>

        <p class="">
              {"With AI's ability to quickly extract APIs and produce fully tested code, businesses can accelerate their digital transformation, integrate diverse systems more efficiently, and harness the full potential of their existing applications in conjunction with AI-driven innovations."}
          </p>
      </div>
  </div>
  
    }
}
