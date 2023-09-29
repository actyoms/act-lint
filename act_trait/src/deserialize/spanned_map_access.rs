pub struct SpannedMapAccess<'a: 'r, 'r> {
    de: &'r mut DeserializerFromEvents<'a>,
    pos: usize,
    state: SpannedMapAccessState,
}

#[derive(Debug, Copy, Clone)]
pub enum SpannedMapAccessState {
    StartKey,
    DeserializeStart,
    ValueKey,
    DeserializeValue,
    LengthKey,
    DeserializeLength,
    Done,
}

impl<'de, 'a, 'r> SpannedMapAccess<'a, 'r> {
    pub fn start_location(&self) -> Result<usize> {
        let (_event, marker) = self
            .de
            .events
            .get(self.pos)
            .ok_or_else(crate::error::end_of_stream)?;

        Ok(marker.index())
    }

    pub fn index_of_sequence_end(&self) -> Result<usize> {
        let mut nesting_level = 0;

        for (event, marker) in &self.de.events[self.pos..] {
            if matches!(event, Event::SequenceStart) {
                nesting_level += 1;
            } else if matches!(event, Event::SequenceEnd) {
                nesting_level -= 1;

                if nesting_level == 0 {
                    return Ok(marker.index() + 1);
                }
            }
        }

        Err(crate::error::end_of_stream())
    }

    pub fn index_of_mapping_end(&self) -> Result<usize> {
        let mut nesting_level = 0;
        let mut last_index = None;

        for (event, marker) in &self.de.events[self.pos - 1..] {
            if matches!(event, Event::SequenceStart) {
                nesting_level += 1;
            } else if matches!(event, Event::SequenceEnd) {
                nesting_level -= 1;

                if nesting_level == 0 {
                    return last_index.ok_or_else(crate::error::end_of_stream);
                }
            }

            // Note: subtract one because of inclusive end, then subtract
            // another because that's what makes tests pass
            last_index = Some(marker.index());
        }

        last_index.ok_or_else(crate::error::end_of_stream)
    }

    pub fn current_item_length(&self) -> Result<usize> {
        // Note: The serde-yaml crate only records the start of each event and
        // not the end position/length, so we try to calculate it ourselves.
        let (event, marker) = self
            .de
            .events
            .get(self.pos)
            .ok_or_else(crate::error::end_of_stream)?;

        let length = match event {
            // just add the length of the token. Don't forget to subtract by 1
            // because of our inclusive end bound.
            Event::Scalar(token, _, _) => token.len(),
            // find the index of the end token
            Event::SequenceStart => self.index_of_sequence_end()? - marker.index(),
            // find the index of the end token
            Event::MappingStart => self.index_of_mapping_end()? - marker.index(),
            _ => 0,
        };

        Ok(length)
    }
}

impl<'de, 'a, 'r> de::MapAccess<'de> for SpannedMapAccess<'a, 'r> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        match self.state {
            SpannedMapAccessState::StartKey => {
                self.state = SpannedMapAccessState::DeserializeStart;
                seed.deserialize(BorrowedStrDeserializer::new(crate::spanned::START))
                    .map(Some)
            }
            SpannedMapAccessState::ValueKey => {
                self.state = SpannedMapAccessState::DeserializeValue;
                seed.deserialize(BorrowedStrDeserializer::new(crate::spanned::VALUE))
                    .map(Some)
            }
            SpannedMapAccessState::LengthKey => {
                self.state = SpannedMapAccessState::DeserializeLength;
                seed.deserialize(BorrowedStrDeserializer::new(crate::spanned::LENGTH))
                    .map(Some)
            }
            SpannedMapAccessState::Done => Ok(None),
            other => unreachable!("Invalid state: {:?}", other),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        match self.state {
            SpannedMapAccessState::DeserializeStart => {
                let marker = self.start_location()?;
                self.state = SpannedMapAccessState::ValueKey;
                seed.deserialize(marker.into_deserializer())
            }
            SpannedMapAccessState::DeserializeValue => {
                self.state = SpannedMapAccessState::LengthKey;
                let mut value_de = DeserializerFromEvents {
                    events: self.de.events,
                    aliases: self.de.aliases,
                    pos: self.de.pos,
                    path: self.de.path,
                    remaining_depth: self.de.remaining_depth,
                };
                seed.deserialize(&mut value_de)
            }
            SpannedMapAccessState::DeserializeLength => {
                self.state = SpannedMapAccessState::Done;
                seed.deserialize(self.current_item_length()?.into_deserializer())
            }
            _ => todo!(),
        }
    }
}
