use yew::{function_component, html};
mod info;
use crate::info::Info;

#[function_component(Main)]
pub fn home() -> Html {
  html! {
   <div>
   <Info num={2} />
   <Info />
   </div>
  }
}

pub fn main() {
  yew::start_app::<Main>();
}
