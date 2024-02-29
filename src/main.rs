use prost_example::snazzy::items;

fn main() {
    println!(
        "{:?}",
        create_shirt("red".to_string(), items::shirt::Size::Medium)
    );
}

fn create_shirt(color: String, size: items::shirt::Size) -> items::Shirt {
    items::Shirt {
        color,
        size: size.into(),
        ..Default::default()
    }
}
