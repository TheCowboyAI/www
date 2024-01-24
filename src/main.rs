pub mod navigate;
use crate::navigate::Navigate;

use leptos::*;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="bg-gradient-to-tl from-black via-blue-500 to-purple-900 app flex">
            <aside class="h-screen sticky top-0 left-0 bottom-0">
                <Navigate />
            </aside>
            <main class="pl-5">
                <Content />
            </main>
        </div>
        <div class="absolute bottom-0 w-full">
            <Footer />
        </div>
    }
}

#[component]
fn Content() -> impl IntoView {
    view! {
        <Page1 />
    }
}

#[component]
fn Next() -> impl IntoView {
    view! {
        <div class="mt-5 items-center justify-center">
            <a href="#" class="border-black hover:text-black hover:bg-white hover:font-bold rounded">
                <svg class="fill-white h-10 w-10" viewBox="0 0 24 24">
                    <title>Next Page</title>
                    <path opacity="0.1" d="M3 12C3 4.5885 4.5885 3 12 3C19.4115 3 21 4.5885 21 12C21 19.4115 19.4115 21 12 21C4.5885 21 3 19.4115 3 12Z" fill="#323232"/>
                    <path d="M3 12C3 4.5885 4.5885 3 12 3C19.4115 3 21 4.5885 21 12C21 19.4115 19.4115 21 12 21C4.5885 21 3 19.4115 3 12Z" stroke="#323232" stroke-width="2"/>
                    <path d="M8 12H8.01" stroke="#FFF" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    <path d="M12 12H12.01" stroke="#FFF" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    <path d="M16 12H16.01" stroke="#FFF" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </a>
        </div>
    }
}

#[component]
fn Page1() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center">
            <div class="flex-row"> 
                <h2 class="font-bold text-white text-4xl md:text-6xl">
                    {"Evolve your Business"}
                </h2>
                <h3 class="text-3xl md:text-4xl mt-12 font-bold">{"Ready to level up?"}</h3>
                <ul class="text-2xl md:text-3xl mt-10">
                    <li>{"Understand existing context"}</li>
                    <li>{"Generate original content"}</li>
                    <li>{"Be deterministic"}</li>
                </ul>
                // <Next />
            </div>
        </div>
    }
}

#[component]
fn Page2() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center">
            <div class="flex-row"> 
                <h2 class="text-white text-4xl md:text-6xl">
                {"Modeled Intelligence"}
                </h2>
                <div class="text-black flex-row mt-10">
                    <ul class="text-2xl">
                    <li>{"Business model"}</li>
                    <li>{"+"}</li>
                    <li class="pb-2">{"simple words"}</li>
                    </ul>
                    <hr class="border border-t-2 border-black" />
                    <h2 class="text-2xl mt-2 font-bold italic">
                    {"AI Composable Insights"}
                    </h2>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Page3() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center">
            <div class="flex-row"> 
                <h2 class="font-bold text-white text-4xl md:text-6xl">
                {"Talk Shop"}
                </h2>
                <blockquote class="mt-10 text-xl italic font-bold">{"Your business has a language"}</blockquote>
                <div class="text-black flex-row mt-10">
                    <ul class="text-2xl">
                    <li>{"Ask it Questions"}</li>
                    <li>{"Generate Code"}</li>
                    <li>{"Use existing Apps"}</li>
                    <li>{"Integrate New Functionality"}</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <div class="rounded-t p-2 text-white bg-black">
        {"Copyright 2024 - Cowboy AI"}
        </div>
    }
}

#[component]
fn SvgImage() -> impl IntoView {
    // Include the contents of the SVG file as a &str
    let svg_content = include_str!("../assets/logo.svg");

    view! {
        <svg class="svg-container" inner_html={svg_content} />
    }
}

// #[component]
// fn DashboardIcon() -> impl IntoView {
//     let (content, set_content) = create_signal(String::from("Initial content"));

//     let handle_click = move |_| {
//         set_content(String::from("New content after click"));
//     };

//     view! {
//         div {
//             "Content: "
//             {content()}
//             button {
//                 on:click=handle_click,
//                 "Change Content"
//             }
//         }
//     }
// }

#[component]
fn Dashboard() -> impl IntoView {
    view! {
        <div class="icon">
            <img
                src="assets/logo.svg"
                class="mx-auto w-20 h-20"
            />
        </div>

    }
}

