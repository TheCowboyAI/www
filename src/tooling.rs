use leptos::*;

#[component]
pub fn Tooling() -> impl IntoView {
    view! {
        <div class="mt-10 flex flex-col items-center animate-fadeIn">
            <div class=""> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                <a href="/modeledintelligence">
                {"Modeled Intelligence"}
                </a>
                </h1>
                <div class="text-black flex-row mt-10">
                    <ul class="text-2xl">
                    <li><a href="/businessmodel">{"Business model"}</a></li>
                    <li>{"+"}</li>
                    <li class="pb-2"><a href="/simplewords">{"simple words"}</a></li>
                    </ul>
                    <hr class="border border-t-2 border-black" />
                    <h2 class="text-2xl mt-2 font-bold italic">
                    <a href="/aicomposable">{"AI Composable Insights"}</a>
                    </h2>
                </div>
                <a
                  href="/gpts"
                >
                    <h2 class="mt-10 font-bold text-white text-3xl md:text-4xl">
                    {"Custom GPTs"}
                    <br />
                    {"Fit Your Model"}
                    </h2>
                </a>
            </div>
        </div>
    }
}
