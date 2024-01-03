use hellopage::HelloComponent;
use wal_core::router::builder::RouterBuilder;

mod hellopage;

fn main() {
    RouterBuilder::default()
        .add_page::<HelloComponent>("/")
        .build()
        .start();
}
