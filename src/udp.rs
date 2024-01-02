use std::collections::VecDeque;
use std::io;
use std::time;

pub enum State {
    Established,
    Closed,
}

pub struct Connection {
    pub(crate) state: State,
    timers: Timers,
    pub(crate) incoming: VecDeque<u8>,
    pub(crate) unacked: VecDeque<u8>,
    closed: bool,
    closed_at: Option<u32>,
}

struct Timers {
    send_times: BTreeMap<u32, time::Instant>,
    srtt: f64,
}

impl Connection {
    pub(crate) fn is_rcv_closed(&self) -> bool {
        matches!(self.state, State::Closed)
    }

    fn availability(&self) -> Available {
        let mut a = Available::empty();
        if self.is_rcv_closed() || !self.incoming.is_empty() {
            a |= Available::READ;
        }
        // TODO: set available WRITE
        a
    }
}

impl Connection {
    pub fn accept(
        _ip_header: etherparse::Ipv4HeaderSlice,
        _udp_header: etherparse::UdpHeaderSlice,
        _data: &[u8],
    ) -> io::Result<Option<Self>> {
        let iss = 0;
        let mut c = Self {
            closed: false,
            closed_at: None,
            timers: Timers {
                send_times: Default::default(),
                srtt: time::Duration::from_secs(60).as_secs_f64(),
            },
            state: State::Established,
            incoming: VecDeque::new(),
            unacked: VecDeque::new(),
        };

        Ok(Some(c))
    }

    fn send_data(
        &mut self,
        _seq: u32,
        _limit: usize,
    ) -> io::Result<usize> {
        // TODO: Implement UDP data sending logic
        Ok(0)
    }

    pub fn on_packet(
        &mut self,
        _udp_header: etherparse::UdpHeaderSlice,
        data: &[u8],
    ) -> io::Result<Available> {
        if !data.is_empty() {
            let mut unread_data_at = self.recv.nxt.wrapping_sub(seqn) as usize;
            if unread_data_at > data.len() {
                // Handle retransmitted data
                unread_data_at = 0;
            }

            // only read what we haven't read yet
            self.incoming.extend(&data[unread_data_at..]);

            // TODO: Implement UDP receive logic
            self.write(nic, self.send.nxt, 0)?;
        }

        Ok(self.availability())
    }

    pub(crate) fn on_tick(
        &mut self,
    ) -> io::Result<()> {
        // TODO: Implement UDP on_tick logic
        Ok(())
    }

    pub(crate) fn close(&mut self) {
        self.closed = true;
    }
}
