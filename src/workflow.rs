use leptos::*;

#[component]
pub fn Workflow() -> impl IntoView {
    view! {
        <div class="flex items-center animate-fadeIn">
            <div class="mt-10 flex-row"> 
                <h2 class="font-bold text-white text-4xl md:text-6xl">
                {"Workflows"}
                </h2>
                <blockquote class="mt-10 text-xl italic font-bold">
                {"Your business;"}
                </blockquote>
                <blockquote class="text-xl italic font-bold">
                {"Your language"}
                </blockquote>
                <div class="text-black flex-row mt-10">
                    <ul class="text-2xl">
                    <li>
                    <a href="/naturallanguage">
                    {"Use Natural Language"}
                    </a>
                    </li>
                    <li class="mt-2">
                    <a href="/askquestions">
                    {"Ask it Questions"}
                    </a>
                    </li>
                    <li class="mt-2">
                    <a href="/codegeneration">
                    {"Generate Code"}
                    </a>
                    </li>
                    <li class="mt-2">
                    <a href="/existingapps">
                    {"Use existing Apps"}
                    </a>
                    </li>
                    <li class="mt-2">
                    <a href="/newfunctionality">
                    {"Integrate New Functionality"}
                    </a>
                    </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
