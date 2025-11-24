

pub struct AocEntry2022 {
    pub year: u32,
    pub day: u32,
    pub solve: fn() -> String,
}

inventory::collect!(AocEntry2022);

pub fn get_entries() -> Vec<&'static AocEntry2022> {
    inventory::iter::<AocEntry2022>.into_iter().collect()
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;