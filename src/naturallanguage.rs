use leptos::*;

#[component]
pub fn NaturalLanguage() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"Natural Language Processing by AI"}
          </h1>
          <p class="mt-5">
              {"AI has made a revolutionary leap by integrating Natural Language Processing (NLP) into business processes and information systems, transforming how businesses interact with data and communicate with customers."}
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
                          {"Enhanced Communication with Customers"}
                      </summary>
                      <p>
                          {"AI-driven NLP allows businesses to understand and communicate with customers in natural language, leading to more engaging, personalized, and effective interactions. This includes everything from intelligent chatbots to sophisticated customer service systems."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Efficient Information Processing"}
                      </summary>
                      <p>
                          {"NLP enables the processing of large volumes of text data, extracting meaningful insights, automating routine tasks, and making informed decisions. This transforms data analysis, market research, and strategic planning processes."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Seamless Integration into Business Processes"}
                      </summary>
                      <p>
                          {"AI with NLP seamlessly integrates into various business processes, enhancing efficiency and productivity. It simplifies complex tasks such as document analysis, report generation, and even provides real-time language translation services."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"The integration of NLP by AI marks a significant advancement in technology, offering businesses unprecedented capabilities to interact, understand, and serve their customers better while processing information with a level of depth and understanding that was previously unattainable."}
          </p>
      </div>
  </div>
  
    }
}
