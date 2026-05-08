//! `AdventureMapMixin:OnLoad` event and inset-pool behavior.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";

#[test]
fn adventure_map_onload_registers_inset_update_event_and_pool() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface: InsetPoolSurface = env
            .eval(
                r#"
                local pool = AdventureMapFrame:GetMapInsetPool()
                local beforeAcquire = pool and pool:GetNumActive() or -1
                local inset = pool and pool:Acquire() or nil
                local afterAcquire = pool and pool:GetNumActive() or -1

                local releaseCalled = false
                if inset then
                    inset.OnReleased = function(self)
                        releaseCalled = true
                        self:Hide()
                    end
                    pool:Release(inset)
                end

                return AdventureMapFrame:IsEventRegistered("ADVENTURE_MAP_UPDATE_INSETS"),
                       type(pool),
                       type(pool and pool.Acquire),
                       type(pool and pool.Release),
                       beforeAcquire,
                       afterAcquire,
                       type(inset),
                       inset and (inset:GetObjectType() or "<nil>") or "<missing>",
                       inset and inset:GetParent() == AdventureMapFrame:GetCanvas(),
                       inset and inset.Initialize == AdventureMapInsetMixin.Initialize,
                       releaseCalled
                "#,
            )
            .expect("AdventureMap OnLoad inset-pool probe must run cleanly");

        assert_inset_pool_surface(surface);
    });
}

type InsetPoolSurface = (
    bool,
    String,
    String,
    String,
    i64,
    i64,
    String,
    String,
    bool,
    bool,
    bool,
);

type InsetPoolShape = (String, String, String, i64, i64);
type AcquiredInsetSurface = (String, String, bool, bool, bool);

fn assert_inset_pool_surface(surface: InsetPoolSurface) {
    let (event_registered, pool_shape, acquired_inset) = split_inset_pool_surface(surface);

    assert_inset_update_event_registered(event_registered);
    assert_inset_pool_shape(pool_shape);
    assert_acquired_inset_surface(acquired_inset);
}

fn split_inset_pool_surface(
    surface: InsetPoolSurface,
) -> (bool, InsetPoolShape, AcquiredInsetSurface) {
    let (
        event_registered,
        pool_type,
        acquire_type,
        release_type,
        before_acquire,
        after_acquire,
        inset_type,
        object_type,
        parent_is_canvas,
        inset_uses_template_mixin,
        release_called,
    ) = surface;

    let pool_shape = (
        pool_type,
        acquire_type,
        release_type,
        before_acquire,
        after_acquire,
    );
    let acquired_inset = (
        inset_type,
        object_type,
        parent_is_canvas,
        inset_uses_template_mixin,
        release_called,
    );

    (event_registered, pool_shape, acquired_inset)
}

fn assert_inset_update_event_registered(event_registered: bool) {
    assert!(
        event_registered,
        "`AdventureMapMixin:OnLoad` must register `ADVENTURE_MAP_UPDATE_INSETS`"
    );
}

fn assert_inset_pool_shape(surface: InsetPoolShape) {
    let (pool_type, acquire_type, release_type, before_acquire, after_acquire) = surface;

    assert_eq!(
        pool_type, "table",
        "`AdventureMapMixin:OnLoad` must store a map inset frame pool"
    );
    assert_eq!(
        acquire_type, "function",
        "map inset pool must acquire frames"
    );
    assert_eq!(
        release_type, "function",
        "map inset pool must release frames"
    );
    assert_eq!(
        before_acquire, 0,
        "map inset pool should start without active frames"
    );
    assert_eq!(
        after_acquire, 1,
        "map inset pool must track the acquired inset frame"
    );
}

fn assert_acquired_inset_surface(surface: AcquiredInsetSurface) {
    let (inset_type, object_type, parent_is_canvas, inset_uses_template_mixin, release_called) =
        surface;

    assert_eq!(inset_type, "table", "map inset pool must create a frame");
    assert_eq!(
        object_type, "Frame",
        "map inset pool must create `FRAME` objects"
    );
    assert!(
        parent_is_canvas,
        "map inset pool must parent acquired frames to `AdventureMapFrame:GetCanvas()`"
    );
    assert!(
        inset_uses_template_mixin,
        "map inset pool must instantiate `AdventureMapInsetTemplate`"
    );
    assert!(
        release_called,
        "map inset pool release callback must call `mapInset:OnReleased()`"
    );
}
