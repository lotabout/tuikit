use crate::attr::Attr;
use crate::event::Event;
use crate::input::KeyBoard;
use crate::key::Key;
use crate::output::Command;
use crate::output::Output;
use crate::raw::{get_tty, IntoRawMode};
use crate::screen::Cell;
use crate::screen::Screen;
use crate::sys::signal::{initialize_signals, notify_on_sigwinch, unregister_sigwinch};
use std::cmp::{max, min};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

const MIN_HEIGHT: usize = 5;
const WAIT_TIMEOUT: Duration = Duration::from_millis(20);

#[derive(Debug)]
pub enum TermHeight {
    Fixed(usize),
    Percent(usize),
}

/// terminal
///
/// ```no_run
/// use tuikit::term::Term;
/// let term = Term::new();
/// term.print(0, 0, "I love tuikit");
/// term.present();
/// ```
pub struct Term {
    stopped: Arc<AtomicBool>,
    term_lock: Mutex<TermLock>,
    event_rx: Mutex<Receiver<Event>>,
    event_tx: Arc<Mutex<Sender<Event>>>,
}

impl Term {
    pub fn with_height(height: TermHeight) -> Term {
        initialize_signals();

        let (event_tx, event_rx) = channel();
        let ret = Term {
            stopped: Arc::new(AtomicBool::new(true)),
            term_lock: Mutex::new(TermLock::with_height(height)),
            event_tx: Arc::new(Mutex::new(event_tx)),
            event_rx: Mutex::new(event_rx),
        };
        let _ = ret.restart();
        ret
    }

    pub fn new() -> Term {
        Term::with_height(TermHeight::Percent(100))
    }

    fn ensure_not_stopped(&self) -> Result<()> {
        if !self.stopped.load(Ordering::Relaxed) {
            Ok(())
        } else {
            Err("Terminal had been paused, should `restart` to use".into())
        }
    }

    fn get_cursor_pos(
        &self,
        keyboard: &mut KeyBoard,
        output: &mut Output,
    ) -> Result<(usize, usize)> {
        output.ask_for_cpr();

        while let Ok(key) = keyboard.next_key_timeout(WAIT_TIMEOUT) {
            if let Key::CursorPos(row, col) = key {
                return Ok((row as usize, col as usize));
            }
        }
        Err("term:get_cursor_pos failed to get CPR response after max retries".into())
    }

    /// restart the terminal
    pub fn restart(&self) -> Result<()> {
        if !self.stopped.load(Ordering::Relaxed) {
            return Ok(());
        }

        let mut termlock = self
            .term_lock
            .lock()
            .expect("term:restart lock term failed");

        let ttyout = get_tty()?.into_raw_mode()?;
        let mut output = Output::new(Box::new(ttyout))?;
        let mut keyboard = KeyBoard::new_with_tty();
        let cursor_pos = self.get_cursor_pos(&mut keyboard, &mut output)?;
        termlock.restart(output, cursor_pos)?;

        self.start_key_listener(keyboard);
        self.start_size_change_listener();
        self.stopped.store(false, Ordering::SeqCst);

        let event_tx = self
            .event_tx
            .lock()
            .expect("term:restart failed to lock event sender");
        let _ = event_tx.send(Event::Restarted);

        Ok(())
    }

    /// Pause the terminal
    pub fn pause(&self) -> Result<()> {
        self.ensure_not_stopped()?;
        self.stopped.store(true, Ordering::SeqCst);
        let mut termlock = self
            .term_lock
            .lock()
            .expect("term:term_size: failed to lock terminal");
        termlock.pause()?;

        let event_tx = self
            .event_tx
            .lock()
            .expect("term:restart failed to lock event sender");
        let _ = event_tx.send(Event::Stopped);

        Ok(())
    }

    fn start_key_listener(&self, mut keyboard: KeyBoard) {
        self.stopped.store(false, Ordering::SeqCst);
        let event_tx_clone = self.event_tx.clone();
        let stopped = self.stopped.clone();
        thread::spawn(move || loop {
            if let Ok(key) = keyboard.next_key_timeout(WAIT_TIMEOUT) {
                let event_tx = event_tx_clone
                    .lock()
                    .expect("term:key-listener failed to lock event sender");
                let _ = event_tx.send(Event::Key(key));
            }

            if stopped.load(Ordering::Relaxed) {
                break;
            }
        });
    }

