use leptos::*;

#[component]
pub fn Tooling() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1
                    class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                    id="architecture"
                >
                    <a href="/modeledintelligence">{"Modeled Intelligence"}</a>
                </h1>
                <div class="text-2xl text-black flex flex-col mt-10">
                    <a href="/businessmodel">{"Business model"}</a>
                    <span>{"+"}</span>
                    <a href="/simplewords">{"simple words"}</a>
                    <hr class="mt-2 border border-t-2 border-black"/>
                    <h2 class="text-2xl mt-2 font-bold italic">
                        <a href="/aicomposable">{"AI Composable Insights"}</a>
                    </h2>
                </div>
                <a href="/gpts">
                    <h2 class="text-center p-2 rounded-lg theme_bg-1-bg-2 mt-10 font-bold theme_bg-1-color-5 text-xl">
                        {"Custom GPTs to fit your Model"}
                    </h2>
                </a>
            </div>
        </div>
    }
}
