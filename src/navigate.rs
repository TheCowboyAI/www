use leptos::*;
use crate::footer::Footer;

#[component]
pub fn Navigate() -> impl IntoView {
    view! {
        <nav
        class="
          flex fixed w-full h-16 rounded-lg rounded-b items-center justify-between 
          px-6 bg-[#74A7C1] text-black border-b border-black border-solid z-10
        "
      >
        <div id="logo" class="p-2">
          <a href="/">
            <div class="py-4 flex items-center w-14">
              <img src="assets/logo.svg" alt="Cowboy AI Logo" title="Cowboy AI" />
            </div>
          </a>
        </div>
      
        <div id="title">
          <span class="font-bold text-4xl text-black">{"Cowboy AI"}</span>
        </div>
      
        <div>
          <label class="menu-button-wrapper" for="">
            <input type="checkbox" class="menu-button" />
            <div class="icon-wrapper">
              <label class="hamburger">
                <input class="hamburger-input" type="checkbox" />
                <span class="hamburger-line first"></span>
                <span class="hamburger-line second"></span>
                <span class="hamburger-line third"></span>
              </label>
            </div>
            <div class="item-list">
              <div>Home</div>
              <div>About</div>
              <div>Profile</div>
              <div>Contact</div>
            </div>
          </label>
        </div>
      </nav>
  }
}
