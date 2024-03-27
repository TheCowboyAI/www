use leptos::*;

#[component]
pub fn CustomerSupport() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
        <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
            <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"Custom API Integration Support"}
                </h1>

                <p>
                    {"Cowboy AI revolutionizes system integration by combining Event Sourcing and Content-Addressing within a clearly defined Business Model. This advanced approach ensures that systems are not only interconnected and data-driven but also meticulously aligned with the business's strategic objectives and operational workflows."}
                </p>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Custom API Integration"}
                    </summary>
                    <p>
                        {"Cowboy AI leverages Event Sourcing to capture and store each event within your systems, providing a rich historical record for analysis and decision-making. This facilitates seamless API integration, enabling systems to communicate and exchange data efficiently, ensuring that every interaction is based on the most accurate and up-to-date information."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Content-Addressing"}
                    </summary>
                    <p>
                        {"Content-Addressing is employed to ensure the integrity and security of data. Each piece of data is uniquely identified, making it easier to retrieve, verify, and reference, thereby enhancing the reliability and trustworthiness of the system's data exchanges and interactions."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Alignment with the Business Model"}
                    </summary>
                    <p>
                        {"Cowboy AI ensures that every technical solution and integration is in perfect harmony with the defined Business Model. By understanding and aligning with the business's goals, customer relationships, and operational processes, Cowboy AI ensures that the system not only supports but also enhances business strategies and delivers tangible value."}
                    </p>
                </details>

                <p>
                    {"Cowboy AI's commitment to integrating cutting-edge technology like Event Sourcing and Content-Addressing with a clear understanding of the Business Model sets the stage for creating systems that are not only technically advanced but also strategically aligned, driving business growth and operational excellence."}
                </p>
            </div>
        </div>
    }
}
