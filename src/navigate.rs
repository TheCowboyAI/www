use leptos::*;
use crate::footer::Footer;

#[component]
pub fn Navigate() -> impl IntoView {
    view! {
        <nav class="
        flex fixed w-full h-16 rounded-lg rounded-b items-center justify-between 
        px-6 theme_bg-1-bg-3 text-black z-10
        ">
            <div id="logo" class="p-2">
                <a href="/">
                    <div class="py-4 flex items-center w-14">
                        <img src="assets/logo.svg" alt="Cowboy AI Logo" title="Cowboy AI"/>
                    </div>
                </a>
            </div>

            <div id="title">
                <span class="font-bold text-4xl theme_bg-1-color-2">{"Cowboy AI"}</span>
            </div>

            <div>
                <label class="menu-button-wrapper" for="menubutton">
                    <input 
                        id="menubutton"
                        type="checkbox" 
                        class="menu-button"                        
                    />

                    <div class="icon-wrapper">
                        <label class="hamburger">
                            <input class="hamburger-input" type="checkbox" />
                            <span class="hamburger-line first"></span>
                            <span class="hamburger-line second"></span>
                            <span class="hamburger-line third"></span>
                        </label>
                    </div>
                    <div class="item-list">
                        <ul class="theme_bg-1-color-5 text-base">
                            <li>
                                <a href="/tooling" 
                                    class="flex flex-row px-2 py-1 border border-transparent border-solid border-1 hover:border-black">
                                    <img src="assets/tools.svg" class="icon-stroke w-8" />
                                    <span class="ml-3">
                                        {"Tooling"}
                                    </span>
                                </a>
                            </li>
                            <li>
                                <a href="/dashboard"
                                class="flex flex-row px-2 py-1 border border-transparent border-solid border-1 hover:border-black">
                                <img src="assets/dashboard.svg" class="icon-stroke w-8" />
                                <span class="ml-3">
                                    {"Dashboard"}
                                </span>
                                </a>
                            </li>
                            <li>
                                <a href="/workflow"
                                class="flex flex-row px-2 py-1 border border-transparent border-solid border-1 hover:border-black">
                                <img src="assets/workflow.svg" class="icon-stroke w-8" />
                                <span class="ml-3">{"Workflow"}</span>
                                
                                </a>
                            </li>
                            <li>
                                <a href="/events"
                                class="flex flex-row px-2 py-1 border border-transparent border-solid border-1 hover:border-black">
                                <img src="assets/events.svg" class="icon-stroke w-8" />
                                <span class="ml-3">{"Events"}</span>
                                </a>
                            </li>
                            <li>
                                <a href="/people"
                                class="flex flex-row px-2 py-1 border border-transparent border-solid border-1 hover:border-black">
                                <img src="assets/people.svg" class="icon-stroke w-8" />
                                <span class="ml-3">{"People"}</span>
                                </a>
                            </li>
                            <li>
                                <a href="/gpts"
                                class="flex flex-row px-2 py-1 border border-transparent border-solid border-1 hover:border-black">
                                <img src="assets/science.svg" class="icon-stroke w-8" />
                                <span class="ml-3">{"Personal Agents"}</span>
                                </a>
                            </li>
                            <li>
                                <a href="/customwork"
                                class="flex flex-row px-2 py-1 border border-transparent border-solid border-1 hover:border-black">
                                <img src="assets/settings.svg" class="icon-stroke w-8" />
                                <span class="ml-3">{"Solutions"}</span>
                                </a>
                            </li>
                        </ul>
                    </div>
                </label>
            </div>
        </nav>
    }
}
