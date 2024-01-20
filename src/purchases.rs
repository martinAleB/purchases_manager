use std::sync::Arc;

use crate::products::Product;

pub struct ProductPurchase {
    product: Arc<Product>,
    quantity: u32,
}

impl ProductPurchase {
    pub fn get_product(&self) -> Arc<Product> {
        self.product.clone()
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }
}

pub struct Purchase {
    id: String,
    cart: Vec<ProductPurchase>,
}

impl Purchase {
    pub fn new(id: String) -> Self {
        Purchase {
            id,
            cart: Vec::new(),
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_cart(&self) -> &Vec<ProductPurchase> {
        &self.cart
    }

    pub fn add_product(&mut self, product: Arc<Product>, quantity: u32) {
        self.cart.push(ProductPurchase { product, quantity });
    }

    pub fn total_price(&self) -> f32 {
        let mut price: f32 = 0.0;
        for purchase in self.cart.iter() {
            price += purchase.product.get_price() * purchase.quantity as f32;
        }
        price
    }
}
