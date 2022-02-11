use tock_registers::interfaces::{ReadWriteable, Writeable};
///classic spi, unbuffered

use {
    vcell::VolatileCell,
    tock_registers::{

        register_bitfields,
        register_structs,
        registers::{ReadOnly, ReadWrite, WriteOnly},
    },
};

/// For access via .val()
const Enable: u8 = 1;
const Disable: u8 = 0;
const LVL_Line0: u8 = 0;
const LVL_Line1: u8 = 1;
///



register_bitfields! {
    u32,
    GCR0 [
        RESET OFFSET(0) NUMBITS(1) [
            InReset = 0,
            OutReset = 1
        ],
    ],
    GCR1 [
        SPIEN OFFSET(24) NUMBITS(1) [],
        LOOPBACK OFFSET(16) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        POWERDOWN OFFSET(8) NUMBITS(1) [
            Active = 0,
            PowerDown = 1
        ],
        CLKMOD OFFSET(1) NUMBITS(1) [
            External = 0,
            Internal = 1
        ],
        MASTER OFFSET(0) NUMBITS(1) [
            Slave = 0,
            Master = 1
        ],
    ],
    INT0 [
        ENABLEHIGHZ OFFSET(24) NUMBITS(1) [
            PullUP = 0,
            HighZ = 1
        ],
        DMAREQEN OFFSET(16) NUMBITS(1) [],
        TXINTENA OFFSET(9) NUMBITS(1) [],
        RXINTENA OFFSET(8) NUMBITS(1) [],
        RXOVRNINTENA OFFSET(6) NUMBITS(1) [],
        BITERRENA OFFSET(4) NUMBITS(1) [],
        DESYNCENA OFFSET(3) NUMBITS(1) [],
        PARERRENA OFFSET(2) NUMBITS(1) [],
        TIMEOUTENA OFFSET(1) NUMBITS(1) [],
        DLENERRENA OFFSET(0) NUMBITS(1) [],
    ],
    LVL [
        TXINTLVL 9,
        RXINTLVL 8,
        RXOVRNINTLVL 6,
        BITERRLVL 4,
        DESYNCLVL 3,
        PARERRLVL 2,
        TIMEOUTLVL 1,
        DLENERRLVL 0
    ],
    FLG [
        BUFINITACTIVE OFFSET(24) NUMBITS(1) [
            Complete = 0,
            Busy = 1
        ],
        TXINTFLG OFFSET(9) NUMBITS(1) [
            Full = 0,
            Empty = 1
        ],
        RXINTFLG OFFSET(8) NUMBITS(1) [
            Empty = 0,
            Full = 1
        ],
        RXOVRNINTFLG OFFSET(6) NUMBITS(1) [],
        BITERRFLG OFFSET(4) NUMBITS(1) [],
        DESYNCFLG OFFSET(3) NUMBITS(1) [],
        PARERRFLG OFFSET(2) NUMBITS(1) [],
        TIMEOUTFLG OFFSET(1) NUMBITS(1) [],
        DLENERRFLG OFFSET(0) NUMBITS(1) [],
    ],
    PC0 [
        SOMIFUN OFFSET(25) NUMBITS(7) [],
        SIMOFUN OFFSET(17) NUMBITS(7) [],
        SOMIFUN0 OFFSET(11) NUMBITS(1) [],
        SIMOFUN0 OFFSET(10) NUMBITS(1) [],
        CLKFUN OFFSET(9) NUMBITS(1) [],
        ENAFUN OFFSET(8) NUMBITS(1) [],
        SCSFUN OFFSET(0) NUMBITS(8) [],
    ],
    PC1 [
        SOMIDIR OFFSET(25) NUMBITS(7) [],
        SIMODIR OFFSET(17) NUMBITS(7) [],
        SOMIDIR0 OFFSET(11) NUMBITS(1) [],
        SIMODIR0 OFFSET(10) NUMBITS(1) [],
        CLKDIR OFFSET(9) NUMBITS(1) [],
        ENADIR OFFSET(8) NUMBITS(1) [],
        SCSDIR OFFSET(0) NUMBITS(8) [],
    ],
    PC2 [
        SOMIDIN OFFSET(25) NUMBITS(7) [],
        SIMODIN OFFSET(17) NUMBITS(7) [],
        SOMIDIN0 OFFSET(11) NUMBITS(1) [],
        SIMODIN0 OFFSET(10) NUMBITS(1) [],
        CLKDIN OFFSET(9) NUMBITS(1) [],
        ENADIN OFFSET(8) NUMBITS(1) [],
        SCSDIN OFFSET(0) NUMBITS(8) [],
    ],
    PC3 [
        SOMIDOUT OFFSET(25) NUMBITS(7) [],
        SIMODOUT OFFSET(17) NUMBITS(7) [],
        SOMIDOUT0 OFFSET(11) NUMBITS(1) [],
        SIMODOUT0 OFFSET(10) NUMBITS(1) [],
        CLKDOUT OFFSET(9) NUMBITS(1) [],
        ENADOUT OFFSET(8) NUMBITS(1) [],
        SCSDOUT OFFSET(0) NUMBITS(8) [],
    ],
    PC4 [
        SOMISET OFFSET(25) NUMBITS(7) [],
        SIMOSET OFFSET(17) NUMBITS(7) [],
        SOMISET0 OFFSET(11) NUMBITS(1) [],
        SIMOSET0 OFFSET(10) NUMBITS(1) [],
        CLKSET OFFSET(9) NUMBITS(1) [],
        ENASET OFFSET(8) NUMBITS(1) [],
        SCSSET OFFSET(0) NUMBITS(8) [],
    ],
    PC5 [
        SOMICLR OFFSET(25) NUMBITS(7) [],
        SIMOCLR OFFSET(17) NUMBITS(7) [],
        SOMICLR0 OFFSET(11) NUMBITS(1) [],
        SIMOCLR0 OFFSET(10) NUMBITS(1) [],
        CLKCLR OFFSET(9) NUMBITS(1) [],
        ENACLR OFFSET(8) NUMBITS(1) [],
        SCSCLR OFFSET(0) NUMBITS(8) [],
    ],
    PC6 [
        SOMIPDR OFFSET(25) NUMBITS(7) [],
        SIMOPDR OFFSET(17) NUMBITS(7) [],
        SOMIPDR0 OFFSET(11) NUMBITS(1) [],
        SIMOPDR0 OFFSET(10) NUMBITS(1) [],
        CLKDPDR OFFSET(9) NUMBITS(1) [],
        ENADPDR OFFSET(8) NUMBITS(1) [],
        SCSDPDR OFFSET(0) NUMBITS(8) [],
    ],
    PC7 [
        SOMIDIS OFFSET(25) NUMBITS(7) [],
        SIMODIS OFFSET(17) NUMBITS(7) [],
        SOMIDIS0 OFFSET(11) NUMBITS(1) [],
        SIMODIS0 OFFSET(10) NUMBITS(1) [],
        CLKDIS OFFSET(9) NUMBITS(1) [],
        ENADIS OFFSET(8) NUMBITS(1) [],
        SCSDIS OFFSET(0) NUMBITS(8) [],
    ],
    PC8 [
        SOMIPSEL OFFSET(25) NUMBITS(7) [],
        SIMOPSEL OFFSET(17) NUMBITS(7) [],
        SOMIPSEL0 OFFSET(11) NUMBITS(1) [],
        SIMOPSEL0 OFFSET(10) NUMBITS(1) [],
        CLKPSEL OFFSET(9) NUMBITS(1) [],
        ENAPSEL OFFSET(8) NUMBITS(1) [],
        SCSPSEL OFFSET(0) NUMBITS(8) [],
    ],
    DAT0[
        TXDATA OFFSET(0) NUMBITS(16) [],
    ],
    DAT1[
        CSHOLD OFFSET(28) NUMBITS(1) [],
        WDEL OFFSET(26) NUMBITS(1) [],
        DFSEL OFFSET(24) NUMBITS(2) [
            FMT0 = 0,
            FMT1 = 1,
            FMT2 = 2,
            FMT3 = 3,
        ],
        CSNR OFFSET(16) NUMBITS(8) [],
        TXDATA OFFSET(0) NUMBITS(16) [],
    ],
    BUF[
        RXEMPTY OFFSET(31) NUMBITS(1) [],
        RXOVR OFFSET(30) NUMBITS(1) [],
        TXFULL OFFSET(29) NUMBITS(1) [],
        BITERR OFFSET(28) NUMBITS(1) [],
        DESYNC OFFSET(27) NUMBITS(1) [],
        PARITYERR OFFSET(26) NUMBITS(1) [],
        TIMEOUT OFFSET(25) NUMBITS(1) [],
        DLENERR OFFSET(24) NUMBITS(1) [],
        LCSNR OFFSET(16) NUMBITS(8) [],
        RXDATA OFFSET(0) NUMBITS(16) [],
    ],
    EMU[
        EMU_RXDATA OFFSET(0) NUMBITS(16) [],
    ],
    DELAY[
        C2TDELAY OFFSET(24) NUMBITS(8) [],
        T2CDELAY OFFSET(16) NUMBITS(8) [],
        T2EDELAY OFFSET(8) NUMBITS(8) [],
        C2EDELAY OFFSET(0) NUMBITS(8) [],
    ],
    DEF[
        CDEF OFFSET(0) NUMBITS(8) [],
    ],
    FMT[
        WDELAY OFFSET(24) NUMBITS(8) [],
        PARPOL OFFSET(23) NUMBITS(1) [],
        PARITYENA OFFSET(22) NUMBITS(1) [],
        WAITENA OFFSET(21) NUMBITS(1) [],
        SHIFTDIR OFFSET(20) NUMBITS(1) [],
        HDUPLEX_ENA OFFSET(19) NUMBITS(1) [],
        DIS_CS_TIMERS OFFSET(18) NUMBITS(1) [],
        POLARITY OFFSET(17) NUMBITS(1) [],
        PHASE OFFSET(16) NUMBITS(1) [],
        PRESCALE OFFSET(8) NUMBITS(8) [],
        CHARLEN OFFSET(0) NUMBITS(5) [],
    ],
    INTVECT[
        VECT OFFSET(1) NUMBITS(5) [],
        SUSPEND OFFSET(0) NUMBITS(1) [],
    ],
    MIBSPIE[
        RXRAM_ACCESS OFFSET(16) NUMBITS(1) [],
        MSPIENA OFFSET(0) NUMBITS(1) [],
    ],
    TGITENST[
        SETINTENRDY OFFSET(16) NUMBITS(16) [],
        SETINTENSUS OFFSET(0) NUMBITS(16) [],
    ],
    TGITENCR[
        CLRINTENRDY OFFSET(16) NUMBITS(16) [],
        CLRINTENSUS OFFSET(0) NUMBITS(16) [],
    ],
    TGITLVST[
        SETINTLVLRDY OFFSET(16) NUMBITS(16) [],
        SETINTLVLSUS OFFSET(0) NUMBITS(16) [],
    ],
    TGITLVCR[
        CLRINTLVLRDY OFFSET(16) NUMBITS(16) [],
        CLRINTLVLSUS OFFSET(0) NUMBITS(16) [],
    ],
    TGINTFLG[
        INTFLGRDY OFFSET(16) NUMBITS(16) [],
        INTFLGSUS OFFSET(0) NUMBITS(16) [],
    ],
    TICKCNT[
        TICKENA OFFSET(31) NUMBITS(1) [],
        RELOAD OFFSET(30) NUMBITS(1) [],
        CLKCTRL OFFSET(28) NUMBITS(2) [],
        TICKVALUE OFFSET(0) NUMBITS(16) [],
    ],
    LTGPEND [
        TG_IN_SERVICE OFFSET(24) NUMBITS(5) [
            NO_TG = 0,
            TG0 = 1,
            TG1 = 2,
            TG2 = 3,
            TG3 = 4,
            TG4 = 5,
            TG5 = 6,
            TG6 = 7,
            TG7 = 8,
            TG8 = 9,
            TG9 = 10,
            TG10 = 11,
            TG11 = 12,
            TG12 = 13,
            TG13 = 14,
            TG14 = 15,
            TG15 = 16,
        ],
        LPEND OFFSET(8) NUMBITS(7) [],
    ],
    TGCTRL[
        TGENA OFFSET(31) NUMBITS(1) [],
        ONESHOT OFFSET(30) NUMBITS(1) [],
        PRST OFFSET(29) NUMBITS(1) [],
        TGTD OFFSET(28) NUMBITS(1) [],
        TRIGEVT OFFSET(20) NUMBITS(4) [],
        TRIGSRC OFFSET(16) NUMBITS(4) [],
        PSTART OFFSET(8) NUMBITS(7) [],
        PCURRENT OFFSET(0) NUMBITS(7) [],
    ]


}

