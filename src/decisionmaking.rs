use leptos::*;

#[component]
pub fn DecisionMaking() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"Enhanced Decision Making"}
                </h1>
                <p>
                    {"Event streams significantly enhance decision-making processes by providing real-time analysis of data. This timely insight allows businesses to react swiftly to changes, capitalize on opportunities, and mitigate risks as they happen, ensuring decisions are informed, strategic, and timely."}
                </p>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Instant Data Analysis"}
                    </summary>
                    <p>
                        {"Event streams provide a continuous flow of data, enabling instant analysis of activities and interactions as they occur. This real-time analysis ensures that businesses have the most current information at their disposal for making decisions."}
                    </p>
                </details>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Proactive Opportunity and Risk Management"}
                    </summary>
                    <p>
                        {"With real-time data, businesses can proactively identify and act on opportunities as well as detect and mitigate risks immediately. This timely response is crucial in today's fast-paced market environment, ensuring that decisions are a step ahead."}
                    </p>
                </details>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Informed Strategic Planning"}
                    </summary>
                    <p>
                        {"The depth and immediacy of insights gained from real-time analysis of event streams allow for more informed strategic planning. Businesses can understand trends, predict outcomes, and tailor their strategies to align with the evolving market dynamics."}
                    </p>
                </details>

                <p>
                    {"By leveraging real-time analysis from event streams, businesses can ensure their decision-making process is not only responsive but also forward-thinking, aligning operational actions with strategic objectives and driving sustainable growth and success."}
                </p>
            </div>
        </div>
    }
}
