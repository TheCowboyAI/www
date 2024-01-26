use leptos::*;

#[component]
pub fn DescTemplate() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                {""}
                </h1>
                <p class="mt-5">
                {""}
                </p>
                <ol
                    class=" 
                        mr-5 py-2
                        text-black text-xl md:text-2xl lg:text-3xl 
                    "
                >
                    <li class="mt-5">                    
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                            {""}
                            </summary>
                            <p>
                            {""}
                            </p>
                        </details>
                    </li>
                </ol>
                <p class="mt-5">
                {""}
                </p>
            </div>
        </div>
    }
}
