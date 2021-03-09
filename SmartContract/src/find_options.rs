mod SmartContract {
    #[derive(Debug)]
    pub(crate) enum FindOptions {
        None = 0,

        KeysOnly = 1 << 0,
        RemovePrefix = 1 << 1,
        ValuesOnly = 1 << 2,
        DeserializeValues = 1 << 3,
        PickField0 = 1 << 4,
        PickField1 = 1 << 5,

        All = FindOptions::KeysOnly
            | FindOptions::RemovePrefix
            | FindOptions::ValuesOnly
            | FindOptions::DeserializeValues
            | FindOptions::PickField0
            | FindOptions::PickField1,
    }
}