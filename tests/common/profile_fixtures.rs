// Shared registration groups for tests consuming profile-owned vendor inputs.
// Root-name absence alone does not establish behavioral fixture absence.

#[macro_export]
macro_rules! post_wrath_ui_fixture {
    ($($item:item)*) => {
        $(
            #[cfg(any(
                feature = "client-retail",
                feature = "client-ptr",
                feature = "client-mists",
                feature = "client-era",
                feature = "client-anniversary",
                feature = "client-wowforever"
            ))]
            $item
        )*
    };
}
