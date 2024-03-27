use leptos::*;

#[component]
pub fn Predictions() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1
                    class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                    id="architecture"
                >
                    {"Predicting the Future"}
                </h1>

                <p>
                    {"Event streams provide a comprehensive chronological record of actions and changes, offering significant advantages for predicting future events. This predictive capability enables businesses to anticipate trends, prepare for potential scenarios, and make proactive, informed decisions."}
                </p>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Rich Data for Analysis"}
                    </summary>
                    <p>
                        {"Event streams capture every interaction and change in the system, providing a rich dataset for analysis. This data richness is critical for understanding past behaviors, identifying patterns, and accurately predicting future trends."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Pattern Recognition and Trend Forecasting"}
                    </summary>
                    <p>
                        {"Advanced algorithms can analyze event streams to recognize patterns and forecast trends. By understanding the sequence and impact of past events, businesses can predict future events with a higher degree of accuracy."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Proactive Decision-Making"}
                    </summary>
                    <p>
                        {"The predictive insights gained from event streams empower businesses to make proactive decisions. Instead of reacting to situations as they occur, businesses can anticipate changes and strategically plan their actions in advance."}
                    </p>
                </details>

                <p>
                    {"Leveraging event streams for predictive analytics offers businesses a significant advantage in navigating the future. This approach not only enhances decision-making but also drives innovation, adaptability, and competitive edge."}
                </p>
            </div>
        </div>
    }
}
