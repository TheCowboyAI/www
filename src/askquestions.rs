use leptos::*;

#[component]
pub fn AskQuestions() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"AI-Powered Queries on Modeled Business Data"}
                </h1>

                <p>
                    {"AI revolutionizes the way we interact with modeled business data by enabling us to ask complex questions and receive insightful answers, making data more accessible and actionable for strategic decision-making."}
                </p>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Natural Language Data Interaction"}
                    </summary>
                    <p>
                        {"AI allows users to query business data using natural language, making it easier for non-technical team members to extract valuable insights without needing to understand complex database queries or data science concepts."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Intelligent Data Analysis"}
                    </summary>
                    <p>
                        {"With AI, businesses can analyze and interpret their data more deeply. AI algorithms can uncover patterns, predict trends, and offer recommendations, turning raw data into actionable intelligence."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Enhanced Decision-Making"}
                    </summary>
                    <p>
                        {"By enabling more intuitive and comprehensive data querying and analysis, AI empowers business leaders and decision-makers to make more informed, data-driven decisions quickly and efficiently."}
                    </p>
                </details>

                <p>
                    {"The ability to ask questions about any modeled data in a business through AI not only democratizes data access but also enriches the decision-making process, ensuring that every business move is backed by solid, data-driven insights."}
                </p>
            </div>
        </div>
    }
}
