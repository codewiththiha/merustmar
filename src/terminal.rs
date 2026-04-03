use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{Write, stdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

// lifecycle

pub fn init() -> Result<(), String> {
    if INITIALIZED.load(Ordering::SeqCst) {
        return Err("terminal already initialized".into());
    }
    let mut out = stdout();
    out.execute(EnterAlternateScreen)
        .map_err(|e| e.to_string())?;
    terminal::enable_raw_mode().map_err(|e| e.to_string())?;
    out.execute(Hide).map_err(|e| e.to_string())?;
    INITIALIZED.store(true, Ordering::SeqCst);
    Ok(())
}

/// Restores the terminal. Safe to call multiple times / when not initialized.
pub fn cleanup() {
    if !INITIALIZED.swap(false, Ordering::SeqCst) {
        return; // was not initialized
    }
    let mut out = stdout();
    let _ = out.execute(Show);
    let _ = terminal::disable_raw_mode();
    let _ = out.execute(LeaveAlternateScreen);
}

// primitives
pub fn clear_screen() -> Result<(), String> {
    stdout()
        .execute(Clear(ClearType::All))
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn flush() -> Result<(), String> {
    stdout().flush().map_err(|e| e.to_string())
}

pub fn size() -> Result<(u16, u16), String> {
    terminal::size().map_err(|e| e.to_string())
}

pub fn print_at(x: u16, y: u16, text: &str) -> Result<(), String> {
    let mut out = stdout();
    out.execute(MoveTo(x, y)).map_err(|e| e.to_string())?;
    out.execute(Print(text)).map_err(|e| e.to_string())?;
    out.flush().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn print_at_center(
    x: u16,
    y: u16,
    box_cols: u16,
    box_rows: u16,
    text: &str,
) -> Result<(), String> {
    let (sw, sh) = size()?;
    let cx = sw.saturating_sub(box_cols) / 2;
    let cy = sh.saturating_sub(box_rows) / 2;
    print_at(cx + x, cy + y, text)
}

//  input
pub fn poll_key(timeout_ms: u64) -> Result<Option<String>, String> {
    if event::poll(Duration::from_millis(timeout_ms)).map_err(|e| e.to_string())? {
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event::read().map_err(|e| e.to_string())?
        {
            return Ok(Some(keycode_str(code)));
        }
    }
    Ok(None)
}

pub fn read_key_blocking() -> Result<String, String> {
    loop {
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event::read().map_err(|e| e.to_string())?
        {
            return Ok(keycode_str(code));
        }
    }
}

fn keycode_str(code: KeyCode) -> String {
    match code {
        KeyCode::Up => "up".into(),
        KeyCode::Down => "down".into(),
        KeyCode::Left => "left".into(),
        KeyCode::Right => "right".into(),
        KeyCode::Enter => "enter".into(),
        KeyCode::Esc => "esc".into(),
        KeyCode::Backspace => "backspace".into(),
        KeyCode::Tab => "tab".into(),
        KeyCode::Char(c) => c.to_string(),
        _ => "unknown".into(),
    }
}

// draw_border (centred)
pub fn draw_border(cols: u16, rows: u16) -> Result<(), String> {
    let (sw, sh) = size()?;
    let sx = sw.saturating_sub(cols) / 2;
    let sy = sh.saturating_sub(rows) / 2;

    // horizontals
    for x in 0..cols {
        print_at(sx + x, sy, "─")?;
        print_at(sx + x, sy + rows - 1, "─")?;
    }
    // verticals
    for y in 0..rows {
        print_at(sx, sy + y, "│")?;
        print_at(sx + cols - 1, sy + y, "│")?;
    }
    // corners
    print_at(sx, sy, "┌")?;
    print_at(sx + cols - 1, sy, "┐")?;
    print_at(sx, sy + rows - 1, "└")?;
    print_at(sx + cols - 1, sy + rows - 1, "┘")?;
    Ok(())
}
