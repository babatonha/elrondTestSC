#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
struct CryptoRingsAnimation {
    mint_number: u64
}

const IMAGE_HASH_BYTES: [u8; 32] = [
    109, 119, 14, 238,
    23, 91, 31, 42,
    76, 168, 1, 42,
    231, 198, 199, 122,
    250, 86, 242, 247,
    115, 75, 45, 32,
    177, 218, 136, 86,
    224, 126, 83, 180
];


#[elrond_wasm::contract]
pub trait TestMinting {

    #[init]
    fn init(
        &self, 
        num_mintable: u64
    ) -> SCResult<()> {
        self.total_mintable().set(&num_mintable);
        self.num_minted().set(&(0 as u64));
        Ok(())
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        #[payment] issue_cost: Self::BigUint,
        token_name: self::ManagedBuffer,
        token_ticker: self::ManagedBuffer,
    ) -> SCResult<AsyncCall> {
        require!(self.nft_token_id().is_empty(), "Token already issued");

        Ok(self.send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                issue_cost,
                &token_name,
                &token_ticker,
                NonFungibleTokenProperties {
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_change_owner: false,
                    can_upgrade: false,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_callback()))
    }

    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) -> SCResult<AsyncCall> {
        self.require_token_issued()?;

        Ok(self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.nft_token_id().get(),
                (&[EsdtLocalRole::NftCreate][..]).into_iter().cloned()
            )
            .async_call())
    }

    #[endpoint]
    fn mint(&self) -> SCResult<u64> {

        self.require_token_issued()?;
        require!(self.num_minted().get() < self.total_mintable().get(), "Mint has ended");

        let token = self.nft_token_id().get();
        let amount_one = &self.types().big_uint_from(1 as u64);
        let num_minted = self.num_minted().get();
        let royalties = BigUint::zero();
        let name = ManagedBuffer::from("THe test minting".as_bytes());

        let attributes = CryptoRingsAnimation {
            mint_number: num_minted + 1
        };

        let hash = self.types().managed_buffer_from(&IMAGE_HASH_BYTES);

        let uri = ManagedBuffer::from(
            "https://gateway.pinata.cloud/ipfs/QmTkw6xSEBcAYGAAD1kb4RrAsAGWa4QYo6EDMRMr1Gvvip".as_bytes()
        );
        let mut uris = ManagedVec::new(self.type_manager());
        uris.push(uri);

        let nft_nonce = self.send().esdt_nft_create(
            &token, 
            &amount_one, 
            &name,
            &royalties, 
            &hash, 
            &attributes, 
            &uris 
        );

        self.num_minted().set(&(num_minted + 1));

        // now send to caller

        let caller = self.blockchain().get_caller();
        self.send().direct(&caller, &token, nft_nonce, &amount_one, &[]);

        Ok(nft_nonce)
    }

    fn require_token_issued(&self) -> SCResult<()> {
        require!(!self.nft_token_id().is_empty(), "Don't have id, token not issued");
        Ok(())
    }

    // storage

    #[view(nftTokenId)]
    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getTotalMintable)]
    #[storage_mapper("getTotalMintable")]
    fn total_mintable(&self) -> SingleValueMapper<u64>;

    #[view(getNumMinted)]
    #[storage_mapper("getNumMinted")]
    fn num_minted(&self) -> SingleValueMapper<u64>;


    // callbacks

    #[callback]
    fn issue_callback(&self, #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.nft_token_id().set(&token_id);
            },
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let (returned_tokens, token_id) = self.call_value().payment_token_pair();
                if token_id.is_egld() && returned_tokens > 0 {
                    self.send()
                        .direct(&caller, &token_id, 0, &returned_tokens, &[]);
                }
            },
        }
    }

}