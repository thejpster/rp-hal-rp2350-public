#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    badpasswd: BADPASSWD,
    vreg_ctrl: VREG_CTRL,
    vreg_sts: VREG_STS,
    vreg: VREG,
    vreg_lp_entry: VREG_LP_ENTRY,
    vreg_lp_exit: VREG_LP_EXIT,
    bod_ctrl: BOD_CTRL,
    bod: BOD,
    bod_lp_entry: BOD_LP_ENTRY,
    bod_lp_exit: BOD_LP_EXIT,
    lposc: LPOSC,
    chip_reset: CHIP_RESET,
    wdsel: WDSEL,
    seq_cfg: SEQ_CFG,
    state: STATE,
    pow_fastdiv: POW_FASTDIV,
    pow_delay: POW_DELAY,
    ext_ctrl0: EXT_CTRL0,
    ext_ctrl1: EXT_CTRL1,
    ext_time_ref: EXT_TIME_REF,
    lposc_freq_khz_int: LPOSC_FREQ_KHZ_INT,
    lposc_freq_khz_frac: LPOSC_FREQ_KHZ_FRAC,
    xosc_freq_khz_int: XOSC_FREQ_KHZ_INT,
    xosc_freq_khz_frac: XOSC_FREQ_KHZ_FRAC,
    set_time_63to48: SET_TIME_63TO48,
    set_time_47to32: SET_TIME_47TO32,
    set_time_31to16: SET_TIME_31TO16,
    set_time_15to0: SET_TIME_15TO0,
    read_time_upper: READ_TIME_UPPER,
    read_time_lower: READ_TIME_LOWER,
    alarm_time_63to48: ALARM_TIME_63TO48,
    alarm_time_47to32: ALARM_TIME_47TO32,
    alarm_time_31to16: ALARM_TIME_31TO16,
    alarm_time_15to0: ALARM_TIME_15TO0,
    timer: TIMER,
    pwrup0: PWRUP0,
    pwrup1: PWRUP1,
    pwrup2: PWRUP2,
    pwrup3: PWRUP3,
    current_pwrup_req: CURRENT_PWRUP_REQ,
    last_swcore_pwrup: LAST_SWCORE_PWRUP,
    dbg_pwrcfg: DBG_PWRCFG,
    bootdis: BOOTDIS,
    dbgconfig: DBGCONFIG,
    scratch0: SCRATCH0,
    scratch1: SCRATCH1,
    scratch2: SCRATCH2,
    scratch3: SCRATCH3,
    scratch4: SCRATCH4,
    scratch5: SCRATCH5,
    scratch6: SCRATCH6,
    scratch7: SCRATCH7,
    boot0: BOOT0,
    boot1: BOOT1,
    boot2: BOOT2,
    boot3: BOOT3,
    intr: INTR,
    inte: INTE,
    intf: INTF,
    ints: INTS,
}
impl RegisterBlock {
    #[doc = "0x00 - Indicates a bad password has been used"]
    #[inline(always)]
    pub const fn badpasswd(&self) -> &BADPASSWD {
        &self.badpasswd
    }
    #[doc = "0x04 - Voltage Regulator Control"]
    #[inline(always)]
    pub const fn vreg_ctrl(&self) -> &VREG_CTRL {
        &self.vreg_ctrl
    }
    #[doc = "0x08 - Voltage Regulator Status"]
    #[inline(always)]
    pub const fn vreg_sts(&self) -> &VREG_STS {
        &self.vreg_sts
    }
    #[doc = "0x0c - Voltage Regulator Settings"]
    #[inline(always)]
    pub const fn vreg(&self) -> &VREG {
        &self.vreg
    }
    #[doc = "0x10 - Voltage Regulator Low Power Entry Settings"]
    #[inline(always)]
    pub const fn vreg_lp_entry(&self) -> &VREG_LP_ENTRY {
        &self.vreg_lp_entry
    }
    #[doc = "0x14 - Voltage Regulator Low Power Exit Settings"]
    #[inline(always)]
    pub const fn vreg_lp_exit(&self) -> &VREG_LP_EXIT {
        &self.vreg_lp_exit
    }
    #[doc = "0x18 - Brown-out Detection Control"]
    #[inline(always)]
    pub const fn bod_ctrl(&self) -> &BOD_CTRL {
        &self.bod_ctrl
    }
    #[doc = "0x1c - Brown-out Detection Settings"]
    #[inline(always)]
    pub const fn bod(&self) -> &BOD {
        &self.bod
    }
    #[doc = "0x20 - Brown-out Detection Low Power Entry Settings"]
    #[inline(always)]
    pub const fn bod_lp_entry(&self) -> &BOD_LP_ENTRY {
        &self.bod_lp_entry
    }
    #[doc = "0x24 - Brown-out Detection Low Power Exit Settings"]
    #[inline(always)]
    pub const fn bod_lp_exit(&self) -> &BOD_LP_EXIT {
        &self.bod_lp_exit
    }
    #[doc = "0x28 - Low power oscillator control register."]
    #[inline(always)]
    pub const fn lposc(&self) -> &LPOSC {
        &self.lposc
    }
    #[doc = "0x2c - Chip reset control and status"]
    #[inline(always)]
    pub const fn chip_reset(&self) -> &CHIP_RESET {
        &self.chip_reset
    }
    #[doc = "0x30 - Allows a watchdog reset to reset the internal state of powman in addition to the power-on state machine (PSM). Note that powman ignores watchdog resets that do not select at least the CLOCKS stage or earlier stages in the PSM. If using these bits, it's recommended to set PSM_WDSEL to all-ones in addition to the desired bits in this register. Failing to select CLOCKS or earlier will result in the POWMAN_WDSEL register having no effect."]
    #[inline(always)]
    pub const fn wdsel(&self) -> &WDSEL {
        &self.wdsel
    }
    #[doc = "0x34 - For configuration of the power sequencer Writes are ignored while POWMAN_STATE_CHANGING=1"]
    #[inline(always)]
    pub const fn seq_cfg(&self) -> &SEQ_CFG {
        &self.seq_cfg
    }
    #[doc = "0x38 - This register controls the power state of the 4 power domains. The current power state is indicated in POWMAN_STATE_CURRENT which is read-only. To change the state, write to POWMAN_STATE_REQ. The coding of POWMAN_STATE_CURRENT &amp; POWMAN_STATE_REQ corresponds to the power states defined in the datasheet: bit 3 = SWCORE bit 2 = XIP cache bit 1 = SRAM0 bit 0 = SRAM1 0 = powered up 1 = powered down When POWMAN_STATE_REQ is written, the POWMAN_STATE_WAITING flag is set while the Power Manager determines what is required. If an invalid transition is requested the Power Manager will still register the request in POWMAN_STATE_REQ but will also set the POWMAN_BAD_REQ flag. It will then implement the power-up requests and ignore the power down requests. To do nothing would risk entering an unrecoverable lock-up state. Invalid requests are: any combination of power up and power down requests any request that results in swcore boing powered and xip unpowered If the request is to power down the switched-core domain then POWMAN_STATE_WAITING stays active until the processors halt. During this time the POWMAN_STATE_REQ field can be re-written to change or cancel the request. When the power state transition begins the POWMAN_STATE_WAITING_flag is cleared, the POWMAN_STATE_CHANGING flag is set and POWMAN register writes are ignored until the transition completes."]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn pow_fastdiv(&self) -> &POW_FASTDIV {
        &self.pow_fastdiv
    }
    #[doc = "0x40 - power state machine delays"]
    #[inline(always)]
    pub const fn pow_delay(&self) -> &POW_DELAY {
        &self.pow_delay
    }
    #[doc = "0x44 - Configures a gpio as a power mode aware control output"]
    #[inline(always)]
    pub const fn ext_ctrl0(&self) -> &EXT_CTRL0 {
        &self.ext_ctrl0
    }
    #[doc = "0x48 - Configures a gpio as a power mode aware control output"]
    #[inline(always)]
    pub const fn ext_ctrl1(&self) -> &EXT_CTRL1 {
        &self.ext_ctrl1
    }
    #[doc = "0x4c - Select a GPIO to use as a time reference, the source can be used to drive the low power clock at 32kHz, or to provide a 1ms tick to the timer, or provide a 1Hz tick to the timer. The tick selection is controlled by the POWMAN_TIMER register."]
    #[inline(always)]
    pub const fn ext_time_ref(&self) -> &EXT_TIME_REF {
        &self.ext_time_ref
    }
    #[doc = "0x50 - Informs the AON Timer of the integer component of the clock frequency when running off the LPOSC."]
    #[inline(always)]
    pub const fn lposc_freq_khz_int(&self) -> &LPOSC_FREQ_KHZ_INT {
        &self.lposc_freq_khz_int
    }
    #[doc = "0x54 - Informs the AON Timer of the fractional component of the clock frequency when running off the LPOSC."]
    #[inline(always)]
    pub const fn lposc_freq_khz_frac(&self) -> &LPOSC_FREQ_KHZ_FRAC {
        &self.lposc_freq_khz_frac
    }
    #[doc = "0x58 - Informs the AON Timer of the integer component of the clock frequency when running off the XOSC."]
    #[inline(always)]
    pub const fn xosc_freq_khz_int(&self) -> &XOSC_FREQ_KHZ_INT {
        &self.xosc_freq_khz_int
    }
    #[doc = "0x5c - Informs the AON Timer of the fractional component of the clock frequency when running off the XOSC."]
    #[inline(always)]
    pub const fn xosc_freq_khz_frac(&self) -> &XOSC_FREQ_KHZ_FRAC {
        &self.xosc_freq_khz_frac
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn set_time_63to48(&self) -> &SET_TIME_63TO48 {
        &self.set_time_63to48
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn set_time_47to32(&self) -> &SET_TIME_47TO32 {
        &self.set_time_47to32
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn set_time_31to16(&self) -> &SET_TIME_31TO16 {
        &self.set_time_31to16
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn set_time_15to0(&self) -> &SET_TIME_15TO0 {
        &self.set_time_15to0
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn read_time_upper(&self) -> &READ_TIME_UPPER {
        &self.read_time_upper
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn read_time_lower(&self) -> &READ_TIME_LOWER {
        &self.read_time_lower
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn alarm_time_63to48(&self) -> &ALARM_TIME_63TO48 {
        &self.alarm_time_63to48
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn alarm_time_47to32(&self) -> &ALARM_TIME_47TO32 {
        &self.alarm_time_47to32
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn alarm_time_31to16(&self) -> &ALARM_TIME_31TO16 {
        &self.alarm_time_31to16
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn alarm_time_15to0(&self) -> &ALARM_TIME_15TO0 {
        &self.alarm_time_15to0
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn timer(&self) -> &TIMER {
        &self.timer
    }
    #[doc = "0x8c - 4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
    #[inline(always)]
    pub const fn pwrup0(&self) -> &PWRUP0 {
        &self.pwrup0
    }
    #[doc = "0x90 - 4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
    #[inline(always)]
    pub const fn pwrup1(&self) -> &PWRUP1 {
        &self.pwrup1
    }
    #[doc = "0x94 - 4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
    #[inline(always)]
    pub const fn pwrup2(&self) -> &PWRUP2 {
        &self.pwrup2
    }
    #[doc = "0x98 - 4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
    #[inline(always)]
    pub const fn pwrup3(&self) -> &PWRUP3 {
        &self.pwrup3
    }
    #[doc = "0x9c - Indicates current powerup request state pwrup events can be cleared by removing the enable from the pwrup register. The alarm pwrup req can be cleared by clearing timer.alarm_enab 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup"]
    #[inline(always)]
    pub const fn current_pwrup_req(&self) -> &CURRENT_PWRUP_REQ {
        &self.current_pwrup_req
    }
    #[doc = "0xa0 - Indicates which pwrup source triggered the last switched-core power up 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup"]
    #[inline(always)]
    pub const fn last_swcore_pwrup(&self) -> &LAST_SWCORE_PWRUP {
        &self.last_swcore_pwrup
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn dbg_pwrcfg(&self) -> &DBG_PWRCFG {
        &self.dbg_pwrcfg
    }
    #[doc = "0xa8 - Tell the bootrom to ignore the BOOT0..3 registers following the next RSM reset (e.g. the next core power down/up). If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by powering the core up and down. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the OTP BOOTDIS register."]
    #[inline(always)]
    pub const fn bootdis(&self) -> &BOOTDIS {
        &self.bootdis
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn dbgconfig(&self) -> &DBGCONFIG {
        &self.dbgconfig
    }
    #[doc = "0xb0 - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn scratch0(&self) -> &SCRATCH0 {
        &self.scratch0
    }
    #[doc = "0xb4 - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn scratch1(&self) -> &SCRATCH1 {
        &self.scratch1
    }
    #[doc = "0xb8 - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn scratch2(&self) -> &SCRATCH2 {
        &self.scratch2
    }
    #[doc = "0xbc - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn scratch3(&self) -> &SCRATCH3 {
        &self.scratch3
    }
    #[doc = "0xc0 - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn scratch4(&self) -> &SCRATCH4 {
        &self.scratch4
    }
    #[doc = "0xc4 - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn scratch5(&self) -> &SCRATCH5 {
        &self.scratch5
    }
    #[doc = "0xc8 - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn scratch6(&self) -> &SCRATCH6 {
        &self.scratch6
    }
    #[doc = "0xcc - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn scratch7(&self) -> &SCRATCH7 {
        &self.scratch7
    }
    #[doc = "0xd0 - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn boot0(&self) -> &BOOT0 {
        &self.boot0
    }
    #[doc = "0xd4 - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn boot1(&self) -> &BOOT1 {
        &self.boot1
    }
    #[doc = "0xd8 - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn boot2(&self) -> &BOOT2 {
        &self.boot2
    }
    #[doc = "0xdc - Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn boot3(&self) -> &BOOT3 {
        &self.boot3
    }
    #[doc = "0xe0 - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0xe4 - Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(&self) -> &INTE {
        &self.inte
    }
    #[doc = "0xe8 - Interrupt Force"]
    #[inline(always)]
    pub const fn intf(&self) -> &INTF {
        &self.intf
    }
    #[doc = "0xec - Interrupt status after masking &amp; forcing"]
    #[inline(always)]
    pub const fn ints(&self) -> &INTS {
        &self.ints
    }
}
#[doc = "BADPASSWD (rw) register accessor: Indicates a bad password has been used  