register_structs! {
    #[allow(non_snake_case)]
    SpiRegisterBlock {
        (0x00=> GCR0: ReadWrite<u32, GCR0::Register>),
        (0x04=> GCR1: ReadWrite<u32, GCR1::Register>),
        (0x08=> INT0: ReadWrite<u32, INT0::Register>),
        (0x0c=> LVL : ReadWrite<u32, LVL::Register>),
        (0x10=> FLG : ReadWrite<u32, FLG::Register>),
        (0x14=> PC0 : ReadWrite<u32, PC0::Register>),
        (0x18=> PC1 : ReadWrite<u32, PC1::Register>),
        (0x1c=> PC2 : ReadWrite<u32, PC2::Register>),
        (0x20=> PC3 : ReadWrite<u32, PC3::Register>),
        (0x24=> PC4 : ReadWrite<u32, PC4::Register>),
        (0x28=> PC5 : ReadWrite<u32, PC5::Register>),
        (0x2c=> PC6 : ReadWrite<u32, PC6::Register>),
        (0x30=> PC7 : ReadWrite<u32, PC7::Register>),
        (0x34=> PC8 : ReadWrite<u32, PC8::Register>),
        (0x38=> DAT0 : ReadWrite<u32, DAT0::Register>),
        (0x3c=> DAT1 : ReadWrite<u32, DAT1::Register>),
        (0x40=> BUF : ReadWrite<u32, BUF::Register>),
        (0x44=> EMU : ReadWrite<u32, EMU::Register>),
        (0x48=> DELAY : ReadWrite<u32, DELAY::Register>),
        (0x4C=> DEF : ReadWrite<u32, DEF::Register>),
        (0x50=> FMT : [ReadWrite<u32, FMT::Register>; 4]),
        (0x60=> INTVECT : [ReadWrite<u32, INTVECT::Register>; 2]),
        (0x68=> PC9 : ReadWrite<u32>), ///will PC9, at this time not needed
        (0x6c=> PMCTRL : ReadWrite<u32>),
        (0x70=> SPIE : ReadWrite<u32, MIBSPIE::Register>),
        (0x74=> TGITENST : ReadWrite<u32, TGITENST::Register>),
        (0x78=> TGITENCR : ReadWrite<u32, TGITENCR::Register>),
        (0x7c=> TGITLVST : ReadWrite<u32, TGITLVST::Register>),
        (0x80=> TGITLVCR : ReadWrite<u32, TGITLVCR::Register>),
        (0x84=> TGINTFLG : ReadWrite<u32, TGINTFLG::Register>),
        (0x88 => __reserved_1),
        (0x90=> TICKCNT : ReadWrite<u32, TICKCNT::Register>),
        (0x94=> LTGPEND : ReadWrite<u32, LTGPEND::Register>),
        (0x98=> TGCTRL : [ReadWrite<u32, TGCTRL::Register>; 8]), ///CHECK IT!!!!

        (0x140 => @END),
    }
}

