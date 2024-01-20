use std::fs;
use std::sync::Arc;
use std::{collections::BTreeMap, collections::HashMap, io::Write};

use crate::{products::Product, products::ProductCategory, purchases::Purchase};

/**
 * Saves the products, purchases and the necessary information
 * to generate the files with the corresponding requests.
 */
pub struct PurchasesManager {
    products: Vec<Arc<Product>>,
    product_purchases_number: BTreeMap<Arc<Product>, u32>,
    products_by_label: HashMap<String, Vec<Arc<Product>>>,
    category_purchases_number: HashMap<ProductCategory, u32>,
    purchases: HashMap<String, Purchase>,
}

/**
 * Functions for the creation of files and directories
 * and error information related to them.
 */
impl PurchasesManager {
    fn generate_ticket_filename(id: &String) -> String {
        format!("ticket_{}.txt", id)
    }

    fn file_system_error(kind: &str, error_with: &str, path: &str) -> String {
        format!("error while {} '{}' {}", kind, path, error_with)
    }

    fn create_dir(dir_path: &str) {
        fs::create_dir_all(dir_path).expect(&PurchasesManager::file_system_error(
            "creating", dir_path, "dir",
        ));
    }

    fn create_file(dir_path: &str, filename: &str) -> fs::File {
        fs::File::create(format!("{}/{}", dir_path, filename))
            .expect(PurchasesManager::file_system_error("creating", filename, "file").as_str())
    }
}

/**
 * Auxiliary functions for the processing
 * of the corresponding requests.
 */
impl PurchasesManager {
    fn find_product_by_id(&self, id: &String) -> Option<Arc<Product>> {
        for product in self.products.iter() {
            if (*id).eq(product.get_id()) {
                return Some(product.clone());
            }
        }
        None
    }

    fn increment_product_purchases_number(&mut self, product: Arc<Product>, units: u32) {
        let v = self.product_purchases_number.entry(product).or_insert(0);
        *v += units;
    }

    fn increment_category_purchases_number(&mut self, category: ProductCategory, units: u32) {
        let v = self.category_purchases_number.entry(category).or_insert(0);
        *v += units;
    }
}

/**
 * Initialization and receiving functions of information
 * coming from product and purchases files.
 */
impl PurchasesManager {
    pub fn new() -> Self {
        PurchasesManager {
            products: Vec::new(),
            product_purchases_number: BTreeMap::new(),
            products_by_label: HashMap::new(),
            category_purchases_number: HashMap::new(),
            purchases: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let prod_ref = Arc::new(product);
        self.products.push(prod_ref.clone());
        for label in prod_ref.get_labels().iter() {
            self.products_by_label
                .entry(label.clone())
                .or_insert(Vec::new())
                .push(prod_ref.clone());
        }
    }

    pub fn add_product_to_purchase(
        &mut self,
        purchase_id: String,
        product_id: String,
        quantity: u32,
    ) {
        let product = self.find_product_by_id(&product_id);
        if let Some(product) = product {
            self.purchases
                .entry(purchase_id.clone())
                .or_insert(Purchase::new(purchase_id))
                .add_product(product.clone(), quantity);
            self.increment_product_purchases_number(product.clone(), quantity);
            self.increment_category_purchases_number(product.get_category(), quantity);
        } else {
            println!("not found product with id {}", product_id);
        }
    }
}

/**
 * Functions designed for the management of the corresponding files.
 */
const SEPARATOR: &str = "================================\n";
const PRODUCTS_BY_LABEL_FILE: &str = "products_by_label.txt";
const PRODUCT_PURCHASES_NUMBER_FILE: &str = "product_purchases_number.txt";
const CATEGORY_PURCHASES_NUMBER_FILE: &str = "category_purchases_number.txt";
const QUERYS_DIR: &str = "response/querys";
const TICKETS_DIR: &str = "response/tickets";
impl PurchasesManager {
    pub fn generate_ticket(&self, purchase_id: &String) {
        let purchase = self.purchases.get(purchase_id);
        if let Some(purchase) = purchase {
            PurchasesManager::create_dir(TICKETS_DIR);
            let filename = PurchasesManager::generate_ticket_filename(purchase_id);
            let mut file = PurchasesManager::create_file(TICKETS_DIR, filename.as_str());
            let mut file_string = String::new();
            file_string.push_str(SEPARATOR);
            file_string
                .push_str(format!("TICKET FOR PURCHASE ID {}\n", purchase.get_id()).as_str());
            file_string.push_str(SEPARATOR);
            file_string.push_str(
                format!("{0: <15}{1: <10}{2: <15}\n", "PRODUCT", "QUANTITY", "PRICE").as_str(),
            );
            for product_purchase in purchase.get_cart().iter() {
                let product = product_purchase.get_product();
                file_string.push_str(
                    format!(
                        "{0: <15}{1: <10}{2: <15}\n",
                        product.get_name(),
                        product_purchase.get_quantity(),
                        product.get_price()
                    )
                    .as_str(),
                );
            }
            file_string.push_str(SEPARATOR);
            file_string
                .push_str(format!("{0: <25}{1: <15}\n", "TOTAL", purchase.total_price()).as_str());
            file_string.push_str(SEPARATOR);
            file.write(file_string.as_bytes())
                .expect(PurchasesManager::file_system_error("writing", &filename, "file").as_str());
        } else {
            println!("not found purchase with id {}", purchase_id);
        }
    }

