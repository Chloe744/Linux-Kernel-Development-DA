#![no_std]
#![no_main]

use kernel::prelude::*;
use kernel::sync::{Mutex, atomic::{AtomicBool, AtomicUsize, Ordering}};

use kernel::chrdev;
use kernel::file::{File, FileOperations};
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};

module! {
    type: CharTestRustModule,
    name: "chartest_rust",
    author: "Wieser",
    description: "Char device module in Rust (open/read/write/release)",
    license: "GPL",
}

const DEVICE_NAME: &CStr = c_str!("chartest");
const BUFFER_SIZE: usize = 1024;

static ALREADY_OPEN: AtomicBool = AtomicBool::new(false);
static OPEN_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Stores bytes written by userspace. LEN tracks valid bytes in the buffer.
static KERNEL_BUFFER: Mutex<[u8; BUFFER_SIZE]> = Mutex::new([0u8; BUFFER_SIZE]);
static KERNEL_BUFFER_LEN: AtomicUsize = AtomicUsize::new(0);

struct CharTestRustModule {
    _reg: chrdev::Registration,
}

impl KernelModule for CharTestRustModule {
    fn init() -> Result<Self> {
        pr_info!("begin {}\n", DEVICE_NAME);

        // Register a character device. Major 0 means dynamic allocation.
        // Depending on kernel version, the constructor name may be new / new_pinned.
        let mut reg = chrdev::Registration::new(0, DEVICE_NAME)?;
        reg.register::<CharTestFile>()?;

        Ok(Self { _reg: reg })
    }
}

impl Drop for CharTestRustModule {
    fn drop(&mut self) {
        // Registration is automatically unregistered on drop.
        pr_info!("finished {}\n", DEVICE_NAME);
    }
}

// File operations for the char device.
// This mirrors the C file_operations: open, release, read, write.
struct CharTestFile;

impl FileOperations for CharTestFile {
    // We do not keep per-open state for now.
    type Data = ();

    // open()
    fn open(_context: &(), _file: &File) -> Result<Self::Data> {
        // Exclusive open, like atomic_cmpxchg in C.
        if ALREADY_OPEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(Error::EBUSY);
        }

        let count = OPEN_COUNTER.fetch_add(1, Ordering::Relaxed);
        pr_info!("You opened this {} times\n", count);

        Ok(())
    }

    // release()
    fn release(_data: Self::Data, _file: &File) {
        ALREADY_OPEN.store(false, Ordering::Release);
        pr_info!("closed\n");
    }

    // read()
    fn read(
        _data: &Self::Data,
        _file: &File,
        writer: &mut impl IoBufferWriter,
        offset: u64,
    ) -> Result<usize> {
        let offset = offset as usize;
        let len = KERNEL_BUFFER_LEN.load(Ordering::Acquire);

        if offset >= len {
            return Ok(0);
        }

        let to_copy = core::cmp::min(writer.len(), len - offset);

        let buf = KERNEL_BUFFER.lock();
        writer.write_slice(&buf[offset..offset + to_copy])?;

        Ok(to_copy)
    }

    // write()
    fn write(
        _data: &Self::Data,
        _file: &File,
        reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        // Cap the write size like the C code (BUFFER_SIZE - 1).
        let mut n = reader.len();
        if n >= BUFFER_SIZE {
            n = BUFFER_SIZE - 1;
        }

        let mut buf = KERNEL_BUFFER.lock();

        // Read from userspace into our kernel buffer.
        // Depending on kernel version, the helper may be read_slice or read_exact.
        reader.read_slice(&mut buf[..n])?;

        // Optional: null-terminate last byte like the C code intent.
        buf[n] = 0;

        KERNEL_BUFFER_LEN.store(n, Ordering::Release);

        pr_info!("successfully written into {}\n", DEVICE_NAME);
        Ok(n)
    }
}
