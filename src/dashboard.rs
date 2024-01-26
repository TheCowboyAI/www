use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Dashboards over Reports"}
                </h1>
                <ul 
                  class=" 
                    mt-10 mr-5 py-2
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