You can [`read`](crate::Reg::read) this register and get [`badpasswd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`badpasswd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@badpasswd`]
module"]
pub type BADPASSWD = crate::Reg<badpasswd::BADPASSWD_SPEC>;
#[doc = "Indicates a bad password has been used"]
pub mod badpasswd;
#[doc = "VREG_CTRL (rw) register accessor: Voltage Regulator Control  

You can [`read`](crate::Reg::read) this register and get [`vreg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@vreg_ctrl`]
module"]
pub type VREG_CTRL = crate::Reg<vreg_ctrl::VREG_CTRL_SPEC>;
#[doc = "Voltage Regulator Control"]
pub mod vreg_ctrl;
#[doc = "VREG_STS (rw) register accessor: Voltage Regulator Status  

You can [`read`](crate::Reg::read) this register and get [`vreg_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@vreg_sts`]
module"]
pub type VREG_STS = crate::Reg<vreg_sts::VREG_STS_SPEC>;
#[doc = "Voltage Regulator Status"]
pub mod vreg_sts;
#[doc = "VREG (rw) register accessor: Voltage Regulator Settings  

You can [`read`](crate::Reg::read) this register and get [`vreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@vreg`]
module"]
pub type VREG = crate::Reg<vreg::VREG_SPEC>;
#[doc = "Voltage Regulator Settings"]
pub mod vreg;
#[doc = "VREG_LP_ENTRY (rw) register accessor: Voltage Regulator Low Power Entry Settings  

You can [`read`](crate::Reg::read) this register and get [`vreg_lp_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg_lp_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@vreg_lp_entry`]
module"]
pub type VREG_LP_ENTRY = crate::Reg<vreg_lp_entry::VREG_LP_ENTRY_SPEC>;
#[doc = "Voltage Regulator Low Power Entry Settings"]
pub mod vreg_lp_entry;
#[doc = "VREG_LP_EXIT (rw) register accessor: Voltage Regulator Low Power Exit Settings  

You can [`read`](crate::Reg::read) this register and get [`vreg_lp_exit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg_lp_exit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@vreg_lp_exit`]
module"]
pub type VREG_LP_EXIT = crate::Reg<vreg_lp_exit::VREG_LP_EXIT_SPEC>;
#[doc = "Voltage Regulator Low Power Exit Settings"]
pub mod vreg_lp_exit;
#[doc = "BOD_CTRL (rw) register accessor: Brown-out Detection Control  

You can [`read`](crate::Reg::read) this register and get [`bod_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bod_ctrl`]
module"]
pub type BOD_CTRL = crate::Reg<bod_ctrl::BOD_CTRL_SPEC>;
#[doc = "Brown-out Detection Control"]
pub mod bod_ctrl;
#[doc = "BOD (rw) register accessor: Brown-out Detection Settings  

You can [`read`](crate::Reg::read) this register and get [`bod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bod`]
module"]
pub type BOD = crate::Reg<bod::BOD_SPEC>;
#[doc = "Brown-out Detection Settings"]
pub mod bod;
#[doc = "BOD_LP_ENTRY (rw) register accessor: Brown-out Detection Low Power Entry Settings  

You can [`read`](crate::Reg::read) this register and get [`bod_lp_entry::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod_lp_entry::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bod_lp_entry`]
module"]
pub type BOD_LP_ENTRY = crate::Reg<bod_lp_entry::BOD_LP_ENTRY_SPEC>;
#[doc = "Brown-out Detection Low Power Entry Settings"]
pub mod bod_lp_entry;
#[doc = "BOD_LP_EXIT (rw) register accessor: Brown-out Detection Low Power Exit Settings  

You can [`read`](crate::Reg::read) this register and get [`bod_lp_exit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod_lp_exit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bod_lp_exit`]
module"]
pub type BOD_LP_EXIT = crate::Reg<bod_lp_exit::BOD_LP_EXIT_SPEC>;
#[doc = "Brown-out Detection Low Power Exit Settings"]
pub mod bod_lp_exit;
#[doc = "LPOSC (rw) register accessor: Low power oscillator control register.  

You can [`read`](crate::Reg::read) this register and get [`lposc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lposc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@lposc`]
module"]
pub type LPOSC = crate::Reg<lposc::LPOSC_SPEC>;
#[doc = "Low power oscillator control register."]
pub mod lposc;
#[doc = "CHIP_RESET (rw) register accessor: Chip reset control and status  

You can [`read`](crate::Reg::read) this register and get [`chip_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chip_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chip_reset`]
module"]
pub type CHIP_RESET = crate::Reg<chip_reset::CHIP_RESET_SPEC>;
#[doc = "Chip reset control and status"]
pub mod chip_reset;
#[doc = "WDSEL (rw) register accessor: Allows a watchdog reset to reset the internal state of powman in addition to the power-on state machine (PSM). Note that powman ignores watchdog resets that do not select at least the CLOCKS stage or earlier stages in the PSM. If using these bits, it's recommended to set PSM_WDSEL to all-ones in addition to the desired bits in this register. Failing to select CLOCKS or earlier will result in the POWMAN_WDSEL register having no effect.  

You can [`read`](crate::Reg::read) this register and get [`wdsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@wdsel`]
module"]
pub type WDSEL = crate::Reg<wdsel::WDSEL_SPEC>;
#[doc = "Allows a watchdog reset to reset the internal state of powman in addition to the power-on state machine (PSM). Note that powman ignores watchdog resets that do not select at least the CLOCKS stage or earlier stages in the PSM. If using these bits, it's recommended to set PSM_WDSEL to all-ones in addition to the desired bits in this register. Failing to select CLOCKS or earlier will result in the POWMAN_WDSEL register having no effect."]
pub mod wdsel;
#[doc = "SEQ_CFG (rw) register accessor: For configuration of the power sequencer Writes are ignored while POWMAN_STATE_CHANGING=1  

You can [`read`](crate::Reg::read) this register and get [`seq_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seq_cfg`]
module"]
pub type SEQ_CFG = crate::Reg<seq_cfg::SEQ_CFG_SPEC>;
#[doc = "For configuration of the power sequencer Writes are ignored while POWMAN_STATE_CHANGING=1"]
pub mod seq_cfg;
#[doc = "STATE (rw) register accessor: This register controls the power state of the 4 power domains. The current power state is indicated in POWMAN_STATE_CURRENT which is read-only. To change the state, write to POWMAN_STATE_REQ. The coding of POWMAN_STATE_CURRENT &amp; POWMAN_STATE_REQ corresponds to the power states defined in the datasheet: bit 3 = SWCORE bit 2 = XIP cache bit 1 = SRAM0 bit 0 = SRAM1 0 = powered up 1 = powered down When POWMAN_STATE_REQ is written, the POWMAN_STATE_WAITING flag is set while the Power Manager determines what is required. If an invalid transition is requested the Power Manager will still register the request in POWMAN_STATE_REQ but will also set the POWMAN_BAD_REQ flag. It will then implement the power-up requests and ignore the power down requests. To do nothing would risk entering an unrecoverable lock-up state. Invalid requests are: any combination of power up and power down requests any request that results in swcore boing powered and xip unpowered If the request is to power down the switched-core domain then POWMAN_STATE_WAITING stays active until the processors halt. During this time the POWMAN_STATE_REQ field can be re-written to change or cancel the request. When the power state transition begins the POWMAN_STATE_WAITING_flag is cleared, the POWMAN_STATE_CHANGING flag is set and POWMAN register writes are ignored until the transition completes.  

You can [`read`](crate::Reg::read) this register and get [`state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@state`]
module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "This register controls the power state of the 4 power domains. The current power state is indicated in POWMAN_STATE_CURRENT which is read-only. To change the state, write to POWMAN_STATE_REQ. The coding of POWMAN_STATE_CURRENT &amp; POWMAN_STATE_REQ corresponds to the power states defined in the datasheet: bit 3 = SWCORE bit 2 = XIP cache bit 1 = SRAM0 bit 0 = SRAM1 0 = powered up 1 = powered down When POWMAN_STATE_REQ is written, the POWMAN_STATE_WAITING flag is set while the Power Manager determines what is required. If an invalid transition is requested the Power Manager will still register the request in POWMAN_STATE_REQ but will also set the POWMAN_BAD_REQ flag. It will then implement the power-up requests and ignore the power down requests. To do nothing would risk entering an unrecoverable lock-up state. Invalid requests are: any combination of power up and power down requests any request that results in swcore boing powered and xip unpowered If the request is to power down the switched-core domain then POWMAN_STATE_WAITING stays active until the processors halt. During this time the POWMAN_STATE_REQ field can be re-written to change or cancel the request. When the power state transition begins the POWMAN_STATE_WAITING_flag is cleared, the POWMAN_STATE_CHANGING flag is set and POWMAN register writes are ignored until the transition completes."]
pub mod state;
#[doc = "POW_FASTDIV (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`pow_fastdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pow_fastdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pow_fastdiv`]
module"]
pub type POW_FASTDIV = crate::Reg<pow_fastdiv::POW_FASTDIV_SPEC>;
#[doc = ""]
pub mod pow_fastdiv;
#[doc = "POW_DELAY (rw) register accessor: power state machine delays  

You can [`read`](crate::Reg::read) this register and get [`pow_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pow_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pow_delay`]
module"]
pub type POW_DELAY = crate::Reg<pow_delay::POW_DELAY_SPEC>;
#[doc = "power state machine delays"]
pub mod pow_delay;
#[doc = "EXT_CTRL0 (rw) register accessor: Configures a gpio as a power mode aware control output  

You can [`read`](crate::Reg::read) this register and get [`ext_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ext_ctrl0`]
module"]
pub type EXT_CTRL0 = crate::Reg<ext_ctrl0::EXT_CTRL0_SPEC>;
#[doc = "Configures a gpio as a power mode aware control output"]
pub mod ext_ctrl0;
#[doc = "EXT_CTRL1 (rw) register accessor: Configures a gpio as a power mode aware control output  

You can [`read`](crate::Reg::read) this register and get [`ext_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ext_ctrl1`]
module"]
pub type EXT_CTRL1 = crate::Reg<ext_ctrl1::EXT_CTRL1_SPEC>;
#[doc = "Configures a gpio as a power mode aware control output"]
pub mod ext_ctrl1;
#[doc = "EXT_TIME_REF (rw) register accessor: Select a GPIO to use as a time reference, the source can be used to drive the low power clock at 32kHz, or to provide a 1ms tick to the timer, or provide a 1Hz tick to the timer. The tick selection is controlled by the POWMAN_TIMER register.  

You can [`read`](crate::Reg::read) this register and get [`ext_time_ref::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_time_ref::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ext_time_ref`]
module"]
pub type EXT_TIME_REF = crate::Reg<ext_time_ref::EXT_TIME_REF_SPEC>;
#[doc = "Select a GPIO to use as a time reference, the source can be used to drive the low power clock at 32kHz, or to provide a 1ms tick to the timer, or provide a 1Hz tick to the timer. The tick selection is controlled by the POWMAN_TIMER register."]
pub mod ext_time_ref;
#[doc = "LPOSC_FREQ_KHZ_INT (rw) register accessor: Informs the AON Timer of the integer component of the clock frequency when running off the LPOSC.  

You can [`read`](crate::Reg::read) this register and get [`lposc_freq_khz_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lposc_freq_khz_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@lposc_freq_khz_int`]
module"]
pub type LPOSC_FREQ_KHZ_INT = crate::Reg<lposc_freq_khz_int::LPOSC_FREQ_KHZ_INT_SPEC>;
#[doc = "Informs the AON Timer of the integer component of the clock frequency when running off the LPOSC."]
pub mod lposc_freq_khz_int;
#[doc = "LPOSC_FREQ_KHZ_FRAC (rw) register accessor: Informs the AON Timer of the fractional component of the clock frequency when running off the LPOSC.  

You can [`read`](crate::Reg::read) this register and get [`lposc_freq_khz_frac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lposc_freq_khz_frac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@lposc_freq_khz_frac`]
module"]
pub type LPOSC_FREQ_KHZ_FRAC = crate::Reg<lposc_freq_khz_frac::LPOSC_FREQ_KHZ_FRAC_SPEC>;
#[doc = "Informs the AON Timer of the fractional component of the clock frequency when running off the LPOSC."]
pub mod lposc_freq_khz_frac;
#[doc = "XOSC_FREQ_KHZ_INT (rw) register accessor: Informs the AON Timer of the integer component of the clock frequency when running off the XOSC.  

You can [`read`](crate::Reg::read) this register and get [`xosc_freq_khz_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc_freq_khz_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@xosc_freq_khz_int`]
module"]
pub type XOSC_FREQ_KHZ_INT = crate::Reg<xosc_freq_khz_int::XOSC_FREQ_KHZ_INT_SPEC>;
#[doc = "Informs the AON Timer of the integer component of the clock frequency when running off the XOSC."]
pub mod xosc_freq_khz_int;
#[doc = "XOSC_FREQ_KHZ_FRAC (rw) register accessor: Informs the AON Timer of the fractional component of the clock frequency when running off the XOSC.  

You can [`read`](crate::Reg::read) this register and get [`xosc_freq_khz_frac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc_freq_khz_frac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@xosc_freq_khz_frac`]
module"]
pub type XOSC_FREQ_KHZ_FRAC = crate::Reg<xosc_freq_khz_frac::XOSC_FREQ_KHZ_FRAC_SPEC>;
#[doc = "Informs the AON Timer of the fractional component of the clock frequency when running off the XOSC."]
pub mod xosc_freq_khz_frac;
#[doc = "SET_TIME_63TO48 (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`set_time_63to48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_time_63to48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@set_time_63to48`]
module"]
pub type SET_TIME_63TO48 = crate::Reg<set_time_63to48::SET_TIME_63TO48_SPEC>;
#[doc = ""]
pub mod set_time_63to48;
#[doc = "SET_TIME_47TO32 (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`set_time_47to32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_time_47to32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@set_time_47to32`]
module"]
pub type SET_TIME_47TO32 = crate::Reg<set_time_47to32::SET_TIME_47TO32_SPEC>;
#[doc = ""]
pub mod set_time_47to32;
#[doc = "SET_TIME_31TO16 (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`set_time_31to16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_time_31to16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@set_time_31to16`]
module"]
pub type SET_TIME_31TO16 = crate::Reg<set_time_31to16::SET_TIME_31TO16_SPEC>;
#[doc = ""]
pub mod set_time_31to16;
#[doc = "SET_TIME_15TO0 (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`set_time_15to0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_time_15to0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@set_time_15to0`]
module"]
pub type SET_TIME_15TO0 = crate::Reg<set_time_15to0::SET_TIME_15TO0_SPEC>;
#[doc = ""]
pub mod set_time_15to0;
#[doc = "READ_TIME_UPPER (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`read_time_upper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_time_upper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@read_time_upper`]
module"]
pub type READ_TIME_UPPER = crate::Reg<read_time_upper::READ_TIME_UPPER_SPEC>;
#[doc = ""]
pub mod read_time_upper;
#[doc = "READ_TIME_LOWER (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`read_time_lower::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_time_lower::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@read_time_lower`]
module"]
pub type READ_TIME_LOWER = crate::Reg<read_time_lower::READ_TIME_LOWER_SPEC>;
#[doc = ""]
pub mod read_time_lower;
#[doc = "ALARM_TIME_63TO48 (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`alarm_time_63to48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm_time_63to48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm_time_63to48`]
module"]
pub type ALARM_TIME_63TO48 = crate::Reg<alarm_time_63to48::ALARM_TIME_63TO48_SPEC>;
#[doc = ""]
pub mod alarm_time_63to48;
#[doc = "ALARM_TIME_47TO32 (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`alarm_time_47to32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm_time_47to32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm_time_47to32`]
module"]
pub type ALARM_TIME_47TO32 = crate::Reg<alarm_time_47to32::ALARM_TIME_47TO32_SPEC>;
#[doc = ""]
pub mod alarm_time_47to32;
#[doc = "ALARM_TIME_31TO16 (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`alarm_time_31to16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm_time_31to16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm_time_31to16`]
module"]
pub type ALARM_TIME_31TO16 = crate::Reg<alarm_time_31to16::ALARM_TIME_31TO16_SPEC>;
#[doc = ""]
pub mod alarm_time_31to16;
#[doc = "ALARM_TIME_15TO0 (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`alarm_time_15to0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm_time_15to0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@alarm_time_15to0`]
module"]
pub type ALARM_TIME_15TO0 = crate::Reg<alarm_time_15to0::ALARM_TIME_15TO0_SPEC>;
#[doc = ""]
pub mod alarm_time_15to0;
#[doc = "TIMER (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timer`]
module"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = ""]
pub mod timer;
#[doc = "PWRUP0 (rw) register accessor: 4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high  

