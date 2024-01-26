use leptos::*;

#[component]
pub fn AskQuestions() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"AI-Powered Queries on Modeled Business Data"}
          </h1>
          <p class="mt-5">
              {"AI revolutionizes the way we interact with modeled business data by enabling us to ask complex questions and receive insightful answers, making data more accessible and actionable for strategic decision-making."}
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
                          {"Natural Language Data Interaction"}
                      </summary>
                      <p>
                          {"AI allows users to query business data using natural language, making it easier for non-technical team members to extract valuable insights without needing to understand complex database queries or data science concepts."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Intelligent Data Analysis"}
                      </summary>
                      <p>
                          {"With AI, businesses can analyze and interpret their data more deeply. AI algorithms can uncover patterns, predict trends, and offer recommendations, turning raw data into actionable intelligence."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Enhanced Decision-Making"}
                      </summary>
                      <p>
                          {"By enabling more intuitive and comprehensive data querying and analysis, AI empowers business leaders and decision-makers to make more informed, data-driven decisions quickly and efficiently."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"The ability to ask questions about any modeled data in a business through AI not only democratizes data access but also enriches the decision-making process, ensuring that every business move is backed by solid, data-driven insights."}
          </p>
      </div>
  </div>
  
    }
}
