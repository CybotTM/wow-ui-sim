use super::*;

#[test]
fn anchor_points_default_relative_point_and_reject_invalid_values() {
    let default_relative = AnchorXml {
        point: Some("BOTTOM".to_string()),
        relative_point: None,
        relative_to: None,
        relative_key: None,
        x: None,
        y: None,
        offset: None,
    };
    assert_eq!(
        anchor_points(&default_relative),
        Some((AnchorPoint::Bottom, AnchorPoint::Bottom))
    );

    let invalid_point = AnchorXml {
        point: Some("NOPE".to_string()),
        ..default_relative.clone()
    };
    assert!(
        anchor_points(&invalid_point).is_none(),
        "invalid primary point should reject the anchor"
    );

    let invalid_relative = AnchorXml {
        relative_point: Some("NOPE".to_string()),
        ..default_relative
    };
    assert!(
        anchor_points(&invalid_relative).is_none(),
        "invalid relative point should reject the anchor"
    );
}
