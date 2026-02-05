use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class = "footer">
            <p>
                { "Made with " }
                <span class="icon">{"❤️"}</span>
                { " by team DharitrI" }
            </p>
        </footer>
    }
}
