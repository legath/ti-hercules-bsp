use crate::dwd::DWD;
use crate::efuse;
use crate::esm;
use crate::esm_ch::EsmError;
use crate::flash;
use crate::pbist;
use crate::pcr;
use crate::rti;
use crate::syscore;
use crate::sysexc;
use crate::system;
use crate::vim;
use cortexr4;
use siliconcr4;

extern "C" {
    fn main(argc: isize, argv: *const *const u8) -> isize;
    pub fn custom_dabort();
    static mut _sbss: u32;
    static mut _ebss: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sidata: u32;
}

#[naked]
pub unsafe extern "C" fn tms570_startup() -> ! {
    syscore::init_core_registers();
    syscore::init_stack_pointers();
    syscore::event_bus_export_enable();

    #[cfg(feature = "errata57")]
    siliconcr4::errata57();

    #[cfg(feature = "errata66")]
    siliconcr4::errata66();

    let wdog: rti::ChipWatchDog = DWD::new();

    let sysex = sysexc::SysException::new();
    if sysex.power_on() {
        sysex.clear_all();
    } else if sysex.wd_iecpick_reset() && wdog.time_violation() {
        sysex.clear(sysexc::Reset::WdIcePick);
    } else if sysex.cpu_reset() {
        sysex.clear(sysexc::Reset::Cpu);
    } else if sysex.sw_reset() {
        sysex.clear(sysexc::Reset::Sw);
    } else if sysex.external() {
        sysex.clear(sysexc::Reset::External);
    }

    // Check if there were ESM group3 errors during power-up.
    // Device operation is not reliable and not recommended in this case.
    if esm::Esm::new().error_is_set(EsmError::EfuseAutoload) {
        panic!("[eFuse] autoload error");
    }

    let sys = system::Sys::new();
    sys.setup_pll();

    let efuse = efuse::Efc::new();
    let efuse_stat = efuse.check();

    // Disable Peripherals before powerup
    sys.enable_pheripherals(false);
    pcr::Pcr::new().enable_all();
    sys.enable_pheripherals(true);

    if efuse_stat == efuse::EfcError::OnGoing {
        // Wait for eFuse controller self-test to complete
        if !efuse.self_test_completed() {
            panic!("[eFuse] won't complete]");
        }
    } else {
        panic!("[eFuse] not reliable]");
    }

    // Setup flash before speed-up PLL otherwise MCU can't
    // properly read data from Flash memory.
    let flash = flash::Flash::new();
    flash.setup(flash::FlashWPowerModes::Active, 3, true, true);

    sys.trim_lpo();
    sys.clock_domain_setup(true, system::ClockDomains::AvClk3);
    sys.wait_pll_lock();

    sys.setup_clock_source(
        system::SysClockSources::Osc,
        system::SysClockSources::Osc,
        system::SysClockSources::Pll1,
    );

    sys.peripherals_clock_divider(1, 1, 1, 1);

    // Now the PLLs are locked and the PLL outputs can be speed up
    sys.set_pll_divider(0, 0);
    sys.eclk_functional_mode(7, false);

    if !sys.clock_supervisor_test() {
        panic!("CLK TEST");
    }

    // Parallel Test on PBIST ROM (can't be done in parallel with others)
    if cfg!(feature = "pbist_rom") {
        sys.pbist_run(
            pbist::test::TRIPLEREADSLOW | pbist::test::TRIPLEREADFAST,
            pbist::mem::PBIST_ROM,
        );
        wait_until_false!(sys.pbist_completed());
        if sys.pbist_fail() {
            panic!("PBIST ROM");
        }
        sys.pbist_stop();

        // PBIST test on STC ROM
        sys.pbist_run(
            pbist::test::TRIPLEREADSLOW | pbist::test::TRIPLEREADFAST,
            pbist::mem::STC_ROM,
        );
        wait_until_false!(sys.pbist_completed());
        if sys.pbist_fail() {
            panic!("PBIST STC ROM");
        }
        sys.pbist_stop();
    }

    // Not available in debug mode because PBIST on ESRAMx can't
    // use stack and We can't guarantee this.
    if cfg!(feature = "pbist_ram") && !cfg!(debug_assertions) {
        // ECC is disabled on reset (AUX register)
        // ESRAM Single Port PBIST
        sys.pbist_run(
            pbist::test::MARCH13N_SP,
            pbist::mem::ESRAM1 | pbist::mem::ESRAM5 | pbist::mem::ESRAM6 | pbist::mem::ESRAM8,
        );
        wait_until_false!(sys.pbist_completed());
        if sys.pbist_fail() {
            panic!("PBIST RAM");
        }
        sys.pbist_stop();
    }
    syscore::ram_ecc_enable();
    let vim = vim::Vim::new();
    vim.parity_enable(true);

    sys.init_memory(system::Ram::Internal);
    sys.init_memory(system::Ram::Vim);
    syscore::irq_vic_enable();

    #[cfg(vfp)]
    syscore::vfp_enable();

    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);

    main(0, ::core::ptr::null());

    loop {
        cortexr4::asm::wfi();
    }
}
