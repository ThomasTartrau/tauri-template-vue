use tauri::{menu::{Menu, MenuItem, Submenu}, AppHandle};

pub fn build_menu(handle: &AppHandle) {
    let submenu = Submenu::with_items(handle, "Edit", true, &[
        &MenuItem::with_id(handle, "preferences-item", "Preferences", true, Some("CommandOrControl+,")).unwrap(),
        &Submenu::with_id_and_items(handle, "test", "test-cc", true, &[
            &MenuItem::with_id(handle, "test-item", "Test", true, Some("CommandOrControl+T")).unwrap(),
        ]).unwrap(),
    ]).unwrap();

    let menu = Menu::with_items(handle, &[&submenu]).unwrap();
    handle.set_menu(menu);
}