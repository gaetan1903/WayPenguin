use waypenguin_assets as assets;

#[test]
fn builtin_pack_loads_with_all_activities() {
    let pack = assets::builtin_pack().expect("built-in pack must load");
    assert_eq!(pack.info.id, assets::DEFAULT_PACK);
    assert!(pack.info.builtin);
    let acts = pack.activities();
    for &name in assets::ACTIVITIES {
        assert!(acts.contains(&name), "built-in pack missing '{name}'");
    }
}

#[test]
fn all_activities_render_nonempty() {
    let pack = assets::builtin_pack().unwrap();
    for &name in assets::ACTIVITIES {
        let (frames, w, h) = pack
            .render(name, 90)
            .unwrap_or_else(|| panic!("{name} failed to render"));
        assert_eq!(frames.len(), 1, "{name} should be single-frame");
        assert_eq!((w, h), (90, 90), "{name} should fit the requested size");
        let opaque = frames[0].iter().filter(|p| (*p >> 24) != 0).count();
        assert!(opaque > 0, "{name} rendered fully transparent");
        eprintln!("{name}: {w}x{h}, {opaque} opaque px");
    }
}

#[test]
fn missing_activity_falls_back_to_walker() {
    let pack = assets::builtin_pack().unwrap();
    // An activity the pack does not define should still render (via walker).
    let (frames, _, _) = pack
        .render("does-not-exist", 90)
        .expect("unknown activity should fall back");
    assert_eq!(frames.len(), 1);
}

#[test]
fn convenience_api_uses_active_pack() {
    let (frames, w, h) = assets::get_activity_frames("walker", 64).unwrap();
    assert_eq!(frames.len(), 1);
    assert_eq!((w, h), (64, 64));
}

#[test]
fn loads_a_pack_from_a_directory() {
    // Simulate a contributor dropping a pack directory on disk.
    let dir = std::env::temp_dir().join(format!("wp-pack-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("pack.toml"),
        r#"
[pack]
id = "test-pet"
name = "Test Pet"

[activities]
walker = "walk.svg"
"#,
    )
    .unwrap();
    std::fs::write(
        dir.join("walk.svg"),
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><circle cx="50" cy="50" r="40" fill="#f00"/></svg>"##,
    )
    .unwrap();

    let pack = assets::load_pack_from_dir(&dir).expect("directory pack should load");
    assert_eq!(pack.info.id, "test-pet");
    assert!(!pack.info.builtin);
    let (frames, w, h) = pack.render("walker", 48).unwrap();
    assert_eq!((frames.len(), w, h), (1, 48, 48));

    let _ = std::fs::remove_dir_all(&dir);
}
