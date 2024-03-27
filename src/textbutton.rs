use leptos::*;

#[component]
pub fn TextButton(route: String, content: String) -> impl IntoView {
    view! {
        <div class="flex align-center">
            <a href=route>
                <button
                    class=" 
                    font-bold 
                    text-white 
                    text-2xl 
                    bg-gradient-to-br 
                    from-black 
                    to-black-500
                    hover:bg-blue-600 
                    active:bg-blue-700
                    focus:ring-4 
                    focus:outline-none 
                    focus:ring-blue-300 
                    dark:focus:ring-blue-800 
                    rounded-lg 
                    p-2.5 
                    text-center
                    "
                    type="button"
                >
                    {content}
                </button>
            </a>
        </div>
    }
}


