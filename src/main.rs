use std::env;

struct Room {
    storagePlaces: Vec<StoragePlace>,
}

struct StoragePlace {
    name: String,
    description: Option<String>,
    capacity: u32,
    items: Vec<Item>,
}

struct Item {
    name: String,
    description: Option<String>,
    storagePlace: String,
    importance: String,
    status: String,
}

enum Command {
    items,
    rooms,
}

fn main() {
    let arguments: Vec<String> = env:args().collect();
    let command = arguments[1].clone();

    // testing data:
    let room = Room{storagePlaces: Vec::new()};

    let bookshelf = StoragePlace {
        name: "bookshelf",
        capacity: 20,
        items: Vec::new(),
        description: None,
    };

    let item = Item {
        name: "cv",
        description: None,
        importance: "low",
        status: "New",
        storagePlace: bookshelf,
    };


}
