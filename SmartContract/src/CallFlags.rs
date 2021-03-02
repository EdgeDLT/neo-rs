mod SmartContract {
    #[derive(Debug)]
    enum CallFlags {
        None = 0,

        ReadStates = 0b00000001,
        WriteStates = 0b00000010,
        AllowCall = 0b00000100,
        AllowNotify = 0b00001000,

        States = CallFlags::ReadStates | CallFlags::WriteStates,
        ReadOnly = CallFlags::ReadStates | CallFlags::AllowCall,
        All = CallFlags::States | CallFlags::AllowCall | CallFlags::AllowNotify,
    }
}