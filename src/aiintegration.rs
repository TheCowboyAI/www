use leptos::*;

#[component]
pub fn AIIntegration() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">

                <h1 class="font-bold rounded-lg shadow shadow-black shadow-text-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"AI Integration Experts"}
                </h1>

                <p>
                    {"Cowboy AI stands at the forefront of AI integration, expertly navigating the complexities of artificial intelligence to tailor solutions that are perfectly aligned with your business needs. As AI Integration Experts, Cowboy AI ensures that your systems are intelligent, efficient, and strategically designed to drive growth and innovation."}
                </p>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Tailored AI Solutions"}
                    </summary>
                    <p>
                        {"Cowboy AI excels in crafting AI solutions that are tailor-made for your business. From understanding your unique challenges to designing custom AI models, every solution is meticulously crafted to fit your specific operational needs and strategic goals."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Seamless System Integration"}
                    </summary>
                    <p>
                        {"Integrating AI into existing systems is a complex task, but it's where Cowboy AI shines. Our experts ensure that AI components are seamlessly integrated with your infrastructure, data pipelines, and workflows, enhancing system capabilities without disrupting your operations."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Strategic Business Alignment"}
                    </summary>
                    <p>
                        {"Cowboy AI goes beyond technical integration, ensuring that AI solutions are in complete harmony with your business model and strategic objectives. Our approach guarantees that AI acts as a catalyst for growth, innovation, and competitive advantage."}
                    </p>
                </details>

                <p>
                    {"As your AI Integration Experts, Cowboy AI is committed to transforming your business with artificial intelligence, ensuring that every integration is strategic, seamless, and perfectly attuned to driving your business forward."}
                </p>
            </div>
        </div>
    }
}
