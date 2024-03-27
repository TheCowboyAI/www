use leptos::*;

#[component]
pub fn Template() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row">
                <h1 class="font-bold text-white text-4xl md:text-6xl">{"Title"}</h1>
                <ul class=" 
                mt-10 mr-5 py-2
                text-black text-xl md:text-2xl lg:text-3xl 
                ">
                    <li>
                        <a href="/location">{"Location Tag"}</a>
                    </li>
                </ul>
            </div>
        </div>
    }
}
