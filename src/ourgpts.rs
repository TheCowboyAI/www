use leptos::*;

use crate::gptaccessform::GptAccessForm;

#[component]
pub fn OurGPTs() -> impl IntoView {
    view! {
        <div class="mt-5 flex items-center animate-fadeIn">
            <div class="flex-row">
                <h2 class="font-bold text-white text-4xl lg:text-6xl text-center">
                    {"Custom GPTs"}
                </h2>
                <h3 class="mt-5 text-center text-2xl md:text-3xl lg:text-4xl font-bold">
                {"Library of Agents"}
                </h3>
                <img src="assets/gpts.png" class="mt-5 rounded-lg shadow-lg shadow-black" />
                <GptAccessForm />
            </div>
        </div>
    }
}