    pub fn generate_all_tickets(&self) {
        for (purchase_id, _) in self.purchases.iter() {
            self.generate_ticket(purchase_id);
        }
    }

    pub fn generate_products_by_label(&self) {
        PurchasesManager::create_dir(QUERYS_DIR);
        let mut file = PurchasesManager::create_file(QUERYS_DIR, PRODUCTS_BY_LABEL_FILE);
        let mut file_string = String::new();
        file_string.push_str(SEPARATOR);
        for (label, products) in self.products_by_label.iter() {
            file_string.push_str(format!("LABEL '{}'\n", label).as_str());
            for product in products {
                file_string.push_str(format!("{}\n", product.to_string()).as_str());
            }
            file_string.push_str(SEPARATOR);
        }

        file.write(file_string.as_bytes()).expect(
            format!(
                "error while writing {}/{}",
                QUERYS_DIR, PRODUCTS_BY_LABEL_FILE
            )
            .as_str(),
        );
    }

    pub fn generate_products_purchases_number(&self) {
        PurchasesManager::create_dir(QUERYS_DIR);
        let mut file = PurchasesManager::create_file(QUERYS_DIR, PRODUCT_PURCHASES_NUMBER_FILE);
        let mut file_string = String::new();
        for (product, quantity) in self.product_purchases_number.iter() {
            file_string
                .push_str(format!("{} bought {} units\n", product.to_string(), quantity).as_str());
        }
        file.write(file_string.as_bytes()).expect(
            format!(
                "error while writing {}/{}",
                QUERYS_DIR, PRODUCT_PURCHASES_NUMBER_FILE
            )
            .as_str(),
        );
    }

    pub fn generate_category_purchases_number(&self) {
        PurchasesManager::create_dir(QUERYS_DIR);
        let mut file = PurchasesManager::create_file(QUERYS_DIR, CATEGORY_PURCHASES_NUMBER_FILE);
        let mut file_string = String::new();
        for (category, quantity) in self.category_purchases_number.iter() {
            file_string.push_str(format!("{:?} bought {} units\n", category, quantity).as_str());
        }
        file.write(file_string.as_bytes()).expect(
            format!(
                "error while writing {}/{}",
                QUERYS_DIR, CATEGORY_PURCHASES_NUMBER_FILE
            )
            .as_str(),
        );
    }
}
