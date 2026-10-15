//! Explicit locale/content selection for Blizzard UI CASC reads.
use cascette_crypto::{ContentKey, FileDataId};
use cascette_formats::root::{ContentFlags, LocaleFlags, RootFile};

pub(super) struct LocalizedCascRoot {
    root: RootFile,
    locale: LocaleFlags,
    content: ContentFlags,
}

impl LocalizedCascRoot {
    pub(super) fn parse(
        data: &[u8],
        locale: LocaleFlags,
        content: ContentFlags,
    ) -> Result<Self, String> {
        let root = RootFile::parse(data).map_err(|error| format!("parse CASC root: {error}"))?;
        Ok(Self {
            root,
            locale,
            content,
        })
    }

    pub(super) fn resolve_fdid(&self, fdid: u32) -> Option<ContentKey> {
        self.root
            .resolve_by_id(FileDataId::new(fdid), self.locale, self.content)
    }

    pub(super) fn resolve_path(&self, path: &str) -> Option<ContentKey> {
        self.root.resolve_by_path(path, self.locale, self.content)
    }
}

#[cfg(test)]
#[path = "locale/tests.rs"]
mod tests;
