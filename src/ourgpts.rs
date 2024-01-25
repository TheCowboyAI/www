use leptos::*;

use crate::gptaccessform::GptAccessForm;

#[component]
pub fn OurGPTs() -> impl IntoView {
    view! {
        <div class="mt-5 flex items-center animate-fadeIn">
            <div class="flex-row">
                <h2 class="font-bold text-white text-4xl lg:text-6xl text-center">
                    {"Custom GPTs"}
                </h2>
                <h3 class="mt-5 text-center text-2xl md:text-3xl lg:text-4xl font-bold">
                {"Library of Agents"}
                </h3>
                <ul class="
                    mt-10 px-5 py-2
                    text-black text-base md:text-xl lg:text-2xl 
                    "
                >
                    <li class="text-white text-xl md:text-2xl lg:text-4xl font-bold">
                    {"Summarize2Md"}
                    </li>
                    <li class="ml-2">{"Upload a document, get a summary back in markdown"}</li>
                    
                    <li class="text-white text-xl md:text-2xl lg:text-4xl font-bold">
                    {"Business Model Canvas Assistant"}
                    </li>
                    <li class="ml-2">{"Understand your model"}</li>
                    <li class="ml-2">{"Generate a BMC"}</li>
                    <li class="ml-2">{"Step-by-step Instruction"}</li>
                    <li class="ml-2">{"Produce a persistant model"}</li>
                    
                    <li class="text-white text-xl md:text-2xl lg:text-4xl font-bold">
                    {"Ontology Architect"}
                    </li>
                    <li class="ml-2">{"Create Taxonomies, Ontologies and Knowledge Graphs"}</li>
                    
                    <li class="text-white text-xl md:text-2xl lg:text-4xl font-bold">{"Event Sourcing Assistant"}</li>
                    <li class="ml-2">{"Learn, Create and Use Event Sourced Systems"}</li>

                    <li class="text-white text-xl md:text-2xl lg:text-4xl font-bold">{"Leptos Assistant"}</li>
                    <li class="ml-2">{"Build systems in Leptos"}</li>
                    <li class="ml-2">{"A cutting-edge Rust web framework"}</li>
                    
                    <li class="text-white text-xl md:text-2xl lg:text-4xl font-bold">{"NixOS Navigator"}</li>
                    <li class="ml-2">{"Build determinate systems"}</li>
                    <li class="ml-2">{"Deploy them anywhere"}</li>
                    
                    <li class="mt-10">
                        <GptAccessForm />
                    </li>
                </ul>                
            </div>
        </div>
    }
}
