use crate::tui;
use color_eyre::{config::HookBuilder, eyre};
use std::panic;
pub fn install_hooks() -> color_eyre::Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
    let panic_hook = panic_hook.into_panic_hook();

    // 转换color_eyre PanicHook到一个标准的 panic hook
    panic::set_hook(Box::new(move |panic_info| {
        tui::restore().unwrap();
        panic_hook(panic_info);
    }));

    // 转换color_eyre EyreHook 到 eyre ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(
        move |error: &(dyn std::error::Error + 'static)| {
            tui::restore().unwrap();
            eyre_hook(error)
        },
    ))?;

    Ok(())
}
