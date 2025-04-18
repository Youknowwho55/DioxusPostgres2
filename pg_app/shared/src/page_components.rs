// shared/src/page_components.rs
#[component]
pub fn NavBar() -> Element {
    // Default implementation or leave empty
    rsx! {
        Outlet::<super::routes::Route> {}
    }
}