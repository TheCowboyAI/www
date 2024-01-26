use leptos::*;

#[component]
pub fn Security() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
        <div class="flex-row"> 
            <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Securing Systems"}
            </h1>
            <p class="mt-5">
                {"Event modeling and API integration, when combined with rigorously tested functionalities, significantly enhance the security of systems. This approach ensures that every component not only performs as expected but also adheres to strict security protocols, providing a robust defense against potential vulnerabilities."}
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
                            {"Structured Approach and Traceability"}
                        </summary>
                        <p>
                            {"Event modeling offers a structured approach to system design, where each interaction is captured as a discrete event. This granularity ensures traceability and accountability, making it easier to monitor, audit, and secure each aspect of the system's operations."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Rigorous Testing of API Integrations"}
                        </summary>
                        <p>
                            {"APIs serve as the communication backbone for different system components. Rigorously testing these integrations ensures that data exchanges are secure, authenticated, and validated, safeguarding the system from unauthorized access or data breaches."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Continuous Monitoring and Improvement"}
                        </summary>
                        <p>
                            {"The combination of event modeling and tested API integration facilitates continuous monitoring and improvement. It enables the early detection of anomalies or vulnerabilities and supports the prompt implementation of corrective measures, ensuring that the system's security posture evolves in line with emerging threats."}
                        </p>
                    </details>
                </li>
            </ol>
            <p class="mt-5">
                {"By employing event modeling and ensuring that API integrations are thoroughly tested and monitored, businesses can build secure, resilient systems that are well-equipped to protect against and rapidly respond to security threats, maintaining the integrity and trustworthiness of their operations."}
            </p>
        </div>
    </div>
    
    }
}
