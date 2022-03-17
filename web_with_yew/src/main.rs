use yew::prelude::*;

struct Model{
    value:i64
}
#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model{
        value;
    })
}