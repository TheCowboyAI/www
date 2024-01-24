use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Dashboards over Reports"}
                </h1>
                <div class="text-black flex-row mt-10">
                    <ul class="text-2xl">
                    <li>{"Real-Time Monitoring"}</li>
                    <li>{"Push to Mobile"}</li>
                    <li>{"Improved Decision-Making"}</li>
                    <li>{"Enhanced Communication"}</li>
                    <li>{"Time and Resource Efficiency"}</li>
                    </ul>
                </div>
            </div>
        </div>

    }
}
