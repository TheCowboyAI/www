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
                        <a href="/getaccess" class="relative rounded px-5 py-2.5 overflow-hidden group bg-red-500 relative hover:bg-gradient-to-r hover:from-red-500 hover:to-red-400 text-white hover:ring-2 hover:ring-offset-2 hover:ring-red-400 transition-all ease-out duration-300">
                            <span class="absolute right-0 w-8 h-32 -mt-12 transition-all duration-1000 transform translate-x-12 bg-white opacity-10 rotate-12 group-hover:-translate-x-40 ease"></span>
                            <span class="relative">Access Our GPTs</span>
                        </a>
                    </li>
                </ul>
            </div>
        </div>
    }
}


