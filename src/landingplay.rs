use leptos::*;

#[component]
pub fn LandingPlay() -> impl IntoView {
    // Include CSS file contents as a string literal
    let css_content = include_str!("style2.css");

    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-col">
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                    <a href="/aiintegration" class="button">
                        {"Integrate and Automate AI"}
                    </a>
                </h1>

                <ul>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/businessevolution" class="button">
                            {"Business Process Evaluation"}
                        </a>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/cim" class="button">
                            {"Connect Microservices Patterns"}
                        </a>
                    </li>
                    <li class="mt-2 font-bold text-black text-xl md:text-2xl">
                        <a href="/workflow" class="button">
                            {"Task Automation"}
                        </a>
                    </li>
                </ul>

                <h2 class="mt-10 font-bold text-white text-4xl md:text-6xl">
                    {"Improve Productivity"}
                </h2>
                <ul class="text-black text-2xl md:text-3xl py-2">
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/realtimemonitoring" class="button">
                            {"Real-Time Monitoring"}
                        </a>
                    </li>

                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/decisionmaking" class="button">
                            {"Improve Decision-Making"}
                        </a>
                    </li>

                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/newfunctionality" class="button">
                            {"Integrate New Functionality"}
                        </a>
                    </li>
                </ul>
                <h3 class="mt-10 font-bold text-center text-xl rounded-lg bg-white text-blue-800 border-white border-2 border-solid p-2">
                    <a href="/gpts" class="button">
                        {"FREE GPTs"}
                    </a>
                </h3>
            </div>
        </div>
    }
}
