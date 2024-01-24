use leptos::*;

#[component]
pub fn GetAccess() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-row">
                <h2 class="font-bold text-white text-4xl md:text-6xl">
                    {"Custom GPTs"}
                </h2>
                <h3 class="text-3xl md:text-4xl mt-12 font-bold">
                {"Access these and more as we grow"}
                </h3>
                <ul class="border-dashed text-base mt-10 px-5 py-2 text-white">
                    <li class="text-xl font-bold">{"Summarize to Markdown"}</li>
                    <li class="ml-2 text-black">{"Upload a document, get a summary back in markdown"}</li>
                    
                    <li class="text-xl font-bold">{"Business Model Canvas Assistant"}</li>
                    <li class="ml-2 text-black">{"Understand your business model by generating a Business Model Canvas. Step-by-step, produce a persistant model"}</li>
                    
                    <li class="text-xl font-bold">{"Ontology Architect"}</li>
                    <li class="ml-2 text-black">{"Friendly tool for Taxonomies, Ontologies and Knowledge Graphs"}</li>
                    
                    <li class="text-xl font-bold">{"Leptos Assistant"}</li>
                    <li class="ml-2 text-black">{"Help with building systems in Leptos, a cutting-edge Rust framework for the modern web"}</li>
                    
                    <li class="text-xl font-bold">{"NixOS Navigator"}</li>
                    <li class="ml-2 text-black">{"Build determinate systems and deploy them anywhere"}</li>
                    
                    <li class="text-xl font-bold">{"Event Sourcing Assistant"}</li>
                    <li class="ml-2 text-black">{"Learn, Create and Use Event Sourced Systems"}</li>

                    <li class="mt-10">
                        <a 
                        href="/customwork"
                        class="text-white 
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
                          me-2 
                          mb-2"
                      >
                      Custom Work
                      </a>
                    </li>
                </ul>                
            </div>
        </div>
    }
}