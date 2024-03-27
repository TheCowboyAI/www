use leptos::*;

#[component]
pub fn Context() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
        <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
            <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"Event Modeling for Business"}
                </h1>
                <p>
                    {"Event Modeling is like telling a story of what happens in your business. It's like drawing a picture or making a map of all the steps and actions that happen, so we can understand everything better."}
                </p>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Creating a Story of Your Business"}
                    </summary>
                    <p>
                        {"Event Modeling is like creating a storybook of your business. It shows each step, like the chapters in a story, helping us see what happens first, next, and last."}
                    </p>
                </details>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Understanding the Business Puzzle"}
                    </summary>
                    <p>
                        {"Think of Event Modeling as putting together a big puzzle. Each piece is an action or event in your business, and when we see how all the pieces fit together, the big picture becomes clear!"}
                    </p>
                </details>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Making Better Decisions"}
                    </summary>
                    <p>
                        {"By looking at the story or map we made, we can make better choices. We understand what happens when we make a decision and can guess what might happen next, just like predicting the next part of a story."}
                    </p>
                </details>

                <p>
                    {"Event Modeling helps us understand our business like a story, making it easier to make good decisions and plan for the future. It's like having a guidebook for the adventure of your business!"}
                </p>
            </div>
        </div>
    }
}
