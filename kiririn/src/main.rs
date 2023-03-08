use kirino_proto::{Item, Message};

fn main() {
  let mut items: Vec<Item> = Vec::new();

  for (id, name) in &[
    ("07", "A few ways to not really clean a record"),
    ("04", "MiniDisc - An Appreciation"),
    ("01", "Brown; color is weird"),
    (
      "02",
      "Flexplay: The Disposable DVD that Failed (Thankfully)",
    ),
    (
      "05",
      "The horrible truth about Apple's repeated engineering failures.",
    ),
    (
      "03",
      "The LED Traffic Light and the Danger of \"But Sometimes!\"",
    ),
    (
      "06",
      "Clinton the cat turns 12 and he's as controversial as ever.",
    ),
    ("08", "[Fan MV] Mili - Nine Point Eight"),
  ] {
    items.push(Item {
      id: id.to_string(),
      name: name.to_string(),
      ..Default::default()
    });
  }

  std::fs::write(
    "archive.pb",
    kirino_proto::Archive { items }.encode_to_vec(),
  )
  .unwrap();
}
