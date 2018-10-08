use super::Engine;
use yew::services::storage::*;
use yew::format::Json;

const KEY: &'static str = "search.state";

#[derive(Serialize, Deserialize)]
pub struct State {
    pub default_engine: Option<Engine>
}

impl Default for State {
    fn default() -> Self {
        State {
            default_engine: None
        }
    }
}

impl State {
    pub fn retrieve() -> (StorageService, Self) {
        let mut storage = StorageService::new(Area::Local);
        let Json(state) = storage.restore(KEY);
        let state = state.unwrap_or_else(|_| Default::default());

        (storage, state)
    }

    pub fn save(&self, storage: &mut StorageService) {
        storage.store(KEY, Json(self))
    }
}
