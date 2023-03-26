use handlebars::handlebars_helper;

#[macro_export]
handlebars_helper!(hbs_i18n: |key: String| {
    i18n(key)
});

fn i18n(key: String) -> &'static str {
  match key.as_str() {
        "account" => "Account",
        "username" => "Username",
        "password" => "Password",

        "welcome-title" => "Thanks for installing Kirino!",
        "welcome-text" => "Kirino is in a highly experimental stage. Your contributions, suggestions, testing, and bug reports are greatly appreciated.",
        "welcome-lets-go" => "Let's get your server set up!",
        "welcome-ok" => "OK",

        &_ => unimplemented!()
    }
}
