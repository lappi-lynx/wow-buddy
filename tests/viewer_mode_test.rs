use wow_buddy::components::model_viewer::{choose_mode, ViewerMode};

#[test]
fn webgl_without_model_is_not_3d() {
    assert_eq!(choose_mode(true, false, true), ViewerMode::PaperDoll);
    assert_eq!(choose_mode(true, false, false), ViewerMode::Placeholder);
    assert_eq!(choose_mode(true, true, true), ViewerMode::Model3d);
    assert_eq!(choose_mode(false, true, true), ViewerMode::PaperDoll);
}
