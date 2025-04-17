use dioxus::prelude::*;

use components::ui::{Table, TableHead, TableBody, TableRow, TableCell, TableHeaderCell, TableFoot};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            h2 { class: "text-2xl font-bold mb-6 text-center", "Good morning user" }
            Table {
                striped: true,
                hoverable: true,
                caption: rsx! { "Employee Information" },
                TableHead {
                    TableRow {
                        TableHeaderCell { "Name" }
                        TableHeaderCell { "Position" }
                        TableHeaderCell { "Department" }
                        TableHeaderCell { "Salary" }
                    }
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