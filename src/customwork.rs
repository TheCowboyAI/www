use leptos::*;

use crate::workform::WorkRequestForm;

#[component]
pub fn CustomWork() -> impl IntoView {
    view! {
        <div class="flex items-center animate-fadeIn mt-5">
            <div class="flex-row">
                <h2 class="mt-10 font-bold text-white text-3xl md:text-4xl lg:text-6xl">
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
                    
                    <li class="text-base md:text-xl font-bold">{"Knowledge Workshops"}</li>
                    <li class="ml-2 text-black">{"Guided generation of your"}</li>
                    <li class="ml-2 text-black">{"Business Model Canvas"}</li>                    
                    <li class="ml-2 mt-2 text-black">{"Knowledge Graphs for Business"}</li>
                    <li class="ml-2 mt-2 text-black">{"Learn, Create and Use"}</li>
                    <li class="ml-2 text-black">{"Event Sourced systems"}</li>
                    
                    <li class="text-base md:text-xl font-bold">{"NixOS Work"}</li>
                    <li class="ml-2 text-black">{"Build determinate systems"}</li>
                    <li class="ml-2 text-black">{"Deploy them anywhere"}</li>

                    <li class="text-base md:text-xl font-bold">{"Custom Development"}</li>
                    <li class="ml-2 text-black">{"Cutting-edge APIs and sites"}</li>
                </ul>
                <WorkRequestForm/>
            </div>
        </div>
    }
}
