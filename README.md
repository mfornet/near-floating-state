# Floating state

Allow floating state using [near-sdk-rs](https://github.com/near/near-sdk-rs). This allows storing objects in the state that are not directly referenced in the main object.

```rs
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
        messages.insert(&account_id, &message)
    }

    pub fn get_message(&self, account_id: AccountId) -> Option<String> {
        self.messages().get(&account_id)
    }
}
```
