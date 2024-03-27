use leptos::*;

#[component]
pub fn Projections() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1
                    class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                    id="architecture"
                >
                    {"Projections: Transforming Data into Insight"}
                </h1>
                <p>
                    {"Event Sourcing Projections are a fundamental part of Event Sourcing, acting as the bridge between raw event data and meaningful insights. They interpret, filter, and transform event streams into understandable, actionable formats, serving as the foundation for decision-making and business intelligence."}
                </p>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Interpreting Event Streams"}
                    </summary>
                    <p>
                        {"Projections process raw event streams, interpreting each event's significance within the context of the business domain. This allows for the extraction of meaningful patterns and trends from the data."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Creating Custom Views"}
                    </summary>
                    <p>
                        {"Event Sourcing Projections enable the creation of tailored views that reflect the specific informational needs of different business functions. This customization ensures that each team receives the most relevant, focused data for their operations."}
                    </p>
                </details>

                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Driving Business Decisions"}
                    </summary>
                    <p>
                        {"By transforming complex event streams into actionable insights, projections empower businesses to make data-driven decisions. They provide a solid foundation for strategy development, operational improvements, and innovation."}
                    </p>
                </details>

                <p>
                    {"Event Sourcing Projections are the key to unlocking the full potential of event-driven data, providing clarity, focus, and direction that empower businesses to navigate the complexities of their domain with confidence and precision."}
                </p>
            </div>
        </div>
    }
}
