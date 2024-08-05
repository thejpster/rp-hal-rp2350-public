#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lock: LOCK,
    force_core_ns: FORCE_CORE_NS,
    cfgreset: CFGRESET,
    gpio_nsmask0: GPIO_NSMASK0,
    gpio_nsmask1: GPIO_NSMASK1,
    rom: ROM,
    xip_main: XIP_MAIN,
    sram0: SRAM0,
    sram1: SRAM1,
    sram2: SRAM2,
    sram3: SRAM3,
    sram4: SRAM4,
    sram5: SRAM5,
    sram6: SRAM6,
    sram7: SRAM7,
    sram8: SRAM8,
    sram9: SRAM9,
    dma: DMA,
    usbctrl: USBCTRL,
    pio0: PIO0,
    pio1: PIO1,
    pio2: PIO2,
    coresight_trace: CORESIGHT_TRACE,
    coresight_periph: CORESIGHT_PERIPH,
    sysinfo: SYSINFO,
    resets: RESETS,
    io_bank0: IO_BANK0,
    io_bank1: IO_BANK1,
    pads_bank0: PADS_BANK0,
    pads_qspi: PADS_QSPI,
    busctrl: BUSCTRL,
    adc0: ADC0,
    hstx: HSTX,
    i2c0: I2C0,
    i2c1: I2C1,
    pwm: PWM,
    spi0: SPI0,
    spi1: SPI1,
    timer0: TIMER0,
    timer1: TIMER1,
    uart0: UART0,
    uart1: UART1,
    otp: OTP,
    tbman: TBMAN,
    powman: POWMAN,
    trng: TRNG,
    sha256: SHA256,
    syscfg: SYSCFG,
    clocks: CLOCKS,
    xosc: XOSC,
    rosc: ROSC,
    pll_sys: PLL_SYS,
    pll_usb: PLL_USB,
    ticks: TICKS,
    watchdog: WATCHDOG,
    rsm: RSM,
    xip_ctrl: XIP_CTRL,
    xip_qmi: XIP_QMI,
    xip_aux: XIP_AUX,
}
impl RegisterBlock {
    #[doc = "0x00 - Once a LOCK bit is written to 1, ACCESSCTRL silently ignores writes from that master. LOCK is writable only by a Secure, Privileged processor or debugger. LOCK bits are only writable when their value is zero. Once set, they can never be cleared, except by a full reset of ACCESSCTRL Setting the LOCK bit does not affect whether an access raises a bus error. Unprivileged writes, or writes from the DMA, will continue to raise bus errors. All other accesses will continue not to."]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
    #[doc = "0x04 - Force core 1's bus accesses to always be Non-secure, no matter the core's internal state. Useful for schemes where one core is designated as the Non-secure core, since some peripherals may filter individual registers internally based on security state but not on master ID."]
    #[inline(always)]
    pub const fn force_core_ns(&self) -> &FORCE_CORE_NS {
        &self.force_core_ns
    }
    #[doc = "0x08 - Write 1 to reset all ACCESSCTRL configuration, except for the LOCK and FORCE_CORE_NS registers. This bit is used in the RP2350 bootrom to quickly restore ACCESSCTRL to a known state during the boot path. Note that, like all registers in ACCESSCTRL, this register is not writable when the writer's corresponding LOCK bit is set, therefore a master which has been locked out of ACCESSCTRL can not use the CFGRESET register to disturb its contents."]
    #[inline(always)]
    pub const fn cfgreset(&self) -> &CFGRESET {
        &self.cfgreset
    }
    #[doc = "0x0c - Control whether GPIO0...31 are accessible to Non-secure code. Writable only by a Secure, Privileged processor or debugger. 0 -> Secure access only 1 -> Secure + Non-secure access"]
    #[inline(always)]
    pub const fn gpio_nsmask0(&self) -> &GPIO_NSMASK0 {
        &self.gpio_nsmask0
    }
    #[doc = "0x10 - Control whether GPIO32..47 are accessible to Non-secure code, and whether QSPI and USB bitbang are accessible through the Non-secure SIO. Writable only by a Secure, Privileged processor or debugger."]
    #[inline(always)]
    pub const fn gpio_nsmask1(&self) -> &GPIO_NSMASK1 {
        &self.gpio_nsmask1
    }
    #[doc = "0x14 - Control whether debugger, DMA, core 0 and core 1 can access ROM, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn rom(&self) -> &ROM {
        &self.rom
    }
    #[doc = "0x18 - Control whether debugger, DMA, core 0 and core 1 can access XIP_MAIN, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn xip_main(&self) -> &XIP_MAIN {
        &self.xip_main
    }
    #[doc = "0x1c - Control whether debugger, DMA, core 0 and core 1 can access SRAM0, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram0(&self) -> &SRAM0 {
        &self.sram0
    }
    #[doc = "0x20 - Control whether debugger, DMA, core 0 and core 1 can access SRAM1, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram1(&self) -> &SRAM1 {
        &self.sram1
    }
    #[doc = "0x24 - Control whether debugger, DMA, core 0 and core 1 can access SRAM2, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram2(&self) -> &SRAM2 {
        &self.sram2
    }
    #[doc = "0x28 - Control whether debugger, DMA, core 0 and core 1 can access SRAM3, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram3(&self) -> &SRAM3 {
        &self.sram3
    }
    #[doc = "0x2c - Control whether debugger, DMA, core 0 and core 1 can access SRAM4, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram4(&self) -> &SRAM4 {
        &self.sram4
    }
    #[doc = "0x30 - Control whether debugger, DMA, core 0 and core 1 can access SRAM5, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram5(&self) -> &SRAM5 {
        &self.sram5
    }
    #[doc = "0x34 - Control whether debugger, DMA, core 0 and core 1 can access SRAM6, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram6(&self) -> &SRAM6 {
        &self.sram6
    }
    #[doc = "0x38 - Control whether debugger, DMA, core 0 and core 1 can access SRAM7, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram7(&self) -> &SRAM7 {
        &self.sram7
    }
    #[doc = "0x3c - Control whether debugger, DMA, core 0 and core 1 can access SRAM8, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram8(&self) -> &SRAM8 {
        &self.sram8
    }
    #[doc = "0x40 - Control whether debugger, DMA, core 0 and core 1 can access SRAM9, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram9(&self) -> &SRAM9 {
        &self.sram9
    }
    #[doc = "0x44 - Control whether debugger, DMA, core 0 and core 1 can access DMA, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn dma(&self) -> &DMA {
        &self.dma
    }
    #[doc = "0x48 - Control whether debugger, DMA, core 0 and core 1 can access USBCTRL, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn usbctrl(&self) -> &USBCTRL {
        &self.usbctrl
    }
    #[doc = "0x4c - Control whether debugger, DMA, core 0 and core 1 can access PIO0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pio0(&self) -> &PIO0 {
        &self.pio0
    }
    #[doc = "0x50 - Control whether debugger, DMA, core 0 and core 1 can access PIO1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pio1(&self) -> &PIO1 {
        &self.pio1
    }
    #[doc = "0x54 - Control whether debugger, DMA, core 0 and core 1 can access PIO2, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pio2(&self) -> &PIO2 {
        &self.pio2
    }
    #[doc = "0x58 - Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_TRACE, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn coresight_trace(&self) -> &CORESIGHT_TRACE {
        &self.coresight_trace
    }
    #[doc = "0x5c - Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_PERIPH, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn coresight_periph(&self) -> &CORESIGHT_PERIPH {
        &self.coresight_periph
    }
    #[doc = "0x60 - Control whether debugger, DMA, core 0 and core 1 can access SYSINFO, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sysinfo(&self) -> &SYSINFO {
        &self.sysinfo
    }
    #[doc = "0x64 - Control whether debugger, DMA, core 0 and core 1 can access RESETS, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn resets(&self) -> &RESETS {
        &self.resets
    }
    #[doc = "0x68 - Control whether debugger, DMA, core 0 and core 1 can access IO_BANK0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn io_bank0(&self) -> &IO_BANK0 {
        &self.io_bank0
    }
    #[doc = "0x6c - Control whether debugger, DMA, core 0 and core 1 can access IO_BANK1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn io_bank1(&self) -> &IO_BANK1 {
        &self.io_bank1
    }
    #[doc = "0x70 - Control whether debugger, DMA, core 0 and core 1 can access PADS_BANK0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pads_bank0(&self) -> &PADS_BANK0 {
        &self.pads_bank0
    }
    #[doc = "0x74 - Control whether debugger, DMA, core 0 and core 1 can access PADS_QSPI, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pads_qspi(&self) -> &PADS_QSPI {
        &self.pads_qspi
    }
    #[doc = "0x78 - Control whether debugger, DMA, core 0 and core 1 can access BUSCTRL, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn busctrl(&self) -> &BUSCTRL {
        &self.busctrl
    }
    #[doc = "0x7c - Control whether debugger, DMA, core 0 and core 1 can access ADC0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn adc0(&self) -> &ADC0 {
        &self.adc0
    }
    #[doc = "0x80 - Control whether debugger, DMA, core 0 and core 1 can access HSTX, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn hstx(&self) -> &HSTX {
        &self.hstx
    }
    #[doc = "0x84 - Control whether debugger, DMA, core 0 and core 1 can access I2C0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn i2c0(&self) -> &I2C0 {
        &self.i2c0
    }
    #[doc = "0x88 - Control whether debugger, DMA, core 0 and core 1 can access I2C1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn i2c1(&self) -> &I2C1 {
        &self.i2c1
    }
    #[doc = "0x8c - Control whether debugger, DMA, core 0 and core 1 can access PWM, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pwm(&self) -> &PWM {
        &self.pwm
    }
    #[doc = "0x90 - Control whether debugger, DMA, core 0 and core 1 can access SPI0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn spi0(&self) -> &SPI0 {
        &self.spi0
    }
    #[doc = "0x94 - Control whether debugger, DMA, core 0 and core 1 can access SPI1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn spi1(&self) -> &SPI1 {
        &self.spi1
    }
    #[doc = "0x98 - Control whether debugger, DMA, core 0 and core 1 can access TIMER0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn timer0(&self) -> &TIMER0 {
        &self.timer0
    }
    #[doc = "0x9c - Control whether debugger, DMA, core 0 and core 1 can access TIMER1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn timer1(&self) -> &TIMER1 {
        &self.timer1
    }
    #[doc = "0xa0 - Control whether debugger, DMA, core 0 and core 1 can access UART0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn uart0(&self) -> &UART0 {
        &self.uart0
    }
    #[doc = "0xa4 - Control whether debugger, DMA, core 0 and core 1 can access UART1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn uart1(&self) -> &UART1 {
        &self.uart1
    }
    #[doc = "0xa8 - Control whether debugger, DMA, core 0 and core 1 can access OTP, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn otp(&self) -> &OTP {
        &self.otp
    }
    #[doc = "0xac - Control whether debugger, DMA, core 0 and core 1 can access TBMAN, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn tbman(&self) -> &TBMAN {
        &self.tbman
    }
    #[doc = "0xb0 - Control whether debugger, DMA, core 0 and core 1 can access POWMAN, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn powman(&self) -> &POWMAN {
        &self.powman
    }
    #[doc = "0xb4 - Control whether debugger, DMA, core 0 and core 1 can access TRNG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn trng(&self) -> &TRNG {
        &self.trng
    }
    #[doc = "0xb8 - Control whether debugger, DMA, core 0 and core 1 can access SHA256, and at what security/privilege levels they can do so. Defaults to Secure, Privileged access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sha256(&self) -> &SHA256 {
        &self.sha256
    }
    #[doc = "0xbc - Control whether debugger, DMA, core 0 and core 1 can access SYSCFG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn syscfg(&self) -> &SYSCFG {
        &self.syscfg
    }
    #[doc = "0xc0 - Control whether debugger, DMA, core 0 and core 1 can access CLOCKS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn clocks(&self) -> &CLOCKS {
        &self.clocks
    }
    #[doc = "0xc4 - Control whether debugger, DMA, core 0 and core 1 can access XOSC, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn xosc(&self) -> &XOSC {
        &self.xosc
    }
    #[doc = "0xc8 - Control whether debugger, DMA, core 0 and core 1 can access ROSC, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn rosc(&self) -> &ROSC {
        &self.rosc
    }
    #[doc = "0xcc - Control whether debugger, DMA, core 0 and core 1 can access PLL_SYS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pll_sys(&self) -> &PLL_SYS {
        &self.pll_sys
    }
    #[doc = "0xd0 - Control whether debugger, DMA, core 0 and core 1 can access PLL_USB, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pll_usb(&self) -> &PLL_USB {
        &self.pll_usb
    }
    #[doc = "0xd4 - Control whether debugger, DMA, core 0 and core 1 can access TICKS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn ticks(&self) -> &TICKS {
        &self.ticks
    }
    #[doc = "0xd8 - Control whether debugger, DMA, core 0 and core 1 can access WATCHDOG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn watchdog(&self) -> &WATCHDOG {
        &self.watchdog
    }
    #[doc = "0xdc - Control whether debugger, DMA, core 0 and core 1 can access RSM, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn rsm(&self) -> &RSM {
        &self.rsm
    }
    #[doc = "0xe0 - Control whether debugger, DMA, core 0 and core 1 can access XIP_CTRL, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn xip_ctrl(&self) -> &XIP_CTRL {
        &self.xip_ctrl
    }
    #[doc = "0xe4 - Control whether debugger, DMA, core 0 and core 1 can access XIP_QMI, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn xip_qmi(&self) -> &XIP_QMI {
        &self.xip_qmi
    }
    #[doc = "0xe8 - Control whether debugger, DMA, core 0 and core 1 can access XIP_AUX, and at what security/privilege levels they can do so. Defaults to Secure, Privileged access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn xip_aux(&self) -> &XIP_AUX {
        &self.xip_aux
    }
}
#[doc = "LOCK (rw) register accessor: Once a LOCK bit is written to 1, ACCESSCTRL silently ignores writes from that master. LOCK is writable only by a Secure, Privileged processor or debugger. LOCK bits are only writable when their value is zero. Once set, they can never be cleared, except by a full reset of ACCESSCTRL Setting the LOCK bit does not affect whether an access raises a bus error. Unprivileged writes, or writes from the DMA, will continue to raise bus errors. All other accesses will continue not to.  

