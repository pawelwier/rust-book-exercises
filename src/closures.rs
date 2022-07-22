pub mod shirts {
    #[derive(Debug, PartialEq)]
    pub enum ShirtColor {
        Yellow,
        Black
    }

    #[derive(Debug)]
    pub struct Inventory {
        pub shirts: Vec<ShirtColor>
    }

    fn get_color_count(shirt_color: ShirtColor, inventory: &Inventory) -> usize {
        inventory.shirts.iter().filter(|&color| *color == shirt_color).count()
    }

    fn get_shirt_color(shirt_color: Option<ShirtColor>, inventory: Inventory) -> ShirtColor {
        match shirt_color {
            Some(color) => color,
            None => {
                let black_count = get_color_count(ShirtColor::Black, &inventory);
                let yellow_count = get_color_count(ShirtColor::Yellow, &inventory);
                return if black_count > yellow_count { ShirtColor::Black } else { ShirtColor::Yellow }
            }
        }
    }

    fn get_sample_inventory() -> Inventory {
        Inventory { shirts: vec![ShirtColor::Yellow, ShirtColor::Yellow, ShirtColor::Black, ShirtColor::Yellow, ShirtColor::Black]}
    }

    pub fn get_shirt_color_with_pref() -> ShirtColor {
        let inventory = get_sample_inventory();
        get_shirt_color(Some(ShirtColor::Black), inventory)
    }
    
    pub fn get_shirt_color_without_pref() -> ShirtColor {
        let inventory = get_sample_inventory();
        get_shirt_color(None, inventory)
    }
}