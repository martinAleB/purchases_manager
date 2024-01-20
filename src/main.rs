use std::env;
use std::fs;

mod manager;
mod products;
mod purchases;

use manager::PurchasesManager;
use products::Product;

fn read_products_file(system: &mut PurchasesManager, products_file: String) {
    let content = fs::read_to_string(products_file)
        .expect("products file does not exist or permissions required to read it");
    let mut split = content.split('\n');
    while let Some(line) = split.next() {
        let product = Product::from(line.to_string());
        if !product.is_empty() {
            system.add_product(product);
        }
    }
}

fn read_purchase_line(line: &str) -> Option<(String, String, u32)> {
    let mut split = line.split(' ');
    let purchase_id = match split.next() {
        None => return None,
        Some(id) => id.to_string(),
    };
    let product_id = match split.next() {
        None => return None,
        Some(id) => id.to_string(),
    };
    let quantity = match split.next() {
        None => return None,
        Some(quantity) => match quantity.parse::<u32>() {
            Err(_) => return None,
            Ok(id) => id,
        },
    };
    Some((purchase_id, product_id, quantity))
}

fn read_purchases_file(system: &mut PurchasesManager, purchases_file: String) {
    let content = fs::read_to_string(purchases_file)
        .expect("purchases file does not exist or permissions required to read it");
    let mut split = content.split('\n');
    while let Some(line) = split.next() {
        let line = read_purchase_line(line);
        if let Some((purchase_id, product_id, quantity)) = line {
            system.add_product_to_purchase(purchase_id, product_id, quantity);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        panic!(
            "Error: {} arguments were received when exactly 2 should be received: the path to the products file and the path to the purchases file.",
            args.len() - 1
        );
    }
    let products_file = args[1].to_string();
    let purchases_file = args[2].to_string();
    let mut products_system = PurchasesManager::new();
    read_products_file(&mut products_system, products_file);
    read_purchases_file(&mut products_system, purchases_file);
    products_system.generate_products_by_label();
    products_system.generate_products_purchases_number();
    products_system.generate_category_purchases_number();
    products_system.generate_all_tickets();
    println!("Finished!");
}
