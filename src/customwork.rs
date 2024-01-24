use leptos::*;

#[component]
pub fn CustomWork() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-row">
                <h2 class="font-bold text-white text-4xl md:text-6xl">
                    {"Custom Work"}
                </h2>
                <h3 class="text-3xl md:text-4xl mt-12 font-bold">
                {"We offer several custom solutions"}
                </h3>
                <ul class="border-dashed text-base mt-10 px-5 py-2 text-white">
                    <li class="text-xl font-bold">{"Custom GPTs"}</li>
                    <li class="ml-2 text-black">{"Let us build a GPT just for you"}</li>
                    
                    <li class="text-xl font-bold">{"Business Model Workshop"}</li>
                    <li class="ml-2 text-black">{"Workshop to help you understand and generate a Business Model Canvas"}</li>
                    
                    <li class="text-xl font-bold">{"Business Knowledge Workshop"}</li>
                    <li class="ml-2 text-black">{"Understand how Taxonomies, Ontologies and Knowledge Graphs help you leverage AI"}</li>
                    
                    <li class="text-xl font-bold">{"Custom Development"}</li>
                    <li class="ml-2 text-black">{"We build systems in Leptos, a cutting-edge Rust framework for the modern web"}</li>
                    
                    <li class="text-xl font-bold">{"NixOS Work"}</li>
                    <li class="ml-2 text-black">{"Build determinate systems and deploy them anywhere"}</li>
                    
                    <li class="text-xl font-bold">{"Event Sourcing"}</li>
                    <li class="ml-2 text-black">{"Learn, Create and Use Event Sourced Systems"}</li>
                </ul>
                <form
                  class="" 
                  action="https://formsubmit.co/support@thecowboy.ai" method="POST">
                  <div class="m-10 p-5 flex flex-col border-black border-2 border-solid rounded-lg">
                    <h2 class="text-lg font-bold">Tell us your need</h2>
                    <label
                      class="font-bold my-2" 
                      for="name">Name:</label>
                    <input
                      class="p-2 rounded-lg" 
                      type="text" name="name" required />
                    <label
                      class="font-bold my-2" 
                      for="email">eMail:</label>
                    <input
                      class="p-2 rounded-lg" 
                      type="email" name="email" required />
                    <label
                      class="font-bold my-2" 
                      rows="4"
                      for="comment">Comments:</label>
                    <textarea
                      class="p-2 rounded-lg" 
                      type="text" name="comment" required />
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
