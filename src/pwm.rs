use vcell::VolatileCell;

#[cfg(target_endian = "little")]
#[repr(C)]
#[allow(non_snake_case)]
struct etPwmRegisters {
    TBCTL: VolatileCell<u16>,           //   0x0000 Time-Base Control Register
    TBSTS: VolatileCell<u16>,           //   0x0002 Time-Base Status Register
    _rsvd1: VolatileCell<u16>,          //   0x0004 Reserved
    TBPHS: VolatileCell<u16>,           //   0x0006 Time-Base Phase Register
    TBCTR: VolatileCell<u16>,           //   0x0008 Time-Base Counter Register
    TBPRD: VolatileCell<u16>,           //   0x000A Time-Base Period Register
    _rsvd2: VolatileCell<u16>,          //   0x000C Reserved
    CMPCTL: VolatileCell<u16>,          //   0x000E Counter-Compare Control Register
    _rsvd3: VolatileCell<u16>,          //   0x0010 Reserved
    CMPA: VolatileCell<u16>,            //   0x0012 Counter-Compare A Register
    CMPB: VolatileCell<u16>,            //   0x0014 Counter-Compare B Register
    AQCTLA: VolatileCell<u16>,          //    0x0016 Action-Qualifier Control Register for Output A (ETPWMxA)
    AQCTLB: VolatileCell<u16>,          //    0x0018 Action-Qualifier Control Register for Output B (ETPWMxB)
    AQSFRC: VolatileCell<u16>,          //    0x001A Action-Qualifier Software Force Register
    AQCSFRC: VolatileCell<u16>,         //    0x001C Action-Qualifier Continuous S/W Force Register Set
    DBCTL: VolatileCell<u16>,           //    0x001E Dead-Band Generator Control Register
    DBRED: VolatileCell<u16>,           //    0x0020 Dead-Band Generator Rising Edge Delay Count Register
    DBFED: VolatileCell<u16>,           //    0x0022 Dead-Band Generator Falling Edge Delay Count Register
    TZSEL: VolatileCell<u16>,           //    0x0024 Trip-Zone Select Register
    TZDCSEL: VolatileCell<u16>,         //    0x0026 Trip Zone Digital Compare Select Register
    TZCTL: VolatileCell<u16>,           //    0x0028 Trip-Zone Control Register
    TZEINT: VolatileCell<u16>,          //    0x002A Trip-Zone Enable Interrupt Register
    TZFLG: VolatileCell<u16>,           //    0x002C Trip-Zone Flag Register
    TZCLR: VolatileCell<u16>,           //    0x002E Trip-Zone Clear Register
    TZFRC: VolatileCell<u16>,           //    0x0030 Trip-Zone Force Register
    ETSEL: VolatileCell<u16>,           //    0x0032 Event-Trigger Selection Register
    ETPS: VolatileCell<u16>,            //    0x0034 Event-Trigger Pre-Scale Register
    ETFLG: VolatileCell<u16>,           //    0x0036 Event-Trigger Flag Register
    ETCLR: VolatileCell<u16>,           //    0x0038 Event-Trigger Clear Register
    ETFRC: VolatileCell<u16>,           //    0x003A Event-Trigger Force Register
    PCCTL: VolatileCell<u16>,           //    0x003C PWM-Chopper Control Register
    _rsvd4: VolatileCell<u16>,          //    0x003E Reserved
    _rsvd5: [VolatileCell<u16>; 16],    //    0x0040 Reserved
    DCTRIPSEL: VolatileCell<u16>,       //    0x0060 Digital Compare Trip Select Register
    DCACTL: VolatileCell<u16>,          //    0x0062 Digital Compare A Control Register
    DCBCTL: VolatileCell<u16>,          //    0x0064 Digital Compare B Control Register
    DCFCTL: VolatileCell<u16>,          //    0x0066 Digital Compare Filter Control Register
    DCCAPCTL: VolatileCell<u16>,        //    0x0068 Digital Compare Capture Control Register
    DCFOFFSET: VolatileCell<u16>,       //    0x006A Digital Compare Filter Offset Register
    DCFOFFSETCNT: VolatileCell<u16>,    //    0x006C Digital Compare Filter Offset Counter Register
    DCFWINDOW: VolatileCell<u16>,       //    0x006E Digital Compare Filter Window Register
    DCFWINDOWCNT: VolatileCell<u16>,    //    0x0070 Digital Compare Filter Window Counter Register
    DCCAP: VolatileCell<u16>,           //    0x0072 Digital Compare Counter Capture Register
}

