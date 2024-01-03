use wal_core::component::Component;
use wal_rsx::rsx;

pub(crate) struct HelloComponent;

impl Component for HelloComponent {
    type Message = ();

    type Properties = ();

    fn new(_props: Self::Properties) -> Self {
        HelloComponent
    }

    fn view(
        &self,
        _behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        rsx!(
            <div class="container">
                <h1> "Hello World - this is Wal" </h1>
                <div  class="container">
                    <p>"Check out Wal resources:"</p>
                    <a href="https://github.com/walrust/wal">"official GitHub repository"</a>
                    <a href="https://github.com/walrust/wal">"wal documentation"</a>
                </div>
            </div>
        )
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}

impl Default for HelloComponent {
    fn default() -> Self {
        Self::new(())
    }
}
