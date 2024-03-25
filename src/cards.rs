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
    class="mx-3 mt-6 flex flex-col rounded-lg bg-white text-surface shadow-secondary-1 dark:bg-surface-dark dark:text-white sm:shrink-0 sm:grow sm:basis-0 shadow-xl">
    <a href={route}>
      <img
        class="rounded-t-lg"
        src={imgsrc} //"https://tecdn.b-cdn.net/img/new/standard/city/042.webp"
        alt={imgalt} // "Palm Springs Road" />
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


// <div
// class="mx-3 mt-6 flex flex-col rounded-lg bg-white text-surface shadow-secondary-1 dark:bg-surface-dark dark:text-white sm:shrink-0 sm:grow sm:basis-0">
// <a href="#!">
//   <img
//     class="rounded-t-lg"
//     src="https://tecdn.b-cdn.net/img/new/standard/city/044.webp"
//     alt="Skyscrapers" />
// </a>
// <div class="p-6">
//   <h5 class="mb-2 text-xl font-medium leading-tight">Card title</h5>
//   <p class="mb-4 text-base">
//     This is a wider card with supporting text below as a natural
//     lead-in to additional content. This content is a little bit
//     longer.
//   </p>
// </div>
// <div
//   class="mt-auto border-t-2 border-neutral-100 px-6 py-3 text-center text-surface/75 dark:border-white/10 dark:text-neutral-300">
//   <small>Last updated 3 mins ago</small>
// </div>
// </div>

// <div
// class="mx-3 mt-6 flex flex-col rounded-lg bg-white text-surface shadow-secondary-1 dark:bg-surface-dark dark:text-white sm:shrink-0 sm:grow sm:basis-0">
// <a href="#!">
//   <img
//     class="rounded-t-lg"
//     src="https://tecdn.b-cdn.net/img/new/standard/city/043.webp"
//     alt="Los Angeles Skyscrapers" />
// </a>
// <div class="p-6">
//   <h5 class="mb-2 text-xl font-medium leading-tight">Card title</h5>
//   <p class="mb-4 text-base">
//     This card has supporting text below as a natural lead-in to
//     additional content.
//   </p>
// </div>
// <div
//   class="mt-auto border-t-2 border-neutral-100 px-6 py-3 text-center text-surface/75 dark:border-white/10 dark:text-neutral-300">
//   <small>Last updated 3 mins ago</small>
// </div>
// </div>
