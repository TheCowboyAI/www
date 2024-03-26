use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
        <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
            <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl" id="architecture">
                {"Dashboards over Reports"}
                </h1>
                <ul 
                  class=" 
                    py-2
                    text-black text-xl md:text-2xl lg:text-3xl 
                  "
                >                
                    <li>
                        <a href="/realtimemonitoring">
                        {"Real-Time Monitoring"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <a href="/pushtomobile">
                        {"Push to Mobile"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <a href="/decisionmaking">
                        {"Improve Decision-Making"}
                        </a>
                    </li>
                    <li class="mt-5">
                        <a href="/communications">
                        {"Empowered Communication"}
                        </a>
                    </li>
                </ul>
            </div>
        </div>

    }
}
