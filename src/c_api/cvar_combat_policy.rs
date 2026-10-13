//! Retail CVar combat write policy shared by C_CVar and legacy SetCVar.
//! The 10.2.5 secure names use the existing simulator combat interpretation.

const COMBAT_PROTECTED_CVARS: &[&str] = &[
    "alwaysshowactionbars",
    "autostand",
    "autounshift",
    "bloatnameplates",
    "bloattest",
    "bloatthreat",
    "consolidatebuffs",
    "fullsizefocusframe",
    "maxalgoplates",
    "nameplatemotion",
    "nameplateoverlaph",
    "nameplateoverlapv",
    "nameplateshowenemies",
    "nameplateshowenemyguardians",
    "nameplateshowenemypets",
    "nameplateshowenemytotems",
    "nameplateshowfriendlyguardians",
    "nameplateshowfriendlypets",
    "nameplateshowfriendlytotems",
    "nameplateshowfriends",
    "repositionfrequency",
    "showarenaenemyframes",
    "showarenaenemypets",
    "showpartypets",
    "showtargetoftarget",
    "softtargetenemy",
    "softtargetenemyrange",
    "softtargetforce",
    "softtargetinteract",
    "softtargetinteractrange",
    "targetoftargetmode",
    "uiscale",
    "usecompactpartyframes",
    "useuiscale",
];

/// Missing/retired defaults are not republished by this policy.
pub(crate) fn is_combat_protected_cvar(name: &str) -> bool {
    cfg!(feature = "client-retail")
        && COMBAT_PROTECTED_CVARS.contains(&name.to_ascii_lowercase().as_str())
}
