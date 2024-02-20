use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-col"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                    <a href="/aiintegration">
                        {"IT Starts with IntenT"}
                    </a>
                </h1>

                <ul>
                    <li class="mt-2 font-bold text-black text-xl md:text-2xl">
                        <a href="/businessevolution">
                            {"Identify and Evaluate"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <details class="font-bold text-black text-xl md:text-2xl">
                            <summary>
                                {"Integrate and Automate"}
                            </summary>
                            <p>
                                {"Integrate all the parts of your system. Make them talk seamlessly as if they were written together."}
                            </p>
                        </details>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/workflow">
                            {"Turn Data into Information"}
                        </a>
                    </li>
                </ul>

                <h2 class="mt-10 font-bold text-white text-4xl md:text-6xl">
                    {"Improve Productivity"}
                </h2>
                <ul class="text-black text-2xl md:text-3xl py-2">
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/realtimemonitoring">
                            {"Real-Time Monitoring"}
                        </a>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/decisionmaking">
                            {"Improve Decision-Making"}
                        </a>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/newfunctionality">
                            {"Integrate New Functionality"}
                        </a>
                    </li>
                </ul>
                <h3 class="mt-10 font-bold text-center text-xl rounded-lg bg-white text-blue-800 border-white border-2 border-solid p-2">
                    <a href="/gpts">
                        {"FREE GPTs"}
                    </a>
                </h3>
            </div>
        </div>
    }
}
