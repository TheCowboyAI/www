use leptos::*;

#[component]
pub fn GenerateContent() -> impl IntoView {
    view! {
      <div class="mt-10 flex items-center animate-fadeIn">
      <div class="flex-row"> 
          <h1 class="font-bold text-white text-4xl md:text-6xl">
              {"AI in Content Generation for Business"}
          </h1>
          <p class="mt-5">
              {"AI can be a creative partner for your business, helping to generate all kinds of content. It's like having a super-smart helper who can write documents, create pictures, and even make videos to tell your business's story in a fun and interesting way."}
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
                          {"Writing Documents"}
                      </summary>
                      <p>
                          {"AI can write articles, reports, and even emails for your business. It's like having a smart writer who knows exactly what you want to say and helps you say it clearly and effectively."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Creating Pictures"}
                      </summary>
                      <p>
                          {"AI can design logos, create artwork, and even edit photos. It's like having an artist who can turn your ideas into beautiful pictures that grab people's attention."}
                      </p>
                  </details>
              </li>
              <li class="mt-5">                    
                  <details class="text-black text-sm">
                      <summary class="text-white text-base font-bold">
                          {"Making Videos"}
                      </summary>
                      <p>
                          {"AI can help make videos for your business, from planning the story to editing the final product. It's like having a director and a film crew who can help bring your business's story to life on the screen."}
                      </p>
                  </details>
              </li>
          </ol>
          <p class="mt-5">
              {"Using AI in content generation means your business can tell its story in many different ways, making sure that every blog post, image, and video is just right for your audience. It's a powerful way to connect with people and show them what makes your business special."}
          </p>
      </div>
  </div>
  
    }
}