You can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Once a LOCK bit is written to 1, ACCESSCTRL silently ignores writes from that master. LOCK is writable only by a Secure, Privileged processor or debugger. LOCK bits are only writable when their value is zero. Once set, they can never be cleared, except by a full reset of ACCESSCTRL Setting the LOCK bit does not affect whether an access raises a bus error. Unprivileged writes, or writes from the DMA, will continue to raise bus errors. All other accesses will continue not to."]
pub mod lock;
#[doc = "FORCE_CORE_NS (rw) register accessor: Force core 1's bus accesses to always be Non-secure, no matter the core's internal state. Useful for schemes where one core is designated as the Non-secure core, since some peripherals may filter individual registers internally based on security state but not on master ID.  

You can [`read`](crate::Reg::read) this register and get [`force_core_ns::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_core_ns::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@force_core_ns`]
module"]
pub type FORCE_CORE_NS = crate::Reg<force_core_ns::FORCE_CORE_NS_SPEC>;
#[doc = "Force core 1's bus accesses to always be Non-secure, no matter the core's internal state. Useful for schemes where one core is designated as the Non-secure core, since some peripherals may filter individual registers internally based on security state but not on master ID."]
pub mod force_core_ns;
#[doc = "CFGRESET (rw) register accessor: Write 1 to reset all ACCESSCTRL configuration, except for the LOCK and FORCE_CORE_NS registers. This bit is used in the RP2350 bootrom to quickly restore ACCESSCTRL to a known state during the boot path. Note that, like all registers in ACCESSCTRL, this register is not writable when the writer's corresponding LOCK bit is set, therefore a master which has been locked out of ACCESSCTRL can not use the CFGRESET register to disturb its contents.  

