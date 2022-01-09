use vcell::VolatileCell;

#[allow(non_snake_case)]
pub_struct! (SpiRegisters {
    GCR0: VolatileCell<u32>,
    // Global Control 0
    GCR1: VolatileCell<u32>,
    // Global Control 1
    INT0: VolatileCell<u32>,
    // Interrupt Register
    LVL: VolatileCell<u32>,
    // Interrupt Level
    FLG: VolatileCell<u32>,
    // Interrupt flags
    PC0: VolatileCell<u32>,
    // Function Pin Enable
    PC1: VolatileCell<u32>,
    // Pin Direction
    PC2: VolatileCell<u32>,
    // Pin Input Latch
    PC3: VolatileCell<u32>,
    // Pin Output Latch
    PC4: VolatileCell<u32>,
    // Output Pin Set
    PC5: VolatileCell<u32>,
    // Output Pin Clr
    PC6: VolatileCell<u32>,
    // Open Drain Output Enable
    PC7: VolatileCell<u32>,
    // Pullup/Pulldown Disable
    PC8: VolatileCell<u32>,
    // Pullup/Pulldown Selection
    DAT0: VolatileCell<u32>,
    // Transmit Data
    DAT1: VolatileCell<u32>,
    // Transmit Data with Format and Chip Select
    BUF: VolatileCell<u32>,
    // Receive Buffer
    EMU: VolatileCell<u32>,
    // Emulation Receive Buffer
    DELAY: VolatileCell<u32>,
    // Delays
    DEF: VolatileCell<u32>,
    // Default Chip Select
    FMT0: VolatileCell<u32>,
    // Data Format 0
    FMT1: VolatileCell<u32>,
    // Data Format 1
    FMT2: VolatileCell<u32>,
    // Data Format 2
    FMT3: VolatileCell<u32>,
    // Data Format 3
    INTVECT0: VolatileCell<u32>,
    // Interrupt Vector 0
    INTVECT1: VolatileCell<u32>,
    // Interrupt Vector 1
    SRSEL: VolatileCell<u32>,
    // Slew Rate Select
    PMCTRL: VolatileCell<u32>,
    // Parallel Mode Control
    MIBSPIE: VolatileCell<u32>,
    // Multi-buffer Mode Enable
    TGITENST: VolatileCell<u32>,
    // TG Interrupt Enable Set
    TGITENCR: VolatileCell<u32>,
    // TG Interrupt Enable Clear
    TGITLVST: VolatileCell<u32>,
    // Transfer Group Interrupt Level Set
    TGITLVCR: VolatileCell<u32>,
    // Transfer Group Interrupt Level Clear
    TGINTFLG: VolatileCell<u32>,
    // Transfer Group Interrupt Flag
    _reserved1: [VolatileCell<u32>; 2],
    // Reserved
    TICKCNT: VolatileCell<u32>,
    // Tick Counter
    LTGPEND: VolatileCell<u32>,
    // Last TG End Pointer
    TGCTRL: [VolatileCell<u32>; 16],
    // Transfer Group Control
    DMACTRL: [VolatileCell<u32>; 8],
    // DMA Control
    DMACOUNT: [VolatileCell<u32>; 8],
    // DMA Count
    DMACNTLEN: VolatileCell<u32>,
    // DMA Control length
    _reserved2: VolatileCell<u32>,
    // Reserved
    UERRCTRL: VolatileCell<u32>,
    // Multi-buffer RAM Uncorrectable Parity Error Control
    UERRSTAT: VolatileCell<u32>,
    // Multi-buffer RAM Uncorrectable Parity Error Status
    UERRADDRRX: VolatileCell<u32>,
    // RXRAM Uncorrectable Parity Error Address
    UERRADDRTX: VolatileCell<u32>,
    // TXRAM Uncorrectable Parity Error Address
    RXOVRN_BUF_ADDR: VolatileCell<u32>,
    // RXRAM Overrun Buffer Address
    IOLPKTSTCR: VolatileCell<u32>,
    // IO loopback
    EXT_PRESCALE1: VolatileCell<u32>,
    EXT_PRESCALE2: VolatileCell<u32>,
});