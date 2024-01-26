use leptos::*;

#[component]
pub fn RealTimeMonitoring() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
        <div class="flex-row"> 
            <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Real-Time Monitoring"}
            </h1>
            <p class="mt-5">
                {"Event Streams, coupled with Projections, provide a powerful mechanism for real-time monitoring, allowing businesses to capture, analyze, and respond to events as they occur. This dynamic duo offers an immediate, accurate view of operations, enhancing responsiveness and situational awareness."}
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
                            {"Immediate Data Capture and Analysis"}
                        </summary>
                        <p>
                            {"Event Streams capture data in real-time, providing an immediate flow of information. Projections process this data instantly, transforming it into actionable insights, ensuring that businesses have up-to-the-minute information at their fingertips."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Customized Views for Different Needs"}
                        </summary>
                        <p>
                            {"Projections can create customized views of the event data, tailored to the specific needs of different business units. This means that each department gets the information that is most relevant to their operations, enhancing efficiency and decision-making."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Enabling Responsive Operations"}
                        </summary>
                        <p>
                            {"The real-time nature of Event Streams and Projections allows businesses to be highly responsive. They can detect and react to changes, anomalies, or new opportunities instantly, providing a competitive edge in fast-paced environments."}
                        </p>
                    </details>
                </li>
            </ol>
            <p class="mt-5">
                {"By harnessing the power of Event Streams and Projections for real-time monitoring, businesses can ensure operational excellence, maintain high standards of performance, and stay ahead of the curve in today's ever-evolving market landscape."}
            </p>
        </div>
    </div>
    
    }
}
