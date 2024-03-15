use std::collections::hash_map::HashMap;
use std::io::stdin;

#[derive(Debug)]
struct CoffeeMachine {
    item: String,
    extra: String,
    price: u16,
}

impl<'a> CoffeeMachine {
    fn new() -> Self {
        Self {
            item: String::new(),
            extra: String::new(),
            price: 0,
        }
    }

    fn list_items(&self, items: &'a HashMap<&'a str, u16>) {
        println!("A következő italok közül választhat:");
        for (extra, price) in items {
            println!("{}: {}Ft", extra, price);
        }
    }

    fn list_extras(&self, extras: &'a HashMap<&'a str, u16>) -> Vec<String> {
        let mut allowed_extras: Vec<String> = Vec::new();
        match self.item.as_str() {
            "tea" => {
                println!("A teát választottad! Most válassz egy extrát:");
                for (extra, price) in extras {
                    if extra != &"tejszín" {
                        println!("{}: {}Ft", extra, price);
                        allowed_extras.push(extra.to_string());
                    }
                }
            }
            "kávé" => {
                println!("A kávét választottad! Most válassz egy extrát:");
                for (extra, price) in extras {
                    if extra != &"citrom" {
                        println!("{}: {}Ft", extra, price);
                        allowed_extras.push(extra.to_string());
                    }
                }
            }
            "forró csoki" => {
                println!("A forró csokit választottad! Most válassz egy extrát:");
                for (extra, price) in extras {
                    if extra != &"tejszín" && extra != &"citrom" {
                        println!("{}: {}Ft", extra, price);
                        allowed_extras.push(extra.to_string());
                    }
                }
            }
            _ => println!("Na ezt vajon hogy sikerült? {:?}", self.item.as_str()),
        }
        allowed_extras
    }

    fn choose_item(&mut self, items: &'a HashMap<&'a str, u16>) {
        let mut item = String::new();
        stdin()
            .read_line(&mut item)
            .expect("Nem lehet ennyire elszúrni");

        item = String::from(item.as_str().lines().next().unwrap());

        while !items.contains_key(item.as_str()) {
            println!(
                "{} nevű ital nem létezik. Kérlek, válassz a fenti listából!",
                item
            );
            item = String::new();
            stdin()
                .read_line(&mut item)
                .expect("Nem lehet ennyire elszúrni");
            item = String::from(item.as_str().lines().next().unwrap());
        }

        self.price += items[item.as_str()];
        self.item = item;
    }

    fn choose_extra(&mut self, extras: &'a HashMap<&'a str, u16>, allowed_extras: Vec<String>) {
        let mut extra = String::new();
        stdin()
            .read_line(&mut extra)
            .expect("Nem lehet ennyire elszúrni");

        extra = String::from(extra.as_str().lines().next().unwrap());

        while !extras.contains_key(extra.as_str()) || !allowed_extras.contains(&extra) {
            println!(
                "{} nevű extra nem létezik, vagy ehhez az italhoz nem elérhető. Kérlek, válassz a fenti listából!",
                extra
            );
            extra = String::new();
            stdin()
                .read_line(&mut extra)
                .expect("Nem lehet ennyire elszúrni");
            extra = String::from(extra.as_str().lines().next().unwrap());
        }

        self.price += extras[extra.as_str()];
        self.extra = extra;
    }
}

fn main() {
    let mut coffeemachine1 = CoffeeMachine::new();

    let mut items = HashMap::new();
    items.insert("forró csoki", 300);
    items.insert("tea", 200);
    items.insert("kávé", 150);

    let mut extras = HashMap::new();
    extras.insert("tej", 50);
    extras.insert("tejszín", 100);
    extras.insert("cukor", 50);
    extras.insert("citrom", 100);

    coffeemachine1.list_items(&items);

    coffeemachine1.choose_item(&items);

    let allowed_extras = coffeemachine1.list_extras(&extras);

    coffeemachine1.choose_extra(&extras, allowed_extras);

    println!("{:?}", coffeemachine1);
}