#[cfg(target_endian = "big")]
#[repr(C)]
#[allow(non_snake_case)]
struct etPwmRegisters {
    TBSTS: VolatileCell<u16>,           //   0x0000 Time-Base Status Register
    TBCTL: VolatileCell<u16>,           //  0x0002 Time-Base Control Register
    TBPHS: VolatileCell<u16>,           // 0x0004 Time-Base Phase Register
    _rsvd1: VolatileCell<u16>,          //  0x0006 Reserved
    TBPRD: VolatileCell<u16>,           //  0x0008 Time-Base Period Register
    TBCTR: VolatileCell<u16>,           //  0x000A Time-Base Counter Register
    CMPCTL: VolatileCell<u16>,          //  0x000C Counter-Compare Control Register
    _rsvd2: VolatileCell<u16>,          //  0x000E Reserved
    CMPA: VolatileCell<u16>,            //  0x0010 Counter-Compare A Register
    _rsvd3: VolatileCell<u16>,          //  0x0012 Reserved
    AQCTLA: VolatileCell<u16>,          //  0x0014 Action-Qualifier Control Register for Output A (ETPWMxA)
    CMPB: VolatileCell<u16>,            //  0x0016 Counter-Compare B Register
    AQSFRC: VolatileCell<u16>,          //  0x0018 Action-Qualifier Software Force Register
    AQCTLB: VolatileCell<u16>,          //  0x001A Action-Qualifier Control Register for Output B (ETPWMxB)
    DBCTL: VolatileCell<u16>,           //  0x001C Dead-Band Generator Control Register*/
    AQCSFRC: VolatileCell<u16>,         //  0x001E Action-Qualifier Continuous S/W Force Register Set
    DBFED: VolatileCell<u16>,           //  0x0020 Dead-Band Generator Falling Edge Delay Count Register
    DBRED: VolatileCell<u16>,           //  0x0022 Dead-Band Generator Rising Edge Delay Count Register
    TZDCSEL: VolatileCell<u16>,         //  0x0024 Trip Zone Digital Compare Select Register
    TZSEL: VolatileCell<u16>,           //  0x0026 Trip-Zone Select Register
    TZEINT: VolatileCell<u16>,          //  0x0028 Trip-Zone Enable Interrupt Register
    TZCTL: VolatileCell<u16>,           //  0x002A Trip-Zone Control Register
    TZCLR: VolatileCell<u16>,           //  0x002C Trip-Zone Clear Register
    TZFLG: VolatileCell<u16>,           //  0x002E Trip-Zone Flag Register
    ETSEL: VolatileCell<u16>,           //  0x0030 Event-Trigger Selection Register
    TZFRC: VolatileCell<u16>,           //  0x0032 Trip-Zone Force Register
    ETFLG: VolatileCell<u16>,           //  0x0034 Event-Trigger Flag Register
    ETPS: VolatileCell<u16>,            //  0x0036 Event-Trigger Pre-Scale Register
    ETFRC: VolatileCell<u16>,           //  0x0038 Event-Trigger Force Register
    ETCLR: VolatileCell<u16>,           //  0x003A Event-Trigger Clear Register
    _rsvd4: VolatileCell<u16>,          //  0x003C Reserved
    PCCTL: VolatileCell<u16>,           //  0x003E PWM-Chopper Control Register
    _rsvd5: [VolatileCell<u16>; 16],    //  0x0040 Reserved
    DCACTL: VolatileCell<u16>,          //  0x0060 Digital Compare A Control Register
    DCTRIPSEL: VolatileCell<u16>,       //  0x0062 Digital Compare Trip Select Register
    DCFCTL: VolatileCell<u16>,          //  0x0064 Digital Compare Filter Control Register
    DCBCTL: VolatileCell<u16>,          //  0x0066 Digital Compare B Control Register
    DCFOFFSET: VolatileCell<u16>,       //  0x0068 Digital Compare Filter Offset Register
    DCCAPCTL: VolatileCell<u16>,        //  0x006A Digital Compare Capture Control Register
    DCFWINDOW: VolatileCell<u16>,       //  0x006C Digital Compare Filter Window Register
    DCFOFFSETCNT: VolatileCell<u16>,    //  0x006E Digital Compare Filter Offset Counter Register
    DCCAP: VolatileCell<u16>,           // 0x0070 Digital Compare Counter Capture Register
    DCFWINDOWCNT: VolatileCell<u16>,    //  0x0072 Digital Compare Filter Window Counter Register
}


const PWM1_BASE_ADDR: *const etPwmRegisters = 0xFCF7_8C00 as *const etPwmRegisters;
const PWM2_BASE_ADDR: *const etPwmRegisters = 0xFCF7_8D00 as *const etPwmRegisters;
const PWM3_BASE_ADDR: *const etPwmRegisters = 0xFCF7_8E00 as *const etPwmRegisters;
const PWM4_BASE_ADDR: *const etPwmRegisters = 0xFCF7_8F00 as *const etPwmRegisters;
const PWM5_BASE_ADDR: *const etPwmRegisters = 0xFCF7_9000 as *const etPwmRegisters;
const PWM6_BASE_ADDR: *const etPwmRegisters = 0xFCF7_9100 as *const etPwmRegisters;
const PWM7_BASE_ADDR: *const etPwmRegisters = 0xFCF7_9200 as *const etPwmRegisters;

const PWM_ADDR: [*const etPwmRegisters; 7] = [PWM1_BASE_ADDR, PWM2_BASE_ADDR, PWM3_BASE_ADDR,
                                              PWM4_BASE_ADDR,PWM5_BASE_ADDR, PWM6_BASE_ADDR,
                                              PWM7_BASE_ADDR];

#[derive(Copy, Clone)]
pub enum PwmId {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6
}

#[allow(dead_code)]
pub struct Pwm {
    pub id: PwmId,
    regs: &'static etPwmRegisters,
}

impl Pwm{
    pub fn new(id: PwmId) -> Pwm {
        let pwm = Pwm {
            id,
            regs: unsafe { &*PWM_ADDR[id as usize] },
        };
        pwm
    }
    pub fn init(&self) {
        self.regs.TBCTL.set(0);
    }
    pub fn setPeriod(&self, Period: u16){
        self.regs.TBPRD.set(Period);
    }
}