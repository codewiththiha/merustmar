#[cfg(not(target_arch = "wasm32"))]
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
#[cfg(not(target_arch = "wasm32"))]
use std::io::{Write, stdout};

#[cfg(not(target_arch = "wasm32"))]
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
static INITIALIZED: AtomicBool = AtomicBool::new(false);

// --- WASM MOCKS ---
#[cfg(target_arch = "wasm32")]
pub fn init() -> Result<(), String> {
    Ok(())
}
#[cfg(target_arch = "wasm32")]
pub fn cleanup() {}
#[cfg(target_arch = "wasm32")]
pub fn clear_screen() -> Result<(), String> {
    Ok(())
}
#[cfg(target_arch = "wasm32")]
pub fn flush() -> Result<(), String> {
    Ok(())
}
#[cfg(target_arch = "wasm32")]
pub fn size() -> Result<(u16, u16), String> {
    Ok((80, 24))
}
#[cfg(target_arch = "wasm32")]
pub fn print_at(_x: u16, _y: u16, _text: &str) -> Result<(), String> {
    Ok(())
}
#[cfg(target_arch = "wasm32")]
pub fn print_at_center(
    _x: u16,
    _y: u16,
    _cols: u16,
    _rows: u16,
    _text: &str,
) -> Result<(), String> {
    Ok(())
}
#[cfg(target_arch = "wasm32")]
pub fn poll_key(_timeout_ms: u64) -> Result<Option<String>, String> {
    Ok(None)
}
#[cfg(target_arch = "wasm32")]
pub fn read_key_blocking() -> Result<String, String> {
    Err("Not supported in browser".into())
}
#[cfg(target_arch = "wasm32")]
pub fn draw_border(_cols: u16, _rows: u16) -> Result<(), String> {
    Ok(())
}

// lifecycle

#[cfg(not(target_arch = "wasm32"))]
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
#[cfg(not(target_arch = "wasm32"))]
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
#[cfg(not(target_arch = "wasm32"))]
pub fn clear_screen() -> Result<(), String> {
    stdout()
        .execute(Clear(ClearType::All))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn flush() -> Result<(), String> {
    stdout().flush().map_err(|e| e.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn size() -> Result<(u16, u16), String> {
    terminal::size().map_err(|e| e.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn print_at(x: u16, y: u16, text: &str) -> Result<(), String> {
    let mut out = stdout();
    out.execute(MoveTo(x, y)).map_err(|e| e.to_string())?;
    out.execute(Print(text)).map_err(|e| e.to_string())?;
    out.flush().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
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
#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
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
#[cfg(not(target_arch = "wasm32"))]
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
