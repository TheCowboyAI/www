use leptos::*;

#[component]
pub fn Projections() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
        <div class="flex-row"> 
            <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Projections: Transforming Data into Insight"}
            </h1>
            <p class="mt-5">
                {"Event Sourcing Projections are a fundamental part of Event Sourcing, acting as the bridge between raw event data and meaningful insights. They interpret, filter, and transform event streams into understandable, actionable formats, serving as the foundation for decision-making and business intelligence."}
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
                            {"Interpreting Event Streams"}
                        </summary>
                        <p>
                            {"Projections process raw event streams, interpreting each event's significance within the context of the business domain. This allows for the extraction of meaningful patterns and trends from the data."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Creating Custom Views"}
                        </summary>
                        <p>
                            {"Event Sourcing Projections enable the creation of tailored views that reflect the specific informational needs of different business functions. This customization ensures that each team receives the most relevant, focused data for their operations."}
                        </p>
                    </details>
                </li>
                <li class="mt-5">                    
                    <details class="text-black text-sm">
                        <summary class="text-white text-base font-bold">
                            {"Driving Business Decisions"}
                        </summary>
                        <p>
                            {"By transforming complex event streams into actionable insights, projections empower businesses to make data-driven decisions. They provide a solid foundation for strategy development, operational improvements, and innovation."}
                        </p>
                    </details>
                </li>
            </ol>
            <p class="mt-5">
                {"Event Sourcing Projections are the key to unlocking the full potential of event-driven data, providing clarity, focus, and direction that empower businesses to navigate the complexities of their domain with confidence and precision."}
            </p>
        </div>
    </div>
  
    }
}
