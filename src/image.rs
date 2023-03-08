pub struct ImgProc {}
impl ImgProc {
  pub fn run(&self) { image::open("").unwrap().re }
}

pub struct ThumbProc {
  thumbs: Vec<String>,
}
impl ThumbProc {
  pub fn run(&self) { self.thumbs.iter() }
}
