use rust_ark::console::window::Window;

#[test]
fn create_window() {
    let win = Window::new(1024, 968);
    assert_eq!(win.width, 1024);
    assert_eq!(win.height, 968);
}

#[test]
fn add_tab() {
    let mut win = Window::new(1024, 968);
    win.add_tab(String::from("Hello"));
    assert_eq!(win.tabs.len(), 1);
    assert!(win.tabs[0].name.eq("Hello"));
}
