use super::app::input;

pub fn task_input(_: &mut input::Context) {
    // Example input task implementation
    // Access shared resources with `cx.shared` and local resources with `cx.local`
    defmt::println!("Input task running");
}