use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                    <a href="/businessevolution">
                    {"Business Evolution"}
                    </a>
                </h1>
                <h3 class="text-3xl md:text-4xl mt-12 font-bold">
                <a href="/gametheory">
                {"Ready to level up?"}
                </a>
                </h3>
                <ul class="border-dashed text-2xl md:text-3xl mt-10 py-2 rounded-lg text-white">
                    <li>
                    <a href="/context">
                    {"Understand context"}
                    </a>
                    </li>
                    <li>
                    <a href="/generatecontent">
                    {"Generate content"}
                    </a>
                    </li>
                    <li>
                    <a href="/deterministic">
                    {"Be deterministic"}
                    </a>
                    </li>
                    <li class="mt-10">
                    <a  
                        href="/gpts"
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
                        </a>
                   </li>
                </ul>
            </div>
        </div>
    }
}


