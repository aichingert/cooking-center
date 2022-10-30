use yew::prelude::*;

pub struct App;

pub enum Msg {
    Test
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context::<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.home_view()
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {

    }
}

impl App {
    pub fn home_view(&self) -> Html {
        html! {
            <>
                <nav>
                    <ul>
                        <li> { format!("Test") } </li>
                    </ul>
                </nav>
            </>
        }
    }
}
