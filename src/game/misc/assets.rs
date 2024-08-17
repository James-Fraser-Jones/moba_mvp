use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct HandleMap<K: Eq + std::hash::Hash, A: Asset>(pub HashMap<K, Handle<A>>);
impl<K: Eq + std::hash::Hash, A: Asset> HandleMap<K, A> {
    pub fn insert_asset(
        &mut self,
        assets: &mut Assets<A>,
        key: K,
        value: impl Into<A>,
    ) -> &Handle<A> {
        let handle = assets.add(value);
        self.0.entry(key).or_insert(handle)
    }
    pub fn insert_asset_path(&mut self, server: &AssetServer, key: K, path: &str) -> &Handle<A> {
        let handle = server.load(path.to_string());
        self.0.entry(key).or_insert(handle)
    }
    pub fn get_asset<'a>(&self, assets: &'a mut Assets<A>, key: &K) -> Option<&'a A> {
        let handle = self.0.get(key)?;
        assets.get(handle)
    }
    pub fn get_asset_mut<'a>(&self, assets: &'a mut Assets<A>, key: &K) -> Option<&'a mut A> {
        let handle = self.0.get(key)?;
        assets.get_mut(handle)
    }
}
impl<K: Eq + std::hash::Hash, A: Asset> Default for HandleMap<K, A> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}
