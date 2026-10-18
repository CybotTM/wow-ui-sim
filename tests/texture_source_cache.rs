use std::{fs, path::Path, time::Instant};
use wow_ui_sim::{lua_api::WowLuaEnv, texture::TextureManager};

fn png(root: &Path, relative: &str, rgba: [u8; 4]) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    image::RgbaImage::from_pixel(2, 1, image::Rgba(rgba))
        .save(path)
        .unwrap();
}

fn roots(env: &WowLuaEnv, paths: &[&Path]) {
    env.state().borrow_mut().addon_base_paths = paths.iter().map(|p| p.to_path_buf()).collect();
}

fn setter(env: &WowLuaEnv, path: &str, expected: bool) {
    env.exec(&format!(
        r#"
        CacheProbe = CacheProbe or UIParent:CreateTexture('CacheProbe')
        local function check(...)
            assert(select('#', ...) == 1)
            assert(type((...)) == 'boolean' and (...) == {expected})
        end
        check(CacheProbe:SetTexture('{path}'))
        assert(CacheProbe:GetTexture() == '{path}')
    "#
    ))
    .unwrap();
}

#[test]
fn repeated_requests_live_create_delete_and_directory_rejection() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(root.path().join("F1/Media")).unwrap();
    fs::write(root.path().join("F1/Media/Present.tga"), []).unwrap();
    fs::create_dir(root.path().join("F1/Media/Directory.tga")).unwrap();
    let env = WowLuaEnv::new().unwrap();
    roots(&env, &[root.path()]);
    for _ in 0..3 {
        setter(&env, "Interface/AddOns/F1/Media/Present", true);
        setter(&env, "interface/addons/f1/media/present", true);
        setter(&env, "Interface/AddOns/F1/Media/Missing", false);
        setter(&env, "interface/addons/f1/media/directory", false);
    }
    fs::write(root.path().join("F1/Media/Missing.tga"), []).unwrap();
    setter(&env, "interface/addons/f1/media/missing", true);
    fs::remove_file(root.path().join("F1/Media/Present.tga")).unwrap();
    setter(&env, "interface/addons/f1/media/present", false);
    setter(&env, "interface/addons/newaddon/media/late", false);
    fs::create_dir_all(root.path().join("NewAddon/Media")).unwrap();
    fs::write(root.path().join("NewAddon/Media/Late.tga"), []).unwrap();
    setter(&env, "interface/addons/newaddon/media/late", true);
}

#[test]
fn setter_discovers_root_changes_and_keeps_environments_isolated() {
    let a = tempfile::tempdir().unwrap();
    let b = tempfile::tempdir().unwrap();
    png(a.path(), "F1/Media/Logo.png", [255, 0, 0, 255]);
    let env = WowLuaEnv::new().unwrap();
    roots(&env, &[a.path()]);
    setter(&env, "interface/addons/f1/media/logo", true);
    roots(&env, &[b.path()]);
    setter(&env, "interface/addons/f1/media/logo", false);
    let other = WowLuaEnv::new().unwrap();
    roots(&other, &[a.path()]);
    setter(&other, "interface/addons/f1/media/logo", true);
    roots(&env, &[a.path()]);
    setter(&env, "interface/addons/f1/media/logo", true);
}

#[test]
fn renderer_retries_missing_source_after_creation() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(root.path().join("F1/Media")).unwrap();
    let mut mgr = TextureManager::new().with_addons_path(root.path());
    let request = "Interface/AddOns/F1/Media/Late";
    assert!(mgr.load(request).is_none());
    png(root.path(), "F1/Media/Late.png", [16, 32, 48, 255]);
    let loaded = mgr
        .load(request)
        .expect("newly created texture must be discovered");
    assert_eq!((loaded.width, loaded.height), (2, 1));
    assert_eq!(&loaded.pixels[..4], &[16, 32, 48, 255]);
}

#[test]
fn exact_case_phase_precedes_ci_extension_phase() {
    let root = tempfile::tempdir().unwrap();
    png(root.path(), "F1/Media/Logo.png", [255, 0, 0, 255]);
    png(root.path(), "F1/Media/logo.webp", [0, 0, 255, 255]);
    let mgr = TextureManager::new().with_addons_path(root.path());
    for _ in 0..3 {
        assert_eq!(
            mgr.resolve_path("Interface/AddOns/F1/Media/Logo"),
            Some(root.path().join("F1/Media/Logo.png"))
        );
        assert_eq!(
            mgr.resolve_path("Interface/AddOns/F1/Media/logo"),
            Some(root.path().join("F1/Media/logo.webp"))
        );
    }
}

