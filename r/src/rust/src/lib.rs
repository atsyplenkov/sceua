use extendr_api::prelude::*;

#[extendr]
fn sceua_available() -> bool {
    let _ = sceua::Config::default();
    true
}

// Macro to generate exports for the current R package scaffold.
extendr_module! {
    mod sceua;
    fn sceua_available;
}
