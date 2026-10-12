# Forever key metadata primary-source check

Date: 2026-10-12. Scope: inspected only `src/asset_resolver_config.rs`, `Cargo.lock`, and source/configuration in the locked `asset-resolver` checkout. Did not read key payloads or values, credentials/session stores, archives/indexes, decoded/cache payloads, or vendor/native acquisition files; did not download, install, run, build, test, hash, format, or operate anything.

## Finding: primary catalog source and Forever match are not configured

The application selects the Forever CASC product identity as `wow_classic_beta` in `src/asset_resolver_config.rs` (`active_profile_casc_product`). This is product selection, not key metadata availability.

`Cargo.lock` identifies the dependency revision as:

> `asset-resolver` source: `git+https://github.com/world-of-osso/asset-resolver?rev=462097bba2aa72d1afed8e0ca5b7c7315096427d#462097bba2aa72d1afed8e0ca5b7c7315096427d`

That is the resolver code repository, not an identified primary key catalog repository.

In that checkout, `src/casc_resolver.rs` declares `EXTERNAL_TACT_KEYS_PATH: &str = "tactkeys/WoW.txt"` and `load_tact_keys` resolves that relative path through `paths.resolve_data_path(...)`, reads it locally if present, and parses it using `TactKeyStore::load_from_txt`. No primary metadata catalog URL/repository, catalog revision/update timestamp, product/build fields, or product-specific matching logic is configured in this source. The source does not show that this local text file records product/build names or distinguish Forever records. I did not inspect the file itself.

The existing handoff's retained `wow_classic_beta` / version `1.60.1.69977` / build `3bd89ce2721f7c75e7525dc83741076f` remains identity-only. It is not current key availability evidence and was not used to infer any keys.

## Exact missing field and safe next boundary

**Missing field:** an authoritative primary key-metadata catalog URL/repository (plus revision or timestamp) explicitly documenting whether its schema associates entries with product and build. The current resolver instead points at a local path, `tactkeys/WoW.txt`, with no catalog provenance/schema contract in inspected code.

**Next boundary:** metadata-only inspection of a separately identified, public primary catalog's schema and product/build record for `wow_classic_beta` and the specified build. Until that source and match are established, status is **unknown / no Forever-specific availability evidence**. No key material or values are included here.