#[test]
fn new_earlier_root_or_extension_visible_without_reload() {
    let a = tempfile::tempdir().unwrap();
    let b = tempfile::tempdir().unwrap();
    png(b.path(), "F1/Media/Logo.png", [0, 0, 255, 255]);
    let mgr =
        TextureManager::new().with_addons_paths([a.path().to_path_buf(), b.path().to_path_buf()]);
    let p = "interface/addons/f1/media/logo";
    assert_eq!(
        mgr.resolve_path(p),
        Some(b.path().join("F1/Media/Logo.png"))
    );
    png(a.path(), "F1/Media/Logo.png", [255, 0, 0, 255]);
    assert_eq!(
        mgr.resolve_path(p),
        Some(a.path().join("F1/Media/Logo.png"))
    );
    png(a.path(), "F1/Media/Logo.webp", [0, 255, 0, 255]);
    assert_eq!(
        mgr.resolve_path(p),
        Some(a.path().join("F1/Media/Logo.webp"))
    );
    fs::remove_file(a.path().join("F1/Media/Logo.webp")).unwrap();
    assert_eq!(
        mgr.resolve_path(p),
        Some(a.path().join("F1/Media/Logo.png"))
    );
}

#[test]
fn reload_and_same_spelling_root_replacement_do_not_retain_misses() {
    let root = tempfile::tempdir().unwrap();
    let env = WowLuaEnv::new().unwrap();
    roots(&env, &[root.path()]);
    setter(&env, "interface/addons/f1/media/logo", false);
    png(root.path(), "F1/Media/Logo.png", [17, 34, 51, 255]);
    env.exec("ReloadUI()").unwrap();
    setter(&env, "interface/addons/f1/media/logo", true);
    fs::rename(root.path().join("F1"), root.path().join("old-F1")).unwrap();
    setter(&env, "interface/addons/f1/media/logo", false);
    png(root.path(), "f1/media/LOGO.png", [68, 85, 102, 255]);
    setter(&env, "interface/addons/f1/media/logo", true);
}

#[cfg(target_os = "linux")]
mod directory_io {
    use super::*;
    use std::{
        ffi::CString,
        io::Read,
        os::fd::{AsRawFd, FromRawFd},
        os::unix::ffi::OsStrExt,
    };

    // Observe actual directory opens through the kernel, not resolver internals.
    struct DirectoryObserver(fs::File);
    impl DirectoryObserver {
        fn new(path: &Path) -> Self {
            let fd = unsafe { libc::inotify_init1(libc::IN_NONBLOCK | libc::IN_CLOEXEC) };
            assert!(fd >= 0, "{}", std::io::Error::last_os_error());
            let file = unsafe { fs::File::from_raw_fd(fd) };
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
        fn opens(&mut self) -> usize {
            let mut count = 0;
            let mut buf = [0u8; 16384];
            loop {
                let size = match self.0.read(&mut buf) {
                    Ok(n) => n,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(e) => panic!("inotify read: {e}"),
                };
                assert!(size > 0);
                let mut offset = 0;
                while offset < size {
                    let event = unsafe {
                        (buf.as_ptr().add(offset) as *const libc::inotify_event).read_unaligned()
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
            count
        }
    }

    #[test]
    fn repeated_mixed_case_setters_bound_directory_enumerations() {
        let root = tempfile::tempdir().unwrap();
        let leaf = root.path().join("F1/Media/Rounded");
        fs::create_dir_all(&leaf).unwrap();
        for n in 0..64 {
            fs::write(leaf.join(format!("Present{n:03}.tga")), []).unwrap();
        }
        for n in 0..461 {
            fs::write(leaf.join(format!("Padding{n:03}.tga")), []).unwrap();
        }
        let env = WowLuaEnv::new().unwrap();
        roots(&env, &[root.path()]);
        let mut observer = DirectoryObserver::new(&leaf);
        // Calibrate independent observer: alternating open/close events prevent coalescing.
        for _ in 0..3 {
            drop(fs::read_dir(&leaf).unwrap());
        }
        assert_eq!(observer.opens(), 3);
        let start = Instant::now();
        let mut opens = 0;
        for _ in 0..4 {
            for n in 0..64 {
                setter(
                    &env,
                    &format!("interface/addons/f1/media/rounded/present{n:03}"),
                    true,
                );
                opens += observer.opens();
                setter(
                    &env,
                    &format!("interface/addons/f1/media/rounded/missing{n:03}"),
                    false,
                );
                opens += observer.opens();
            }
        }
        eprintln!(
            "F1_FIXTURE files=525 requests=512 leaf_opens={opens} elapsed_us={}",
            start.elapsed().as_micros()
        );
        assert!(
            opens <= 512,
            "repeated requests enumerated fixture directory {opens} times (budget 512)"
        );
    }
}
