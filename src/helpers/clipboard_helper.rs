use clipboard::{ClipboardContext, ClipboardProvider};

pub fn paste(data: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(data.to_owned()).unwrap();
}
