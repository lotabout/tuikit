use crate::attr::Attr;
use crate::event::Event;
use crate::input::KeyBoard;
use crate::key::Key;
use crate::output::Command;
use crate::output::Output;
use crate::raw::{get_tty, IntoRawMode};
use crate::screen::Cell;
use crate::screen::Screen;
use std::cmp::{max, min};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use crate::sys::signal::{notify_on_sigwinch, unregister_sigwinch, initialize_signals};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

const MIN_HEIGHT: usize = 5;

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

    state: RwLock<TermState>,
    screen: Mutex<Screen>,
    output: Mutex<Option<Output>>,

    event_rx: Mutex<Receiver<Event>>,
    event_tx: Sender<Event>,
}

#[derive(Debug)]
struct TermState {
    pub prefer_height: TermHeight,
    pub bottom_intact: bool, // keep bottom intact when resize?
    pub cursor_row: usize,
    pub screen_height: usize,
    pub screen_width: usize,
}

impl Default for TermState {
    fn default() -> Self {
        Self {
            prefer_height: TermHeight::Percent(100),
            bottom_intact: false,
            cursor_row: 0,
            screen_height: 0,
            screen_width: 0,
        }
    }
}

impl Term {
    pub fn with_height(height: TermHeight) -> Term {
        initialize_signals();

        let (event_tx, event_rx) = channel();
        let ret = Term {
            stopped: Arc::new(AtomicBool::new(true)),
            state: RwLock::new(TermState {
                prefer_height: height,
                ..TermState::default()
            }),
            screen: Mutex::new(Screen::new(0, 0)),
            output: Mutex::new(None),
            event_tx,
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

    fn calc_preferred_height(&self, prefer: &TermHeight, height: usize) -> usize {
        match *prefer {
            TermHeight::Fixed(h) => min(h, height),
            TermHeight::Percent(p) => max(MIN_HEIGHT, height * min(p, 100) / 100),
        }
    }

    fn resize(&self, screen_width: usize, screen_height: usize) -> Result<()> {
        let mut state = self.state.write().unwrap();

        let width = screen_width;
        let height = self.calc_preferred_height(&state.prefer_height, screen_height);

        // update the cursor position
        if state.cursor_row + height >= screen_height {
            state.bottom_intact = true;
        }

        if state.bottom_intact {
            state.cursor_row = screen_height - height;
        }

        state.screen_height = screen_height;
        state.screen_width = screen_width;

        let mut screen = self
            .screen
            .lock()
            .expect("termbox:resize failed to get screen");
        screen.resize(width, height);
        Ok(())
    }

    /// Present the content to the terminal
    pub fn present(&self) -> Result<()> {
        self.ensure_not_stopped()?;

        // lock necessary components
        let state = self.state.read().unwrap();
        let mut commands = self
            .screen
            .lock()
            .expect("termbox:present failed to get screen")
            .present();
        let mut mutex_output = self
            .output
            .lock()
            .expect("termbox:present faied to lock output");
        let output = mutex_output.as_mut().unwrap();

        let cursor_row = state.cursor_row;
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

    /// Pause the terminal
    pub fn pause(&self) -> Result<()> {
        self.stopped.store(true, Ordering::Relaxed);
        self.output.lock().unwrap().take().map(|mut output| {
            output.quit_alternate_screen();
            output.flush();
        });
        Ok(())
    }

    fn get_cursor_pos(
        &self,
        keyboard: &mut KeyBoard,
        output: &mut Output,
    ) -> Result<(usize, usize)> {
        output.ask_for_cpr();

        const MAX_RETRY: u8 = 10;
        for _ in 0..MAX_RETRY {
            let key = keyboard.next_key();
            if let Ok(Key::CursorPos(row, col)) = key {
                return Ok((row as usize, col as usize));
            }
        }
        Err("termbox:get_cursor_pos failed to get CPR response after max retries".into())
    }

    fn ensure_height(&self, mut keyboard: &mut KeyBoard, mut output: &mut Output) {
        let mut state = self.state.write().unwrap();

        // initialize
        let (screen_width, screen_height) = output
            .terminal_size()
            .expect("term:restart get terminal size failed");
        let (cursor_row, _cursor_col) = self.get_cursor_pos(&mut keyboard, &mut output).unwrap();
        let height_to_be = self.calc_preferred_height(&state.prefer_height, screen_height);

        if height_to_be >= screen_height {
            // whole screen
            output.enter_alternate_screen();
            state.bottom_intact = false;
            state.cursor_row = 0;
        } else if (cursor_row + height_to_be) <= screen_height {
            state.bottom_intact = false;
            state.cursor_row = cursor_row;
        } else {
            for _ in 0..(height_to_be - 1) {
                output.write("\n");
            }
            state.bottom_intact = true;
            state.cursor_row = min(cursor_row, screen_height - height_to_be);
        }
        output.cursor_goto(state.cursor_row, 0);
        output.flush();
        state.screen_height = screen_height;
        state.screen_width = screen_width;
    }

    /// restart the terminal
    pub fn restart(&self) -> Result<()> {
        if !self.stopped.load(Ordering::Relaxed) {
            return Ok(());
        }

        let mut mutex_output = self
            .output
            .lock()
            .expect("termbox:restart failed to lock output");

        // grab input/output
        let ttyout = get_tty()?.into_raw_mode()?;
        let mut output = Output::new(Box::new(ttyout))?;
        let mut keyboard = KeyBoard::new_with_tty();

        // ensure the output area had enough height
        self.ensure_height(&mut keyboard, &mut output);
        let (screen_width, screen_height) = output
            .terminal_size()
            .expect("term:restart get terminal size failed");
        self.resize(screen_width, screen_height)?;

        // store the output
        mutex_output.replace(output);
        self.start_key_listener(keyboard);
        self.start_size_change_listener();
        self.stopped.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn start_key_listener(&self, mut keyboard: KeyBoard) {
        self.stopped.store(false, Ordering::SeqCst);
        let event_tx = self.event_tx.clone();
        let stopped = self.stopped.clone();
        thread::spawn(move || {
            let timeout = Duration::from_millis(20);
            loop {
                if let Ok(key) = keyboard.next_key_timeout(timeout) {
                    let _ = event_tx.send(Event::Key(key));
                }

                if stopped.load(Ordering::Relaxed) {
                    break;
                }
            }
        });
    }

    fn start_size_change_listener(&self) {
        self.stopped.store(false, Ordering::SeqCst);
        let event_tx = self.event_tx.clone();
        let stopped = self.stopped.clone();
        thread::spawn(move ||{
            let (id, sigwinch_rx) = notify_on_sigwinch();
            loop {
                if let Ok(_) = sigwinch_rx.recv_timeout(Duration::from_millis(20)) {
                    let _ = event_tx.send(Event::Resize {width: 0, height: 0});
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
            Event::Resize{width: _, height: _} => {
                // resize should be handled internally before sending out
                let mut mutex_output = self.output.lock().unwrap();
                let output = mutex_output.as_mut().unwrap();
                let (width, height) = output.terminal_size().unwrap();
                let _ = self.resize(width, height);
                Event::Resize {width, height}
            }
            ev => ev
        }
    }

    /// wait an event up to timeout_mills milliseconds and return it
    pub fn peek_event(&self, timeout: Duration) -> Result<Event> {
        self.ensure_not_stopped()?;
        let event_rx = self.event_rx.lock().unwrap();
        event_rx
            .recv_timeout(timeout)
            .map(|ev| self.filter_event(ev))
            .map_err(|_| "timeout".to_string().into())
    }

    /// wait for an event and return it
    pub fn poll_event(&self) -> Result<Event> {
        self.ensure_not_stopped()?;
        let event_rx = self.event_rx.lock().unwrap();
        event_rx.recv()
            .map(|ev| self.filter_event(ev))
            .map_err(|_| "timeout".to_string().into())
    }

    /// return the printable size(width, height) of the term
    pub fn term_size(&self) -> Result<(usize, usize)> {
        self.ensure_not_stopped()?;
        let screen = self.screen.lock().unwrap();
        Ok((screen.width(), screen.height()))
    }

    pub fn clear(&self) -> Result<()> {
        self.ensure_not_stopped()?;
        let mut screen = self.screen.lock().unwrap();
        screen.clear();
        Ok(())
    }

    /// change a cell of position `(row, col)` to `cell`
    pub fn put_cell(&self, row: usize, col: usize, cell: Cell) -> Result<()> {
        self.ensure_not_stopped()?;
        let mut screen = self.screen.lock().unwrap();
        screen.put_cell(row, col, cell);
        Ok(())
    }

    /// print `content` starting with position `(row, col)`
    pub fn print(&self, row: usize, col: usize, content: &str) -> Result<()> {
        self.print_with_attr(row, col, content, Attr::default())
    }

    /// print `content` starting with position `(row, col)` with `attr`
    pub fn print_with_attr(&self, row: usize, col: usize, content: &str, attr: Attr) -> Result<()> {
        self.ensure_not_stopped()?;
        let mut screen = self.screen.lock().unwrap();
        screen.print(row, col, content, attr);
        Ok(())
    }

    /// set cursor position to (row, col)
    pub fn set_cursor(&mut self, row: usize, col: usize) -> Result<()> {
        self.ensure_not_stopped()?;
        let mut screen = self.screen.lock().unwrap();
        screen.set_cursor(row, col);
        Ok(())
    }

    /// show/hide cursor, set `show` to `false` to hide the cursor
    pub fn show_cursor(&mut self, show: bool) -> Result<()> {
        self.ensure_not_stopped()?;
        let mut screen = self.screen.lock().unwrap();
        screen.show_cursor(show);
        Ok(())
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        let _ = self.pause();
    }
}
