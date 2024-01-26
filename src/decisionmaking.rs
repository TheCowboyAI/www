use leptos::*;

#[component]
pub fn DecisionMaking() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
        <div class="flex-row"> 
            <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Enhanced Decision Making"}
            </h1>
            <p class="mt-5">
                {"Event streams significantly enhance decision-making processes by providing real-time analysis of data. This timely insight allows businesses to react swiftly to changes, capitalize on opportunities, and mitigate risks as they happen, ensuring decisions are informed, strategic, and timely."}
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
                            {"Instant Data Analysis"}
                        </summary>
                        <p>
                            {"Event streams provide a continuous flow of data, enabling instant analysis of activities and interactions as they occur. This real-time analysis ensures that businesses have the most current information at their disposal for making decisions."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Proactive Opportunity and Risk Management"}
                        </summary>
                        <p>
                            {"With real-time data, businesses can proactively identify and act on opportunities as well as detect and mitigate risks immediately. This timely response is crucial in today's fast-paced market environment, ensuring that decisions are a step ahead."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Informed Strategic Planning"}
                        </summary>
                        <p>
                            {"The depth and immediacy of insights gained from real-time analysis of event streams allow for more informed strategic planning. Businesses can understand trends, predict outcomes, and tailor their strategies to align with the evolving market dynamics."}
                        </p>
                    </details>
                </li>
            </ol>
            <p class="mt-5">
                {"By leveraging real-time analysis from event streams, businesses can ensure their decision-making process is not only responsive but also forward-thinking, aligning operational actions with strategic objectives and driving sustainable growth and success."}
            </p>
        </div>
    </div>
    
    }
}
