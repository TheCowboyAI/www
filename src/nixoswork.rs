use leptos::*;

#[component]
pub fn NixOSWork() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"NixOS at Cowboy AI"}
          </h1>
          <p class="mt-5">
              {"NixOS is a Linux-based operating system that specializes in creating reproducible and declarative configurations, ensuring that systems are determinate and reliable. Cowboy AI harnesses the power of NixOS to develop, deploy, and configure robust systems, including advanced Flake development, virtual machines, and networks, all designed to seamlessly integrate with the CIM platform, ensuring precision, consistency, and scalability."}
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
                          {"Reproducible and Determinate Systems with NixOS"}
                      </summary>
                      <p>
                          {"NixOS's unique approach to system configuration allows for reproducible builds, meaning that systems can be replicated exactly, ensuring consistency and reducing the chances of discrepancies. Cowboy AI leverages this feature to create determinate systems that behave as expected under any circumstances."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Advanced Flake Development and Configuration"}
                      </summary>
                      <p>
                          {"With the introduction of Flakes, NixOS provides an even more powerful way to manage system configurations and dependencies. Cowboy AI utilizes Flakes to ensure that every aspect of the system, from software packages to system configurations, is precisely defined and controlled."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Seamless Integration with CIM Platform"}
                      </summary>
                      <p>
                          {"Cowboy AI ensures that the systems developed on NixOS are perfectly aligned with the CIM platform. This includes setting up virtual machines, networks, and other infrastructure components, all configured to work harmoniously within the CIM ecosystem, enhancing the efficiency and scalability of business operations."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"Leveraging NixOS's capabilities, Cowboy AI delivers determinate, reliable, and scalable systems, all intricately designed to support and enhance the business processes and strategic objectives defined within the CIM platform."}
          </p>
      </div>
  </div>
  
    }
}
