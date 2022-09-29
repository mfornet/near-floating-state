# Floating state

Allow floating state using [near-sdk-rs](https://github.com/near/near-sdk-rs). This means allowing storing objects in the state that are not directly referenced in the main struct Contract object.

```rs
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CounterStatusMessage {
    counter: u8,
}

#[derive(BorshStorageKey, BorshDeserialize, BorshSerialize, Clone)]
enum StorageKey {
    StatusMessageHeader,
    StatusMassageContent,
}

#[near_bindgen]
impl CounterStatusMessage {
    #[init]
    pub fn new() -> Self {
        floating_state::State::init(
            StorageKey::StatusMessageHeader,
            UnorderedMap::<AccountId, String>::new(StorageKey::StatusMassageContent),
        );

        Self { counter: 0 }
    }

    fn messages(&self) -> floating_state::State<UnorderedMap<AccountId, String>, StorageKey> {
        floating_state::State::read(StorageKey::StatusMessageHeader).unwrap()
    }

    pub fn add_message(&mut self, message: String) -> Option<String> {
        let account_id = env::predecessor_account_id();
        let mut messages = self.messages();
        messages.as_mut().insert(&account_id, &message)
    }

    pub fn get_message(&self, account_id: AccountId) -> Option<String> {
        self.messages().as_ref().get(&account_id)
    }
}
```
