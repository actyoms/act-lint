pub trait Expecting {
    fn expecting(formatter: &mut std::fmt::Formatter) -> std::fmt::Result;
}
