use leptos::*;

#[component]
pub fn Workflow() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1
                    class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl"
                    id="architecture"
                >
                    {"Workflows"}
                </h1>

                <div class="flex flex-row">
                    <span class="text-2xl italic font-bold">{"Your business;"}</span>
                    <span class="pl-3 text-2xl italic font-bold">{"Your language"}</span>
                </div>
                <div class="text-black text-2xl flex flex-col gap-4">
                    <a href="/naturallanguage">{"Use Natural Language"}</a>
                    <a href="/askquestions">{"Ask it Questions"}</a>
                    <a href="/codegeneration">{"Generate Code"}</a>
                    <a href="/existingapps">{"Use existing Apps"}</a>
                    <a href="/newfunctionality">{"Integrate New Functionality"}</a>
                </div>
            </div>
        </div>
    }
}