You can [`read`](crate::Reg::read) this register and get [`pwrup0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrup0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pwrup0`]
module"]
pub type PWRUP0 = crate::Reg<pwrup0::PWRUP0_SPEC>;
#[doc = "4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
pub mod pwrup0;
#[doc = "PWRUP1 (rw) register accessor: 4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high  

You can [`read`](crate::Reg::read) this register and get [`pwrup1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrup1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pwrup1`]
module"]
pub type PWRUP1 = crate::Reg<pwrup1::PWRUP1_SPEC>;
#[doc = "4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
pub mod pwrup1;
#[doc = "PWRUP2 (rw) register accessor: 4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high  

You can [`read`](crate::Reg::read) this register and get [`pwrup2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrup2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pwrup2`]
module"]
pub type PWRUP2 = crate::Reg<pwrup2::PWRUP2_SPEC>;
#[doc = "4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
pub mod pwrup2;
#[doc = "PWRUP3 (rw) register accessor: 4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high  

You can [`read`](crate::Reg::read) this register and get [`pwrup3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrup3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pwrup3`]
module"]
pub type PWRUP3 = crate::Reg<pwrup3::PWRUP3_SPEC>;
#[doc = "4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
pub mod pwrup3;
#[doc = "CURRENT_PWRUP_REQ (rw) register accessor: Indicates current powerup request state pwrup events can be cleared by removing the enable from the pwrup register. The alarm pwrup req can be cleared by clearing timer.alarm_enab 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup  

