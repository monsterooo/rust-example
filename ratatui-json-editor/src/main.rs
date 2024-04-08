use app::{App, CurrentScreen, CurrentlyEditing};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::error::Error;
use std::io;
use ui::ui;

mod app;
mod ui;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // 跳过非按下键盘事件
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        // 主屏幕按下e进入编辑
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(app::CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        // 主屏幕按下q进入推出提示
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    },
                    KeyCode::Char('n') => {
                        return Ok(false);
                    },
                    _ => {}
                },
                CurrentScreen::Editing => match key.code {
                    KeyCode::Enter => {
                        // 用户在编辑界面按下Enter要判断,
                        // 1. 如果是编辑key则进入编辑value界面
                        // 2. 如果是编辑value则保存值
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.currently_editing = Some(CurrentlyEditing::Value);
                                },
                                CurrentlyEditing::Value => {
                                    app.save_key_value();
                                    app.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    },
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.pop();
                                },
                                CurrentlyEditing::Value => {
                                    app.value_input.pop();
                                }
                            }
                        }
                    },
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    },
                    KeyCode::Tab => {
                        app.toggle_editing();
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing { 
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.push(value)
                                },
                                CurrentlyEditing::Value => {
                                    app.value_input.push(value);
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode();
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
