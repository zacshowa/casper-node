use std::{collections::HashMap, time::Duration};

use crate::{
    components::fetcher::{metrics::Metrics, Event, FetchResponder, Fetcher, ItemFetcher},
    effect::{requests::StorageRequest, EffectBuilder, EffectExt, Effects},
    types::{Deploy, DeployId, NodeId},
};

impl ItemFetcher<Deploy> for Fetcher<Deploy> {
    const SAFE_TO_RESPOND_TO_ALL: bool = true;

    fn responders(
        &mut self,
    ) -> &mut HashMap<DeployId, HashMap<NodeId, Vec<FetchResponder<Deploy>>>> {
        &mut self.responders
    }

    fn validation_metadata(&self) -> &() {
        &()
    }

    fn metrics(&mut self) -> &Metrics {
        &self.metrics
    }

    fn peer_timeout(&self) -> Duration {
        self.get_from_peer_timeout
    }

    /// Gets a `Deploy` from the storage component.
    fn get_from_storage<REv>(
        &mut self,
        effect_builder: EffectBuilder<REv>,
        id: DeployId,
        peer: NodeId,
        _validation_metadata: (),
        responder: FetchResponder<Deploy>,
    ) -> Effects<Event<Deploy>>
    where
        REv: From<StorageRequest> + Send,
    {
        effect_builder
            .get_stored_deploy(id)
            .event(move |results| Event::GetFromStorageResult {
                id,
                peer,
                validation_metadata: (),
                maybe_item: Box::new(results),
                responder,
            })
    }

    fn put_to_storage<REv>(
        &self,
        _item: Deploy,
        _peer: NodeId,
        _effect_builder: EffectBuilder<REv>,
    ) -> Option<Effects<Event<Deploy>>>
    where
        REv: From<StorageRequest> + Send,
    {
        // Incoming deploys are routed to the deploy acceptor for validation before being stored.
        None
    }
}
