use leptos::*;

#[component]
pub fn HistoricalInsight() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1
                    class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                    id="architecture"
                >
                    {"Historical Insight"}
                </h1>
                <p>
                    {"Event Sourcing is not just a method for recording events; it's a strategic tool that provides deep historical insights, enabling businesses to understand past behaviors, make informed decisions, and evolve their strategies and operations over time."}
                </p>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Understanding Past Behaviors"}
                    </summary>
                    <p>
                        {"Event Sourcing captures every change and interaction, offering an in-depth view of past behaviors and trends. This comprehensive historical record allows businesses to analyze patterns and understand the outcomes of past decisions."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Informed Decision-Making"}
                    </summary>
                    <p>
                        {"The insights gained from Event Sourcing equip businesses with the knowledge needed for informed decision-making. It provides a factual basis for predicting future trends, optimizing operations, and planning strategic initiatives."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Evolving Business Strategies"}
                    </summary>
                    <p>
                        {"Event Sourcing allows businesses to evolve by learning from the past. By understanding the impact of previous actions, businesses can refine their strategies, innovate processes, and adapt to changes more effectively, ensuring continuous improvement and growth."}
                    </p>
                </details>

                <p>
                    {"Through the detailed historical insights provided by Event Sourcing, businesses can embark on a path of continuous evolution, transforming data into actionable intelligence and fostering an environment of informed strategy and dynamic growth."}
                </p>
            </div>
        </div>
    }
}
