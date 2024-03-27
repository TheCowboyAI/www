use leptos::*;

#[component]
pub fn NewWorld() -> impl IntoView {
    view! {
      <div class="mt-5 animate-fadeIn">
      <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
          <h1 class="shadow-text-black font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
              {"The Brave, New World"}
                </h1>
                <p>
                    {"The challenge is to harness the immense potential of technology in a way that enhances individual freedom and creativity while also addressing the critical issues of resource management and sustainability. Through thoughtful design, governance, and cooperation, it's possible to navigate these complex dynamics and create a digital ecosystem that benefits everyone."}
                </p>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Making Sense of Information"}
                    </summary>
                    <div class="flex flex-col gap-4">
                      <p>
                          {"Typically we are told that to work with information, we need a computer. That's all great, until you realize that this is just the introduction."}
                      </p>
                      <p>
                        {"Then we are told about programs, apps, operating systems, networks, containers, databases, micro services, APIs... and math, you need to know math... and our head explodes."}
                      </p>
                      <p>
                        {"We watched decades of propietary software duke it out, just to eventually go Open Source with almost everything and move to a subscription model."}
                      </p>
                      <p>
                        {"Perhaps, you're a computer professional and you have been doing this a long time..."}
                      </p>
                      <p>
                        {"Just keeping up with the daily industry changes is a full-time job. It doesn't matter what Operating System you choose, or where you put things, it's all changed."}
                      </p>
                      <p>
                        {"We would have to buy new equipment every few years just to keep up with the steady march of Moore's Law. This is not only impractical for most people, it's become something we don't 'need'."}
                      </p>
                      <p>
                        {"I'm certainly not saying you should throw everything away and use the cloud for everything... We usually have some blend of equipment and that in itself becomes a nightmare to maintain."}
                      </p>
                      <p>
                        {"Shift to Containers, became the battle-cry a few years ago, that's lovely... Now, how do I manage a hundred containers and keep them all up to date?"}
                      </p>
                      <p>
                        {"The cloud changes everything, but it's also the same.  We just shifted platforms. Hardware became a commodity we can rent or I can do a bunch of things with IOT that used to need full PCs. I don't need to go buy a giant workstation to do some work, then get stuck with it not being utilized for a long time."}
                      </p>
                      <p>
                        <span class="font-bold">{"Enter AI - "}</span>
                        <span>{"AI truly changes everything, and I am not just referring to things like ChatGPT and Co-pilot, we are talking about internal, tuned engines that can be instructed to perform amazing language based tasks."}</span>
                      </p>
                    </div>
                </details>
                <h2 class="font-bold italic text-lg">
                {"Cowboy AI changes the landscape."}                </h2>
                <p>
                    {"We use a Business Model coupled with known standards of configuring and deploying compute resources in real-time. You create configurations... No, not by hand, but instead with real guidance drawn directly from the Applications and Workflows that are important to you."}
                </p>
                <p class="shadow-text-black rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-lg">
                <span class="font-bold italic">
                  {"It's a paradigm shift in innovation."}
                </span>
                <br/>
                <span>
                  {"We can create our own platform and not get lost in the maintenance."}
                </span>
                </p>
            </div>
        </div>
    }
}
