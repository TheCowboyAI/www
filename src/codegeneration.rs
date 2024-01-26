use leptos::*;

#[component]
pub fn CodeGeneration() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"AI in Source Code Generation"}
          </h1>
          <p class="mt-5">
              {"AI is transforming the landscape of software development by producing source code in unprecedented ways. This is not merely about replacing text strings, but involves a sophisticated analysis known as vectorized analysis, which understands context and intent to generate high-quality, functional code."}
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
                          {"Beyond String Replacement"}
                      </summary>
                      <p>
                          {"AI's approach to generating code goes far beyond simple string replacement. It deeply understands the structure, syntax, and semantics of code, enabling it to generate functionally correct and logically coherent source code, much like a skilled developer."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Vectorized Analysis"}
                      </summary>
                      <p>
                          {"Vectorized analysis allows AI to understand and generate code by converting code into mathematical vectors. This means the AI can grasp the deeper meaning and relationships in the code, ensuring that the generated code aligns with the intended functionality and context."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Revolutionizing Development Processes"}
                      </summary>
                      <p>
                          {"The capability of AI to produce source code is revolutionizing software development. It speeds up the development process, reduces errors, and allows your team to focus on strategic, creative tasks while the AI handles the routine coding."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"Integrating AI into source code generation marks a significant leap forward, offering a powerful tool that enhances productivity, fosters innovation, and ensures that your software solutions are not only robust but also aligned perfectly with your business goals."}
          </p>
      </div>
  </div>
  
    }
}
