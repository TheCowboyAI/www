use leptos::*;

#[component]
pub fn PushToMobile() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"Real-Time Business Intelligence"}
                </h1>
                <p>
                    {"Event streams and projections are at the forefront of delivering real-time business intelligence, enabling the creation of live dashboards, instant reports, and the seamless dissemination of information to mobile devices and social media platforms, ensuring that key insights are always within reach."}
                </p>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Live Dashboards and Instant Analytics"}
                    </summary>
                    <p>
                        {"Event streams power live dashboards, providing a real-time view of business operations. Projections analyze and transform this data into digestible visuals and metrics, allowing for instant insight into performance, trends, and anomalies."}
                    </p>
                </details>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Reports on the Go"}
                    </summary>
                    <p>
                        {"The integration of projections with mobile technology enables the generation and push of reports directly to users' mobile devices. Whether in the office or on the move, stakeholders can receive timely updates, making decision-making agile and informed."}
                    </p>
                </details>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Social Media Integration"}
                    </summary>
                    <p>
                        {"Event streams can be configured to trigger notifications or post updates directly to social media platforms, ensuring broader engagement and real-time dissemination of crucial information, keeping teams and clients in the loop at all times."}
                    </p>
                </details>

                <p>
                    {"Leveraging event streams and projections for real-time monitoring and information dissemination transforms the pace at which businesses operate. It fosters a culture of immediate insight, proactive response, and connectedness, driving performance and competitive advantage."}
                </p>
            </div>
        </div>
    }
}
