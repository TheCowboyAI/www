use leptos::*;

#[component]
pub fn People() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="text-2xl flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"Peoplecentric"}
                </h1>
                <a href="/security">{"Secure Everything"}</a>
                <a href="/communications">{"Communicate Efficiently"}</a>
                <a href="/visualize">{"Visualize Relationships"}</a>
                <a href="/collaborate">{"Collaborate with Ease"}</a>
                <a href="/ubiquitouslanguage">{"Use a Common Language"}</a>
            </div>
        </div>
    }
}
