#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Item {
    pub first_long_name_a: u16,
    pub item_count: u16,
    pub area: u16,
    pub address: u16,
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::Item;

    proptest! {
        #[test]
        fn read_item_chars(s in "\\PC*") {
            let pi = Item {
                 first_long_name_a: 0,item_count: 0,area: 0,address: 0
            };
        }
        }
}
