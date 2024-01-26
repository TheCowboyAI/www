use leptos::*;

#[component]
pub fn CustomerSupport() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"Custom API Integration Support"}
          </h1>
          <p class="mt-5">
              {"Cowboy AI revolutionizes system integration by combining Event Sourcing and Content-Addressing within a clearly defined Business Model. This advanced approach ensures that systems are not only interconnected and data-driven but also meticulously aligned with the business's strategic objectives and operational workflows."}
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
                          {"Custom API Integration"}
                      </summary>
                      <p>
                          {"Cowboy AI leverages Event Sourcing to capture and store each event within your systems, providing a rich historical record for analysis and decision-making. This facilitates seamless API integration, enabling systems to communicate and exchange data efficiently, ensuring that every interaction is based on the most accurate and up-to-date information."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Content-Addressing"}
                      </summary>
                      <p>
                          {"Content-Addressing is employed to ensure the integrity and security of data. Each piece of data is uniquely identified, making it easier to retrieve, verify, and reference, thereby enhancing the reliability and trustworthiness of the system's data exchanges and interactions."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Alignment with the Business Model"}
                      </summary>
                      <p>
                          {"Cowboy AI ensures that every technical solution and integration is in perfect harmony with the defined Business Model. By understanding and aligning with the business's goals, customer relationships, and operational processes, Cowboy AI ensures that the system not only supports but also enhances business strategies and delivers tangible value."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"Cowboy AI's commitment to integrating cutting-edge technology like Event Sourcing and Content-Addressing with a clear understanding of the Business Model sets the stage for creating systems that are not only technically advanced but also strategically aligned, driving business growth and operational excellence."}
          </p>
      </div>
  </div>
  
    }
}
