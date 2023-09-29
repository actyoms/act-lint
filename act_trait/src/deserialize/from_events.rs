pub struct DeserializerFromEvents<'a>(serde::de::DeserializerFromEvents);

impl<'a> DeserializerFromEvents<'a> {
    fn visit_spanned<'de, V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.recursion_check(|de| {
            let pos = *de.pos;
            let mut map = SpannedMapAccess {
                de,
                pos,
                state: SpannedMapAccessState::StartKey,
            };
            visitor.visit_map(&mut map)
        })
    }
}
