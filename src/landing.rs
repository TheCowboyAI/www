use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
  view! {
        <div class="h-screen flex items-center animate-fadeIn">
          <div class="flex-col"> 
            <h1 class="font-bold text-white text-4xl md:text-6xl">
              <a href="/aiintegration">
                {"Integrate and Automate AI"}
              </a>
            </h1>

            <ul>
                
                <li class="font-bold text-black text-xl md:text-2xl">
                    <a href="/businessevolution">
                        {"Business Process Evaluation"}
                    </a>
                </li>
                <li class="font-bold text-black text-xl md:text-2xl">
                    <a href="/cim">
                        {"Connect Microservices Patterns"}
                    </a>
                </li>
                <li class="mt-2 font-bold text-black text-xl md:text-2xl">
                    <a href="/workflow">
                        {"Task Automation"}
                    </a>
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
            <h3 class="mt-10 font-bold text-center text-xl rounded-lg bg-white text-blue-800 border-white border-2 border-solid p-2">
                <a href="/gpts">
                {"FREE GPTs"}
                </a>
            </h3>
        </div>
    </div>
  }
}


