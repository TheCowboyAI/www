use leptos::*;

use crate::workform::WorkRequestForm;

#[component]
pub fn Solutions() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1
                    class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                    id="architecture"
                >
                    {"Solutions"}
                </h1>
                <h3 class="text-xl md:text-2xl lg:text-4xl mt-5 font-bold">
                    {"Our custom solutions"}
                </h3>
                <ul class="mr-5 
                border-black border-2 border-solid rounded-lg 
                text-sm md:text-base mt-5 px-5 py-2 theme_bg-1-color-5">
                    <li class="text-base md:text-xl font-bold">
                        <a href="/customersupport">{"Customer Support"}</a>
                    </li>
                    <li class="ml-2 text-black">{"We support custom API Integration"}</li>

                    <li class="text-base md:text-xl font-bold">
                        <a href="/customgpts">{"Custom GPTs"}</a>
                    </li>
                    <li class="ml-2 text-black">{"We build a GPT just for you"}</li>

                    <li class="text-base md:text-xl font-bold">
                        <a href="/workshops">{"Knowledge Workshops"}</a>
                    </li>
                    <li class="ml-2 text-black">
                        {"Guided generation of your Business Model Canvas"}
                    </li>
                    <li class="ml-2 mt-2 text-black">{"Knowledge Graphs for Business"}</li>
                    <li class="ml-2 mt-2 text-black">
                        {"Learn, Create and Use Event Sourced systems"}
                    </li>

                    <li class="text-base md:text-xl font-bold">
                        <a href="/nixoswork">{"NixOS Work"}</a>
                    </li>
                    <li class="ml-2 text-black">
                        {"Build determinate systems, Deploy them anywhere"}
                    </li>

                    <li class="text-base md:text-xl font-bold">{"Custom Development"}</li>
                    <li class="ml-2 text-black">{"Cutting-edge APIs and sites"}</li>
                </ul>
                <WorkRequestForm/>
            </div>
        </div>
    }
}
