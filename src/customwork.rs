use leptos::*;

#[component]
pub fn CustomWork() -> impl IntoView {
    view! {
        <div class="flex items-center animate-fadeIn mt-5">
            <div class="flex-row">
                <h2 class="font-bold text-white text-3xl md:text-4xl lg:text-6xl">
                    {"Solutions"}
                </h2>
                <h3 class="text-xl md:text-2xl lg:text-4xl mt-5 font-bold">
                {"Our custom solutions"}
                </h3>
                <ul class="mr-5 
                  border-black border-2 border-solid rounded-lg 
                  text-sm md:text-base mt-5 px-5 py-2 text-white">
                    <li class="text-base md:text-xl font-bold">{"Customer Support"}</li>
                    <li class="ml-2 text-black">{"We support custom API Integration"}</li>
                    
                    <li class="text-base md:text-xl font-bold">{"Custom GPTs"}</li>
                    <li class="ml-2 text-black">{"We build a GPT just for you"}</li>
                    
                    <li class="text-base md:text-xl font-bold">{"Business Model Workshop"}</li>
                    <li class="ml-2 text-black">{"Guided generation of your"}</li>
                    <li class="ml-2 text-black">{"Business Model Canvas"}</li>
                    
                    <li class="text-base md:text-xl font-bold">{"Knowledge Workshop"}</li>
                    <li class="ml-2 text-black">{"Knowledge Graphs for Business"}</li>
                    
                    <li class="text-base md:text-xl font-bold">{"Event Sourcing"}</li>
                    <li class="ml-2 text-black">{"Learn, Create and Use"}</li>
                    <li class="ml-2 text-black">{"Event Sourced systems"}</li>
                    
                    <li class="text-base md:text-xl font-bold">{"NixOS Work"}</li>
                    <li class="ml-2 text-black">{"Build determinate systems"}</li>
                    <li class="ml-2 text-black">{"Deploy them anywhere"}</li>

                    <li class="text-base md:text-xl font-bold">{"Custom Development"}</li>
                    <li class="ml-2 text-black">{"Cutting-edge APIs and sites"}</li>
                </ul>
                <form
                  class="mr-5" 
                  action="https://formsubmit.co/support@thecowboy.ai" method="POST">
                  <div class="my-5 p-5 flex flex-col border-black border-2 border-solid rounded-lg">
                    <h2 class="text-lg font-bold">Tell us your need</h2>
                    <label
                      class="font-bold my-2" 
                      for="name">Name:</label>
                    <input
                      class="p-2 rounded-lg" 
                      type="text" name="name" id="name" autocomplete="on" required />
                    <label
                      class="font-bold my-2" 
                      for="email">eMail:</label>
                    <input
                      class="p-2 rounded-lg" 
                      type="email" name="email" id="email" autocomplete="on" required />
                    <label
                      class="font-bold my-2" 
                      rows="4"
                      for="comment">Comments:</label>
                    <textarea
                      class="p-2 rounded-lg" 
                      type="text" name="comment" id="comment" required />
                    <button
                      class="
                        text-white 
                        bg-gradient-to-br 
                        from-purple-600 
                        to-blue-500 
                        hover:bg-gradient-to-bl 
                        focus:ring-4 
                        focus:outline-none 
                        focus:ring-blue-300 
                        dark:focus:ring-blue-800 
                        font-bold 
                        rounded-lg 
                        text-base 
                        px-5 
                        py-2.5 
                        text-center 
                        mt-5
                        mx-20" 
                        type="submit"
                    >
                      Send
                    </button>
                  </div>
                </form>                
            </div>
        </div>
    }
}
