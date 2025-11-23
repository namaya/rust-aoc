

pub struct AocEntry2023 {
    pub year: u32,
    pub day: u32,
    pub solve: fn() -> String,
}

inventory::collect!(AocEntry2023);

pub fn get_entries() -> Vec<&'static AocEntry2023> {
    inventory::iter::<AocEntry2023>.into_iter().collect()
}

pub mod day01;
pub mod day02;