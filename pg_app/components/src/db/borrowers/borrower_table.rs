use dioxus::{
    logger::tracing, prelude::*
};
use crate::ui::input::{Input, InputType};
use crate::ui::button::{Button, ButtonScheme};
use server::post_functions::{delete_post, update_post, get_all_posts};

use shared::models;
use std::sync::Arc;

static POSTS: GlobalSignal<Vec<models::Post>> = GlobalSignal::new(Vec::new);



use crate::ui::{Table, TableHead, TableBody, TableRow, TableCell, TableHeaderCell, TableFoot};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            h2 { class: "text-2xl font-bold mb-6 text-center", "Good morning user" }
            Table {
                striped: true,
                hoverable: true,
                caption: rsx! { "Borrower Information" },
                TableHead {
                    TableRow {
                        TableHeaderCell { "First Name" }
                        TableHeaderCell { "Last Name" }
                        TableHeaderCell { "Department" }
                        TableHeaderCell { "Salary" }
                    }
                    <th scope="col" class="px-1 py-3">#</th>
                    <th scope="col" class="px-4 py-3">Status</th>
            
                    <th scope="col" class="px-4 py-3">Name</th>
                    <th scope="col" class="px-4 py-3">Loan #</th>
                    <th scope="col" class="px-4 py-3">Start Date</th>
                    <th scope="col" class="px-4 py-3">Type</th>
                    <th scope="col" class="px-4 py-3">Amount</th>
                    <th scope="col" class="px-4 py-3">Rate</th>
                    <th scope="col" class="px-4 py-3">State</th>
                    <th scope="col" class="px-4 py-3">View</th>





                }
                TableBody {
                    TableRow {
                        TableCell { "John Doe" }
                        TableCell { "Software Engineer" }
                        TableCell { "Engineering" }
                        TableCell { "$95,000" }
                    }
                    TableRow {
                        TableCell { "Jane Smith" }
                        TableCell { "Product Manager" }
                        TableCell { "Product" }
                        TableCell { "$105,000" }
                    }
                    TableRow {
                        TableCell { "Bob Johnson" }
                        TableCell { "UX Designer" }
                        TableCell { "Design" }
                        TableCell { "$85,000" }
                    }
                }
                TableFoot {
                    TableRow {
                        TableHeaderCell { colspan: Some(3), "Total" }
                        TableCell { "$285,000" }
                    }
                }
            }
        }
    }
}
















#[component]
pub fn Post(post: models::Post) -> Element {
    // Get the refresh function from context
    let refresh_posts = use_context::<Arc<dyn Fn()>>();
    let mut title = use_signal(|| post.title.clone());
    let mut body = use_signal(|| post.body.clone());
    
    // Clone the refresh function outside the closures to avoid move issues
    let update_refresh = refresh_posts.clone();
    let delete_refresh = refresh_posts.clone();
    
    rsx! {
        div { class: "my-1 flex flex-row items-center gap-2",
            Input {
                name: "title".to_string(),
                input_type: Some(InputType::Text),
                placeholder: Some("Title".to_string()),
                value: Some(title()),
                label: Some("Post Title".to_string()),
                oninput: move |event: FormEvent| title.set(event.value()),
            }
            Input {
                name: "body".to_string(),
                input_type: Some(InputType::Text),
                placeholder: Some("Body".to_string()),
                value: Some(body()),
                label: Some("Post Body".to_string()),
                oninput: move |event: FormEvent| body.set(event.value()),
            }
            Button {
                button_scheme: ButtonScheme::Default,
                on_click: move |_| {
                    let title_val = title();
                    let body_val = body();
                    let refresh = update_refresh.clone();
                    spawn(async move {
                        match update_post(post.id, title_val, body_val).await {
                            Ok(_) => {
                                refresh();
                            }
                            Err(err) => tracing::error!("update posts error: {err}"),
                        }
                    });
                },
                text: "Update".to_string(),
            }
            Button {
                button_scheme: ButtonScheme::Danger,
                on_click: move |_| {
                    let refresh = delete_refresh.clone();
                    spawn(async move {
                        match delete_post(post.id).await {
                            Ok(_) => {
                                refresh();
                            }
                            Err(err) => tracing::error!("delete post error: {err}"),
                        }
                    });
                },
                text: "Delete".to_string(),
            }
        }
    }
}
