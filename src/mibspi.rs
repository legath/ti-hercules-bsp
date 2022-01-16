///
use vcell::VolatileCell;
use crate::spireg::SpiRegisters;


const MIBSPI1_ADDR: *const SpiRegisters = 0xFFF7_F400 as *const SpiRegisters;
const MIBSPI3_ADDR: *const SpiRegisters = 0xFFF7_F800 as *const SpiRegisters;
const MIBSPI5_ADDR: *const SpiRegisters = 0xFFF7_FC00 as *const SpiRegisters;
const MIBSPI_ADDR: [*const SpiRegisters; 3] = [MIBSPI1_ADDR, MIBSPI3_ADDR, MIBSPI5_ADDR];

#[cfg(target_endian = "little")]
#[allow(dead_code)]
struct MibSpiBufferTx {
    data: u16,
    // tx buffer data
    control: u16, // tx buffer control
}

#[cfg(target_endian = "big")]
#[allow(dead_code)]
struct MibSpiBufferTx {
    control: u16,
    // tx buffer control
    data: u16,    // tx buffer data
}

#[cfg(target_endian = "little")]
#[allow(dead_code)]
struct MibSpiBufferRx {
    data: u16,
    // rx buffer data
    flags: u16, // rx buffer flags
}

#[cfg(target_endian = "big")]
#[allow(dead_code)]
struct MibSpiBufferRx {
    data: u16,
    // rx buffer data
    flags: u16, // rx buffer flags
}

#[allow(dead_code)]
struct MibspiRam {
    tx: [MibSpiBufferTx; 128],
    rx: [MibSpiBufferRx; 128],
}

const MIBSPI1_RAM_ADDR: *const MibspiRam = 0xFF0E_0000 as *const MibspiRam;
const MIBSPI3_RAM_ADDR: *const MibspiRam = 0xFF0C_0000 as *const MibspiRam;
const MIBSPI5_RAM_ADDR: *const MibspiRam = 0xFF0A_0000 as *const MibspiRam;
const MIBSPI_RAM_ADDR: [*const MibspiRam; 3] =
    [MIBSPI1_RAM_ADDR, MIBSPI3_RAM_ADDR, MIBSPI5_RAM_ADDR];

#[derive(Copy, Clone)]
pub enum MibSpiID {
    One = 0,
    Three = 1,
    Five = 2,
}


#[allow(dead_code)]

pub enum lbType {
     ANALOG =  0,
     DIGITAL = 1,
}

#[allow(dead_code)]
pub struct MibSpi {
    pub id: MibSpiID,
    regs: &'static SpiRegisters,
    ram: &'static MibspiRam,
}

impl MibSpi {
    pub fn new(id: MibSpiID, master: bool) -> MibSpi {
        let mibspi = MibSpi {
            id,
            regs: unsafe { &*MIBSPI_ADDR[id as usize] },
            ram: unsafe { &*MIBSPI_RAM_ADDR[id as usize] },
        };
        mibspi.init(master);
        mibspi
    }

    pub fn init(&self, master: bool) {
        self.regs.GCR0.set(0x0);
        self.regs.GCR0.set(0x1);
        let internal_clock = 0x1 << 1;

        let gcr1 = internal_clock | (master as u32);
        self.regs.GCR1.set(self.regs.GCR1.get() | gcr1);

        // startup the module
        self.regs.GCR1.set(self.regs.GCR1.get() | 0x0100_0000);
    }

    /// SPIENA pin high-impedance enable. When active, the SPIENA pin
    /// is forced to high-impedance when not driving a low signal.
    /// If inactive, then the pin will output both a high and a low signal.
    pub fn highz(&self, enable: bool) {
        let enablehighz = (enable as u32) << 24;
        let int0 = self.regs.INT0.get() & !enablehighz;
        self.regs.INT0.set(int0 | enablehighz);
    }
    pub fn enable_loopback(&self, t: lbType) {
        /// Clear Loopback incase enabled already
        self.regs.IOLPKTSTCR.set(0);
        self.regs.IOLPKTSTCR.set(0x00000A00 | ((t as u32) << 1));
    }

    pub fn disable_loopback(&self) {
        self.regs.IOLPKTSTCR.set(0x00000500);
    }
}
