use leptos::*;

use crate::cards::Card;

#[component]
pub fn CardTest() -> impl IntoView {
  view! {
    <div class="grid-cols-1 sm:grid md:grid-cols-3 items-center">
      <Card
        route="/architecture".to_string()
        imgsrc="assets/CIM-Card.png".to_string() 
        imgalt="CIM".to_string()
        title="Composable Information Machine".to_string()
        text="CIM is a new way to see the world of information. We are transitioning from the world of Location-based Hierarchical Files, Rows and Columns, to something else.".to_string()
        footer="Model your success".to_string()
      />
      <Card
        route="/aiintegration".to_string()
        imgsrc="assets/hands.png".to_string() 
        imgalt="AI Integration".to_string()
        title="AI Integration".to_string()
        text="Cowboy AI stands at the forefront of AI integration, expertly navigating the complexities of artificial intelligence to tailor solutions that are perfectly aligned with your business needs. As AI Integration Experts, Cowboy AI ensures that your systems are intelligent, efficient, and strategically designed to drive growth and innovation.".to_string()
        footer="Automated Excellence".to_string()
      />
      <Card
        route="/businessevolution".to_string()
        imgsrc="assets/bizevo.png".to_string() 
        imgalt="evolution".to_string()
        title="Business Evolution".to_string()
        text="Business evolution refers to the continuous transformation and adaptation of a business to meet changing market demands, technology advancements, and shifts in consumer behavior.".to_string()
        footer="Automated Excellence".to_string()
      />
      <Card
        route="https://github.com/TheCowboyAI/cim-start".to_string()
        imgsrc="assets/FOSS.png".to_string() 
        imgalt="open source".to_string()
        title="FREE and Open Source".to_string()
        text="This is a machine to control your information and how it is processed. When we say machine... think of the entire internet as a single machine. This is a new enabling technology that we will guide you to create a personal information system capable of semantic engagement with all your information, no matter where you choose to create it or store it.".to_string()
        footer="Software freedom for all".to_string()
      />
      <Card
        route="/gpts".to_string()
        imgsrc="assets/custom-gpts.png".to_string() 
        imgalt="Custom GPTs".to_string()
        title="Custom GPTs".to_string()
        text="Cowboy AI excels in creating tailored GPTs and AI Agents that are not just technologically advanced but also intricately aligned with your business model. These intelligent agents are designed to understand your business's unique landscape, communicate with each other, and securely access internal company data to drive informed decisions and actions.".to_string()
        footer="Personal AI Assistants".to_string()
      />
      <div class="mb-10">
      </div>  
    </div>
  }
}
