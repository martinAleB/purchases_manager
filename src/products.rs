use std::hash::Hash;

#[derive(Clone, Debug, Hash, Copy, PartialEq, Eq)]
pub enum ProductCategory {
    None,
    Gym,
    Technology,
    Tablegames,
    Videogames,
    Television,
    Tables,
}

pub struct Product {
    id: String,
    name: String,
    category: ProductCategory,
    price: f32,
    labels: Vec<String>,
}

impl Product {
    pub fn new() -> Self {
        Product {
            id: String::from(""),
            name: String::from(""),
            category: ProductCategory::None,
            price: 0.0,
            labels: vec![],
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_category(&self) -> ProductCategory {
        self.category
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    pub fn get_labels(&self) -> &Vec<String> {
        &self.labels
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
}

impl Hash for Product {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Product {}

impl Ord for Product {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Product {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl From<String> for Product {
    fn from(value: String) -> Self {
        let mut split = value.split(' ');
        let id = match split.next() {
            None => return Product::new(),
            Some(id) => id.to_string(),
        };
        let name = match split.next() {
            None => return Product::new(),
            Some(name) => name.to_string(),
        };
        let category = match split.next() {
            None => return Product::new(),
            Some(str_category) => match str_category.to_lowercase().as_str() {
                "gym" => ProductCategory::Gym,
                "technology" => ProductCategory::Technology,
                "tablegames" => ProductCategory::Tablegames,
                "videogames" => ProductCategory::Videogames,
                "television" => ProductCategory::Television,
                "tables" => ProductCategory::Tables,
                _ => ProductCategory::None,
            },
        };
        let price = match split.next() {
            None => return Product::new(),
            Some(price) => match price.parse::<f32>() {
                Err(_) => return Product::new(),
                Ok(price) => price,
            },
        };
        let mut labels_vec: Vec<String> = Vec::new();
        let _ = match split.next() {
            None => None,
            Some(labels) => {
                let mut labels = labels.split(',');
                while let Some(label) = labels.next() {
                    labels_vec.push(label.to_lowercase());
                }
                Some("ok")
            }
        };
        Product {
            id,
            name,
            category,
            price,
            labels: labels_vec,
        }
    }
}

impl Clone for Product {
    fn clone(&self) -> Product {
        Product {
            id: self.id.clone(),
            name: self.name.clone(),
            category: self.category.clone(),
            price: self.price,
            labels: self.labels.clone(),
        }
    }
}

impl ToString for Product {
    fn to_string(&self) -> String {
        format!(
            "Product with id: {}, name: {}, category: {:?} and price: ${}",
            self.id, self.name, self.category, self.price
        )
    }
}
