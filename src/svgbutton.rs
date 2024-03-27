use leptos::*;

#[component]
pub fn Next() -> impl IntoView {
    view! {
        <div class="mt-5 items-center justify-center">
            <a
                href="#"
                class="border-black hover:text-black hover:bg-white hover:font-bold rounded"
            >
                <svg class="fill-white h-10 w-10" viewBox="0 0 24 24">
                    <title>Next Page</title>
                    <path
                        opacity="0.1"
                        d="M3 12C3 4.5885 4.5885 3 12 3C19.4115 3 21 4.5885 21 12C21 19.4115 19.4115 21 12 21C4.5885 21 3 19.4115 3 12Z"
                        fill="#323232"
                    ></path>
                    <path
                        d="M3 12C3 4.5885 4.5885 3 12 3C19.4115 3 21 4.5885 21 12C21 19.4115 19.4115 21 12 21C4.5885 21 3 19.4115 3 12Z"
                        stroke="#323232"
                        stroke-width="2"
                    ></path>
                    <path
                        d="M8 12H8.01"
                        stroke="#FFF"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                    <path
                        d="M12 12H12.01"
                        stroke="#FFF"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                    <path
                        d="M16 12H16.01"
                        stroke="#FFF"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                </svg>
            </a>
        </div>
    }
}