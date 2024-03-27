use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="text-2xl flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"Dashboards over Reports"}
                </h1>
                <a href="/realtimemonitoring">{"Real-Time Monitoring"}</a>
                <a href="/pushtomobile">{"Push to Mobile"}</a>
                <a href="/decisionmaking">{"Improve Decision-Making"}</a>
                <a href="/communications">{"Empowered Communication"}</a>
            </div>
        </div>
    }
}
