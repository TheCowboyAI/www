use leptos::*;

use crate::workform::WorkRequestForm;

#[component]
pub fn Solutions() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center animate-fadeIn mt-5">
            <div class="">
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Solutions"}
                </h1>
                <h3 class="text-xl md:text-2xl lg:text-4xl mt-5 font-bold">
                {"Our custom solutions"}
                </h3>
                <ul class="mr-5 
                  border-black border-2 border-solid rounded-lg 
                  text-sm md:text-base mt-5 px-5 py-2 text-white">
                    <li class="text-base md:text-xl font-bold">
                        <a href="/customersupport">
                        {"Customer Support"}
                        </a>
                    </li>
                    <li class="ml-2 text-black">{"We support custom API Integration"}</li>
                    
                    <li class="text-base md:text-xl font-bold">
                        <a href="/customgpts">
                        {"Custom GPTs"}
                        </a>
                    </li>
                    <li class="ml-2 text-black">{"We build a GPT just for you"}</li>
                    
                    <li class="text-base md:text-xl font-bold">
                        <a href="/workshops">
                        {"Knowledge Workshops"}
                        </a>
                    </li>
                    <li class="ml-2 text-black">{"Guided generation of your"}</li>
                    <li class="ml-2 text-black">{"Business Model Canvas"}</li>                    
                    <li class="ml-2 mt-2 text-black">{"Knowledge Graphs for Business"}</li>
                    <li class="ml-2 mt-2 text-black">{"Learn, Create and Use"}</li>
                    <li class="ml-2 text-black">{"Event Sourced systems"}</li>
                    
                    <li class="text-base md:text-xl font-bold">
                        <a href="/nixoswork">
                        {"NixOS Work"}
                        </a>
                    </li>
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
