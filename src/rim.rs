
struct RIM {}

impl RIM {
  fn log(&txt) {
    println!(txt);
  }
  
  fn error(&head,&msg) {
    eprintln!("{head}: {msg}");
  }

  fn debug_mode() {
    // TODO
  }
}

macro_rules! log {
    ($($args:expr),*) => {{
        $(
            println!("{}", $args);
        )*
    }}
}
