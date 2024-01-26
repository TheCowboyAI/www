use leptos::*;

#[component]
pub fn Deterministic() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"Composable Information Machine (CIM) for Web 3.0"}
          </h1>
          <p class="mt-5">
              {"A Deterministic System is a system in which no randomness is involved in the development of future states of the system. A Composable Information Machine (CIM) is an advanced model ideal for Web 3.0, offering modularity, flexibility, and automation in managing and delivering services."}
          </p>
          <ol
              class=" 
                  mr-5 py-2
                  text-black text-xl md:text-2xl lg:text-3xl 
              "
          >
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Deterministic System"}
                      </summary>
                      <p>
                          {"A Deterministic System is a predictable system where the same set of initial conditions will always produce the same results. This makes it reliable and consistent, crucial for processes where precision and repeatability are key."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Composable Information Machine (CIM)"}
                      </summary>
                      <p>
                        <a href="/cim">CIMs</a>   
                          {" represent the next generation of web technology (Web 3.0), focusing on decentralized, user-centric models. They allow for the composition and reusability of different information systems and services, offering a flexible and efficient way to manage complex systems."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"CIM - Fabricator"}
                      </summary>
                      <p>
                          {"The Fabricator is a comprehensive tool for automating the setup and configuration of a CIM, managing everything from creating Git repositories, managing component configurations, setting up storage, and creating the semantic framework, to generating API code and creating workflows. It embodies the principles of automation, precision, and efficiency, crucial for Web 3.0's distributed, decentralized, and user-empowered architecture."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"The integration of Deterministic Systems with CIMs, especially through automation tools like the Fabricator, paves the way for a more structured, reliable, and efficient Web 3.0, focusing on user empowerment, data security, and seamless service delivery."}
          </p>
      </div>
  </div>
  
    }
}
