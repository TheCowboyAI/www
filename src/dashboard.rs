use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Dashboards over Reports"}
                </h1>
                <div class="text-black flex-row mt-10">
                    <ul class="text-xl md:text-2xl">
                    <li>{"Real-Time Monitoring"}</li>
                    <li class="mt=5">{"Push to Mobile"}</li>
                    <li class="mt=5">{"Improve Decision-Making"}</li>
                    <li class="mt=5">{"Enhance Communication"}</li>
                    </ul>
                </div>
            </div>
        </div>

    }
}
