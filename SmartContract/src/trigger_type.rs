mod SmartContract {
    #[derive(Debug)]
    enum TriggerType {
        OnPersist = 0x01,
        PostPersist = 0x02,
        /// <summary>
        /// The verification trigger indicates that the contract is being invoked as a verification function.
        /// The verification function can accept multiple parameters, and should return a boolean value that indicates the validity of the Transaction or block.
        /// The entry point of the contract will be invoked if the contract is triggered by Verification:
        ///     main(...);
        /// The entry point of the contract must be able to handle this type of invocation.
        /// </summary>
        Verification = 0x20,
        /// <summary>
        /// The application trigger indicates that the contract is being invoked as an application function.
        /// The application function can accept multiple parameters, change the states of the blockchain, and return any type of value.
        /// The contract can have any form of entry point, but we recommend that all contracts should have the following entry point:
        ///     public byte[] main(string operation, params object[] args)
        /// The functions can be invoked by creating an InvocationTransaction.
        /// </summary>
        Application = 0x40,

        System = TriggerType::OnPersist | TriggerType::PostPersist,
        All = TriggerType::OnPersist | TriggerType::PostPersist | TriggerType::Verification | TriggerType::Application,
    }
}