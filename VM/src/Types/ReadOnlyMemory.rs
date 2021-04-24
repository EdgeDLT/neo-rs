pub struct ReadOnlyMemory([u8]);

impl ReadOnlyMemory {

    // fn Empty

    fn IsEmpty(&self) -> bool {
        self.0.is_empty()
    }


    fn Length(&self) -> usize { self.0.len() }


    fn Span(&self) ->
}