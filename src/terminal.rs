use mio::unix::SourceFd;
use mio::{Events, Interest, Poll, Token};
use std::io;
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{AtomicBool, Ordering};
use termios::*;

pub struct InputBuffering {
    input_buffering_enabled: AtomicBool,
    original_tio: Option<Termios>,
}

impl InputBuffering {
    pub fn disable() -> Self {
        /* disable input buffering */

        let fd = io::stdin().as_raw_fd();
        let mut tio = Termios::from_fd(fd).unwrap();

        tio.c_lflag &= !(ICANON | ECHO);
        tcsetattr(fd, TCSANOW, &tio).unwrap();

        Self {
            input_buffering_enabled: AtomicBool::new(false),
            original_tio: Some(tio),
        }
    }
}

impl Drop for InputBuffering {
    fn drop(&mut self) {
        /* restore input buffering */

        let stdin = io::stdin();
        let fd = stdin.as_raw_fd();

        if let Some(ref original_tio) = self.original_tio {
            tcsetattr(fd, TCSANOW, original_tio).unwrap();
            self.input_buffering_enabled.store(true, Ordering::SeqCst);
        }
    }
}

const STDIN: Token = Token(0);

pub fn check_key() -> io::Result<bool> {
    let stdin_fd = io::stdin().as_raw_fd();

    let mut poll = Poll::new().expect("Failed to create Poll instance");
    let mut events = Events::with_capacity(1024);

    let mut source_fd = SourceFd(&stdin_fd);
    poll.registry()
        .register(&mut source_fd, STDIN, Interest::READABLE)
        .expect("Failed to register stdin");

    poll.poll(&mut events, Some(std::time::Duration::from_secs(0)))
        .expect("Failed to poll events");

    for event in events.iter() {
        if event.token() == STDIN && event.is_readable() {
            return Ok(true);
        }
    }

    Ok(false)
}
