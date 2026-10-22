// Shared registration groups for tests consuming profile-owned vendor inputs.
// Root-name absence alone does not establish behavioral fixture absence.

/// Select the native action callback case without compiling Wrath-only inputs
/// into another profile's fixture. Modern test bodies remain unchanged.
#[macro_export]
macro_rules! wrath_action_case {
    ($callback:ident) => {
        #[cfg(feature = "client-wrath")]
        if $crate::common::action_bar_fixture::is_wrath() {
            return $crate::common::action_bar_fixture::assertions::$callback();
        }
    };
}

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
