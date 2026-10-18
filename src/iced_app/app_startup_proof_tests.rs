//! App-level oracles for startup budgets and the CPU-to-first-quad boundary.
use super::app_tests::{build_test_app, draw_test_canvas};
use super::*;
use crate::iced_app::Message;
use crate::render::WowUiPrimitive;
use crate::screen::ScreenKind;
use std::time::{Duration, Instant};

fn tick(app: &mut App) {
    let _ = app.update(Message::ProcessTimers(Instant::now()));
}

fn assert_no_quads(primitive: &WowUiPrimitive) {
    assert!(
        primitive
            .strata_batches
            .iter()
            .flatten()
            .all(|batch| batch.vertices.is_empty())
    );
    assert!(primitive.overlay.vertices.is_empty());
}

#[test]
fn saturated_startup_carries_ten_pass_groups_and_sixteen_ticks_across_updates() {
    let mut app = build_test_app(ScreenKind::Game);
    app.env
        .borrow()
        .exec(
            r#"
        generations, updates, trace = 0, 0, {}
        local function recur()
            generations = generations + 1
            table.insert(trace, 'T' .. generations)
            C_Timer.After(0, recur)
        end
        C_Timer.After(0, recur)
        local frame = CreateFrame('Frame', nil, UIParent)
        frame:SetScript('OnUpdate', function()
            updates = updates + 1
            table.insert(trace, 'U' .. updates)
        end)
    "#,
        )
        .unwrap();
    let size = Size::new(800.0, 600.0);
    assert_no_quads(&draw_test_canvas(&app, size));
    let mut expected_trace = Vec::new();
    // Literal contract values, independent of the implementation constants.
    for turn in 1..=170 {
        assert!(
            !app.gui_startup_complete.get(),
            "completed before turn {turn}"
        );
        tick(&mut app);
        let expected_updates = (turn - 1) / 10;
        if turn > 10 && turn % 10 == 1 {
            expected_trace.push(format!("U{expected_updates}"));
        }
        expected_trace.push(format!("T{turn}"));
        let (generations, updates, trace): (usize, usize, String) = app
            .env
            .borrow()
            .eval("return generations, updates, table.concat(trace, ',')")
            .unwrap();
        assert_eq!(
            (generations, updates),
            (turn, expected_updates),
            "turn {turn}"
        );
        assert_eq!(trace, expected_trace.join(","), "turn {turn}");
        assert_eq!(app.gui_startup_complete.get(), turn == 170);
        if turn < 170 {
            assert_no_quads(&draw_test_canvas(&app, size));
        }
    }
    assert!(
        app.env.borrow().next_timer_delay().is_some(),
        "cap must finish with recursive work still pending"
    );
    eprintln!(
        "B1 saturated updates=170 generations=170 OnUpdates=16 initial_passes=10 passes_per_tick=10"
    );
}

#[test]
fn empty_startup_groups_short_circuit_and_keep_fast_subscription_without_timers() {
    let mut app = build_test_app(ScreenKind::Game);
    app.env
        .borrow()
        .exec(
            r#"
        updates = 0
        local frame = CreateFrame('Frame', nil, UIParent)
        frame:SetScript('OnUpdate', function() updates = updates + 1 end)
    "#,
        )
        .unwrap();
    let size = Size::new(800.0, 600.0);
    assert_no_quads(&draw_test_canvas(&app, size));
    for turn in 1..=17 {
        assert!(app.env.borrow().next_timer_delay().is_none());
        // Remove unrelated redraw/texture reasons; startup alone must stay fast.
        app.strata_dirty.set(0);
        app.textures_pending.set(false);
        assert_eq!(
            app.compute_tick_interval(),
            Some(Duration::from_micros(16_667))
        );
        tick(&mut app);
        let updates: usize = app.env.borrow().eval("return updates").unwrap();
        assert_eq!(updates, turn - 1, "empty timer group at turn {turn}");
        assert_eq!(app.gui_startup_complete.get(), turn == 17);
        if turn < 17 {
            assert_no_quads(&draw_test_canvas(&app, size));
        }
    }
    app.strata_dirty.set(0);
    app.textures_pending.set(false);
    assert_eq!(app.compute_tick_interval(), Some(Duration::from_secs(1)));
    eprintln!("B1 empty updates=17 OnUpdates=16 timer_pending=0 fast_intervals=17");
}

fn write_texture(root: &std::path::Path, name: &str, rgba: [u8; 4]) {
    let path = root.join("Proof/Media").join(format!("{name}.png"));
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    image::RgbaImage::from_pixel(2, 1, image::Rgba(rgba))
        .save(path)
        .unwrap();
}

fn set_roots(app: &App, root: &std::path::Path) {
    app.env.borrow().state().borrow_mut().addon_base_paths = vec![root.to_path_buf()];
}

