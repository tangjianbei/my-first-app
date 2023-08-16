use tauri::{SystemTrayEvent, SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri::Manager;

pub fn handler(app_handle: &tauri::AppHandle, event: SystemTrayEvent) {
    // 匹配点击事件
    match event {
        // 根据菜单 id 进行事件匹配
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
              }
              "hide" => {
                let item_handle = app_handle.tray_handle().get_item(&id);
                let window = app_handle.get_window("external").unwrap();
                if window.is_visible().unwrap() {
                  window.hide().unwrap();
                  item_handle.set_title("显示窗口").unwrap();
                } else {
                  window.show().unwrap();
                  item_handle.set_title("隐藏窗口").unwrap();
                }
              }
              "oa" => {
                tauri::WindowBuilder::new(
                  app_handle,
                  "external", /* the unique window label */
                  tauri::WindowUrl::External("http://oa.chison.com.cn:44366/wui/index.html#/?logintype=1&time=1673233732321&_key=hjxkva".parse().unwrap())
                ).build().unwrap();
              }
            _ => {}
        },
        _ => {}
    }
}

pub fn menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "关闭窗口");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏窗口");
    let oa = CustomMenuItem::new("oa".to_string(), "OA系统");
    let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide)
    .add_item(oa);

    // 设置在右键单击系统托盘时显示菜单
    SystemTray::new().with_menu(tray_menu)
}

// pub fn open_oa(handle: tauri::AppHandle) {
//     let docs_window = tauri::WindowBuilder::new(
//       &handle,
//       "external", /* the unique window label */
//       tauri::WindowUrl::External("http://oa.chison.com.cn:44366/wui/index.html#/?logintype=1&time=1673233732321&_key=hjxkva".parse().unwrap())
//     ).build().unwrap();
// }