#[cfg(feature = "casc")]
use std::path::PathBuf;
#[cfg(feature = "casc")]
use std::sync::OnceLock;

#[cfg(feature = "casc")]
static RESOLVER: OnceLock<asset_resolver::CascListfileResolver> = OnceLock::new();

#[cfg(feature = "casc")]
pub fn resolver() -> &'static asset_resolver::CascListfileResolver {
    RESOLVER.get_or_init(|| asset_resolver::CascListfileResolver::new(config()))
}

#[cfg(feature = "casc")]
fn config() -> asset_resolver::AssetResolverConfig {
    let cache_root = cache_root();
    asset_resolver::AssetResolverConfig::new()
        .with_cache_root(&cache_root)
        .with_shared_data_root(cache_root.join("data"))
}

#[cfg(feature = "casc")]
fn cache_root() -> PathBuf {
    std::env::var_os("ASSET_RESOLVER_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(default_cache_root)
}

#[cfg(feature = "casc")]
fn default_cache_root() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("asset-resolver")
}

#[cfg(all(test, feature = "casc"))]
mod tests {
    #[test]
    fn resolver_can_be_created_without_game_engine_root() {
        let resolver = super::new_test_resolver();

        assert!(resolver.lookup_path("not/a/real/path.blp").is_none());
    }
}

#[cfg(all(test, feature = "casc"))]
fn new_test_resolver() -> asset_resolver::CascListfileResolver {
    asset_resolver::CascListfileResolver::new(config())
}
