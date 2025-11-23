
pub trait AocEntry {
    fn year(&self) -> u32;
    fn day(&self) -> u32;
    fn solve(&self) -> fn() -> String;
}
