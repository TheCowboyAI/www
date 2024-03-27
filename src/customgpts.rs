use leptos::*;

#[component]
pub fn CustomGPTs() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row">
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                    {"Crafting Custom AI Agents"}
                </h1>
                <p class="mt-5">
                    {"Cowboy AI excels in creating tailored GPTs and AI Agents that are not just technologically advanced but also intricately aligned with your business model. These intelligent agents are designed to understand your business's unique landscape, communicate with each other, and securely access internal company data to drive informed decisions and actions."}
                </p>
                <ol class=" 
                mr-5 py-2
                text-black text-xl md:text-2xl lg:text-3xl 
                ">
                    <li class="mt-5">
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                                {"Custom GPTs and AI Agents"}
                            </summary>
                            <p>
                                {"Cowboy AI specializes in developing custom GPTs and AI Agents that are specifically programmed to understand and operate within the nuances of your business. These agents are not one-size-fits-all but are uniquely tailored to meet your business's specific needs and objectives."}
                            </p>
                        </details>
                    </li>
                    <li class="mt-5">
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                                {"Inter-Agent Communication"}
                            </summary>
                            <p>
                                {"The AI Agents possess the capability to communicate with each other, creating a network of intelligence that collaborates to solve complex problems, optimize processes, and provide comprehensive insights, making your business more connected and intelligent."}
                            </p>
                        </details>
                    </li>
                    <li class="mt-5">
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                                {"Access to Internal Company Data"}
                            </summary>
                            <p>
                                {"Cowboy AI Agents are granted secure access to your internal company data, enabling them to gather, analyze, and act upon information in real-time. This access ensures that every decision and action taken by the agents is data-driven, informed, and aligned with your business strategy."}
                            </p>
                        </details>
                    </li>
                </ol>
                <p class="mt-5">
                    {"With Cowboy AI's custom GPTs and AI Agents, your business is equipped with a suite of intelligent tools that understand your business model, communicate seamlessly, and leverage internal data to drive growth, efficiency, and innovation."}
                </p>
            </div>
        </div>
    }
}
