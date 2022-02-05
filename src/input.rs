use web_sys::HtmlInputElement;
use yew::events::KeyboardEvent;
use yew::prelude::Callback;

#[derive(PartialEq, Properties, Clone)]
pub struct CustomInputProps {
  pub onadd: Callback<String>,
}

#[function_component(CustomInput)]
pub fn custom_input(props: &CustomInputProps) -> Html {
  let onkeypress = {
    let onadd = props.onadd.clone();
    move |e: keyboardEvent| {
      if e.key() == "Enter" {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value = input.value();
        input.set_value("");
        onadd.emit(value);
      }
    }
  };

  html! {
    <input
        class="new-todo"
        placeholder="What needs to be done?"
        {onkeypress}
    />
  }
}
