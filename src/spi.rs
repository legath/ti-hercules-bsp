///classic spi, unbuffered

use vcell::VolatileCell;
use crate::spireg::SpiRegisters;

const SPI1_ADDR: *const SpiRegisters = 0xFFF7_F400 as *const SpiRegisters;
const SPI2_ADDR: *const SpiRegisters = 0xFFF7_F600 as *const SpiRegisters;
const SPI3_ADDR: *const SpiRegisters = 0xFFF7_F800 as *const SpiRegisters;
const SPI4_ADDR: *const SpiRegisters = 0xFFF7_FA00 as *const SpiRegisters;
const SPI5_ADDR: *const SpiRegisters = 0xFFF7_FC00 as *const SpiRegisters;


const SPI_ADDR: [*const SpiRegisters; 5] = [SPI1_ADDR,SPI2_ADDR, SPI3_ADDR, SPI4_ADDR,SPI5_ADDR];


#[derive(Copy, Clone)]
pub enum SpiID { ///TODO: Need to make reference for mibspi
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
}

#[allow(dead_code)]
pub struct Spi {
    pub id: SpiID,
    regs: &'static SpiRegisters,
    master: bool
}

///According to
/// Technical Reference Manual
/// Literature Number: SPNU514C
///
///Enable SPI by setting RESET bit.
/// • Configure the SIMO, SOMI, SPICLK, and optional SPICS and SPIENA pins for SPI functionality by
/// setting the corresponding bit in SPIPC0 register.
/// • Configure the module to function as Master or Slave using CLKMOD and MASTER bits.
/// • Configure the required SPI data format using SPIFMTx register.
/// • If the module is selected to function as Master, the delay parameters can be configured using
/// SPIDELAY register.
/// • Enable the Interrupts using SPIINT0 register if required.
/// • Select the chip select to be used by setting CSNR bits in SPIDAT1 register.
/// • Configure CSHOLD and WDEL bits in SPIDAT1 register if required.
/// • Select the Data word format by setting DFSEL bits. Select the Number of the configured SPIFMTx
/// register (0 to 3) to used for the communication.
/// • Set LOOPBACK bit to connect the transmitter to the receiver internally. (This feature is used to perform
/// a self-test. Do not configure for normal communication to external devices).
/// • Set SPIEN bit to 1 after the SPI is configured.
/// • Perform Transmit and receive data, using SPIDAT1 and SPIBUF register.
/// • You must wait for TXFULL to reset or TXINT before writing next data to SPIDAT1 register.
/// • You must wait for RXEMPTY to reset or RXINT before reading the data from SPIBUF register

impl Spi {
    pub fn new(id: SpiID, master: bool) -> Spi {
        let spi = Spi {
            id,
            regs: unsafe { &*SPI_ADDR[id as usize] },
            master,
        };
        spi.init(master);
        spi
    }
    pub fn init(&self, master: bool) {
        self.regs.GCR0.set(0x0);
        self.regs.GCR0.set(0x1);
        let CLOKMOD = 0x1 << 1;

        let gcr1 = CLOKMOD | (master as u32);
        self.regs.GCR1.set(self.regs.GCR1.get() | gcr1);

        // startup the module
        self.regs.GCR1.set(self.regs.GCR1.get() | 0x0100_0000);
    }
    pub fn preconf_format(&self, fmt_num:u8){

    }
    pub fn set_delays(&self, C2T: u32, T2C: u32, T2E: u32, C2E:u32){

    }
}