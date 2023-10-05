use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use parking_lot::RwLock;

use super::{list, read, Data, DataBuilder};

pub struct Playlist {
    pub paths: Arc<RwLock<Vec<PathBuf>>>,
}

#[async_trait]
impl super::Playlist for Playlist {
    async fn read(&self, query: read::Query) -> Result<Option<read::Response>> {
        let path_guard = self.paths.read();
        let path = path_guard.get(query.id as usize);
        if let Some(path) = path {
            Ok(read::Response::builder()
                .id(query.id)
                .path(path.display().to_string())
                .build()
                .ok())
        } else {
            Ok(None)
        }
    }

    async fn list(&self, query: list::Query) -> Result<list::Response> {
        let matcher = SkimMatcherV2::default();
        let data = self
            .paths
            .read()
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                let p = p.display().to_string();
                if query.search.is_empty()
                    || matcher
                        .fuzzy_match(p.as_str(), query.search.as_str())
                        .is_some()
                {
                    DataBuilder::default().id(i as u32).path(p).build().ok()
                } else {
                    None
                }
            })
            .collect::<Vec<Data>>();
        let count = data.len();
        let data = data
            .into_iter()
            .skip(query.offset as usize)
            .take(query.length as usize)
            .collect();
        Ok(list::Response::builder()
            .data(data)
            .count(count as u32)
            .build()?)
    }
}
