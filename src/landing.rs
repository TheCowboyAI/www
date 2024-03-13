use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="h-screen flex animate-fadeIn mr-5">
            <div class="flex-col"> 
            <div>
            <a href="/aiintegration">
                <h1 class="mt-10 p-3 font-bold justify-start bbblue-text text-4xl md:text-6xl">
                        {"IT Starts with IntenT"}
                </h1>
            </a>
            </div>
                <ul>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/cim">
                            {"AI Integration and Automation"}
                        </a>
                        <p class="text-black text-sm">
                            {"For Small & Medium sized Businesses"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/businessevolution">
                            {"Evaluate & Identify"}
                        </a>
                        <p class="text-black text-sm">
                            {"Processes for AI Automation"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/newfunctionality">
                            {"Superior Data and Business 
                              Intelligence"}
                        </a>
                        <p class="text-black text-sm">
                            {"For Projections, Predictions and
                              Intelligent Decision Making"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/realtimemonitoring">
                            {"Cost Efficient Hosting"}
                        </a>
                        <p class="text-black text-sm">
                            {"For the increased costs associated with 
                              the use of AI"}
                        </p>
                    </li>
                </ul>
                <div>
                    <a href="/security">
                    <h2 class="mt-5 p-3 font-bold text-purple-700 text-4xl md:text-6xl">
                        {"Ransomware"}
                    </h2>
                    </a>
                    <p class="mt-3 font-bold text-black text-xl md:text-2xl">
                    {"Proactive Ransomware Prevention and Protection"}
                    </p>
                </div>
                <div>
                <a href="/architecture">
                    <button
                        class="
                          mt-5 
                          p-3 
                          font-bold 
                          text-white 
                          text-2xl 
                          md:text-4xl
                          bg-gradient-to-br 
                          from-purple-600 
                          to-blue-500 
                          hover:bg-purple-600 
                          active:bg-purple-700
                          focus:ring-4 
                          focus:outline-none 
                          focus:ring-blue-300 
                          dark:focus:ring-blue-800 
                          font-bold 
                          rounded-lg 
                          text-base 
                          px-3 
                          py-2.5 
                          text-center 
                          mx-20
                          mb-10"
                          type="button"
                    >
                        How It Works...
                    </button>    
                </a>
            </div>
                <div>
                <a href="/customgpts">
                <h3 class="mt-5 p-3 font-bold text-white text-4xl md:text-6xl">
                    {"Personal AI Assistants"}
                </h3>
                </a>
                <p class="mt-3 font-bold text-black text-xl md:text-2xl">
                {"Your vision, our AI expertise"}
                </p>
            </div>
                <div>
                <a href="/gpts">
                <button
                            class="
                              text-white 
                              text-2xl 
                              md:text-4xl
                              bg-gradient-to-br 
                              from-purple-600 
                              to-blue-500 
                              hover:bg-purple-600 
                              active:bg-purple-700
                              focus:ring-4 
                              focus:outline-none 
                              focus:ring-blue-300 
                              dark:focus:ring-blue-800 
                              font-bold 
                              rounded-lg 
                              text-base 
                              px-3
                              py-2.5 
                              text-center 
                              mt-5
                              mx-20
                              mb-10" 
                              type="button"
                          >
                            FREE GPTs
                          </button>
                </a>
                </div>
            </div>
        </div>
    }
}
