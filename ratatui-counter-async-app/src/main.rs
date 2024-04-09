mod tui;

use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::KeyCode::Char;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{self, UnboundedSender};
use tui::Event;

// App 状态
struct App {
    counter: i64,
    should_quit: bool,
    action_tx: UnboundedSender<Action>,
}

// App 动作，可以理解表示程序有哪些功能
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

/// 将按键映射为Action
fn get_action(_app: &App, event: Event) -> Action {
    match event {
        Event::Error => Action::None,
        Event::Tick => Action::Tick,
        Event::Render => Action::Render,
        Event::Key(key) => {
            match key.code {
                Char('j') => Action::Increment,
                Char('k') => Action::Decrement,
                Char('J') => Action::NetworkRequestAndThenIncrement,
                Char('K') => Action::NetworkRequestAndThenDecrement,
                Char('q') => Action::Quit,
                _ => Action::None
            }
        },
        _ => Action::None
    }
}

fn update(app: &mut App, action: Action) {
    match action {
        Action::Increment => {
            app.counter += 1;
        },
        Action::Decrement => {
            app.counter -= 1;
        },
        Action::NetworkRequestAndThenIncrement => {
            let tx = app.action_tx.clone();
            tokio
        }
    }
}

async fn run() -> Result<()> {
    // 创建一个无界的通道用于通信
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();
    let mut tui = tui::Tui::new()?.tick_rate(1.0).frame_rate(30.0);
    tui.enter()?;

    let mut app = App {
        counter: 0,
        should_quit: false,
        action_tx: action_tx.clone()
    };

    loop {
        // 阻塞等待通道的事件
        let e = tui.next().await?;
        match e {
            tui::Event::Quit => action_tx.send(Action::Quit)?,
            tui::Event::Tick => action_tx.send(Action::Tick)?,
            tui::Event::Render => action_tx.send(Action::Render)?,
            tui::Event::Key(_) => {
                let action = get_action(&app, e);
                action_tx.send(action.clone())?;
            }
            _ => {}
        };

        while let Ok(action) = action_rx.try_recv() {
            // 应用程序更新
            update(&mut app, action.clone());
        }
    } 

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let result = run().await;

    result?;

    Ok(())
}
