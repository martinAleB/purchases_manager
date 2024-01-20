This repository allows to register products and purchases of a business, from which it is able to generate the tickets corresponding to each of the purchases and to collect certain information.

## Installation
To install, simply clone the repository.

## Quick start
To run the program, simply execute the following command:
> cargo run <path_to_products_file> <path_to_purchases_file>

Where each line of the product file represents a product available for sale and must be strictly in the following format: 
> <product_id> <product_name> <product_category> <product_price> <product_labels>

* <product_id>: It must be a string. There cannot be two products with the same product_id.
* <product_name>: It must be a string without spaces.
* <product_category>: It must be one of the following (regardless of whether its letters are upper or lower case): Gym, Technology, Tablegames, Videogames or Television
* <product_price>: It must be 32-bit float number.
* <product_labels>: It must be a string without spaces and with each of the tags separated by a comma.

and where each line of the purchase file represents the addition of a product along with its quantity corresponding to a purchase with a given id, and where each line must be strictly in the following format:
> <purchase_id> <product_id> <product_quantity>

* <purchase_id>: It must be a string.
* <product_id>: It must be a string. It must belongs to one of the ids in the product file.
* <product_quantity>: It must be a positive integer.

You can have both files stored anywhere on your computer, or you can create the data directory in the same directory in which the repository was cloned and store them there.

## Results
In a correct execution of the program, the results will be generated in the directory "response" in the same directory in which the repository was cloned, which has the structure represented in the following diagram:

```
|-- response
|   |-- querys
|   |   |-- category_purchases_number.txt
|   |   |-- product_purchases_number.txt
|   |   |-- products_by_label.txt
|   |-- tickets
|   |   |-- all purchase tickets...
```

* In 'response/querys/category_purchases_number.txt' will be stored the total amount of purchases of products belonging to a specific category.
* In 'response/querys/product_purchases_number.txt' will be stored the total amount of purchases for each product. The products will be sorted alphabetically by id.
* In 'response/querys/products_by_label.txt' will be stored all the categories and, consequently, all the products that belong to it.