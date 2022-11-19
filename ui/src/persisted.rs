use gloo_storage::{LocalStorage, Storage as _};
use std::rc::Rc;
use yew::functional::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PersistedAction {
    SetAccessToken(Option<String>),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PersistedValue {
    pub access_token: Option<String>,
}

impl PersistedValue {
    pub fn new() -> Self {
        Self {
            access_token: LocalStorage::get("morum.access_token").ok(),
        }
    }
}

impl Reducible for PersistedValue {
    type Action = PersistedAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut persisted = self.as_ref().clone();
        match action {
            PersistedAction::SetAccessToken(new) => {
                persisted.access_token = new.clone();
                match new {
                    Some(new) => {
                        LocalStorage::set("morum.access_token", new)
                            .expect("set access token failed");
                    }
                    None => {
                        LocalStorage::delete("morum.access_token");
                    }
                }
            }
        }
        Rc::new(persisted)
    }
}

pub type Persisted = UseReducerHandle<PersistedValue>;
