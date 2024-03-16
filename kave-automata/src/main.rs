use std::collections::hash_map::HashMap;
use std::io::stdin;

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
        println!("A következő italok közül választhatsz:");
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

    fn pay_for_drink(&mut self, coins: &'a Vec<u8>) {
        println!("A fizetendő összeg {}Ft", self.price);
        let mut amount_paid: u16 = 0;
        let mut accepted_coins_string = String::from("Dobj be egy érmét a következőek közül:");
        for coin in coins {
            accepted_coins_string = format!("{} {}Ft", accepted_coins_string, coin);
        }
        while self.price > amount_paid {
            println!("{}", accepted_coins_string);
            let mut coin_input = String::new();
            stdin()
                .read_line(&mut coin_input)
                .expect("Nem lehet ennyire elszúrni");
            let mut coin: u8 = coin_input.lines().next().unwrap().parse().unwrap();
            while !coins.contains(&coin) {
                println!("Kérlek, egy elfogadott érmét dobj be!");
                coin_input = String::new();
                stdin()
                    .read_line(&mut coin_input)
                    .expect("Nem lehet ennyire elszúrni");
                coin = coin_input.lines().next().unwrap().parse().unwrap();
            }

            amount_paid += coin as u16;

            if self.price > amount_paid {
                println!("Eddig {} forintot fizettél ki.", amount_paid);
            } else {
                let mut change = amount_paid - self.price;
                if change == 0 {
                    println!(
                        "Kifizetted a teljes összeget. Nincs visszajáró. Köszönjük a vásárlást!"
                    );
                } else {
                    let mut change_coins = vec![0; coins.len()];
                    for (index, coin) in coins.iter().enumerate() {
                        change_coins[index] = change / *coin as u16;
                        change = change % *coin as u16;
                    }
                    let mut change_string =
                        String::from("Kifizetted a teljes összeget. A visszajáród");
                    for (index, coin) in change_coins.iter().enumerate() {
                        if index != coins.len() - 1 {
                            change_string =
                                format!("{} {}x{}Ft,", change_string, coin, coins[index]);
                        } else {
                            change_string =
                                format!("{} {}x{}Ft.", change_string, coin, coins[index]);
                        }
                    }

                    println!("{}", change_string);
                }
            }
        }
    }

    fn main(
        &mut self,
        items: &'a HashMap<&'a str, u16>,
        extras: &'a HashMap<&'a str, u16>,
        coins: &'a Vec<u8>,
    ) {
        self.list_items(items);
        self.choose_item(items);
        let allowed_extras = self.list_extras(extras);
        self.choose_extra(extras, allowed_extras);
        self.pay_for_drink(coins);
    }
}

fn main() {
    println!("Üdvözöllek. (V)ásárolsz, vagy (k)ilépsz? ");
    let mut choice = String::new();
    stdin()
        .read_line(&mut choice)
        .expect("Ennyire nem lehet elszúrni");
    choice = choice.lines().next().unwrap().to_lowercase();

    while choice == "v" {
        let mut items = HashMap::new();
        items.insert("forró csoki", 300);
        items.insert("tea", 200);
        items.insert("kávé", 150);

        let mut extras = HashMap::new();
        extras.insert("tej", 50);
        extras.insert("tejszín", 100);
        extras.insert("cukor", 50);
        extras.insert("citrom", 100);

        let coins: Vec<u8> = vec![200, 100, 50, 20, 10];

        let mut coffeemachine = CoffeeMachine::new();

        coffeemachine.main(&items, &extras, &coins);

        println!("Üdvözöllek. (V)ásárolsz, vagy (k)ilépsz? ");
        choice = String::new();
        stdin()
            .read_line(&mut choice)
            .expect("Ennyire nem lehet elszúrni");
        choice = choice.lines().next().unwrap().to_lowercase();
    }
}
