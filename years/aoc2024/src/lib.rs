pub struct AocEntry2024 {
    pub year: u32,
    pub day: u32,
    pub solve: fn(full: bool) -> String,
}

inventory::collect!(AocEntry2024);

pub fn get_entries() -> Vec<&'static AocEntry2024> {
    inventory::iter::<AocEntry2024>.into_iter().collect()
}

pub mod day01;
pub mod day02;

