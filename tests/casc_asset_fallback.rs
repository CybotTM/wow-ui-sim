#[cfg(feature = "casc")]
#[test]
fn legacy_casc_asset_fallbacks_cover_known_missing_texture_paths() {
    let probes = [
        (
            "interface/common/currencywindow.blp",
            "6DB0A357702C10D71C4F945DA8DC28E5",
        ),
        (
            "interface/framegeneral/uiframemetal2x.blp",
            "729D039CA266E29BD582D7B2244687D1",
        ),
        (
            "interface/framegeneral/uiframemetalhorizontal2x.blp",
            "9AC6043E78C8A6AE72B94E46ED2A7142",
        ),
        (
            "interface/framegeneral/uiframemetalvertical2x.blp",
            "A8F52A3D17E2FD4C08D5C36FB1FF76AC",
        ),
        (
            "interface/framegeneral/uiframetabs.blp",
            "4F12CFB0612E91FDB48E60EA21241B64",
        ),
        (
            "interface/options/optionsexpandlistbutton.blp",
            "9A09A727F4A6AFAA39F29E6AE538FC7B",
        ),
        (
            "interface/paperdollinfoframe/paperdollinfopart1.blp",
            "B8D8BDE4505B8AEFC0D513008C91DDD7",
        ),
    ];

    for (path, expected_key) in probes {
        let actual = wow_ui_sim::casc_asset_fallback::lookup_encoding_key_hex(path)
            .unwrap_or_else(|| panic!("missing fallback encoding key for {path}"));
        assert_eq!(actual, expected_key);
    }
}
