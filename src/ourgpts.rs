use leptos::*;

use crate::gptaccessform::GptAccessForm;

#[component]
pub fn OurGPTs() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col gap-4 rounded-xl p-3">
                <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl" id="architecture">
                    {"Custom GPTs - Library of Agents"}
                </h1>
                <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl" id="architecture">
                {"Available on OpenAI and with API Access"}
                </h1>
                <img src="assets/gpts.png" class="mt-5 rounded-lg shadow-lg shadow-black" />
                <GptAccessForm />
            </div>
        </div>
    }
}
