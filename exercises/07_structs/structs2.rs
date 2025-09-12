#[derive(Debug)]
struct Order {
    pub name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();

        // TODO: Create your own order using the update syntax and template above!
        let my_order = Order {
            name: String::from("Hacker in Rust"),
            count: 1,
            ..order_template
        };

        assert_eq!(my_order.name, "Hacker in Rust");
        assert_eq!(my_order.year, order_template.year);
        assert_eq!(my_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(my_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(my_order.made_by_email, order_template.made_by_email);
        assert_eq!(my_order.item_number, order_template.item_number);
        assert_eq!(my_order.count, 1);
    }
}
