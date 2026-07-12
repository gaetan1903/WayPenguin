#[test]
fn all_activities_render_nonempty() {
    for &name in waypenguin_assets::ACTIVITIES {
        let (frames, w, h) = waypenguin_assets::get_activity_frames(name, 90)
            .unwrap_or_else(|| panic!("{name} failed to render"));
        assert_eq!(frames.len(), 1, "{name} should be single-frame");
        assert_eq!((w, h), (90, 90), "{name} should fit the requested size");
        let opaque: usize = frames[0].iter().filter(|p| (*p >> 24) != 0).count();
        assert!(opaque > 0, "{name} rendered fully transparent");
        eprintln!("{name}: {w}x{h}, {opaque} opaque px");
    }
}
