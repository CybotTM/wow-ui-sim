use cascette_crypto::{ContentKey, FileDataId};
use cascette_formats::root::{ContentFlags, LocaleFlags, RootBuilder, RootVersion};
const FDID: u32 = 5612369;
const PATH: &str = "Interface/AddOns/Blizzard_LoadLocale/LoadLocale.lua";
const PTPT_MASK: u32 = 0x0001_0000;
const UI_CONTENT: u64 = 0x0238_0000;
fn fixture(version: RootVersion, records: &[(u32, u64, u8)]) -> Vec<u8> {
    let mut builder = RootBuilder::new(version);
    for &(locale, content, byte) in records {
        builder.add_file(
            FileDataId::new(FDID),
            ContentKey::from_bytes([byte; 16]),
            Some(PATH),
            LocaleFlags::new(locale),
            ContentFlags::new(content),
        );
    }
    builder.build().expect("serialize root fixture")
}
fn selector(bytes: &[u8], content: u64) -> super::LocalizedCascRoot {
    super::LocalizedCascRoot::parse(
        bytes,
        LocaleFlags::new(LocaleFlags::ENUS),
        ContentFlags::new(content),
    )
    .expect("parse fixture")
}
#[test]
fn fdid_selects_enus_not_last_ptpt() {
    let bytes = fixture(
        RootVersion::V2,
        &[
            (LocaleFlags::ENUS, UI_CONTENT, 1),
            (PTPT_MASK, UI_CONTENT, 2),
        ],
    );
    assert_eq!(
        selector(&bytes, ContentFlags::NONE).resolve_fdid(FDID),
        Some(ContentKey::from_bytes([1; 16]))
    );
}
#[test]
fn missing_enus_does_not_select_another_locale() {
    let bytes = fixture(RootVersion::V2, &[(PTPT_MASK, UI_CONTENT, 2)]);
    let root = selector(&bytes, ContentFlags::NONE);
    assert_eq!(root.resolve_fdid(FDID), None);
    assert_eq!(root.resolve_path(PATH), None);
}
#[test]
fn path_and_fdid_select_same_enus_content() {
    let bytes = fixture(
        RootVersion::V2,
        &[
            (LocaleFlags::ENUS, UI_CONTENT, 1),
            (PTPT_MASK, UI_CONTENT, 2),
        ],
    );
    let root = selector(&bytes, ContentFlags::NONE);
    assert_eq!(
        root.resolve_path(PATH),
        Some(ContentKey::from_bytes([1; 16]))
    );
    assert_eq!(
        root.resolve_path(&PATH.to_lowercase().replace('/', "\\")),
        root.resolve_fdid(FDID)
    );
}
#[test]
fn requested_content_flags_are_not_discarded() {
    let bytes = fixture(
        RootVersion::V2,
        &[
            (LocaleFlags::ENUS, ContentFlags::NONE, 1),
            (LocaleFlags::ENUS, UI_CONTENT, 3),
        ],
    );
    let root = selector(&bytes, UI_CONTENT);
    assert_eq!(
        root.resolve_fdid(FDID),
        Some(ContentKey::from_bytes([3; 16]))
    );
    assert_eq!(root.resolve_path(PATH), root.resolve_fdid(FDID));
    assert_eq!(
        selector(&bytes, ContentFlags::INSTALL).resolve_fdid(FDID),
        None
    );
}
#[test]
fn v4_content_high_bits_and_version_are_preserved() {
    const HIGH_CONTENT: u64 = 1_u64 << 36;
    let bytes = fixture(
        RootVersion::V4,
        &[
            (LocaleFlags::ENUS, UI_CONTENT, 1),
            (LocaleFlags::ENUS, HIGH_CONTENT | UI_CONTENT, 4),
        ],
    );
    let root = selector(&bytes, HIGH_CONTENT);
    assert_eq!(
        root.resolve_fdid(FDID),
        Some(ContentKey::from_bytes([4; 16]))
    );
    assert_eq!(root.resolve_path(PATH), root.resolve_fdid(FDID));
}
