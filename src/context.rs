use leptos::*;

#[component]
pub fn Context() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"Event Modeling for Business"}
          </h1>
          <p class="mt-5">
              {"Event Modeling is like telling a story of what happens in your business. It's like drawing a picture or making a map of all the steps and actions that happen, so we can understand everything better."}
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
                          {"Creating a Story of Your Business"}
                      </summary>
                      <p>
                          {"Event Modeling is like creating a storybook of your business. It shows each step, like the chapters in a story, helping us see what happens first, next, and last."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Understanding the Business Puzzle"}
                      </summary>
                      <p>
                          {"Think of Event Modeling as putting together a big puzzle. Each piece is an action or event in your business, and when we see how all the pieces fit together, the big picture becomes clear!"}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Making Better Decisions"}
                      </summary>
                      <p>
                          {"By looking at the story or map we made, we can make better choices. We understand what happens when we make a decision and can guess what might happen next, just like predicting the next part of a story."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"Event Modeling helps us understand our business like a story, making it easier to make good decisions and plan for the future. It's like having a guidebook for the adventure of your business!"}
          </p>
      </div>
  </div>
  
    }
}
