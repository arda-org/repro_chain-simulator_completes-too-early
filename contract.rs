#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Contract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint(issue)]
    #[payable("EGLD")]
    fn issue(&self) {
        let payment = self.call_value().egld_value().clone_value();
        self.send().esdt_system_sc_proxy()
            .issue_fungible(
                payment.clone(),
                &ManagedBuffer::new_from_bytes("TEST".as_bytes()),
                &ManagedBuffer::new_from_bytes("TEST".as_bytes()),
                &BigUint::zero(),
                FungibleTokenProperties::default(),
            )
            .async_call_and_exit();
    }

    #[storage_mapper("token")]
    fn token(&self) -> FungibleTokenMapper<Self::Api>;
}