You can [`read`](crate::Reg::read) this register and get [`current_pwrup_req::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`current_pwrup_req::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@current_pwrup_req`]
module"]
pub type CURRENT_PWRUP_REQ = crate::Reg<current_pwrup_req::CURRENT_PWRUP_REQ_SPEC>;
#[doc = "Indicates current powerup request state pwrup events can be cleared by removing the enable from the pwrup register. The alarm pwrup req can be cleared by clearing timer.alarm_enab 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup"]
pub mod current_pwrup_req;
#[doc = "LAST_SWCORE_PWRUP (rw) register accessor: Indicates which pwrup source triggered the last switched-core power up 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup  

You can [`read`](crate::Reg::read) this register and get [`last_swcore_pwrup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`last_swcore_pwrup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@last_swcore_pwrup`]
module"]
pub type LAST_SWCORE_PWRUP = crate::Reg<last_swcore_pwrup::LAST_SWCORE_PWRUP_SPEC>;
#[doc = "Indicates which pwrup source triggered the last switched-core power up 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup"]
pub mod last_swcore_pwrup;
#[doc = "DBG_PWRCFG (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`dbg_pwrcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_pwrcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbg_pwrcfg`]
module"]
pub type DBG_PWRCFG = crate::Reg<dbg_pwrcfg::DBG_PWRCFG_SPEC>;
#[doc = ""]
pub mod dbg_pwrcfg;
#[doc = "BOOTDIS (rw) register accessor: Tell the bootrom to ignore the BOOT0..3 registers following the next RSM reset (e.g. the next core power down/up). If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by powering the core up and down. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the OTP BOOTDIS register.  

