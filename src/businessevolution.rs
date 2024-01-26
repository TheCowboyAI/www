use leptos::*;

#[component]
pub fn BusinessEvolution() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"Business Evolution"}
          </h1>
          <p class="mt-5">
              {"Business evolution refers to the continuous transformation and adaptation of a business to meet changing market demands, technology advancements, and shifts in consumer behavior."}
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
                          {"Adaptation to Market Changes"}
                      </summary>
                      <p>
                          {"It ensures that your business remains relevant and competitive by adapting to market trends, economic shifts, and changing consumer preferences."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Innovation and Growth"}
                      </summary>
                      <p>
                          {"Business evolution fosters innovation, allowing you to explore new markets, develop new products or services, and improve existing offerings."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Risk Mitigation"}
                      </summary>
                      <p>
                          {"By evolving, you can anticipate and respond to potential threats, such as new competitors or disruptive technologies, thereby minimizing risks."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Resource Optimization"}
                      </summary>
                      <p>
                          {"It encourages efficient use of resources by continually reassessing and optimizing operations, processes, and strategies in response to internal and external changes."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Customer Satisfaction"}
                      </summary>
                      <p>
                          {"Evolving with customer needs and expectations ensures that you maintain customer satisfaction and loyalty, which is crucial for long-term success."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Sustainability"}
                      </summary>
                      <p>
                          {"Business evolution promotes sustainability by encouraging adaptability, resilience, and a forward-thinking approach, ensuring that your business thrives in the long run."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Reputation and Brand Strength"}
                      </summary>
                      <p>
                          {"Keeping pace with or leading change can enhance your brand's reputation, positioning your business as an industry leader and innovator."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"In essence, business evolution is not just about survival in a changing environment but about seizing opportunities for growth, innovation, and sustained success. It involves a proactive approach to reshaping your business to align with the dynamic business landscape."}
          </p>
      </div>
  </div>
  
    }
}
