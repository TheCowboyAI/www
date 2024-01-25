use leptos::*;

#[component]
pub fn Tooling() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Modeled Intelligence"}
                </h1>
                <div class="text-black flex-row mt-10">
                    <ul class="text-2xl">
                    <li>{"Business model"}</li>
                    <li>{"+"}</li>
                    <li class="pb-2">{"simple words"}</li>
                    </ul>
                    <hr class="border border-t-2 border-black" />
                    <h2 class="text-2xl mt-2 font-bold italic">
                    {"AI Composable Insights"}
                    </h2>
                </div>

                <h2 class="mt-10 font-bold text-white text-3xl md:text-4xl">
                {"Custom GPTs"}
                <br />
                {"Fit Your Model"}
                </h2>
                <div class="mt-10">
                    <a  
                    href="/gpts"
                    class="
                        text-white 
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
                    ">
                    {"Custom GPTs"}
                    </a>
                </div>
            </div>
        </div>
    }
}
