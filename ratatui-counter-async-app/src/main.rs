use color_eyre::eyre::Result;
use tokio::sync::mpsc::{self, UnboundedSender};
use tui::Event;
mod tui;

// App 状态
struct App {
    counter: i64,
    should_quit: bool,
    action_tx: UnboundedSender<Action>,
}

// App 动作
#[derive(Clone)]
pub enum Action {
    Tick,
    Increment,
    Decrement,
    NetworkRequestAndThenIncrement,
    NetworkRequestAndThenDecrement,
    Quit,
    Render,
    None,
}

async fn run() -> Result<()> {
    // 创建一个无界的通道用于通信
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();
    let mut tui = tui::Tui::new()?.tick_rate(1.0).frame_rate(30.0);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let result = run().await;

    result?;

    Ok(())
}