You can [`read`](crate::Reg::read) this register and get [`cfgreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cfgreset`]
module"]
pub type CFGRESET = crate::Reg<cfgreset::CFGRESET_SPEC>;
#[doc = "Write 1 to reset all ACCESSCTRL configuration, except for the LOCK and FORCE_CORE_NS registers. This bit is used in the RP2350 bootrom to quickly restore ACCESSCTRL to a known state during the boot path. Note that, like all registers in ACCESSCTRL, this register is not writable when the writer's corresponding LOCK bit is set, therefore a master which has been locked out of ACCESSCTRL can not use the CFGRESET register to disturb its contents."]
pub mod cfgreset;
#[doc = "GPIO_NSMASK0 (rw) register accessor: Control whether GPIO0...31 are accessible to Non-secure code. Writable only by a Secure, Privileged processor or debugger. 0 -> Secure access only 1 -> Secure + Non-secure access  

You can [`read`](crate::Reg::read) this register and get [`gpio_nsmask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_nsmask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_nsmask0`]
module"]
pub type GPIO_NSMASK0 = crate::Reg<gpio_nsmask0::GPIO_NSMASK0_SPEC>;
#[doc = "Control whether GPIO0...31 are accessible to Non-secure code. Writable only by a Secure, Privileged processor or debugger. 0 -> Secure access only 1 -> Secure + Non-secure access"]
pub mod gpio_nsmask0;
#[doc = "GPIO_NSMASK1 (rw) register accessor: Control whether GPIO32..47 are accessible to Non-secure code, and whether QSPI and USB bitbang are accessible through the Non-secure SIO. Writable only by a Secure, Privileged processor or debugger.  

