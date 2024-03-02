use leptos::*;

#[component]
pub fn LandingPlay() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-col">
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                    <a href="/aiintegration" class="button">
                        {"Integrate and Automate AI"}
                    </a>
                </h1>

                <ul>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/businessevolution" class="button">
                            {"Business Process Evaluation"}
                        </a>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/cim" class="button">
                            {"Connect Microservices Patterns"}
                        </a>
                    </li>
                    <li class="mt-2 font-bold text-black text-xl md:text-2xl">
                        <a href="/workflow" class="button">
                            {"Task Automation"}
                        </a>
                    </li>
                </ul>

                <h2 class="mt-10 font-bold text-white text-4xl md:text-6xl">
                    {"Improve Productivity"}
                </h2>
                <ul class="text-black text-2xl md:text-3xl py-2">
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/realtimemonitoring" class="button">
                            {"Real-Time Monitoring"}
                        </a>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/decisionmaking" class="button">
                            {"Improve Decision-Making"}
                        </a>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/newfunctionality" class="button">
                            {"Integrate New Functionality"}
                        </a>
                    </li>
                </ul>
                <h3 class="mt-10 font-bold text-center text-xl rounded-lg bg-white text-blue-800 border-white border-2 border-solid p-2">
                    <a href="/gpts" class="button">
                        {"FREE GPTs"}
                    </a>
                </h3>
            </div>
        </div>
    }
}

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-col">
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                    <a href="/aiintegration" class="button">
                        {"Integrate and Automate AI"}
                    </a>
                </h1>

                <ul>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/businessevolution" class="button">
                            {"Business Process Evaluation"}
                        </a>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/cim" class="button">
                            {"Connect Microservices Patterns"}
                        </a>
                    </li>
                    <li class="mt-2 font-bold text-black text-xl md:text-2xl">
                        <a href="/workflow" class="button">
                            {"Task Automation"}
                        </a>
                    </li>
                </ul>

                <h2 class="mt-10 font-bold text-white text-4xl md:text-6xl">
                    {"Improve Productivity"}
                </h2>
                <ul class="text-black text-2xl md:text-3xl py-2">
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/realtimemonitoring" class="button">
                            {"Real-Time Monitoring"}
                        </a>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/decisionmaking" class="button">
                            {"Improve Decision-Making"}
                        </a>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/newfunctionality" class="button">
                            {"Integrate New Functionality"}
                        </a>
                    </li>
                </ul>
                <h3 class="mt-10 font-bold text-center text-xl rounded-lg bg-white text-blue-800 border-white border-2 border-solid p-2">
                    <a href="/gpts" class="button">
                        {"FREE GPTs"}
                    </a>
                </h3>
            </div>
        </div>
    }
}








Cut from landing.rs
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