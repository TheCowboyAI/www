use leptos::*;

#[component]
pub fn RealTimeMonitoring() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"Real-Time Monitoring"}
                </h1>
                <p>
                    {"Event Streams, coupled with Projections, provide a powerful mechanism for real-time monitoring, allowing businesses to capture, analyze, and respond to events as they occur. This dynamic duo offers an immediate, accurate view of operations, enhancing responsiveness and situational awareness."}
                </p>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Immediate Data Capture and Analysis"}
                    </summary>
                    <p>
                        {"Event Streams capture data in real-time, providing an immediate flow of information. Projections process this data instantly, transforming it into actionable insights, ensuring that businesses have up-to-the-minute information at their fingertips."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Customized Views for Different Needs"}
                    </summary>
                    <p>
                        {"Projections can create customized views of the event data, tailored to the specific needs of different business units. This means that each department gets the information that is most relevant to their operations, enhancing efficiency and decision-making."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Enabling Responsive Operations"}
                    </summary>
                    <p>
                        {"The real-time nature of Event Streams and Projections allows businesses to be highly responsive. They can detect and react to changes, anomalies, or new opportunities instantly, providing a competitive edge in fast-paced environments."}
                    </p>
                </details>

                <p>
                    {"By harnessing the power of Event Streams and Projections for real-time monitoring, businesses can ensure operational excellence, maintain high standards of performance, and stay ahead of the curve in today's ever-evolving market landscape."}
                </p>
            </div>
        </div>
    }
}
