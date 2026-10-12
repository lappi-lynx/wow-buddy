use wow_buddy::router::Route;

#[test]
fn test_all_routes_defined() {
    let routes = vec![
        Route::FactionPicker {},
        Route::RaceSelector {},
        Route::DressingRoom {},
        Route::ProgressionGuide {},
        Route::DungeonCodex {},
    ];
    assert_eq!(routes.len(), 5);
}
