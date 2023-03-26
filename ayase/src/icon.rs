pub const COMPUTER: &str = "🖥";
pub const IMAGE: &str = "🖼";
pub const MUSIC_NOTE: &str = "🎵";
pub const TELEVISION: &str = "📺";
// actually tape drive, but whatever
pub const MOVIE_TAPE: &str = "✇";
pub const MOVIE_CAMERA: &str = "🎥";
pub const TAPE_CARTRIDGE: &str = "🖭";
pub const PLUG: &str = "🔌";
pub const ADD: &str = "+";
pub const BOOK: &str = "📖";
pub const HEART: &str = "♥";
pub const GITHUB: &str = "";
pub const BUG: &str = "🐛";
pub const COG: &str = "⚙";
pub const BOOKS: &str = "📚";
pub const SMILE: &str = "☺";
pub const SEARCH: &str = "🔍";

pub const PLAY: &str = "⏵";
pub const PREV: &str = "⏪";
pub const NEXT: &str = "⏩";

#[macro_export]
macro_rules! icon {
  ($icon:ident, $text:literal) => {
    format!("{} {}", icon::$icon, $text)
  };
}
