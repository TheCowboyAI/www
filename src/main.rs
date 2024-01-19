use leptos::*;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app flex flex-wrap">
            <div class="w-1/5">
                <Navigate />
            </div>
            <div class="w-4/5">
                <Content />
            </div>
        </div>
    }
}

#[component]
fn Content() -> impl IntoView {
    view! {
        <div class="ml-5 md:ml-24">
            <h1 class="text-4xl md:text-5xl m-2.5 caption bg-black shadow-lg text-center rounded-lg">
            {"AI-Driven Business Transformation"}
            </h1>
            <div class="rounded-md m-2.5">
                <div class="lg:mr-20 text-white">

                    <h2 class="text-2xl md:text-4xl my-2.5 caption">
                    {"Imagine..."}
                    </h2>

                    <p class="text-lg text-shadow-lg shadow-black">
                    {"a world where every facet of your business is not just enhanced,
                    but transformed with insight through artificial intelligence."}
                    </p>

                    <blockquote class="italic text-lg text-white-300 mt-5 pl-10">
                    {"- That's the world Cowboy AI is creating. -"}
                    </blockquote>

                    <h2 class="text-2xl md:text-4xl my-2.5 caption">
                    {"Your platform as a partnership"}
                    </h2>

                    <p class="lg:mr-20 text-lg text-shadow-lg shadow-black text-white">
                    {"AI solutions are not just applied but intricately tailored to you.
                    Forget the one-size-fits-all approach, you are special."}
                    </p>

                    <h2 class="text-2xl md:text-4xl my-2.5 caption">
                    {"Strategic extensions"}
                    </h2>

                    <ul>
                        <li>{"Automate mundane tasks"}</li>
                        <li>{"Understand workflows"}</li>
                        <li>{"Derive profound insights from your data"}</li>
                        <li>{"Engaging with your customers on a whole new level."}</li>
                    </ul>

                    <h2 class="text-2xl md:text-4xl my-2.5 caption">
                    {"AI tools and APIs"}
                    </h2>

                    <ul>
                        <li>{"Any LLM"}</li>
                        <li>{"Machine Learning"}</li>
                        <li>{"Natural Language Processing"}</li>
                        <li>{"Voice Recognition"}</li>
                        <li>{"Predictive Analytics"}</li>
                    </ul>

                    <h2 class="text-2xl md:text-4xl my-2.5 caption">
                    {"Strategic partnerships"}
                    </h2>
                    <p class="text-shadow-lg shadow-black text-white">
                    {"We work with the tech titans and navigate open-source solutions to fit your model.
                    Cowboy AI is not just your provider but your co-pilot in the 
                    journey toward digital excellence."}
                    </p>

                    <p class="font-bold mt-10 text-xl text-shadow-lg shadow-black text-white">
                    {"With Cowboy AI, it's not just business as usual;"}
                    </p>
                    <p class="font-bold text-xl text-shadow-lg shadow-black text-white">
                    {"it's business ahead of the curve."}
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SvgImage() -> impl IntoView {
    // Include the contents of the SVG file as a &str
    let svg_content = include_str!("../assets/logo.svg");

    view! {
        <div class="icon md:invisible">
            <div class="svg-container" inner_html={svg_content} />
        </div>
    }
}

use leptos::*;

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

#[component]
fn Navigate() -> impl IntoView {
    view! {
        // mini menu
        <div>
    <nav class="flex flex-col w-48 h-screen px-4 border border-black rounded-md">
        <div class="mt-3">
            <div class="icon ml-10 px-4 py-4 flex flex-row svg-container items-center">
                <img src="assets/logo.svg" class="mx-auto" />
                <div class="font-bold caption text-center mb-4">Cowboy AI</div>
            </div>
            <div class="mt-10 mb-4">
            <ul class="ml-4">
            <li class="mb-2 px-4 py-4 flex flex-row">
            <a href="#" class="border-black hover:text-black hover:bg-white hover:font-bold rounded">
              <svg class="fill-current h-5 w-5 " viewBox="0 0 24 24">
                <title>Dashboard</title>
                <path d="M16 20h4v-4h-4m0-2h4v-4h-4m-6-2h4V4h-4m6 4h4V4h-4m-6 10h4v-4h-4m-6 4h4v-4H4m0 10h4v-4H4m6 4h4v-4h-4M4 8h4V4H4v4z"/>
              </svg>
            </a>
            </li>

             <li class="mb-2 px-4 py-4 flex flex-row">
              <a href="#" class="border-black hover:text-black hover:bg-white hover:font-bold rounded">
                <span>
                  <svg class="fill-current h-5 w-5 " viewBox="0 0 24 24">
                  <title>Articles</title>
                    <path
                      d="M12 13H7v5h5v2H5V10h2v1h5v2M8
                          4v2H4V4h4m2-2H2v6h8V2m10 9v2h-4v-2h4m2-2h-8v6h8V9m-2
                          9v2h-4v-2h4m2-2h-8v6h8v-6z"
                    ></path>
                  </svg>
                </span>
                </a>
              </li>

              <li class="mb-2 px-4 py-4 flex flex-row">
              <a href="#" class="border-black hover:text-black hover:bg-white hover:font-bold rounded">
                <span>
                  <svg
                    class="fill-current h-5 w-5 "
                    viewBox="0 0 24 24"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <title>Customers</title>
                    <path
                      fill-rule="evenodd"
                      clip-rule="evenodd"
                      d="M16 7C16 9.20914 14.2091 11 12 11C9.79086 11 8 9.20914 8 7C8 4.79086 9.79086 3 12 3C14.2091 3 16 4.79086 16 7ZM14 7C14 8.10457 13.1046 9 12 9C10.8954 9 10 8.10457 10 7C10 5.89543 10.8954 5 12 5C13.1046 5 14 5.89543 14 7Z"
                      fill="currentColor"
                    />
                    <path
                      d="M16 15C16 14.4477 15.5523 14 15 14H9C8.44772 14 8 14.4477 8 15V21H6V15C6 13.3431 7.34315 12 9 12H15C16.6569 12 18 13.3431 18 15V21H16V15Z"
                      fill="currentColor"
                    />
                  </svg>
                </span>
                </a>
              </li>

              <li class="mb-2 px-4 py-4 flex flex-row">
              <a href="#" class="border-black hover:text-black hover:bg-white hover:font-bold rounded">
                      <span>
                        <svg class="fill-current h-5 w-5 " viewBox="0 0 24 24">
                            <title>Milestones</title>
                            <path
                                d="M19 19H5V8h14m-3-7v2H8V1H6v2H5c-1.11 0-2 .89-2
                                2v14a2 2 0 002 2h14a2 2 0 002-2V5a2 2 0
                                00-2-2h-1V1m-1 11h-5v5h5v-5z" />
                        </svg>
                    </span>
                </a>
              </li>

              <li class="mb-2 px-4 py-4 flex flex-row">
              <a href="#" class="border-black hover:text-black hover:bg-white hover:font-bold rounded">
                  <span>
                    <svg class="fill-current h-5 w-5" viewBox="0 0 24 24">
                    <title>Team</title>
                        <path
                        d="M12 4a4 4 0 014 4 4 4 0 01-4 4 4 4 0 01-4-4 4 4 0
                            014-4m0 10c4.42 0 8 1.79 8 4v2H4v-2c0-2.21 3.58-4
                            8-4z"
                        ></path>
                    </svg>
                    </span>
                </a>
              </li>
            </ul>
          </div>
        </div>
        </nav>
      </div>
      }
}
