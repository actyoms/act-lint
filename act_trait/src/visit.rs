use serde::de::MapAccess;

pub trait VisitMap<T> {
    fn is_map_visitor() -> bool {
        true
    }
    fn visit_map<'a, A>(map: A) -> Result<T, A::Error>
    where
        A: MapAccess<'a>;
}