You can [`read`](crate::Reg::read) this register and get [`bootdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootdis`]
module"]
pub type BOOTDIS = crate::Reg<bootdis::BOOTDIS_SPEC>;
#[doc = "Tell the bootrom to ignore the BOOT0..3 registers following the next RSM reset (e.g. the next core power down/up). If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by powering the core up and down. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the OTP BOOTDIS register."]
pub mod bootdis;
#[doc = "DBGCONFIG (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`dbgconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbgconfig`]
module"]
pub type DBGCONFIG = crate::Reg<dbgconfig::DBGCONFIG_SPEC>;
#[doc = ""]
pub mod dbgconfig;
#[doc = "SCRATCH0 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`scratch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch0`]
module"]
pub type SCRATCH0 = crate::Reg<scratch0::SCRATCH0_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod scratch0;
#[doc = "SCRATCH1 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`scratch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch1`]
module"]
pub type SCRATCH1 = crate::Reg<scratch1::SCRATCH1_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod scratch1;
#[doc = "SCRATCH2 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`scratch2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch2`]
module"]
pub type SCRATCH2 = crate::Reg<scratch2::SCRATCH2_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod scratch2;
#[doc = "SCRATCH3 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`scratch3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch3`]
module"]
pub type SCRATCH3 = crate::Reg<scratch3::SCRATCH3_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod scratch3;
#[doc = "SCRATCH4 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`scratch4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch4`]
module"]
pub type SCRATCH4 = crate::Reg<scratch4::SCRATCH4_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod scratch4;
#[doc = "SCRATCH5 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`scratch5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch5`]
module"]
pub type SCRATCH5 = crate::Reg<scratch5::SCRATCH5_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod scratch5;
#[doc = "SCRATCH6 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`scratch6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch6`]
module"]
pub type SCRATCH6 = crate::Reg<scratch6::SCRATCH6_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod scratch6;
#[doc = "SCRATCH7 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`scratch7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scratch7`]
module"]
pub type SCRATCH7 = crate::Reg<scratch7::SCRATCH7_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod scratch7;
#[doc = "BOOT0 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`boot0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@boot0`]
module"]
pub type BOOT0 = crate::Reg<boot0::BOOT0_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod boot0;
#[doc = "BOOT1 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`boot1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@boot1`]
module"]
pub type BOOT1 = crate::Reg<boot1::BOOT1_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod boot1;
#[doc = "BOOT2 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`boot2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@boot2`]
module"]
pub type BOOT2 = crate::Reg<boot2::BOOT2_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod boot2;
#[doc = "BOOT3 (rw) register accessor: Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`boot3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@boot3`]
module"]
pub type BOOT3 = crate::Reg<boot3::BOOT3_SPEC>;
#[doc = "Scratch register. Information persists in low power mode"]
pub mod boot3;
#[doc = "INTR (rw) register accessor: Raw Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "INTE (rw) register accessor: Interrupt Enable  

You can [`read`](crate::Reg::read) this register and get [`inte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte`]
module"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "INTF (rw) register accessor: Interrupt Force  

You can [`read`](crate::Reg::read) this register and get [`intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "INTS (rw) register accessor: Interrupt status after masking &amp; forcing  

You can [`read`](crate::Reg::read) this register and get [`ints::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ints::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints`]
module"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing"]
pub mod ints;
