extern crate mio;
extern crate socketcan;

use std::io;
use mio::{Poll, PollOpt, Ready, Token};
use socketcan::{CANSocket, CANFrame};
use mio::event::Evented;
use mio::unix::EventedFd;

pub struct MIOCANSocket {
    socket: CANSocket
}

impl MIOCANSocket {
    pub fn from(socket: CANSocket) -> MIOCANSocket {
        MIOCANSocket {
            socket: socket
        }
    }

    pub fn read_frame(&self) -> io::Result<CANFrame> {
        self.socket.read_frame()
    }
}

impl Evented for MIOCANSocket {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()>
    {
        EventedFd(&self.socket.raw_fd()).register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        EventedFd(&self.socket.raw_fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.socket.raw_fd()).deregister(poll)
    }
}
