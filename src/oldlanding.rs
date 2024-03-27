use leptos::*;

use crate::textbutton::TextButton;

#[component]
fn OldLanding() -> impl IntoView {
  view! {
      <div class="h-screen flex animate-fadeIn">

          <div class="flex flex-col gap-5">

              <div>
                  <a href="/aiintegration">
                      <h1 class="font-bold text-white text-4xl">{"Model your Success"}</h1>
                  </a>
              </div>

              <div>
                  <ul>
                      <li class="font-bold text-black text-xl">
                          <a href="/cim">{"AI Integration and Automation"}</a>
                          <p class="text-black text-sm">{"For Small & Medium sized Businesses"}</p>
                      </li>
                      <li class="mt-5 font-bold text-black text-xl">
                          <a href="/businessevolution">{"Evaluate & Identify"}</a>
                          <p class="text-black text-sm">{"Processes for AI Automation"}</p>
                      </li>
                      <li class="mt-5 font-bold text-black text-xl">
                          <a href="/newfunctionality">{"Superior Business Intelligence"}</a>
                          <p class="text-black text-sm">
                              {"Projections, Predictions and Intelligent Decision Making"}
                          </p>
                      </li>
                      <li class="mt-5 font-bold text-black text-xl">
                          <a href="/realtimemonitoring">{"Cost Efficient Solutions"}</a>
                          <p class="text-black text-sm">
                              {"Decrease costs associated with the use of AI"}
                          </p>
                      </li>
                  </ul>
              </div>

              <div class="mt-2">
                  <h2 class="font-bold text-blue-900 text-3xl mb-5">
                      {"Composable Information Machine"}
                  </h2>
                  <TextButton
                      route="/architecture".to_string()
                      content="How It Works...".to_string()
                  />
              </div>
              <div>
                  <a href="https://github.com/TheCowboyAI/cim-start">
                      <h3 class="font-bold text-blue-900 text-4xl">{"FREE and Open Source"}</h3>
                  </a>
              </div>
              <div>
                  <TextButton
                      route="https://github.com/TheCowboyAI/cim-start".to_string()
                      content="cim-start on GitHub".to_string()
                  />
              </div>
              <div>
                  <a href="/customgpts">
                      <h3 class="font-bold text-blue-900 text-4xl">{"Personal AI Assistants"}</h3>
                  </a>
              </div>
              <div>
                  <TextButton route="/gpts".to_string() content="Free GPTs on OpenAI".to_string()/>
              </div>
              <hr class="mt-10 border-black border-2"/>
              <div>
                  <h3 class="font-bold font-black text-2xl">{"Member of:"}</h3>
              </div>
              <div>
                  <a
                      href="https://www.microsoft.com/en-us/startups/"
                      target="_blank"
                      rel="noopener noreferrer"
                      class="font-bold text-black text-lg"
                  >
                      {"Microsoft for Startups Founders Hub"}
                  </a>
              </div>
              <hr class="border-black border-2"/>
              <div>
                  <h3 class="font-bold font-black text-2xl">{"Awards:"}</h3>
              </div>
              <div class="logo-container">
                  <a
                      class="font-bold mb-20"
                      href="https://mvp.microsoft.com/en-US/"
                      target="_blank"
                      rel="noopener noreferrer"
                  >
                      <img
                          src="assets/mvp_alumni_logo.png"
                          alt="Microsoft MVP Logo"
                          class="w-36 mb-20"
                      />
                      {"10 Consecutive Years"}
                  </a>
              </div>
          </div>
      </div>
  }
}
