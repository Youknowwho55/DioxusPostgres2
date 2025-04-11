fn App() -> Element {
    let username = use_signal(|| "".to_string());
    let password = use_signal(|| "".to_string());
    
    rsx! {
        form {
            TextInput {
                i_value: username.get().clone(),
                i_placeholder: Some("Username".to_string()),
                on_input: move |evt| username.set(evt.value.clone()),
                class: None,
            }
            PasswordInput {
                i_value: password.get().clone(),
                i_placeholder: Some("Password".to_string()),
                on_input: move |evt| password.set(evt.value.clone()),
                class: None,
            }
            button { "Submit" }
        }
    }
}