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
        messages.insert(&account_id, &message)
    }

    pub fn get_message(&self, account_id: AccountId) -> Option<String> {
        self.messages().get(&account_id)
    }
}

mod floating_state {
    use std::ops::{Deref, DerefMut};

    use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
    use near_sdk::env;
    use near_sdk::IntoStorageKey;

    pub struct State<T, K>
    where
        T: BorshDeserialize + BorshSerialize,
        K: IntoStorageKey + Clone,
    {
        mutated: bool,
        prefix: K,
        data: T,
    }

    impl<T, K> State<T, K>
    where
        T: BorshDeserialize + BorshSerialize,
        K: IntoStorageKey + Clone,
    {
        fn new(prefix: K, data: T) -> Self {
            Self {
                mutated: false,
                prefix,
                data,
            }
        }

        pub fn read(key: K) -> Option<Self> {
            env::storage_read(&key.clone().into_storage_key())
                .map(|data| Self::new(key, T::try_from_slice(&data).unwrap()))
        }

        pub fn init(prefix: K, data: T) -> Self {
            Self {
                mutated: true,
                prefix,
                data,
            }
        }
    }

    impl<T, K> AsRef<T> for State<T, K>
    where
        T: BorshDeserialize + BorshSerialize,
        K: IntoStorageKey + Clone,
    {
        fn as_ref(&self) -> &T {
            &self.data
        }
    }

    impl<T, K> Deref for State<T, K>
    where
        T: BorshDeserialize + BorshSerialize,
        K: IntoStorageKey + Clone,
    {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    impl<T, K> AsMut<T> for State<T, K>
    where
        T: BorshDeserialize + BorshSerialize,
        K: IntoStorageKey + Clone,
    {
        fn as_mut(&mut self) -> &mut T {
            self.mutated = true;
            &mut self.data
        }
    }

    impl<T, K> DerefMut for State<T, K>
    where
        T: BorshDeserialize + BorshSerialize,
        K: IntoStorageKey + Clone,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.as_mut()
        }
    }

    impl<T, K> Drop for State<T, K>
    where
        T: BorshDeserialize + BorshSerialize,
        K: IntoStorageKey + Clone,
    {
        fn drop(&mut self) {
            if self.mutated {
                env::storage_write(
                    &self.prefix.clone().into_storage_key(),
                    &self.data.try_to_vec().unwrap(),
                );
            }
        }
    }
}