fn assert_cached_pixels(app: &App, request: &str, rgba: [u8; 4]) {
    let manager = app.texture_manager.borrow();
    let texture = manager
        .get(request)
        .unwrap_or_else(|| panic!("{request} must be decoded before draw"));
    assert_eq!((texture.width, texture.height), (2, 1));
    assert_eq!(&*texture.pixels, &[rgba, rgba].concat());
}

#[test]
fn startup_cleanup_layout_and_texture_warmup_precede_first_nonempty_quad() {
    let root = tempfile::tempdir().unwrap();
    write_texture(root.path(), "Visible", [23, 47, 89, 255]);
    let mut app = build_test_app(ScreenKind::Game);
    set_roots(&app, root.path());
    let request = "interface/addons/proof/media/visible";
    app.env
        .borrow()
        .exec(
            r#"
        local function textured(name)
            local f = CreateFrame('Frame', name, UIParent)
            f:SetSize(40, 20)
            f:SetPoint('TOPLEFT', UIParent, 'TOPLEFT', 30, -50)
            local t = f:CreateTexture(nil, 'ARTWORK')
            t:SetAllPoints()
            assert(t:SetTexture('interface/addons/proof/media/visible'))
            return f
        end
        C_Timer.After(0, function()
            VisibleProof = textured('VisibleProof')
            Baganator_WelcomeFrame = textured('Baganator_WelcomeFrame')
            QuestInfoRequiredMoneyFrame = textured('QuestInfoRequiredMoneyFrame')
            QuestInfoGroupSize = textured('QuestInfoGroupSize')
        end)
    "#,
        )
        .unwrap();
    let size = Size::new(800.0, 600.0);
    assert_no_quads(&draw_test_canvas(&app, size));
    for _ in 0..200 {
        tick(&mut app);
        if app.gui_startup_complete.get() {
            break;
        }
        assert_no_quads(&draw_test_canvas(&app, size));
    }
    assert!(app.gui_startup_complete.get());
    let visibility: (bool, bool, bool, bool) = app
        .env
        .borrow()
        .eval(
            r#"
        return VisibleProof:IsShown(), Baganator_WelcomeFrame:IsShown(),
               QuestInfoRequiredMoneyFrame:IsShown(), QuestInfoGroupSize:IsShown()
    "#,
        )
        .unwrap();
    assert_eq!(visibility, (true, false, false, false));
    // Observe CPU pixels before invoking the first drawable shader call.
    assert_cached_pixels(&app, request, [23, 47, 89, 255]);
    let primitive = draw_test_canvas(&app, size);
    let vertices: Vec<_> = primitive
        .strata_batches
        .iter()
        .flatten()
        .flat_map(|batch| batch.vertices.iter())
        .collect();
    assert_eq!(
        vertices.len(),
        4,
        "only the visible fixture may emit a quad"
    );
    let mut positions: Vec<_> = vertices.iter().map(|vertex| vertex.position).collect();
    positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(
        positions,
        vec![[30.0, 50.0], [30.0, 70.0], [70.0, 50.0], [70.0, 70.0]]
    );
    let texture = primitive
        .textures
        .iter()
        .find(|texture| texture.path == request)
        .expect("first nonempty primitive must carry fixture upload pixels");
    assert_eq!((texture.width, texture.height), (2, 1));
    assert_eq!(&*texture.rgba, &[23, 47, 89, 255, 23, 47, 89, 255]);
    eprintln!("B2 first_quad vertices=4 hidden_roots=3 decoded_before_draw=1 upload_textures=1");
}

#[cfg(target_os = "linux")]
mod filesystem {
    use super::*;
    use std::ffi::CString;
    use std::io::Read;
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;

    struct DirectoryObserver(std::fs::File);

    impl DirectoryObserver {
        fn watch(path: &std::path::Path) -> Self {
            let fd = unsafe { libc::inotify_init1(libc::IN_NONBLOCK | libc::IN_CLOEXEC) };
            assert!(fd >= 0, "{}", std::io::Error::last_os_error());
            let file = unsafe { std::fs::File::from_raw_fd(fd) };
            let path = CString::new(path.as_os_str().as_bytes()).unwrap();
            let watch = unsafe {
                libc::inotify_add_watch(
                    file.as_raw_fd(),
                    path.as_ptr(),
                    libc::IN_OPEN | libc::IN_CLOSE_NOWRITE | libc::IN_ONLYDIR,
                )
            };
            assert!(watch >= 0, "{}", std::io::Error::last_os_error());
            Self(file)
        }

