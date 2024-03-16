use std::cmp;
use std::fmt::{self, Display};

const AGED_BRIE_ITEM: &str = "Aged Brie";
const BACKSTAGE_PASSES_ITEM: &str = "Backstage passes to a TAFKAL80ETC concert";
const COMMON_ITEM: &str = "Elixir of the Mongoose";
const CONJURED_ITEM: &str = "Conjured Mana Cake";
const LEGENDARY_ITEM: &str = "Sulfuras, Hand of Ragnaros";

const MAXIMUM_ALLOWED_QUALITY: i32 = 50;
const MINIMUM_ALLOWED_QUALITY: i32 = 0;


pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}


pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {

            if item.name == LEGENDARY_ITEM {
                continue;
            }

            item.quality = match item.name.as_str() {
                AGED_BRIE_ITEM => Self::quality_increasing_update_strategy(item),
                BACKSTAGE_PASSES_ITEM => Self::backstage_passes_update_strategy(item),
                CONJURED_ITEM => Self::faster_degrading_update_strategy(item),
                _ => Self::default_update_strategy(item)
            };

            item.sell_in -= 1;
        }
    }

    fn quality_increasing_update_strategy(item: &Item) -> i32 {
        let quality_adjustment =
            if item.sell_in <= 0 {
                2
            } else {
                1
            };
        return Self::get_updated_quality_within_bounds(item, quality_adjustment);
    }

    fn backstage_passes_update_strategy(item: &Item) -> i32 {
        if item.sell_in <= 0 {
             return 0
        }

        let quality_adjustment =
            if item.sell_in <= 5 {
                3
            } else if item.sell_in <= 10 {
                2
            } else {
                1
            };
        return Self::get_updated_quality_within_bounds(item, quality_adjustment);
    }

    fn faster_degrading_update_strategy(item: &Item) -> i32 {
        let quality_adjustment =
            if item.sell_in <= 0 {
                -4
            } else {
                -2
            };
        return Self::get_updated_quality_within_bounds(item, quality_adjustment);
    }

    fn default_update_strategy(item: &Item) -> i32 {
        let quality_adjustment =
            if item.sell_in <= 0 {
                -2
            } else {
                -1
            };
        return Self::get_updated_quality_within_bounds(item, quality_adjustment);
    }

    fn get_updated_quality_within_bounds(item: &Item, adjust_by: i32) -> i32 {
        let new_quality = item.quality + adjust_by;
        cmp::max(
            cmp::min(
                new_quality,
                MAXIMUM_ALLOWED_QUALITY
            ),
            MINIMUM_ALLOWED_QUALITY
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item, COMMON_ITEM, CONJURED_ITEM, LEGENDARY_ITEM, AGED_BRIE_ITEM, BACKSTAGE_PASSES_ITEM};

    mod regression_test_suite {
        use gildedrose::tests::{COMMON_ITEM, LEGENDARY_ITEM, AGED_BRIE_ITEM, BACKSTAGE_PASSES_ITEM};
        use super::{GildedRose, Item};

        #[test]
        fn test_common_items() {
            // GIVEN common items
            let items = vec![
                Item::new(COMMON_ITEM, 10, 20),
                Item::new(COMMON_ITEM, 5, 7),
            ];
            let mut rose = GildedRose::new(items);

            // WHEN updating quality
            rose.update_quality();

            // THEN name should stay the same and quality should decrease
            assert_eq!(COMMON_ITEM, rose.items[0].name);
            assert_eq!(9, rose.items[0].sell_in);
            assert_eq!(19, rose.items[0].quality);

            assert_eq!(6, rose.items[1].quality);

            // ...and WHEN update quality 4 more times
            for _ in 1..=4 {
                rose.update_quality();
            }

            // THEN quality should continue to degrade
            assert_eq!(5, rose.items[0].sell_in);
            assert_eq!(15, rose.items[0].quality);

            assert_eq!(0, rose.items[1].sell_in);
            assert_eq!(2, rose.items[1].quality);

            // ...and WHEN updating quality again so sell in becomes negative
            rose.update_quality();

            // THEN sell in should become negative and quality should degrade twice as fast
            assert_eq!(-1, rose.items[1].sell_in);
            assert_eq!(0, rose.items[1].quality);

            // ...and WHEN updating quality when quality is 0
            rose.update_quality();

            // THEN quality should not become negative
            assert_eq!(-2, rose.items[1].sell_in);
            assert_eq!(0, rose.items[1].quality);
        }

        #[test]
        fn test_legendary_items() {
            // GIVEN legendary items of different sell in
            let items = vec![
                Item::new(LEGENDARY_ITEM, 1, 80),
                Item::new(LEGENDARY_ITEM, 0, 80),
                Item::new(LEGENDARY_ITEM, -1, 80)
            ];
            let mut rose = GildedRose::new(items);

            // WHEN updating quality
            rose.update_quality();

            // THEN nothing should change
            assert_eq!(LEGENDARY_ITEM, rose.items[0].name);
            assert_eq!(1, rose.items[0].sell_in);
            assert_eq!(80, rose.items[0].quality);

            assert_eq!(0, rose.items[1].sell_in);
            assert_eq!(80, rose.items[1].quality);

            assert_eq!(-1, rose.items[2].sell_in);
            assert_eq!(80, rose.items[2].quality);

            // ...and WHEN updating the quality many times
            for _ in 1..=999 {
                rose.update_quality();
            }

            // THEN nothing should change
            assert_eq!(1, rose.items[0].sell_in);
            assert_eq!(80, rose.items[0].quality);

            assert_eq!(0, rose.items[1].sell_in);
            assert_eq!(80, rose.items[1].quality);

            assert_eq!(-1, rose.items[2].sell_in);
            assert_eq!(80, rose.items[2].quality);
        }

        #[test]
        fn test_quality_increasing_items() {
            // GIVEN items of which quality increases when it gets older
            let items = vec![
                Item::new(AGED_BRIE_ITEM, 2, 0),
                Item::new(AGED_BRIE_ITEM, 4, 49),
            ];
            let mut rose = GildedRose::new(items);

            // WHEN updating quality
            rose.update_quality();

            // THEN quality should increase
            assert_eq!(1, rose.items[0].sell_in);
            assert_eq!(1, rose.items[0].quality);

            assert_eq!(50, rose.items[1].quality);

            // ...and WHEN updating quality again
            rose.update_quality();

            // THEN quality should increase but be limited by 50
            assert_eq!(0, rose.items[0].sell_in);
            assert_eq!(2, rose.items[0].quality);

            assert_eq!(50, rose.items[1].quality);

            // ...and WHEN updating quality again so sell in becomes negative
            rose.update_quality();

            // THEN quality should increase by 2
            assert_eq!(-1, rose.items[0].sell_in);
            assert_eq!(4, rose.items[0].quality);

            // ...and WHEN updating quality again when sell in is already negative
            rose.update_quality();

            // THEN quality should continue to increase by 2
            assert_eq!(6, rose.items[0].quality);
        }

        #[test]
        fn test_quality_zero_after_sell_in_items() {
            // GIVEN items of which quality increases more towards sell-in date and becomes 0 after
            let items = vec![
                Item::new(BACKSTAGE_PASSES_ITEM, 15, 20),
                Item::new(BACKSTAGE_PASSES_ITEM, 10, 0),
                Item::new(BACKSTAGE_PASSES_ITEM, 5, 48),
            ];
            let mut rose = GildedRose::new(items);

            // WHEN updating quality
            rose.update_quality();

            // THEN quality should increase
            assert_eq!(14, rose.items[0].sell_in);
            assert_eq!(21, rose.items[0].quality);

            assert_eq!(9, rose.items[1].sell_in);
            assert_eq!(2, rose.items[1].quality); // increase by 2 when <= 10 days left

            assert_eq!(4, rose.items[2].sell_in);
            assert_eq!(50, rose.items[2].quality); // not increase past 50

            // ...and WHEN updating quality 4 more times
            for _ in 1..=4 {
                rose.update_quality();
            }

            // THEN quality should increase 4 more times
            assert_eq!(10, rose.items[0].sell_in);
            assert_eq!(25, rose.items[0].quality);

            assert_eq!(5, rose.items[1].sell_in);
            assert_eq!(10, rose.items[1].quality); // increase by 4 x 2 = 8 when <= 10 days left

            assert_eq!(0, rose.items[2].sell_in);
            assert_eq!(50, rose.items[2].quality); // not increase past 50

            // ...and WHEN updating 1 more time
            rose.update_quality();

            // THEN quality should become 0 "after the concert"
            assert_eq!(-1, rose.items[2].sell_in);
            assert_eq!(0, rose.items[2].quality);

            // ...and WHEN updating 1 more time
            rose.update_quality();

            // THEN quality should stay 0 "after the concert"
            assert_eq!(-2, rose.items[2].sell_in);
            assert_eq!(0, rose.items[2].quality);
        }
    }

    mod new_conjured_item_feature {
        use gildedrose::tests::{CONJURED_ITEM};
        use super::{GildedRose, Item};

        #[test]
        fn test_conjured_item_quality_update() {
            // GIVEN a conjured item
            let items = vec![
                Item::new(CONJURED_ITEM, 3, 18),
            ];
            let mut rose = GildedRose::new(items);

            // WHEN updating quality
            rose.update_quality();

            // THEN quality should degrade by 2
            assert_eq!(2, rose.items[0].sell_in);
            assert_eq!(16, rose.items[0].quality);

            // ...and WHEN updating quality 2 more times
            for _ in 1..=2 {
                rose.update_quality();
            }

            // THEN quality should degrade twice by 2 for a total of 4
            assert_eq!(0, rose.items[0].sell_in);
            assert_eq!(12, rose.items[0].quality);

            // ...and WHEN updating quality 1 more time
            rose.update_quality();

            // THEN quality should degrade twice as fast, so by 4
            assert_eq!(-1, rose.items[0].sell_in);
            assert_eq!(8, rose.items[0].quality);
        }
    }

}
