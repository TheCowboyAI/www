use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="h-screen flex animate-fadeIn">
            <div class="flex-col"> 
            <div>
            <a href="/aiintegration">
                <h1 class="mt-10 p-3 font-bold justify-start text-white text-4xl md:text-6xl">
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
                            {"For Small-Medium sized Businesses"}
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
                            {"Superior Data and Business Intelligence"}
                        </a>
                        <p class="text-black text-sm">
                            {"For Projections, Predictions and Intelligent Decision Making"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/workflow">
                            {"Cost Efficient Hosting"}
                        </a>
                        <p class="text-black text-sm">
                            {"For the increased costs associated with the use of AI"}
                        </p>
                    </li>
                </ul>
                <div>
                <h2 class="mt-10 p-3 font-bold text-white text-4xl md:text-6xl">
                    {"Cowboy AI: Capture your Data"}
                </h2>
                <ul class="mt-3 text-black text-2xl md:text-3xl py-2">
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/modeledintelligence">
                            {"Unleash your True Data's Potential"}
                        </a>
                        <p class="text-black text-sm">
                            {"Begin with a deep dive into your business DNA for tailored AI Solutions"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/workflow">
                            {"Automate Intelligently"}
                        </a>
                        <p class="text-black text-sm">
                            {"Inject our AI expertise to streamline and elevate your workflows"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/decisionmaking">
                            {"Empower Decisions"}
                        </a>
                        <p class="text-black text-sm">
                            {"Equip your strategy with our top-tier analytics and AI memory"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/predictions">
                            {"Drive Results"}
                        </a>
                        <p class="text-black text-sm">
                            {"Superior data for your AI means outpreforming the competition, every time."}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/realtimemonitoring">
                            {"Cost Efficient Cloud"}
                        </a>
                        <p class="text-black text-sm">
                            {"Migrate to smarter hosting for leaner costs with Cowboy AI's unique infrastructure"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/gametheory">
                            {"Optimized Performance"}
                        </a>
                        <p class="text-black text-sm">
                            {"Our Cloud prowess fuels your AI, cutting expenses without compromise"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/">
                            {"Command Excellence"}
                        </a>
                        <p class="text-black text-sm">
                            {"Access Advanced AI tools that set your business operations apart"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/">
                            {"Transform your Trajectory"}
                        </a>
                        <p class="text-black text-sm">
                            {"Partner with Cowboy AI for an evolutionary leap in business intelligence"}
                        </p>
                    </li>
                </ul>
                </div>
                <div>
                    <h3 class="mt-10 mb-10 p-3 font-bold text-red-500 text-4xl md:text-6xl">
                        {"Ransomware"}
                    </h3>
                    <p>
                    {"ASK US ABOUT our Proactive Ransomware Prevention and Protection"}
                    </p>
                </div>
                <div>
                <a href="/gpts">
                <button
                            class="
                              text-white 
                              bg-gradient-to-br 
                              from-purple-600 
                              to-blue-500 
                              hover:bg-gradient-to-bl 
                              focus:ring-4 
                              focus:outline-none 
                              focus:ring-blue-300 
                              dark:focus:ring-blue-800 
                              font-bold 
                              rounded-lg 
                              text-base 
                              px-5 
                              py-2.5 
                              text-center 
                              mt-5
                              mx-20" 
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
