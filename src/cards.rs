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
) -> impl IntoView {
  view!{
    <div
      class="bg-[#74A7C1]
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
    <div class="p-2 shadow-lg">
      <h5 class="text-black font-bold mb-3 text-2xl">{title}</h5>
      <p class="shadow-lg shadow-black rounded-lg p-3 bg-slate-100 text-black mb-4 text-base">
        {text}
      </p>
    </div>
    <div class="bg-slate-200 rounded-lg text-black px-6 py-3 text-center">
      <small>{footer}</small>
    </div>
  </div>
  }
}