    fn start_size_change_listener(&self) {
        self.stopped.store(false, Ordering::SeqCst);
        let event_tx_clone = self.event_tx.clone();
        let stopped = self.stopped.clone();
        thread::spawn(move || {
            let (id, sigwinch_rx) = notify_on_sigwinch();
            loop {
                if let Ok(_) = sigwinch_rx.recv_timeout(WAIT_TIMEOUT) {
                    let event_tx = event_tx_clone
                        .lock()
                        .expect("term:size-listener failed to lock event sender");
                    let _ = event_tx.send(Event::Resize {
                        width: 0,
                        height: 0,
                    });
                }

                if stopped.load(Ordering::Relaxed) {
                    break;
                }
            }
            unregister_sigwinch(id);
        });
    }

    fn filter_event(&self, event: Event) -> Event {
        match event {
            Event::Resize {
                width: _,
                height: _,
            } => {
                {
                    let mut termlock = self
                        .term_lock
                        .lock()
                        .expect("term:filter_event failed to lock terminal");
                    let _ = termlock.on_resize();
                }
                let (width, height) = self.term_size().unwrap_or((0, 0));
                Event::Resize { width, height }
            }
            ev => ev,
        }
    }

    /// wait an event up to timeout_mills milliseconds and return it
    pub fn peek_event(&self, timeout: Duration) -> Result<Event> {
        let event_rx = self.event_rx.lock().unwrap();
        event_rx
            .recv_timeout(timeout)
            .map(|ev| self.filter_event(ev))
            .map_err(|_| "timeout".to_string().into())
    }

    /// wait for an event and return it
    pub fn poll_event(&self) -> Result<Event> {
        let event_rx = self.event_rx.lock().unwrap();
        event_rx
            .recv()
            .map(|ev| self.filter_event(ev))
            .map_err(|_| "timeout".to_string().into())
    }

    /// Present the content to the terminal
    pub fn present(&self) -> Result<()> {
        self.ensure_not_stopped()?;
        let mut termlock = self
            .term_lock
            .lock()
            .expect("term:term_size: failed to lock terminal");
        termlock.present()
    }

    /// return the printable size(width, height) of the term
    pub fn term_size(&self) -> Result<(usize, usize)> {
        self.ensure_not_stopped()?;
        let termlock = self
            .term_lock
            .lock()
            .expect("term:term_size: failed to lock terminal");
        Ok(termlock.term_size()?)
    }

    pub fn clear(&self) -> Result<()> {
        self.ensure_not_stopped()?;
        let mut termlock = self
            .term_lock
            .lock()
            .expect("term:term_size: failed to lock terminal");
        termlock.clear()
    }

    /// change a cell of position `(row, col)` to `cell`
    pub fn put_cell(&self, row: usize, col: usize, cell: Cell) -> Result<()> {
        self.ensure_not_stopped()?;
        let mut termlock = self
            .term_lock
            .lock()
            .expect("term:term_size: failed to lock terminal");
        termlock.put_cell(row, col, cell)
    }

    /// print `content` starting with position `(row, col)`
    pub fn print(&self, row: usize, col: usize, content: &str) -> Result<()> {
        self.print_with_attr(row, col, content, Attr::default())
    }

    /// print `content` starting with position `(row, col)` with `attr`
    pub fn print_with_attr(&self, row: usize, col: usize, content: &str, attr: Attr) -> Result<()> {
        self.ensure_not_stopped()?;
        let mut termlock = self
            .term_lock
            .lock()
            .expect("term:term_size: failed to lock terminal");
        termlock.print(row, col, content, attr)
    }

    /// set cursor position to (row, col)
    pub fn set_cursor(&mut self, row: usize, col: usize) -> Result<()> {
        self.ensure_not_stopped()?;
        let mut termlock = self
            .term_lock
            .lock()
            .expect("term:term_size: failed to lock terminal");
        termlock.set_cursor(row, col)
    }

    /// show/hide cursor, set `show` to `false` to hide the cursor
    pub fn show_cursor(&mut self, show: bool) -> Result<()> {
        self.ensure_not_stopped()?;
        let mut termlock = self
            .term_lock
            .lock()
            .expect("term:term_size: failed to lock terminal");
        termlock.show_cursor(show)
    }
}

struct TermLock {
    prefer_height: TermHeight,
    bottom_intact: bool, // keep bottom intact when resize?
    cursor_row: usize,
    screen_height: usize,
    screen_width: usize,
    screen: Screen,
    output: Option<Output>,
}

impl Default for TermLock {
    fn default() -> Self {
        Self {
            prefer_height: TermHeight::Percent(100),
            bottom_intact: false,
            cursor_row: 0,
            screen_height: 0,
            screen_width: 0,
            screen: Screen::new(0, 0),
            output: None,
        }
    }
}

impl TermLock {
    pub fn with_height(height: TermHeight) -> Self {
        let mut term = TermLock::default();
        term.prefer_height = height;
        term
    }

