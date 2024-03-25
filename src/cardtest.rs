use leptos::*;

use crate::{cards::Card, textbutton::TextButton};

#[component]
pub fn CardTest() -> impl IntoView {
  view! {
    <div class="grid-cols-1 sm:grid md:grid-cols-3 items-center">
      <Card
        route="/cim".to_string()
        imgsrc="assets/CIM-Card.png".to_string() 
        imgalt="CIM".to_string()
        title="Composable Information Machine".to_string()
        text="CIM is a systemized graph of information, every node and edge has a single purpose, designed for composition. We are transitioning from the world of Location-based Hierarchical Files, Rows and Columns, to something else.".to_string()
        footer="Model your success".to_string()
      />
      <div class="mt-5 flex flex-1">
        <TextButton
          route="/architecture".to_string() 
          content="How It Works...".to_string() 
        />
      </div>
      <Card
        route="/aiintegration".to_string()
        imgsrc="assets/hands.jpg".to_string() 
        imgalt="AI Integration".to_string()
        title="AI Integration".to_string()
        text="Cowboy AI stands at the forefront of AI integration, expertly navigating the complexities of artificial intelligence to tailor solutions that are perfectly aligned with your business needs. As AI Integration Experts, Cowboy AI ensures that your systems are intelligent, efficient, and strategically designed to drive growth and innovation.".to_string()
        footer="Automated Excellence".to_string()
      />
      <Card
        route="/businessevolution".to_string()
        imgsrc="assets/evolution.png".to_string() 
        imgalt="evolution".to_string()
        title="Business Evolution".to_string()
        text="".to_string()
        footer="Automated Excellence".to_string()
      />
      <Card
        route="/aiintegration".to_string()
        imgsrc="assets/hands.jpg".to_string() 
        imgalt="AI Integration".to_string()
        title="AI Integration".to_string()
        text="Cowboy AI stands at the forefront of AI integration, expertly navigating the complexities of artificial intelligence to tailor solutions that are perfectly aligned with your business needs. As AI Integration Experts, Cowboy AI ensures that your systems are intelligent, efficient, and strategically designed to drive growth and innovation.".to_string()
        footer="Automated Excellence".to_string()
      />
    </div>
  }
}
