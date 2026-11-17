use crate::protocol::Packet;
use std::{
    collections::VecDeque,
    io::{self, Read, Write},
    net::TcpStream,
};

pub const MAX_FRAME: usize = 64 * 1024;
const MAX_QUEUE: usize = 256 * 1024;

pub struct Connection {
    stream: TcpStream,
    incoming: Vec<u8>,
    outgoing: VecDeque<(Vec<u8>, usize)>,
    queued: usize,
    closed: bool,
}

impl Connection {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        stream.set_nodelay(true)?;
        stream.set_nonblocking(true)?;
        Ok(Self {
            stream,
            incoming: Vec::new(),
            outgoing: VecDeque::new(),
            queued: 0,
            closed: false,
        })
    }

    pub fn send(&mut self, packet: &Packet) -> io::Result<()> {
        let payload = serde_json::to_vec(packet).map_err(invalid)?;
        if payload.len() > MAX_FRAME || self.queued + payload.len() + 4 > MAX_QUEUE {
            return Err(invalid("connection send buffer exceeded"));
        }
        let mut frame = (payload.len() as u32).to_be_bytes().to_vec();
        frame.extend(payload);
        self.queued += frame.len();
        self.outgoing.push_back((frame, 0));
        self.flush()
    }

    pub fn flush(&mut self) -> io::Result<()> {
        while let Some((data, offset)) = self.outgoing.front_mut() {
            match self.stream.write(&data[*offset..]) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "peer stopped accepting data",
                    ));
                }
                Ok(n) => {
                    *offset += n;
                    self.queued -= n;
                    if *offset == data.len() {
                        self.outgoing.pop_front();
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub fn poll(&mut self) -> io::Result<Vec<Packet>> {
        self.flush()?;
        let mut chunk = [0; 8192];
        while !self.closed && self.incoming.len() < MAX_QUEUE {
            match self.stream.read(&mut chunk) {
                Ok(0) => self.closed = true,
                Ok(n) => self.incoming.extend_from_slice(&chunk[..n]),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
        let mut consumed = 0;
        let mut packets = Vec::new();
        while self.incoming.len() - consumed >= 4 && packets.len() < 64 {
            let size = u32::from_be_bytes(self.incoming[consumed..consumed + 4].try_into().unwrap())
                as usize;
            if size == 0 || size > MAX_FRAME {
                return Err(invalid("invalid frame length"));
            }
            if self.incoming.len() - consumed < size + 4 {
                break;
            }
            packets.push(
                serde_json::from_slice(&self.incoming[consumed + 4..consumed + 4 + size])
                    .map_err(invalid)?,
            );
            consumed += size + 4;
        }
        self.incoming.drain(..consumed);
        if self.incoming.len() >= MAX_QUEUE {
            return Err(invalid("connection receive buffer exceeded"));
        }
        if self.closed && packets.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "peer disconnected",
            ));
        }
        Ok(packets)
    }
}

pub fn invalid(error: impl ToString) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}
