pub trait Normalise {
    fn none_if_empty(self) -> Option<String>;
}
impl Normalise for String {
    fn none_if_empty(self) -> Option<String> {
        if self.trim().is_empty() { None } else { Some(self) }
    }
}