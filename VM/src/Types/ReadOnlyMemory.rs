pub struct ReadOnlyMemory([u8]);

impl ReadOnlyMemory {

    // fn Empty

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }


    fn length(&self) -> usize { self.0.len() }


    fn span(&self) ->
}