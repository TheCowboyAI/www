use leptos::*;

#[component]
pub fn Communications() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
        <div class="flex flex-col gap-4 theme_bg-1-bg-3 rounded-xl p-3 shadow-xl shadow-black">
            <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl" id="architecture">
              {"Empowering Communications"}
          </h1>
          <p class="">
              {"Event streams, combined with projections, AI analysis, and content generation, revolutionize communications within businesses, enabling more effective, timely, and insightful interactions. These technologies ensure that communication is not just faster but also more aligned with business goals and market needs."}
          </p>
        <details class="text-black text-sm">
            <summary class="text-white text-base font-bold">
                {"Real-Time Insights and Automated Content Creation"}
            </summary>
            <p>
                {"Event streams and AI analysis provide real-time insights, while AI-driven content generation automatically creates reports, updates, and notifications. This seamless integration ensures that communication is consistent, timely, and data-driven, enhancing decision-making and responsiveness."}
            </p>
        </details>

        <details class="text-black text-sm">
            <summary class="text-white text-base font-bold">
                {"Conway's Law and Organizational Communication"}
            </summary>
            <p>
                {"Conway's Law suggests that organizations design systems that mirror their own communication structure. This often leads to siloed systems that reflect internal communication barriers. Efficient and integrated communication systems are crucial to overcoming these barriers and ensuring that the technology architecture supports rather than hinders business objectives."}
            </p>
        </details>

        <details class="text-black text-sm">
            <summary class="text-white text-base font-bold">
                {"Business Model Canvas"}
            </summary>
            <p>
                {"A Business Model Canvas helps mitigate Conway's Law by providing a clear, shared understanding of the business's value proposition, customer relationships, channels, and key activities. It aligns the entire organization around common goals, promoting a cohesive and integrated approach to system design and communication strategies."}
            </p>
        </details>

        <p class="">
              {"By harnessing the synergy of event streams, AI, and content generation, businesses can transcend traditional communication barriers, fostering a culture of informed, agile, and cohesive communication that resonates with the strategic vision outlined in their Business Model Canvas."}
          </p>
      </div>
  </div>
  
    }
}
