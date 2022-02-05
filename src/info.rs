use yew::MouseEvent;
use yew::{function_component, html, use_state, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct InfoProps {
  #[prop_or_default]
  pub info: String,
  #[prop_or_default]
  pub num: u64,
}

#[function_component(Info)]
pub fn info(props: &InfoProps) -> Html {
  let counter = use_state(|| 0);
  let onclick = {
    let counter = counter.clone();
    Callback::from(move |_| counter.set(*counter + 1))
  };
  let num = match &props.num {
    0 => "".to_string(),
    _ => props.num.to_string(),
  };

  html! {
    <div>
      <button onclick={onclick}>{"incremente"}</button>
      <p>
        <b>{ "Current value: " }</b>
        { *counter }
      </p>
      <h1>{"Info: props"}</h1>
      <h1>{&props.info}</h1>
      <h1>{&num}</h1>
    </div>
  }
}
