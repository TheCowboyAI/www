use leptos::*;

#[component]
pub fn Security() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1
                    class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                    id="architecture"
                >
                    {"Securing Systems"}
                </h1>

                <p>
                    {"Event modeling and API integration, when combined with rigorously tested functionalities, significantly enhance the security of systems. This approach ensures that every component not only performs as expected but also adheres to strict security protocols, providing a robust defense against potential vulnerabilities."}
                </p>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Structured Approach and Traceability"}
                    </summary>
                    <p>
                        {"Event modeling offers a structured approach to system design, where each interaction is captured as a discrete event. This granularity ensures traceability and accountability, making it easier to monitor, audit, and secure each aspect of the system's operations."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Rigorous Testing of API Integrations"}
                    </summary>
                    <p>
                        {"APIs serve as the communication backbone for different system components. Rigorously testing these integrations ensures that data exchanges are secure, authenticated, and validated, safeguarding the system from unauthorized access or data breaches."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Continuous Monitoring and Improvement"}
                    </summary>
                    <p>
                        {"The combination of event modeling and tested API integration facilitates continuous monitoring and improvement. It enables the early detection of anomalies or vulnerabilities and supports the prompt implementation of corrective measures, ensuring that the system's security posture evolves in line with emerging threats."}
                    </p>
                </details>

                <p>
                    {"By employing event modeling and ensuring that API integrations are thoroughly tested and monitored, businesses can build secure, resilient systems that are well-equipped to protect against and rapidly respond to security threats, maintaining the integrity and trustworthiness of their operations."}
                </p>
            </div>
        </div>
    }
}