        fn drain_opens(&mut self) -> usize {
            let mut count = 0;
            let mut buffer = [0u8; 16384];
            loop {
                let size = match self.0.read(&mut buffer) {
                    Ok(size) => size,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => return count,
                    Err(error) => panic!("inotify read: {error}"),
                };
                assert!(size > 0);
                let mut offset = 0;
                while offset < size {
                    let event = unsafe {
                        (buffer.as_ptr().add(offset) as *const libc::inotify_event).read_unaligned()
                    };
                    assert_eq!(event.mask & libc::IN_Q_OVERFLOW, 0, "observer overflow");
                    if event.mask & (libc::IN_OPEN | libc::IN_ISDIR)
                        == (libc::IN_OPEN | libc::IN_ISDIR)
                    {
                        count += 1;
                    }
                    offset += std::mem::size_of::<libc::inotify_event>() + event.len as usize;
                }
            }
        }
    }

    fn assign_texture(app: &App, name: &str, expected: bool) {
        app.env
            .borrow()
            .exec(&format!(
                r#"
            ProbeTexture = ProbeTexture or UIParent:CreateTexture('ProbeTexture', 'ARTWORK')
            ProbeTexture:SetSize(40, 20)
            ProbeTexture:SetPoint('TOPLEFT', UIParent, 'TOPLEFT', 30, -50)
            assert(ProbeTexture:SetTexture('interface/addons/proof/media/{name}') == {expected})
        "#
            ))
            .unwrap();
    }

    #[test]
    fn app_preload_setter_render_reuse_root_change_and_gui_reload_recover_sources() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        write_texture(first.path(), "Queued", [1, 2, 3, 255]);
        write_texture(second.path(), "RootOnly", [4, 5, 6, 255]);
        let mut observer = DirectoryObserver::watch(&first.path().join("Proof/Media"));
        // Calibrate kernel event delivery against two real enumerations.
        for _ in 0..2 {
            assert_eq!(
                std::fs::read_dir(first.path().join("Proof/Media"))
                    .unwrap()
                    .count(),
                1
            );
            assert_eq!(observer.drain_opens(), 1);
        }
        let mut app = build_test_app(ScreenKind::Game);
        set_roots(&app, first.path());
        let queued = "interface/addons/proof/media/queued";
        assign_texture(&app, "queued", true);
        assert_eq!(observer.drain_opens(), 1);
        app.env
            .borrow()
            .state()
            .borrow_mut()
            .enqueue_texture_preloads([queued.to_string()]);
        app.preload_current_render_requests(None);
        assert_cached_pixels(&app, queued, [1, 2, 3, 255]);
        assert!(
            app.env
                .borrow()
                .state()
                .borrow()
                .pending_texture_preloads
                .is_empty()
        );
        assert_eq!(
            observer.drain_opens(),
            0,
            "App queued preloader must reuse setter enumeration"
        );
        let size = Size::new(800.0, 600.0);
        let _ = draw_test_canvas(&app, size);
        for _ in 0..200 {
            tick(&mut app);
            if app.gui_startup_complete.get() {
                break;
            }
        }
        assert!(app.gui_startup_complete.get());
        for _ in 0..32 {
            assign_texture(&app, "queued", true);
            let primitive = draw_test_canvas(&app, size);
            assert!(
                primitive
                    .strata_batches
                    .iter()
                    .flatten()
                    .any(|batch| !batch.vertices.is_empty())
            );
            assert_eq!(
                observer.drain_opens(),
                0,
                "warm setter and actual App draw must reuse listing"
            );
        }
        assign_texture(&app, "rootonly", false);
        // The App's renderer has observed a CPU miss before the roots change.
        let _ = draw_test_canvas(&app, size);
        set_roots(&app, second.path());
        let mut second_observer = DirectoryObserver::watch(&second.path().join("Proof/Media"));
        assign_texture(&app, "rootonly", true);
        let primitive = draw_test_canvas(&app, size);
        assert!(
            primitive
                .textures
                .iter()
                .any(|texture| texture.path.ends_with("/rootonly")
                    && texture.rgba[..4] == [4, 5, 6, 255])
        );
        assert_eq!(
            second_observer.drain_opens(),
            1,
            "root change rebuilds one shared listing"
        );
        assign_texture(&app, "late", false);
        let _ = draw_test_canvas(&app, size);
        // Remove the failed source decision without requiring filesystem freshness.
        // Successful decoded-byte hot reload is intentionally outside F1's contract.
        let _ = app.update(Message::ReloadUI);
        write_texture(second.path(), "Late", [7, 8, 9, 255]);
        assign_texture(&app, "late", true);
        let primitive = draw_test_canvas(&app, size);
        assert!(
            primitive
                .textures
                .iter()
                .any(|texture| texture.path.ends_with("/late")
                    && texture.rgba[..4] == [7, 8, 9, 255])
        );
        assert_eq!(
            second_observer.drain_opens(),
            1,
            "GUI reload invalidates actual App source listing"
        );
        eprintln!(
            "B2 directory_opens setter=1 queued_preload=0 warm_draws32=0 root_change=1 gui_reload=1"
        );
    }
}
