use yew::{html, props, Component, Context, Properties};

#[derive(Clone, PartialEq)]
pub enum LinkColor {
    Blue,
    Red,
    Green,
    Black,
    Purple,
}

#[derive(Clone, PartialEq)]
pub enum NameButton {
    Blue(String),
    Red,
    Green,
    Black,
    Purple,
}
impl Default for NameButton {
    fn default() -> Self {
        NameButton::Blue("kdkdk".to_string())
    }
}

fn create_default_link_color() -> LinkColor {
    LinkColor::Blue
}

fn default_name_button() -> NameButton {
    NameButton::Blue("blue".to_string())
    //"Default---".to_string()
}

#[derive(PartialEq, Properties, Default)]
pub struct Props {
    #[prop_or_else(default_name_button)]
    button_text: NameButton,
}

impl Props {
    pub fn name_button(button_text: NameButton) -> Self {
        props! {
            Props{
                button_text
            }
        }
    }
}
enum Msg {
    AddOne(String),
}
struct App {
    value: i64,
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne(_) => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let link = ctx.link();
        let handle_add_on = link.callback(|_| Msg::AddOne("cliquei".to_string()));
        // let text = Props::name_button("name2 ".to_string());
        // Props::name_button(NameButton::Blue("blue".to_string()));
        let name = ctx.props().button_text.clone();
        html! {
            <div>
                <button
                onclick={handle_add_on}
                >
                {match name {
                    Red =>{"ll"},
                    Blue => {"Blue"},
                    _ =>{"ppp"}
                }}
                </button>
                <p>{self.value}</p>
            </div>
        }
    }
}

pub fn main() {
    yew::start_app::<App>();
}
