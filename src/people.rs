use leptos::*;

#[component]
pub fn People() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"People-centric"}
                </h1>
                <div class="text-black flex-row mt-10">
                    <ul class="text-2xl">
                    <li>{"Secure Everything"}</li>
                    <li class="mt-5">{"Communicate Efficiently"}</li>
                    <li class="mt-5">{"Visualize Relationships"}</li>
                    <li class="mt-5">{"Collaborate with Ease"}</li>
                    <li class="mt-5">{"Use a Common Language"}</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
