use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                    <a href="/businessevolution">
                    {"Revolutionize Your Business Model"}
                    </a>
                </h1>
                <h3 class="text-3xl md:text-4xl mt-12 font-bold">
                <a href="/cim">
                {"AI Strategy for the Next Stage"}
                </a>
                </h3>
                <ul class="border-dashed text-2xl md:text-3xl 
                    mt-8 py-2 rounded-lg text-white">
                    
                    <li>
                    <a href="/generatecontent">
                    {"Generate Content"}
                    </a>
                    </li>
                    
                    <li class="mt-5">
                    <a href="/context">
                    {"Understand Context"}
                    </a>
                    </li>
                    
                    <li class="mt-5">
                    <a href="/deterministic">
                    {"Be Deterministic"}
                    </a>
                    </li>
                    
                    <li class="mt-10">
                    <a  
                        href="/gpts"
                        class="font-bold text-blue-800
                            border-black border-2 border-dashed
                            p-2
                        ">
                            {"Access our GPTs"}
                        </a>
                   </li>
                </ul>
            </div>
        </div>
    }
}


