use tracing::debug;

use super::models::*;
use super::*;

const BASE_URL_INDEX: &str = "https://index.crates.io";
const BASE_URL_CRATES: &str = "https://crates.io/api/v1/crates";
const QUERY_STRING_CRATES: &str = "?include=downloads"; // Fetches only the most basic data instead of full

pub const CRAWL_MAX_INTERVAL_SECONDS: u64 = 5;
pub const CRAWL_USER_AGENT_VALUE: &str =
    concat!(env!("CARGO_PKG_NAME"), "@", env!("CARGO_PKG_VERSION"),);

impl CratesWrapper {
    pub async fn get_index_metadatas(&self, name: &str) -> CratesResult<Vec<IndexMetadata>> {
        let name_low = name.to_ascii_lowercase();
        let index_url = if name_low.len() <= 2 {
            format!("{BASE_URL_INDEX}/{}/{name_low}", name_low.len())
        } else if name_low.len() == 3 {
            format!("{BASE_URL_INDEX}/3/{}/{name_low}", &name_low[..1])
        } else {
            format!(
                "{BASE_URL_INDEX}/{}/{}/{name_low}",
                &name_low[..2],
                &name_low[2..4]
            )
        };

        let fut = async {
            debug!("Fetching crates index metadatas for '{name}'");

            // NOTE: We make this inner scope so that
            // we can catch and emit all errors at once
            let mut inner = async {
                let response = self.client().get(&index_url).send().await?;
                let status = response.status();
                let text = response.text().await?;
                if !status.is_success() {
                    return Err(CratesError::new(format!(
                        "{} {} - {}",
                        status.as_u16(),
                        status.canonical_reason().unwrap(),
                        text
                    )));
                }
                Ok(IndexMetadata::try_from_lines(text.lines().collect())?)
            }
            .await;

            // NOTE: We should sort by most recent version first
            if let Ok(vec) = &mut inner {
                vec.reverse();
            }

            self.emit_result(&inner);

            inner
        };

        self.cache
            .index_metadatas
            .with_caching(index_url.clone(), fut)
            .await
    }

    /**
        Fetches crate data using the crates.io API directly.

        This allows us to access things such as crate description,
        links to repo/docs, as well as download counters, helping
        users of the language server gauge the legitimacy of packages.

        ### Caching

        This method caches its result for the given `name` with a
        duration of *one hour or longer*. For more up-to-date info
        on versions of a package, please use [`get_index_metadatas`].

        ### Rate Limiting

        This method is heavily rate limited, and can only process
        ***one request every five seconds, globally***. This is due
        to the [crates.io crawling policy](https://crates.io/policies).
    */
    pub async fn get_crate_data(&self, name: &str) -> CratesResult<CrateData> {
        let name_low = name.to_ascii_lowercase();
        let crates_url = format!("{BASE_URL_CRATES}/{name_low}{QUERY_STRING_CRATES}");

        let fut = async {
            self.wait_for_crawl_limit().await;
            self.set_crawl_limited();

            debug!("Fetching crate data for '{name}'");

            // NOTE: We make this inner scope so that
            // we can catch and emit all errors at once
            let inner = async {
                let response = self.client().get(&crates_url).send().await?;
                let status = response.status();
                let bytes = response.bytes().await?;
                if !status.is_success() {
                    return Err(CratesError::new(format!(
                        "{} {} - {}",
                        status.as_u16(),
                        status.canonical_reason().unwrap(),
                        String::from_utf8_lossy(&bytes)
                    )));
                }
                Ok(serde_json::from_slice::<CrateData>(&bytes)?)
            }
            .await;

            self.emit_result(&inner);

            inner
        };

        self.cache
            .crate_datas
            .with_caching(crates_url.clone(), fut)
            .await
    }
}
