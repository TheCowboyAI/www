use leptos::*;

#[component]
pub fn AuditTrail() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"An Immutable Audit Trail"}
          </h1>
          <p class="mt-5">
              {"Event Sourcing serves as a powerful method to create an immutable audit trail in your system. It meticulously records each event or action, providing a transparent, secure, and unchangeable history of all operations, ensuring high levels of integrity and accountability."}
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
                          {"Immutable Record of Events"}
                      </summary>
                      <p>
                          {"Every event in the system is stored in a sequence that cannot be altered or deleted. This immutability ensures that the audit trail is secure and reliable, providing an accurate historical record of all actions and changes."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Transparency and Traceability"}
                      </summary>
                      <p>
                          {"Event Sourcing offers complete transparency, allowing businesses to trace any action back to its origin. This traceability is invaluable for debugging, understanding system behavior, and ensuring compliance with regulations and standards."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Enhanced System Reliability and Accountability"}
                      </summary>
                      <p>
                          {"The immutable audit trail provided by Event Sourcing enhances system reliability and accountability. It enables businesses to verify the integrity of their operations and provides a robust foundation for auditing and decision-making processes."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"By leveraging Event Sourcing as an immutable audit trail, businesses can ensure the integrity of their data, maintain high standards of operational transparency, and foster a culture of accountability and trust."}
          </p>
      </div>
  </div>
  
    }
}
