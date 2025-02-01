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
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    // new
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        Order::validate_product_name(&product_name);
        Order::validate_quantity(&quantity);
        Order::validate_unit_price(&unit_price);

        Order {
            product_name: product_name,
            quantity: quantity,
            unit_price: unit_price,
        }
    }

    // set
    pub fn set_product_name(&mut self, new_name: String) {
        Order::validate_product_name(&new_name);
        self.product_name = new_name;
    }
    pub fn set_quantity(&mut self, new_quantity: u32) {
        Order::validate_quantity(&new_quantity);
        self.quantity = new_quantity;
    }
    pub fn set_unit_price(&mut self, new_unit_price: u32) {
        Order::validate_unit_price(&new_unit_price);
        self.unit_price = new_unit_price;
    }

    // get
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    // validate attributes
    fn validate_product_name(product_name: &String) {
        if product_name.is_empty() || product_name.len() > 300 {
            panic!("invalid product_name");
        }
    }
    fn validate_quantity(quantity: &u32) {
        if quantity == &0 {
            panic!("invalid quantity");
        }
    }
    fn validate_unit_price(unit_price: &u32) {
        if unit_price == &0 {
            panic!("invalid unit_price");
        }
    }

    // total
    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}
