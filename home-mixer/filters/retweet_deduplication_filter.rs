use crate::candidate_pipeline::candidate::PostCandidate;
use crate::candidate_pipeline::query::ScoredPostsQuery;
use std::collections::HashSet;
use tonic::async_trait;
use xai_candidate_pipeline::filter::{Filter, FilterResult};

/// Deduplicates retweets, keeping only the first occurrence of a tweet
/// (whether as an original or as a retweet).
pub struct RetweetDeduplicationFilter;

#[async_trait]
impl Filter<ScoredPostsQuery, PostCandidate> for RetweetDeduplicationFilter {
    async fn filter(
        &self,
        _query: &ScoredPostsQuery,
        candidates: Vec<PostCandidate>,
    ) -> Result<FilterResult<PostCandidate>, String> {
        let original_tweet_ids: HashSet<u64> = candidates
            .iter()
            .filter_map(|c| {
                if c.retweeted_tweet_id.is_none() {
                    Some(c.tweet_id as u64)
                } else {
                    None
                }
            })
            .collect();

        let mut seen_tweet_ids: HashSet<u64> = HashSet::new();
        let mut kept = Vec::new();
        let mut removed = Vec::new();

        for candidate in candidates {
            let tweet_id_to_check = candidate
                .retweeted_tweet_id
                .unwrap_or(candidate.tweet_id as u64);

            if seen_tweet_ids.contains(&tweet_id_to_check) {
                removed.push(candidate);
                continue;
            }

            if original_tweet_ids.contains(&tweet_id_to_check) {
                seen_tweet_ids.insert(tweet_id_to_check);
            }

            kept.push(candidate);
        }

        Ok(FilterResult { kept, removed })
    }
}