You can [`read`](crate::Reg::read) this register and get [`gpio_nsmask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_nsmask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_nsmask1`]
module"]
pub type GPIO_NSMASK1 = crate::Reg<gpio_nsmask1::GPIO_NSMASK1_SPEC>;
#[doc = "Control whether GPIO32..47 are accessible to Non-secure code, and whether QSPI and USB bitbang are accessible through the Non-secure SIO. Writable only by a Secure, Privileged processor or debugger."]
pub mod gpio_nsmask1;
#[doc = "ROM (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access ROM, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`rom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rom`]
module"]
pub type ROM = crate::Reg<rom::ROM_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access ROM, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod rom;
#[doc = "XIP_MAIN (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access XIP_MAIN, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`xip_main::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_main::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@xip_main`]
module"]
pub type XIP_MAIN = crate::Reg<xip_main::XIP_MAIN_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_MAIN, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod xip_main;
#[doc = "SRAM0 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SRAM0, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sram0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sram0`]
module"]
pub type SRAM0 = crate::Reg<sram0::SRAM0_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM0, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sram0;
#[doc = "SRAM1 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SRAM1, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sram1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sram1`]
module"]
pub type SRAM1 = crate::Reg<sram1::SRAM1_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM1, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sram1;
#[doc = "SRAM2 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SRAM2, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sram2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sram2`]
module"]
pub type SRAM2 = crate::Reg<sram2::SRAM2_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM2, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sram2;
#[doc = "SRAM3 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SRAM3, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sram3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sram3`]
module"]
pub type SRAM3 = crate::Reg<sram3::SRAM3_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM3, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sram3;
#[doc = "SRAM4 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SRAM4, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sram4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sram4`]
module"]
pub type SRAM4 = crate::Reg<sram4::SRAM4_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM4, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sram4;
#[doc = "SRAM5 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SRAM5, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sram5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sram5`]
module"]
pub type SRAM5 = crate::Reg<sram5::SRAM5_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM5, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sram5;
#[doc = "SRAM6 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SRAM6, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sram6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sram6`]
module"]
pub type SRAM6 = crate::Reg<sram6::SRAM6_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM6, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sram6;
#[doc = "SRAM7 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SRAM7, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sram7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sram7`]
module"]
pub type SRAM7 = crate::Reg<sram7::SRAM7_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM7, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sram7;
#[doc = "SRAM8 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SRAM8, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sram8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sram8`]
module"]
pub type SRAM8 = crate::Reg<sram8::SRAM8_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM8, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sram8;
#[doc = "SRAM9 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SRAM9, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sram9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sram9`]
module"]
pub type SRAM9 = crate::Reg<sram9::SRAM9_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM9, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sram9;
#[doc = "DMA (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access DMA, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access DMA, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod dma;
#[doc = "USBCTRL (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access USBCTRL, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`usbctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usbctrl`]
module"]
pub type USBCTRL = crate::Reg<usbctrl::USBCTRL_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access USBCTRL, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod usbctrl;
#[doc = "PIO0 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access PIO0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`pio0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pio0`]
module"]
pub type PIO0 = crate::Reg<pio0::PIO0_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PIO0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod pio0;
#[doc = "PIO1 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access PIO1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`pio1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pio1`]
module"]
pub type PIO1 = crate::Reg<pio1::PIO1_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PIO1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod pio1;
#[doc = "PIO2 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access PIO2, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`pio2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pio2`]
module"]
pub type PIO2 = crate::Reg<pio2::PIO2_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PIO2, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod pio2;
#[doc = "CORESIGHT_TRACE (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_TRACE, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`coresight_trace::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coresight_trace::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@coresight_trace`]
module"]
pub type CORESIGHT_TRACE = crate::Reg<coresight_trace::CORESIGHT_TRACE_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_TRACE, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod coresight_trace;
#[doc = "CORESIGHT_PERIPH (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_PERIPH, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`coresight_periph::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coresight_periph::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@coresight_periph`]
module"]
pub type CORESIGHT_PERIPH = crate::Reg<coresight_periph::CORESIGHT_PERIPH_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_PERIPH, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod coresight_periph;
#[doc = "SYSINFO (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SYSINFO, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sysinfo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysinfo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sysinfo`]
module"]
pub type SYSINFO = crate::Reg<sysinfo::SYSINFO_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SYSINFO, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sysinfo;
#[doc = "RESETS (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access RESETS, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`resets::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resets::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@resets`]
module"]
pub type RESETS = crate::Reg<resets::RESETS_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access RESETS, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod resets;
#[doc = "IO_BANK0 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access IO_BANK0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`io_bank0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_bank0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@io_bank0`]
module"]
pub type IO_BANK0 = crate::Reg<io_bank0::IO_BANK0_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access IO_BANK0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod io_bank0;
#[doc = "IO_BANK1 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access IO_BANK1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`io_bank1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_bank1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@io_bank1`]
module"]
pub type IO_BANK1 = crate::Reg<io_bank1::IO_BANK1_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access IO_BANK1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod io_bank1;
#[doc = "PADS_BANK0 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access PADS_BANK0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`pads_bank0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pads_bank0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pads_bank0`]
module"]
pub type PADS_BANK0 = crate::Reg<pads_bank0::PADS_BANK0_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PADS_BANK0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod pads_bank0;
#[doc = "PADS_QSPI (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access PADS_QSPI, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`pads_qspi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pads_qspi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pads_qspi`]
module"]
pub type PADS_QSPI = crate::Reg<pads_qspi::PADS_QSPI_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PADS_QSPI, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod pads_qspi;
#[doc = "BUSCTRL (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access BUSCTRL, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`busctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@busctrl`]
module"]
pub type BUSCTRL = crate::Reg<busctrl::BUSCTRL_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access BUSCTRL, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod busctrl;
#[doc = "ADC0 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access ADC0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`adc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@adc0`]
module"]
pub type ADC0 = crate::Reg<adc0::ADC0_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access ADC0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod adc0;
#[doc = "HSTX (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access HSTX, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`hstx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@hstx`]
module"]
pub type HSTX = crate::Reg<hstx::HSTX_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access HSTX, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod hstx;
#[doc = "I2C0 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access I2C0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`i2c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@i2c0`]
module"]
pub type I2C0 = crate::Reg<i2c0::I2C0_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access I2C0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod i2c0;
#[doc = "I2C1 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access I2C1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`i2c1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@i2c1`]
module"]
pub type I2C1 = crate::Reg<i2c1::I2C1_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access I2C1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod i2c1;
#[doc = "PWM (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access PWM, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`pwm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pwm`]
module"]
pub type PWM = crate::Reg<pwm::PWM_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PWM, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod pwm;
#[doc = "SPI0 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SPI0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`spi0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@spi0`]
module"]
pub type SPI0 = crate::Reg<spi0::SPI0_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SPI0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod spi0;
#[doc = "SPI1 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SPI1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`spi1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@spi1`]
module"]
pub type SPI1 = crate::Reg<spi1::SPI1_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SPI1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod spi1;
#[doc = "TIMER0 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access TIMER0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`timer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timer0`]
module"]
pub type TIMER0 = crate::Reg<timer0::TIMER0_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access TIMER0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod timer0;
#[doc = "TIMER1 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access TIMER1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`timer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timer1`]
module"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access TIMER1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod timer1;
#[doc = "UART0 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access UART0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`uart0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uart0`]
module"]
pub type UART0 = crate::Reg<uart0::UART0_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access UART0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod uart0;
#[doc = "UART1 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access UART1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`uart1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@uart1`]
module"]
pub type UART1 = crate::Reg<uart1::UART1_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access UART1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod uart1;
#[doc = "OTP (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access OTP, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`otp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@otp`]
module"]
pub type OTP = crate::Reg<otp::OTP_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access OTP, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod otp;
#[doc = "TBMAN (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access TBMAN, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`tbman::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbman::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tbman`]
module"]
pub type TBMAN = crate::Reg<tbman::TBMAN_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access TBMAN, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod tbman;
#[doc = "POWMAN (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access POWMAN, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`powman::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powman::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@powman`]
module"]
pub type POWMAN = crate::Reg<powman::POWMAN_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access POWMAN, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod powman;
#[doc = "TRNG (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access TRNG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`trng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trng`]
module"]
pub type TRNG = crate::Reg<trng::TRNG_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access TRNG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod trng;
#[doc = "SHA256 (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SHA256, and at what security/privilege levels they can do so. Defaults to Secure, Privileged access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`sha256::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha256::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sha256`]
module"]
pub type SHA256 = crate::Reg<sha256::SHA256_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SHA256, and at what security/privilege levels they can do so. Defaults to Secure, Privileged access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod sha256;
#[doc = "SYSCFG (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access SYSCFG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`syscfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@syscfg`]
module"]
pub type SYSCFG = crate::Reg<syscfg::SYSCFG_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SYSCFG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod syscfg;
#[doc = "CLOCKS (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access CLOCKS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`clocks::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clocks::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@clocks`]
module"]
pub type CLOCKS = crate::Reg<clocks::CLOCKS_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access CLOCKS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod clocks;
#[doc = "XOSC (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access XOSC, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`xosc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@xosc`]
module"]
pub type XOSC = crate::Reg<xosc::XOSC_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access XOSC, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod xosc;
#[doc = "ROSC (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access ROSC, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`rosc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rosc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rosc`]
module"]
pub type ROSC = crate::Reg<rosc::ROSC_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access ROSC, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod rosc;
#[doc = "PLL_SYS (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access PLL_SYS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`pll_sys::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_sys::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pll_sys`]
module"]
pub type PLL_SYS = crate::Reg<pll_sys::PLL_SYS_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PLL_SYS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod pll_sys;
#[doc = "PLL_USB (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access PLL_USB, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`pll_usb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_usb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pll_usb`]
module"]
pub type PLL_USB = crate::Reg<pll_usb::PLL_USB_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PLL_USB, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod pll_usb;
#[doc = "TICKS (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access TICKS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`ticks::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ticks::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ticks`]
module"]
pub type TICKS = crate::Reg<ticks::TICKS_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access TICKS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod ticks;
#[doc = "WATCHDOG (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access WATCHDOG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`watchdog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`watchdog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@watchdog`]
module"]
pub type WATCHDOG = crate::Reg<watchdog::WATCHDOG_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access WATCHDOG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod watchdog;
#[doc = "RSM (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access RSM, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`rsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rsm`]
module"]
pub type RSM = crate::Reg<rsm::RSM_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access RSM, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod rsm;
#[doc = "XIP_CTRL (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access XIP_CTRL, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`xip_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@xip_ctrl`]
module"]
pub type XIP_CTRL = crate::Reg<xip_ctrl::XIP_CTRL_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_CTRL, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod xip_ctrl;
#[doc = "XIP_QMI (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access XIP_QMI, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`xip_qmi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_qmi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@xip_qmi`]
module"]
pub type XIP_QMI = crate::Reg<xip_qmi::XIP_QMI_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_QMI, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod xip_qmi;
#[doc = "XIP_AUX (rw) register accessor: Control whether debugger, DMA, core 0 and core 1 can access XIP_AUX, and at what security/privilege levels they can do so. Defaults to Secure, Privileged access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`xip_aux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_aux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@xip_aux`]
module"]
pub type XIP_AUX = crate::Reg<xip_aux::XIP_AUX_SPEC>;
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_AUX, and at what security/privilege levels they can do so. Defaults to Secure, Privileged access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
pub mod xip_aux;
