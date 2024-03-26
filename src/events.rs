use leptos::*;

#[component]
pub fn Events() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
        <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
            <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl" id="architecture">
                <a href="/eventsourcing">
                    {"It's all Events..."}
                </a>
                </h1>
                <a href="/audittrail">
                {"Audit Trail"}
                </a>
                <a href="/historicalinsight">
                {"Historical Insight"}
                </a>
                <a href="/projections">
                {"Projections"}
                </a>
                <a href="/eventstreams">
                {"Undo and Replay"}
                </a>
                <a href="predictions">
                {"Predictions"}
                </a>
            </div>
        </div>
    }
}
