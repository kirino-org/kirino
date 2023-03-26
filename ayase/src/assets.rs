pub const STYLESHEET: &'static [u8] = include_bytes!("../assets/style.css");
pub const FALLING_SKY: &'static [u8] = include_bytes!("../assets/fallingsky.otf");

use handlebars::handlebars_helper;

macro_rules! icons {
        ( $( $name:ident ),* ) => {
            fn icon(name: String) -> &'static str {
                match name.as_str() {
                $(
                    stringify!($name) => include_str!(concat!("../assets/icons/", stringify!($name), ".svg")),
                )*
                    &_ => unimplemented!()
                }
            }
        };
    }

icons! {
    heart,
    xml,
    cog,
    arrow_right,
    language_rust,

    television,
    music,
    movie,

    play,
    skip_next,
    skip_previous
}

#[macro_export]
handlebars_helper!(hbs_icon: |name: String| {
    icon(name)
});