const SPI1_ADDR: *const SpiRegisterBlock = 0xFFF7_F400 as *const SpiRegisterBlock;
const SPI2_ADDR: *const SpiRegisterBlock = 0xFFF7_F600 as *const SpiRegisterBlock;
const SPI3_ADDR: *const SpiRegisterBlock = 0xFFF7_F800 as *const SpiRegisterBlock;
const SPI4_ADDR: *const SpiRegisterBlock = 0xFFF7_FA00 as *const SpiRegisterBlock;
const SPI5_ADDR: *const SpiRegisterBlock = 0xFFF7_FC00 as *const SpiRegisterBlock;


const SPI_ADDR: [*const SpiRegisterBlock; 5] = [SPI1_ADDR, SPI2_ADDR, SPI3_ADDR, SPI4_ADDR, SPI5_ADDR];


#[derive(Copy, Clone)]
pub enum SpiID {
    ///TODO: Need to make reference for mibspi
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
}

#[allow(dead_code)]
pub struct Spi {
    pub id: SpiID,
    regs: &'static SpiRegisterBlock,
    master: bool,
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
    fn init(&self, master: bool) {
        self.regs.GCR0.write(GCR0::RESET::InReset);
        self.regs.GCR0.write(GCR0::RESET::OutReset);
        if master{
            self.regs.GCR1.modify(GCR1::MASTER::Master + GCR1::CLKMOD::Internal);
        }

    }
    pub fn open(&self){
        self.regs.GCR1.modify(GCR1::SPIEN::SET);
    }
    pub fn preconf_format(&mut self, fmt_num: u8) -> &mut Self {

        self
    }
    pub fn loopback(&mut self, enable: bool)-> &mut Self{
        if enable{
            self.regs.GCR1.modify(GCR1::LOOPBACK::Enabled);
        }
        else { self.regs.GCR1.modify(GCR1::LOOPBACK::Disabled) }
        self

    }
    pub fn set_delays(&self, C2T: u32, T2C: u32, T2E: u32, C2E: u32) {}
    pub fn set_prescaller(&mut self, prsc: u32) -> &mut Self {

        self
    }

}