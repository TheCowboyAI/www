use leptos::*;

#[component]
pub fn Cards() -> impl IntoView {
  view! {
    <div class="grid-cols-1 sm:grid md:grid-cols-3 ">
    </div>
  }
}

#[component]
pub fn Card(
  route: String, 
  imgsrc: String, 
  imgalt: String,
  title: String,
  text: String,
  footer: String,
  footlink: String,
) -> impl IntoView {
  view!{
    <div
      class="
        bg-[#699EBF]
        mx-3 mt-6 p-1
        border border-black border-solid
        flex flex-col
        rounded-lg
        text-surface 
        shadow-xl 
        shadow-black 
        sm:shrink-0 
        sm:grow sm:basis-0
      "
    >
    <a href={route}>
      <img
        class="rounded-lg"
        src={imgsrc} 
        alt={imgalt}
      />
    </a>
    <div class="p-2">
      <h5 class="text-black font-bold mb-3 text-xl">{title}</h5>
      <p class="shadow-lg shadow-black rounded-lg p-3 bg-[#5B83A6] text-black mb-4 text-base">
        {text}
      </p>
    </div>
    <div class="flex items-center justify-center mt-auto p-4">
      <div class="relative inline-flex group">
          <div
              class="absolute transitiona-all duration-1000 opacity-70 -inset-px bg-gradient-to-r from-[#44BCFF] via-[#FF44EC] to-[#FF675E] rounded-xl blur-lg group-hover:opacity-100 group-hover:-inset-1 group-hover:duration-200 animate-tilt">
          </div>
          <a href={footlink} title="Would you like to know more?"
              class="relative inline-flex items-center justify-center px-8 py-2 text-base text-white transition-all duration-200 bg-slate-700 font-pj rounded-xl focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900"
              role="button">{footer}
          </a>
      </div>
    </div>
  </div>
  }
}
