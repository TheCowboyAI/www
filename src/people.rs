use leptos::*;

#[component]
pub fn People() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"People-centric"}
                </h1>
                <ul 
                  class=" 
                    mt-10 mr-5 py-2
                    text-black text-xl md:text-2xl lg:text-3xl 
                  "
                >                
                    <li>{"Secure Everything"}</li>
                    <li class="mt-5">{"Communicate Efficiently"}</li>
                    <li class="mt-5">{"Visualize Relationships"}</li>
                    <li class="mt-5">{"Collaborate with Ease"}</li>
                    <li class="mt-5">{"Use a Common Language"}</li>
                </ul>
            </div>
        </div>
    }
}