    /// Present the content to the terminal
    pub fn present(&mut self) -> Result<()> {
        let output = self.output.as_mut().ok_or("term had been stopped")?;
        let mut commands = self.screen.present();

        let cursor_row = self.cursor_row;
        // add cursor_row to all CursorGoto commands
        for cmd in commands.iter_mut() {
            if let Command::CursorGoto { row, col } = *cmd {
                *cmd = Command::CursorGoto {
                    row: row + cursor_row,
                    col,
                }
            }
        }

        for cmd in commands.into_iter() {
            output.execute(cmd);
        }
        output.flush();
        Ok(())
    }

    /// Resize the internal buffer to according to new terminal size
    pub fn on_resize(&mut self) -> Result<()> {
        let output = self.output.as_mut().ok_or("term had been stopped")?;
        let (screen_width, screen_height) = output
            .terminal_size()
            .expect("term:restart get terminal size failed");
        let width = screen_width;
        let height = Self::calc_preferred_height(&self.prefer_height, screen_height);

        // update the cursor position
        if self.cursor_row + height >= screen_height {
            self.bottom_intact = true;
        }

        if self.bottom_intact {
            self.cursor_row = screen_height - height;
        }

        self.screen_height = screen_height;
        self.screen_width = screen_width;
        self.screen.resize(width, height);
        Ok(())
    }

    fn calc_preferred_height(prefer: &TermHeight, height: usize) -> usize {
        match *prefer {
            TermHeight::Fixed(h) => min(h, height),
            TermHeight::Percent(p) => max(MIN_HEIGHT, height * min(p, 100) / 100),
        }
    }

    /// Pause the terminal
    pub fn pause(&mut self) -> Result<()> {
        self.output.take().map(|mut output| {
            output.quit_alternate_screen();
            output.flush();
        });
        Ok(())
    }

    /// ensure the screen had enough height
    /// If the prefer height is full screen, it will enter alternate screen
    /// otherwise it will ensure there are enough lines at the bottom
    fn ensure_height(&mut self, cursor_pos: (usize, usize)) -> Result<()> {
        let output = self.output.as_mut().ok_or("term had been stopped")?;

        // initialize

        let (screen_width, screen_height) = output
            .terminal_size()
            .expect("termlock:ensure_height get terminal size failed");
        let height_to_be = Self::calc_preferred_height(&self.prefer_height, screen_height);

        let (cursor_row, _cursor_col) = cursor_pos;
        if height_to_be >= screen_height {
            // whole screen
            output.enter_alternate_screen();
            self.bottom_intact = false;
            self.cursor_row = 0;
        } else if (cursor_row + height_to_be) <= screen_height {
            self.bottom_intact = false;
            self.cursor_row = cursor_row;
        } else {
            for _ in 0..(height_to_be - 1) {
                output.write("\n");
            }
            self.bottom_intact = true;
            self.cursor_row = min(cursor_row, screen_height - height_to_be);
        }

        output.cursor_goto(self.cursor_row, 0);
        output.flush();
        self.screen_height = screen_height;
        self.screen_width = screen_width;
        Ok(())
    }

    /// restart the terminal
    pub fn restart(&mut self, output: Output, cursor_pos: (usize, usize)) -> Result<()> {
        // ensure the output area had enough height
        self.output.replace(output);
        self.ensure_height(cursor_pos)?;
        self.on_resize()?;
        Ok(())
    }

    /// return the printable size(width, height) of the term
    pub fn term_size(&self) -> Result<(usize, usize)> {
        Ok((self.screen.width(), self.screen.height()))
    }

    /// clear internal buffer
    pub fn clear(&mut self) -> Result<()> {
        self.screen.clear();
        Ok(())
    }

    /// change a cell of position `(row, col)` to `cell`
    pub fn put_cell(&mut self, row: usize, col: usize, cell: Cell) -> Result<()> {
        self.screen.put_cell(row, col, cell);
        Ok(())
    }

    /// print `content` starting with position `(row, col)`
    pub fn print(&mut self, row: usize, col: usize, content: &str, attr: Attr) -> Result<()> {
        self.screen.print(row, col, content, attr);
        Ok(())
    }

    /// set cursor position to (row, col)
    pub fn set_cursor(&mut self, row: usize, col: usize) -> Result<()> {
        self.screen.set_cursor(row, col);
        Ok(())
    }

    /// show/hide cursor, set `show` to `false` to hide the cursor
    pub fn show_cursor(&mut self, show: bool) -> Result<()> {
        self.screen.show_cursor(show);
        Ok(())
    }
}

impl Drop for TermLock {
    fn drop(&mut self) {
        let _ = self.pause();
    }
}
