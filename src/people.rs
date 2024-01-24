use leptos::*;

#[component]
pub fn People() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"People are the Cornerstone"}
                </h1>
                <div class="text-black flex-row mt-10">
                    <ul class="text-2xl">
                    <li>{"Secure Everything"}</li>
                    <li>{"Communicate Efficiently"}</li>
                    <li>{"Visualize Relationships"}</li>
                    <li>{"Collaborate with Ease"}</li>
                    <li>{"Use a Common Language"}</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
