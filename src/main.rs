use std::env;

struct Room {
    storage_places: Vec<StoragePlace>,
}

impl Room {
    fn add_storage_place(&mut self, name: String) {
        let storage_place = StoragePlace {name, items: Vec::new()};
        self.storage_places.push(storage_place);
    }

    fn addItem(&mut self, item_name: String, storage_name: String) {
        let item = Item{name: item_name};
        let storage_place = self.storage_places.iter().find(|&s| s.name.to_string() == storage_name);
        match storage_place {
            Some (x) => x.items.push(item),
        }

    }

    fn print_items(&self) {
        for storage_place in self.storage_places.iter() {
            for item in storage_place.items.iter() {
                println!("{} ({})", item.name, storage_place.name);
            }
        }
    }
}

struct StoragePlace {
    name: String,
    items: Vec<Item>,
}

struct Item {
    name: String,
}

enum Command {
    Items,
    AddRoom(String),
}

fn main() {
    let mut room = Room{storage_places: Vec::new()};
    room.add_storage_place("living room".to_string());

    let arguments: Vec<String> = env::args().collect();
    let command = match arguments[1].as_str() {
        "items" => Command::Items,
        "add" => Command::AddRoom(arguments[2].clone()),
        _ => panic!("Wrong command"),
    };

    match command {
        Command::Items => room.print_items(),
        Command::AddRoom(room_name) => {
            room.add_storage_place(room_name);
        }
    }
}
