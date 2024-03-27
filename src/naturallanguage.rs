use leptos::*;

#[component]
pub fn NaturalLanguage() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"Natural Language Processing by AI"}
                </h1>

                <p>
                    {"AI has made a revolutionary leap by integrating Natural Language Processing (NLP) into business processes and information systems, transforming how businesses interact with data and communicate with customers."}
                </p>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Enhanced Communication with Customers"}
                    </summary>
                    <p>
                        {"AI-driven NLP allows businesses to understand and communicate with customers in natural language, leading to more engaging, personalized, and effective interactions. This includes everything from intelligent chatbots to sophisticated customer service systems."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Efficient Information Processing"}
                    </summary>
                    <p>
                        {"NLP enables the processing of large volumes of text data, extracting meaningful insights, automating routine tasks, and making informed decisions. This transforms data analysis, market research, and strategic planning processes."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Seamless Integration into Business Processes"}
                    </summary>
                    <p>
                        {"AI with NLP seamlessly integrates into various business processes, enhancing efficiency and productivity. It simplifies complex tasks such as document analysis, report generation, and even provides real-time language translation services."}
                    </p>
                </details>

                <p>
                    {"The integration of NLP by AI marks a significant advancement in technology, offering businesses unprecedented capabilities to interact, understand, and serve their customers better while processing information with a level of depth and understanding that was previously unattainable."}
                </p>
            </div>
        </div>
    }
}
