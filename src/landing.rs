use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="h-screen flex items-center animate-fadeIn">
            <div class="flex-col"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                    <a href="/aiintegration">
                        {"IT Starts with IntenT"}
                    </a>
                </h1>

                <ul>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/cim">
                            {"AI Integration and Automation"}
                        </a>
                        <p>
                            {"For Small-Medium sized Businesses"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/businessevolution">
                            {"Evaluate & Identify"}
                        </a>
                        <p>
                            {"Processes for AI Automation"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/newfunctionality">
                            {"Superior Data and Business Intelligence"}
                        </a>
                        <p>
                            {"For Projections, Predictions and Intelligent Decision Making"}
                        </p>
                    </li>
                    <li class="mt-3 font-bold text-black text-xl md:text-2xl">
                        <a href="/workflow">
                            {"Cost Efficient Hosting"}
                        </a>
                        <p>
                            {"For the increased costs associated with the use of AI"}
                        </p>
                    </li>
                </ul>

                <h2 class="mt-10 font-bold text-white text-4xl md:text-6xl">
                    {"Improve Productivity"}
                </h2>
                <ul class="text-black text-2xl md:text-3xl py-2">
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/realtimemonitoring">
                            {"Real-Time Monitoring"}
                        </a>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/decisionmaking">
                            {"Improve Decision-Making"}
                        </a>
                    </li>
                    <li class="font-bold text-black text-xl md:text-2xl">
                        <a href="/newfunctionality">
                            {"Integrate New Functionality"}
                        </a>
                    </li>
                </ul>
                <h3 class="mt-10 font-bold text-red text-4xl md:text-6xl">
                    {"Ransomeware"}
                </h3>
                <p>
                    {"ASK US ABOUT our Proactive Ransomeware Prevention and Protection"}
                </p>
                <h4 class="mt-10 font-bold text-center text-xl rounded-lg md-white text-blue-800 border-white border-2 border-solid p-2">
                    <a href="/gpts">
                        {"FREE GPTs"}
                    </a>
                </h4>
            </div>
        </div>
    }
}
