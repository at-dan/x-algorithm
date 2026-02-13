use crate::candidate_pipeline::candidate::PostCandidate;
use crate::candidate_pipeline::query::ScoredPostsQuery;
use std::collections::HashSet;
use tonic::async_trait;
use xai_candidate_pipeline::hydrator::Hydrator;

pub struct InNetworkCandidateHydrator;

#[async_trait]
impl Hydrator<ScoredPostsQuery, PostCandidate> for InNetworkCandidateHydrator {
    #[xai_stats_macro::receive_stats]
    async fn hydrate(
        &self,
        query: &ScoredPostsQuery,
        candidates: &[PostCandidate],
    ) -> Result<Vec<PostCandidate>, String> {
        let followed_ids: HashSet<u64> = query
            .user_features
            .followed_user_ids
            .iter()
            .copied()
            .map(|id| id as u64)
            .collect();

        let hydrated_candidates = candidates
            .iter()
            .map(|candidate| {
                let is_in_network = followed_ids.contains(&candidate.author_id);
                PostCandidate {
                    in_network: Some(is_in_network),
                    ..Default::default()
                }
            })
            .collect();

        Ok(hydrated_candidates)
    }

    fn update(&self, candidate: &mut PostCandidate, hydrated: PostCandidate) {
        candidate.in_network = hydrated.in_network;
    }
}
