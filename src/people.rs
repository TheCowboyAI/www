use leptos::*;

#[component]
pub fn People() -> impl IntoView {
    view! {
        <div class="mt-10 flex flex-col items-center animate-fadeIn">
            <div class=""> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Peoplecentric"}
                </h1>
                <ul 
                  class=" 
                    mt-10 mr-5 py-2
                    text-black text-xl md:text-2xl lg:text-3xl 
                  "
                >                
                    <li>
                        <a href="/security">
                        {"Secure Everything"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <a href="/communications">
                        {"Communicate Efficiently"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <a href="/visualize">
                        {"Visualize Relationships"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <a href="/collaborate">
                        {"Collaborate with Ease"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <a href="/ubiquitouslanguage">
                        {"Use a Common Language"}
                        </a>
                    </li>
                </ul>
            </div>
        </div>
    }
}
