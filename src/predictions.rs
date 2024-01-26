use leptos::*;

#[component]
pub fn Predictions() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
        <div class="flex-row"> 
            <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Predicting the Future"}
            </h1>
            <p class="mt-5">
                {"Event streams provide a comprehensive chronological record of actions and changes, offering significant advantages for predicting future events. This predictive capability enables businesses to anticipate trends, prepare for potential scenarios, and make proactive, informed decisions."}
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
                            {"Rich Data for Analysis"}
                        </summary>
                        <p>
                            {"Event streams capture every interaction and change in the system, providing a rich dataset for analysis. This data richness is critical for understanding past behaviors, identifying patterns, and accurately predicting future trends."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Pattern Recognition and Trend Forecasting"}
                        </summary>
                        <p>
                            {"Advanced algorithms can analyze event streams to recognize patterns and forecast trends. By understanding the sequence and impact of past events, businesses can predict future events with a higher degree of accuracy."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Proactive Decision-Making"}
                        </summary>
                        <p>
                            {"The predictive insights gained from event streams empower businesses to make proactive decisions. Instead of reacting to situations as they occur, businesses can anticipate changes and strategically plan their actions in advance."}
                        </p>
                    </details>
                </li>
            </ol>
            <p class="mt-5">
                {"Leveraging event streams for predictive analytics offers businesses a significant advantage in navigating the future. This approach not only enhances decision-making but also drives innovation, adaptability, and competitive edge."}
            </p>
        </div>
    </div>
  
    }
}
