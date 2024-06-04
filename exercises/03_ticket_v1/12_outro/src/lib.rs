// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    pub product_name: String,
    pub quantity: u32,
    pub unit_price: u32,
}

fn is_product_name_valid(product_name: &String) {
    if product_name.is_empty() {
        panic!("Product name cannot be empty");
    }
    if product_name.len() > 300 {
        panic!("Product name cannot be longer than 300 bytes");
    }
}

fn is_quantity_valid(quantity: &u32) {
    if quantity == &0 {
        panic!("Quantity must be greater than zero");
    }
}

fn is_unit_price_valid(unit_price: &u32) {
    if unit_price == &0 {
        panic!("Unit price must be greater than zero");
    }
}

impl Order {
    // Getters
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }
    pub fn total(&self) -> u32 {
        &self.quantity * &self.unit_price
    }

    // Setters
    pub fn set_product_name(&mut self, new_name: String) {
        is_product_name_valid(&new_name);
        self.product_name = new_name;
    }
    pub fn set_quantity(&mut self, new_quantity: u32) {
        is_quantity_valid(&new_quantity);
        self.quantity = new_quantity;
    }
    pub fn set_unit_price(&mut self, new_unit_price: u32) {
        is_unit_price_valid(&new_unit_price);
        self.unit_price = new_unit_price;
    }

    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        is_product_name_valid(&product_name);
        is_quantity_valid(&quantity);
        is_unit_price_valid(&unit_price);
        Order {
            product_name,
            quantity,
            unit_price,
        }
    }
}
