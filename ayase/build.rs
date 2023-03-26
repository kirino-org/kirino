// use std::{fs::File, io::Write};

// use minify_html::Cfg;

// extern crate grass;
// extern crate minify_html;

// fn main() {
//   println!("cargo:rerun-if-changed=assets/style.sass");

//   // Minifies the compiled SASS
//   let css = minify_html::minify(
//     // grass::include! gets its base path from the build dir rather than the current
//     // file, so we have to do this
//     grass::include!("ayase/assets/style.sass").as_bytes(),
//     &Cfg::default(),
//   );

//   let mut f = File::options()
//     .write(true)
//     .create(true)
//     .truncate(true)
//     .open("assets/style.css")
//     .unwrap();
//   f.write(css.as_slice()).unwrap();
// }
fn main() {}