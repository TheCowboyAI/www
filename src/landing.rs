use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
  view! {
        <div class="h-screen flex items-center animate-fadeIn">
          <div class="flex-col"> 
            <h1 class="font-bold text-white text-4xl md:text-6xl">
              <a href="/aiintegration">
                {"AI Integration Experts"}
              </a>
            </h1>

            <ul>
                <li class="mt-2 font-bold text-black text-xl md:text-2xl">
                    <a href="/workflow">
                        <span>{"Workflow Automation"}</span>
                    </a>
                </li>
                <li class="font-bold text-black text-xl md:text-2xl">
                    <a href="/businessevolution">
                        {"Evolutionary Model"}
                    </a>
                </li>
                <li class="font-bold text-black text-xl md:text-2xl">
                    <a href="/cim">
                        {"CIM - Microservice Pattern"}
                    </a>
                </li>
            </ul>

            <h2 class="mt-10 font-bold text-white text-4xl md:text-6xl">
            {"Improve Productivity"}
            </h2>
            <ul class="text-black text-2xl md:text-3xl py-2">
                <li class="font-bold text-black text-xl md:text-2xl">
                    <a href="/generatecontent">
                        {"Generate Content"}
                    </a>
                </li>
            
                <li class="font-bold text-black text-xl md:text-2xl">
                    <a href="/context">
                    {"Grasp what's happening"}
                    </a>
                </li>
            
                <li class="font-bold text-black text-xl md:text-2xl">
                    <a href="/deterministic">
                    {"Be unwavering in your intentions"}
                    </a>
                </li>
            </ul>
            <h3 class="mt-10 font-bold text-center text-xl rounded-lg bg-white text-blue-800 border-white border-2 border-solid p-2">
                <a href="/gpts">
                {"Access our GPTs"}
                </a>
            </h3>
        </div>
    </div>
  }
}


