use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                    {"Business Evolution"}
                </h1>
                <h3 class="text-3xl md:text-4xl mt-12 font-bold">{"Ready to level up?"}</h3>
                <ul class="border-dashed text-2xl md:text-3xl mt-10 py-2 rounded-lg text-white">
                    <li>{"Understand context"}</li>
                    <li>{"Generate content"}</li>
                    <li>{"Be deterministic"}</li>
                    <li class="mt-10">
                    <button type="button" 
                        onclick="window.location.href='/getaccess';"
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
                            {"Access our GPTs"}
                        </button>
                   </li>
                </ul>
            </div>
        </div>
    }
}


