//! Bindings for serial port I/O and futures
//!
//! This crate provides bindings between `mio_serial`, a mio crate for
//! serial port I/O, and `futures`.  The API is very similar to the
//! bindings in `mio_serial`
//!
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

// Re-export serialport types and traits from mio_serial
pub use mio_serial::{
    ClearBuffer, DataBits, Error, ErrorKind, FlowControl, Parity, SerialPort, SerialPortSettings,
    StopBits,
};

/// A type for results generated by interacting with serial ports.
pub type Result<T> = mio_serial::Result<T>;

use futures::{Async, Poll};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_reactor::{Handle, PollEvented};

use std::io::{self, Read, Write};
use std::path::Path;
use std::time::Duration;

/// Serial port I/O struct.
pub struct Serial {
    io: PollEvented<mio_serial::Serial>,
}

impl Serial {
    /// Open serial port from a provided path, using the default reactor.
    pub fn from_path<P>(path: P, settings: &mio_serial::SerialPortSettings) -> io::Result<Serial>
    where
        P: AsRef<Path>,
    {
        let port = mio_serial::Serial::from_path(path.as_ref(), settings)?;
        let io = PollEvented::new(port);

        Ok(Serial { io })
    }

    /// Open serial port from a provided path, using the specified reactor.
    pub fn from_path_with_handle<P>(
        path: P,
        settings: &mio_serial::SerialPortSettings,
        handle: &Handle,
    ) -> io::Result<Serial>
    where
        P: AsRef<Path>,
    {
        let port = mio_serial::Serial::from_path(path.as_ref(), settings)?;
        let io = PollEvented::new_with_handle(port, handle)?;

        Ok(Serial { io })
    }

    /// Create a pair of pseudo serial terminals using the default reactor
    ///
    /// ## Returns
    /// Two connected, unnamed `Serial` objects.
    ///
    /// ## Errors
    /// Attempting any IO or parameter settings on the slave tty after the master
    /// tty is closed will return errors.
    ///
    #[cfg(unix)]
    pub fn pair() -> Result<(Self, Self)> {
        let (master, slave) = mio_serial::Serial::pair()?;

        let master = Serial {
            io: PollEvented::new(master),
        };
        let slave = Serial {
            io: PollEvented::new(slave),
        };
        Ok((master, slave))
    }

    /// Create a pair of pseudo serial terminals using the specified reactor.
    ///
    /// ## Returns
    /// Two connected, unnamed `Serial` objects.
    ///
    /// ## Errors
    /// Attempting any IO or parameter settings on the slave tty after the master
    /// tty is closed will return errors.
    ///
    #[cfg(unix)]
    pub fn pair_with_handle(handle: &Handle) -> Result<(Self, Self)> {
        let (master, slave) = mio_serial::Serial::pair()?;

        let master = Serial {
            io: PollEvented::new_with_handle(master, handle)?,
        };
        let slave = Serial {
            io: PollEvented::new_with_handle(slave, handle)?,
        };
        Ok((master, slave))
    }

    /// Sets the exclusivity of the port
    ///
    /// If a port is exclusive, then trying to open the same device path again
    /// will fail.
    ///
    /// See the man pages for the tiocexcl and tiocnxcl ioctl's for more details.
    ///
    /// ## Errors
    ///
    /// * `Io` for any error while setting exclusivity for the port.
    #[cfg(unix)]
    pub fn set_exclusive(&mut self, exclusive: bool) -> Result<()> {
        self.io.get_mut().set_exclusive(exclusive)
    }

    /// Returns the exclusivity of the port
    ///
    /// If a port is exclusive, then trying to open the same device path again
    /// will fail.
    #[cfg(unix)]
    pub fn exclusive(&self) -> bool {
        self.io.get_ref().exclusive()
    }
}

impl SerialPort for Serial {
    fn settings(&self) -> SerialPortSettings {
        self.io.get_ref().settings()
    }

    fn name(&self) -> Option<String> {
        self.io.get_ref().name()
    }

    fn baud_rate(&self) -> Result<u32> {
        self.io.get_ref().baud_rate()
    }

    fn data_bits(&self) -> Result<DataBits> {
        self.io.get_ref().data_bits()
    }

    fn flow_control(&self) -> Result<FlowControl> {
        self.io.get_ref().flow_control()
    }

    fn parity(&self) -> Result<Parity> {
        self.io.get_ref().parity()
    }

    fn stop_bits(&self) -> Result<StopBits> {
        self.io.get_ref().stop_bits()
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(0)
    }

    fn set_all(&mut self, settings: &SerialPortSettings) -> Result<()> {
        self.io.get_mut().set_all(settings)
    }

    fn set_baud_rate(&mut self, baud_rate: u32) -> Result<()> {
        self.io.get_mut().set_baud_rate(baud_rate)
    }

    fn set_data_bits(&mut self, data_bits: DataBits) -> Result<()> {
        self.io.get_mut().set_data_bits(data_bits)
    }

    fn set_flow_control(&mut self, flow_control: FlowControl) -> Result<()> {
        self.io.get_mut().set_flow_control(flow_control)
    }

    fn set_parity(&mut self, parity: Parity) -> Result<()> {
        self.io.get_mut().set_parity(parity)
    }

    fn set_stop_bits(&mut self, stop_bits: StopBits) -> Result<()> {
        self.io.get_mut().set_stop_bits(stop_bits)
    }

    fn set_timeout(&mut self, _: Duration) -> Result<()> {
        Ok(())
    }

    fn write_request_to_send(&mut self, level: bool) -> Result<()> {
        self.io.get_mut().write_request_to_send(level)
    }

    fn write_data_terminal_ready(&mut self, level: bool) -> Result<()> {
        self.io.get_mut().write_data_terminal_ready(level)
    }

    fn read_clear_to_send(&mut self) -> Result<bool> {
        self.io.get_mut().read_clear_to_send()
    }

    fn read_data_set_ready(&mut self) -> Result<bool> {
        self.io.get_mut().read_data_set_ready()
    }

    fn read_ring_indicator(&mut self) -> Result<bool> {
        self.io.get_mut().read_ring_indicator()
    }

    fn read_carrier_detect(&mut self) -> Result<bool> {
        self.io.get_mut().read_carrier_detect()
    }

    fn bytes_to_read(&self) -> Result<u32> {
        self.io.get_ref().bytes_to_read()
    }

    fn bytes_to_write(&self) -> Result<u32> {
        self.io.get_ref().bytes_to_write()
    }

    fn clear(&self, buffer_to_clear: ClearBuffer) -> Result<()> {
        self.io.get_ref().clear(buffer_to_clear)
    }

    fn try_clone(&self) -> Result<Box<dyn SerialPort>> {
        self.io.get_ref().try_clone()
    }
}

impl Read for Serial {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.io.read(buf)
    }
}

impl Write for Serial {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.io.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.io.flush()
    }
}

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(unix)]
impl AsRawFd for Serial {
    fn as_raw_fd(&self) -> RawFd {
        self.io.get_ref().as_raw_fd()
    }
}

impl AsyncRead for Serial {
    fn poll_read(&mut self, buf: &mut [u8]) -> io::Result<Async<usize>> {
        self.io.poll_read(buf)
    }
}

impl AsyncWrite for Serial {
    fn poll_write(&mut self, buf: &[u8]) -> io::Result<Async<usize>> {
        self.io.poll_write(buf)
    }

    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.io.shutdown()
    }
}
