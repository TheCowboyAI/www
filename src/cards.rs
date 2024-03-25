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
    class="mx-3 mt-6 flex flex-col rounded-lg bg-white text-surface shadow-xl shadow-black dark:bg-surface-dark dark:text-white sm:shrink-0 sm:grow sm:basis-0 shadow-xl">
    <a href={route}>
      <img
        class="rounded-t-lg"
        src={imgsrc} 
        alt={imgalt}
      />
    </a>
    <div class="p-6 shadow-lg">
      <h5 class="text-black font-bold mb-3 text-2xl">{title}</h5>
      <p class="shadow-lg shadow-black rounded-lg p-3 text-black mb-4 text-base">
        {text}
      </p>
    </div>
    <div class="text-black mt-auto border-t-2 border-neutral-100 px-6 py-3 text-center text-surface/75 dark:border-white/10">
      <small>{footer}</small>
    </div>
  </div>
  }
}