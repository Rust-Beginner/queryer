#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Menu, Submenu, MenuItem};

#[tauri::command]
async fn example_sql () -> String {
  queryer::example_sql()
}

#[tauri::command]
async fn query (sql: &str, output_fmt: Option<&str>) -> Result<String, String> {
  match output_fmt {
    Some("csv") | None => {
      let data = queryer::query(sql).await;

      data.map(|x| x.to_csv().unwrap()).map_err(|e| e.to_string())
    },
    _ => Err("Only support csv in the moment".into()),
  }
}

fn main() {
  let menu = Menu::new()
    .add_submenu(Submenu::new(
      "Queryer",
      Menu::new()
        .add_native_item(MenuItem::About("Queryer".to_string()))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Services)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::HideOthers)
        .add_native_item(MenuItem::ShowAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit),
    ))
    .add_submenu(Submenu::new("Edit", {
      let mut menu = Menu::new();
      menu = menu.add_native_item(MenuItem::Undo);
      menu = menu.add_native_item(MenuItem::Redo);
      menu = menu.add_native_item(MenuItem::Separator);
      menu = menu.add_native_item(MenuItem::Cut);
      menu = menu.add_native_item(MenuItem::Copy);
      menu = menu.add_native_item(MenuItem::Paste);
      #[cfg(not(target_os = "macos"))]
      {
        menu = menu.add_native_item(MenuItem::Separator);
      }
      menu = menu.add_native_item(MenuItem::SelectAll);
      menu
    }));

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![example_sql, query])
    .menu(menu)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
