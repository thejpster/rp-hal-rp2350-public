#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch: [CH; 16],
    intr: INTR,
    inte0: INTE0,
    intf0: INTF0,
    ints0: INTS0,
    intr1: INTR1,
    inte1: INTE1,
    intf1: INTF1,
    ints1: INTS1,
    intr2: INTR2,
    inte2: INTE2,
    intf2: INTF2,
    ints2: INTS2,
    intr3: INTR3,
    inte3: INTE3,
    intf3: INTF3,
    ints3: INTS3,
    timer0: TIMER0,
    timer1: TIMER1,
    timer2: TIMER2,
    timer3: TIMER3,
    multi_chan_trigger: MULTI_CHAN_TRIGGER,
    sniff_ctrl: SNIFF_CTRL,
    sniff_data: SNIFF_DATA,
    _reserved24: [u8; 0x04],
    fifo_levels: FIFO_LEVELS,
    chan_abort: CHAN_ABORT,
    n_channels: N_CHANNELS,
    _reserved27: [u8; 0x14],
    seccfg_ch0: SECCFG_CH0,
    seccfg_ch1: SECCFG_CH1,
    seccfg_ch2: SECCFG_CH2,
    seccfg_ch3: SECCFG_CH3,
    seccfg_ch4: SECCFG_CH4,
    seccfg_ch5: SECCFG_CH5,
    seccfg_ch6: SECCFG_CH6,
    seccfg_ch7: SECCFG_CH7,
    seccfg_ch8: SECCFG_CH8,
    seccfg_ch9: SECCFG_CH9,
    seccfg_ch10: SECCFG_CH10,
    seccfg_ch11: SECCFG_CH11,
    seccfg_ch12: SECCFG_CH12,
    seccfg_ch13: SECCFG_CH13,
    seccfg_ch14: SECCFG_CH14,
    seccfg_ch15: SECCFG_CH15,
    seccfg_irq0: SECCFG_IRQ0,
    seccfg_irq1: SECCFG_IRQ1,
    seccfg_irq2: SECCFG_IRQ2,
    seccfg_irq3: SECCFG_IRQ3,
    seccfg_misc: SECCFG_MISC,
    _reserved48: [u8; 0x2c],
    mpu_ctrl: MPU_CTRL,
    mpu_bar0: MPU_BAR0,
    mpu_lar0: MPU_LAR0,
    mpu_bar1: MPU_BAR1,
    mpu_lar1: MPU_LAR1,
    mpu_bar2: MPU_BAR2,
    mpu_lar2: MPU_LAR2,
    mpu_bar3: MPU_BAR3,
    mpu_lar3: MPU_LAR3,
    mpu_bar4: MPU_BAR4,
    mpu_lar4: MPU_LAR4,
    mpu_bar5: MPU_BAR5,
    mpu_lar5: MPU_LAR5,
    mpu_bar6: MPU_BAR6,
    mpu_lar6: MPU_LAR6,
    mpu_bar7: MPU_BAR7,
    mpu_lar7: MPU_LAR7,
    _reserved65: [u8; 0x02bc],
    ch0_dbg_ctdreq: CH0_DBG_CTDREQ,
    ch0_dbg_tcr: CH0_DBG_TCR,
    _reserved67: [u8; 0x38],
    ch1_dbg_ctdreq: CH1_DBG_CTDREQ,
    ch1_dbg_tcr: CH1_DBG_TCR,
    _reserved69: [u8; 0x38],
    ch2_dbg_ctdreq: CH2_DBG_CTDREQ,
    ch2_dbg_tcr: CH2_DBG_TCR,
    _reserved71: [u8; 0x38],
    ch3_dbg_ctdreq: CH3_DBG_CTDREQ,
    ch3_dbg_tcr: CH3_DBG_TCR,
    _reserved73: [u8; 0x38],
    ch4_dbg_ctdreq: CH4_DBG_CTDREQ,
    ch4_dbg_tcr: CH4_DBG_TCR,
    _reserved75: [u8; 0x38],
    ch5_dbg_ctdreq: CH5_DBG_CTDREQ,
    ch5_dbg_tcr: CH5_DBG_TCR,
    _reserved77: [u8; 0x38],
    ch6_dbg_ctdreq: CH6_DBG_CTDREQ,
    ch6_dbg_tcr: CH6_DBG_TCR,
    _reserved79: [u8; 0x38],
    ch7_dbg_ctdreq: CH7_DBG_CTDREQ,
    ch7_dbg_tcr: CH7_DBG_TCR,
    _reserved81: [u8; 0x38],
    ch8_dbg_ctdreq: CH8_DBG_CTDREQ,
    ch8_dbg_tcr: CH8_DBG_TCR,
    _reserved83: [u8; 0x38],
    ch9_dbg_ctdreq: CH9_DBG_CTDREQ,
    ch9_dbg_tcr: CH9_DBG_TCR,
    _reserved85: [u8; 0x38],
    ch10_dbg_ctdreq: CH10_DBG_CTDREQ,
    ch10_dbg_tcr: CH10_DBG_TCR,
    _reserved87: [u8; 0x38],
    ch11_dbg_ctdreq: CH11_DBG_CTDREQ,
    ch11_dbg_tcr: CH11_DBG_TCR,
    _reserved89: [u8; 0x38],
    ch12_dbg_ctdreq: CH12_DBG_CTDREQ,
    ch12_dbg_tcr: CH12_DBG_TCR,
    _reserved91: [u8; 0x38],
    ch13_dbg_ctdreq: CH13_DBG_CTDREQ,
    ch13_dbg_tcr: CH13_DBG_TCR,
    _reserved93: [u8; 0x38],
    ch14_dbg_ctdreq: CH14_DBG_CTDREQ,
    ch14_dbg_tcr: CH14_DBG_TCR,
    _reserved95: [u8; 0x38],
    ch15_dbg_ctdreq: CH15_DBG_CTDREQ,
    ch15_dbg_tcr: CH15_DBG_TCR,
}
impl RegisterBlock {
    #[doc = "0x00..0x400 - Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x400 - Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x400 - Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x404 - Interrupt Enables for IRQ 0"]
    #[inline(always)]
    pub const fn inte0(&self) -> &INTE0 {
        &self.inte0
    }
    #[doc = "0x408 - Force Interrupts"]
    #[inline(always)]
    pub const fn intf0(&self) -> &INTF0 {
        &self.intf0
    }
    #[doc = "0x40c - Interrupt Status for IRQ 0"]
    #[inline(always)]
    pub const fn ints0(&self) -> &INTS0 {
        &self.ints0
    }
    #[doc = "0x410 - Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr1(&self) -> &INTR1 {
        &self.intr1
    }
    #[doc = "0x414 - Interrupt Enables for IRQ 1"]
    #[inline(always)]
    pub const fn inte1(&self) -> &INTE1 {
        &self.inte1
    }
    #[doc = "0x418 - Force Interrupts"]
    #[inline(always)]
    pub const fn intf1(&self) -> &INTF1 {
        &self.intf1
    }
    #[doc = "0x41c - Interrupt Status for IRQ 1"]
    #[inline(always)]
    pub const fn ints1(&self) -> &INTS1 {
        &self.ints1
    }
    #[doc = "0x420 - Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr2(&self) -> &INTR2 {
        &self.intr2
    }
    #[doc = "0x424 - Interrupt Enables for IRQ 2"]
    #[inline(always)]
    pub const fn inte2(&self) -> &INTE2 {
        &self.inte2
    }
    #[doc = "0x428 - Force Interrupts"]
    #[inline(always)]
    pub const fn intf2(&self) -> &INTF2 {
        &self.intf2
    }
    #[doc = "0x42c - Interrupt Status for IRQ 2"]
    #[inline(always)]
    pub const fn ints2(&self) -> &INTS2 {
        &self.ints2
    }
    #[doc = "0x430 - Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr3(&self) -> &INTR3 {
        &self.intr3
    }
    #[doc = "0x434 - Interrupt Enables for IRQ 3"]
    #[inline(always)]
    pub const fn inte3(&self) -> &INTE3 {
        &self.inte3
    }
    #[doc = "0x438 - Force Interrupts"]
    #[inline(always)]
    pub const fn intf3(&self) -> &INTF3 {
        &self.intf3
    }
    #[doc = "0x43c - Interrupt Status for IRQ 3"]
    #[inline(always)]
    pub const fn ints3(&self) -> &INTS3 {
        &self.ints3
    }
    #[doc = "0x440 - Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    #[inline(always)]
    pub const fn timer0(&self) -> &TIMER0 {
        &self.timer0
    }
    #[doc = "0x444 - Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    #[inline(always)]
    pub const fn timer1(&self) -> &TIMER1 {
        &self.timer1
    }
    #[doc = "0x448 - Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    #[inline(always)]
    pub const fn timer2(&self) -> &TIMER2 {
        &self.timer2
    }
    #[doc = "0x44c - Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    #[inline(always)]
    pub const fn timer3(&self) -> &TIMER3 {
        &self.timer3
    }
    #[doc = "0x450 - Trigger one or more channels simultaneously"]
    #[inline(always)]
    pub const fn multi_chan_trigger(&self) -> &MULTI_CHAN_TRIGGER {
        &self.multi_chan_trigger
    }
    #[doc = "0x454 - Sniffer Control"]
    #[inline(always)]
    pub const fn sniff_ctrl(&self) -> &SNIFF_CTRL {
        &self.sniff_ctrl
    }
    #[doc = "0x458 - Data accumulator for sniff hardware"]
    #[inline(always)]
    pub const fn sniff_data(&self) -> &SNIFF_DATA {
        &self.sniff_data
    }
    #[doc = "0x460 - Debug RAF, WAF, TDF levels"]
    #[inline(always)]
    pub const fn fifo_levels(&self) -> &FIFO_LEVELS {
        &self.fifo_levels
    }
    #[doc = "0x464 - Abort an in-progress transfer sequence on one or more channels"]
    #[inline(always)]
    pub const fn chan_abort(&self) -> &CHAN_ABORT {
        &self.chan_abort
    }
    #[doc = "0x468 - The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    #[inline(always)]
    pub const fn n_channels(&self) -> &N_CHANNELS {
        &self.n_channels
    }
    #[doc = "0x480 - Security configuration for channel 0. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch0(&self) -> &SECCFG_CH0 {
        &self.seccfg_ch0
    }
    #[doc = "0x484 - Security configuration for channel 1. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch1(&self) -> &SECCFG_CH1 {
        &self.seccfg_ch1
    }
    #[doc = "0x488 - Security configuration for channel 2. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch2(&self) -> &SECCFG_CH2 {
        &self.seccfg_ch2
    }
    #[doc = "0x48c - Security configuration for channel 3. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch3(&self) -> &SECCFG_CH3 {
        &self.seccfg_ch3
    }
    #[doc = "0x490 - Security configuration for channel 4. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch4(&self) -> &SECCFG_CH4 {
        &self.seccfg_ch4
    }
    #[doc = "0x494 - Security configuration for channel 5. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch5(&self) -> &SECCFG_CH5 {
        &self.seccfg_ch5
    }
    #[doc = "0x498 - Security configuration for channel 6. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch6(&self) -> &SECCFG_CH6 {
        &self.seccfg_ch6
    }
    #[doc = "0x49c - Security configuration for channel 7. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch7(&self) -> &SECCFG_CH7 {
        &self.seccfg_ch7
    }
    #[doc = "0x4a0 - Security configuration for channel 8. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch8(&self) -> &SECCFG_CH8 {
        &self.seccfg_ch8
    }
    #[doc = "0x4a4 - Security configuration for channel 9. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch9(&self) -> &SECCFG_CH9 {
        &self.seccfg_ch9
    }
    #[doc = "0x4a8 - Security configuration for channel 10. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch10(&self) -> &SECCFG_CH10 {
        &self.seccfg_ch10
    }
    #[doc = "0x4ac - Security configuration for channel 11. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch11(&self) -> &SECCFG_CH11 {
        &self.seccfg_ch11
    }
    #[doc = "0x4b0 - Security configuration for channel 12. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch12(&self) -> &SECCFG_CH12 {
        &self.seccfg_ch12
    }
    #[doc = "0x4b4 - Security configuration for channel 13. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch13(&self) -> &SECCFG_CH13 {
        &self.seccfg_ch13
    }
    #[doc = "0x4b8 - Security configuration for channel 14. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch14(&self) -> &SECCFG_CH14 {
        &self.seccfg_ch14
    }
    #[doc = "0x4bc - Security configuration for channel 15. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch15(&self) -> &SECCFG_CH15 {
        &self.seccfg_ch15
    }
    #[doc = "0x4c0 - Security configuration for IRQ 0. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
    #[inline(always)]
    pub const fn seccfg_irq0(&self) -> &SECCFG_IRQ0 {
        &self.seccfg_irq0
    }
    #[doc = "0x4c4 - Security configuration for IRQ 1. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
    #[inline(always)]
    pub const fn seccfg_irq1(&self) -> &SECCFG_IRQ1 {
        &self.seccfg_irq1
    }
    #[doc = "0x4c8 - Security configuration for IRQ 2. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
    #[inline(always)]
    pub const fn seccfg_irq2(&self) -> &SECCFG_IRQ2 {
        &self.seccfg_irq2
    }
    #[doc = "0x4cc - Security configuration for IRQ 3. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
    #[inline(always)]
    pub const fn seccfg_irq3(&self) -> &SECCFG_IRQ3 {
        &self.seccfg_irq3
    }
    #[doc = "0x4d0 - Miscellaneous security configuration"]
    #[inline(always)]
    pub const fn seccfg_misc(&self) -> &SECCFG_MISC {
        &self.seccfg_misc
    }
    #[doc = "0x500 - Control register for DMA MPU. Accessible only from a Privileged context."]
    #[inline(always)]
    pub const fn mpu_ctrl(&self) -> &MPU_CTRL {
        &self.mpu_ctrl
    }
    #[doc = "0x504 - Base address register for MPU region 0. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar0(&self) -> &MPU_BAR0 {
        &self.mpu_bar0
    }
    #[doc = "0x508 - Limit address register for MPU region 0. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar0(&self) -> &MPU_LAR0 {
        &self.mpu_lar0
    }
    #[doc = "0x50c - Base address register for MPU region 1. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar1(&self) -> &MPU_BAR1 {
        &self.mpu_bar1
    }
    #[doc = "0x510 - Limit address register for MPU region 1. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar1(&self) -> &MPU_LAR1 {
        &self.mpu_lar1
    }
    #[doc = "0x514 - Base address register for MPU region 2. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar2(&self) -> &MPU_BAR2 {
        &self.mpu_bar2
    }
    #[doc = "0x518 - Limit address register for MPU region 2. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar2(&self) -> &MPU_LAR2 {
        &self.mpu_lar2
    }
    #[doc = "0x51c - Base address register for MPU region 3. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar3(&self) -> &MPU_BAR3 {
        &self.mpu_bar3
    }
    #[doc = "0x520 - Limit address register for MPU region 3. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar3(&self) -> &MPU_LAR3 {
        &self.mpu_lar3
    }
    #[doc = "0x524 - Base address register for MPU region 4. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar4(&self) -> &MPU_BAR4 {
        &self.mpu_bar4
    }
    #[doc = "0x528 - Limit address register for MPU region 4. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar4(&self) -> &MPU_LAR4 {
        &self.mpu_lar4
    }
    #[doc = "0x52c - Base address register for MPU region 5. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar5(&self) -> &MPU_BAR5 {
        &self.mpu_bar5
    }
    #[doc = "0x530 - Limit address register for MPU region 5. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar5(&self) -> &MPU_LAR5 {
        &self.mpu_lar5
    }
    #[doc = "0x534 - Base address register for MPU region 6. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar6(&self) -> &MPU_BAR6 {
        &self.mpu_bar6
    }
    #[doc = "0x538 - Limit address register for MPU region 6. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar6(&self) -> &MPU_LAR6 {
        &self.mpu_lar6
    }
    #[doc = "0x53c - Base address register for MPU region 7. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar7(&self) -> &MPU_BAR7 {
        &self.mpu_bar7
    }
    #[doc = "0x540 - Limit address register for MPU region 7. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar7(&self) -> &MPU_LAR7 {
        &self.mpu_lar7
    }
    #[doc = "0x800 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch0_dbg_ctdreq(&self) -> &CH0_DBG_CTDREQ {
        &self.ch0_dbg_ctdreq
    }
    #[doc = "0x804 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch0_dbg_tcr(&self) -> &CH0_DBG_TCR {
        &self.ch0_dbg_tcr
    }
    #[doc = "0x840 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch1_dbg_ctdreq(&self) -> &CH1_DBG_CTDREQ {
        &self.ch1_dbg_ctdreq
    }
    #[doc = "0x844 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch1_dbg_tcr(&self) -> &CH1_DBG_TCR {
        &self.ch1_dbg_tcr
    }
    #[doc = "0x880 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch2_dbg_ctdreq(&self) -> &CH2_DBG_CTDREQ {
        &self.ch2_dbg_ctdreq
    }
    #[doc = "0x884 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch2_dbg_tcr(&self) -> &CH2_DBG_TCR {
        &self.ch2_dbg_tcr
    }
    #[doc = "0x8c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch3_dbg_ctdreq(&self) -> &CH3_DBG_CTDREQ {
        &self.ch3_dbg_ctdreq
    }
    #[doc = "0x8c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch3_dbg_tcr(&self) -> &CH3_DBG_TCR {
        &self.ch3_dbg_tcr
    }
    #[doc = "0x900 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch4_dbg_ctdreq(&self) -> &CH4_DBG_CTDREQ {
        &self.ch4_dbg_ctdreq
    }
    #[doc = "0x904 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch4_dbg_tcr(&self) -> &CH4_DBG_TCR {
        &self.ch4_dbg_tcr
    }
    #[doc = "0x940 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch5_dbg_ctdreq(&self) -> &CH5_DBG_CTDREQ {
        &self.ch5_dbg_ctdreq
    }
    #[doc = "0x944 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch5_dbg_tcr(&self) -> &CH5_DBG_TCR {
        &self.ch5_dbg_tcr
    }
    #[doc = "0x980 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch6_dbg_ctdreq(&self) -> &CH6_DBG_CTDREQ {
        &self.ch6_dbg_ctdreq
    }
    #[doc = "0x984 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch6_dbg_tcr(&self) -> &CH6_DBG_TCR {
        &self.ch6_dbg_tcr
    }
    #[doc = "0x9c0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch7_dbg_ctdreq(&self) -> &CH7_DBG_CTDREQ {
        &self.ch7_dbg_ctdreq
    }
    #[doc = "0x9c4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch7_dbg_tcr(&self) -> &CH7_DBG_TCR {
        &self.ch7_dbg_tcr
    }
    #[doc = "0xa00 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch8_dbg_ctdreq(&self) -> &CH8_DBG_CTDREQ {
        &self.ch8_dbg_ctdreq
    }
    #[doc = "0xa04 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch8_dbg_tcr(&self) -> &CH8_DBG_TCR {
        &self.ch8_dbg_tcr
    }
    #[doc = "0xa40 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch9_dbg_ctdreq(&self) -> &CH9_DBG_CTDREQ {
        &self.ch9_dbg_ctdreq
    }
    #[doc = "0xa44 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch9_dbg_tcr(&self) -> &CH9_DBG_TCR {
        &self.ch9_dbg_tcr
    }
    #[doc = "0xa80 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch10_dbg_ctdreq(&self) -> &CH10_DBG_CTDREQ {
        &self.ch10_dbg_ctdreq
    }
    #[doc = "0xa84 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch10_dbg_tcr(&self) -> &CH10_DBG_TCR {
        &self.ch10_dbg_tcr
    }
    #[doc = "0xac0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch11_dbg_ctdreq(&self) -> &CH11_DBG_CTDREQ {
        &self.ch11_dbg_ctdreq
    }
    #[doc = "0xac4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch11_dbg_tcr(&self) -> &CH11_DBG_TCR {
        &self.ch11_dbg_tcr
    }
    #[doc = "0xb00 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch12_dbg_ctdreq(&self) -> &CH12_DBG_CTDREQ {
        &self.ch12_dbg_ctdreq
    }
    #[doc = "0xb04 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch12_dbg_tcr(&self) -> &CH12_DBG_TCR {
        &self.ch12_dbg_tcr
    }
    #[doc = "0xb40 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch13_dbg_ctdreq(&self) -> &CH13_DBG_CTDREQ {
        &self.ch13_dbg_ctdreq
    }
    #[doc = "0xb44 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch13_dbg_tcr(&self) -> &CH13_DBG_TCR {
        &self.ch13_dbg_tcr
    }
    #[doc = "0xb80 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch14_dbg_ctdreq(&self) -> &CH14_DBG_CTDREQ {
        &self.ch14_dbg_ctdreq
    }
    #[doc = "0xb84 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch14_dbg_tcr(&self) -> &CH14_DBG_TCR {
        &self.ch14_dbg_tcr
    }
    #[doc = "0xbc0 - Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn ch15_dbg_ctdreq(&self) -> &CH15_DBG_CTDREQ {
        &self.ch15_dbg_ctdreq
    }
    #[doc = "0xbc4 - Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn ch15_dbg_tcr(&self) -> &CH15_DBG_TCR {
        &self.ch15_dbg_tcr
    }
}
#[doc = "Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing CH?_READ_ADDR,CH??_READ_ADDR, CH?_WRITE_ADDR,CH??_WRITE_ADDR, CH?_TRANS_COUNT,CH??_TRANS_COUNT, CH?_CTRL_TRIG,CH??_CTRL_TRIG, CH?_AL1_CTRL,CH??_AL1_CTRL, CH?_AL1_READ_ADDR,CH??_AL1_READ_ADDR, CH?_AL1_WRITE_ADDR,CH??_AL1_WRITE_ADDR, CH?_AL1_TRANS_COUNT_TRIG,CH??_AL1_TRANS_COUNT_TRIG, CH?_AL2_CTRL,CH??_AL2_CTRL, CH?_AL2_TRANS_COUNT,CH??_AL2_TRANS_COUNT, CH?_AL2_READ_ADDR,CH??_AL2_READ_ADDR, CH?_AL2_WRITE_ADDR_TRIG,CH??_AL2_WRITE_ADDR_TRIG, CH?_AL3_CTRL,CH??_AL3_CTRL, CH?_AL3_WRITE_ADDR,CH??_AL3_WRITE_ADDR, CH?_AL3_TRANS_COUNT,CH??_AL3_TRANS_COUNT, CH?_AL3_READ_ADDR_TRIG,CH??_AL3_READ_ADDR_TRIG"]
pub mod ch;
#[doc = "INTR (rw) register accessor: Interrupt Status (raw)  

You can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt Status (raw)"]
pub mod intr;
#[doc = "INTE0 (rw) register accessor: Interrupt Enables for IRQ 0  

You can [`read`](crate::Reg::read) this register and get [`inte0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte0`]
module"]
pub type INTE0 = crate::Reg<inte0::INTE0_SPEC>;
#[doc = "Interrupt Enables for IRQ 0"]
pub mod inte0;
#[doc = "INTF0 (rw) register accessor: Force Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf0`]
module"]
pub type INTF0 = crate::Reg<intf0::INTF0_SPEC>;
#[doc = "Force Interrupts"]
pub mod intf0;
#[doc = "INTS0 (rw) register accessor: Interrupt Status for IRQ 0  

You can [`read`](crate::Reg::read) this register and get [`ints0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ints0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints0`]
module"]
pub type INTS0 = crate::Reg<ints0::INTS0_SPEC>;
#[doc = "Interrupt Status for IRQ 0"]
pub mod ints0;
#[doc = "INTR1 (rw) register accessor: Interrupt Status (raw)  

You can [`read`](crate::Reg::read) this register and get [`intr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr1`]
module"]
pub type INTR1 = crate::Reg<intr1::INTR1_SPEC>;
#[doc = "Interrupt Status (raw)"]
pub mod intr1;
#[doc = "INTE1 (rw) register accessor: Interrupt Enables for IRQ 1  

You can [`read`](crate::Reg::read) this register and get [`inte1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte1`]
module"]
pub type INTE1 = crate::Reg<inte1::INTE1_SPEC>;
#[doc = "Interrupt Enables for IRQ 1"]
pub mod inte1;
#[doc = "INTF1 (rw) register accessor: Force Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf1`]
module"]
pub type INTF1 = crate::Reg<intf1::INTF1_SPEC>;
#[doc = "Force Interrupts"]
pub mod intf1;
#[doc = "INTS1 (rw) register accessor: Interrupt Status for IRQ 1  

You can [`read`](crate::Reg::read) this register and get [`ints1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ints1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints1`]
module"]
pub type INTS1 = crate::Reg<ints1::INTS1_SPEC>;
#[doc = "Interrupt Status for IRQ 1"]
pub mod ints1;
#[doc = "INTR2 (rw) register accessor: Interrupt Status (raw)  

You can [`read`](crate::Reg::read) this register and get [`intr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr2`]
module"]
pub type INTR2 = crate::Reg<intr2::INTR2_SPEC>;
#[doc = "Interrupt Status (raw)"]
pub mod intr2;
#[doc = "INTE2 (rw) register accessor: Interrupt Enables for IRQ 2  

You can [`read`](crate::Reg::read) this register and get [`inte2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte2`]
module"]
pub type INTE2 = crate::Reg<inte2::INTE2_SPEC>;
#[doc = "Interrupt Enables for IRQ 2"]
pub mod inte2;
#[doc = "INTF2 (rw) register accessor: Force Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf2`]
module"]
pub type INTF2 = crate::Reg<intf2::INTF2_SPEC>;
#[doc = "Force Interrupts"]
pub mod intf2;
#[doc = "INTS2 (rw) register accessor: Interrupt Status for IRQ 2  

You can [`read`](crate::Reg::read) this register and get [`ints2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ints2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints2`]
module"]
pub type INTS2 = crate::Reg<ints2::INTS2_SPEC>;
#[doc = "Interrupt Status for IRQ 2"]
pub mod ints2;
#[doc = "INTR3 (rw) register accessor: Interrupt Status (raw)  

You can [`read`](crate::Reg::read) this register and get [`intr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr3`]
module"]
pub type INTR3 = crate::Reg<intr3::INTR3_SPEC>;
#[doc = "Interrupt Status (raw)"]
pub mod intr3;
#[doc = "INTE3 (rw) register accessor: Interrupt Enables for IRQ 3  

You can [`read`](crate::Reg::read) this register and get [`inte3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte3`]
module"]
pub type INTE3 = crate::Reg<inte3::INTE3_SPEC>;
#[doc = "Interrupt Enables for IRQ 3"]
pub mod inte3;
#[doc = "INTF3 (rw) register accessor: Force Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf3`]
module"]
pub type INTF3 = crate::Reg<intf3::INTF3_SPEC>;
#[doc = "Force Interrupts"]
pub mod intf3;
#[doc = "INTS3 (rw) register accessor: Interrupt Status for IRQ 3  

You can [`read`](crate::Reg::read) this register and get [`ints3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ints3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints3`]
module"]
pub type INTS3 = crate::Reg<ints3::INTS3_SPEC>;
#[doc = "Interrupt Status for IRQ 3"]
pub mod ints3;
#[doc = "TIMER0 (rw) register accessor: Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.  

You can [`read`](crate::Reg::read) this register and get [`timer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timer0`]
module"]
pub type TIMER0 = crate::Reg<timer0::TIMER0_SPEC>;
#[doc = "Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer0;
#[doc = "TIMER1 (rw) register accessor: Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.  

You can [`read`](crate::Reg::read) this register and get [`timer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timer1`]
module"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.  

You can [`read`](crate::Reg::read) this register and get [`timer2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timer2`]
module"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = "Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer2;
#[doc = "TIMER3 (rw) register accessor: Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.  

You can [`read`](crate::Reg::read) this register and get [`timer3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@timer3`]
module"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = "Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
pub mod timer3;
#[doc = "MULTI_CHAN_TRIGGER (rw) register accessor: Trigger one or more channels simultaneously  

You can [`read`](crate::Reg::read) this register and get [`multi_chan_trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`multi_chan_trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@multi_chan_trigger`]
module"]
pub type MULTI_CHAN_TRIGGER = crate::Reg<multi_chan_trigger::MULTI_CHAN_TRIGGER_SPEC>;
#[doc = "Trigger one or more channels simultaneously"]
pub mod multi_chan_trigger;
#[doc = "SNIFF_CTRL (rw) register accessor: Sniffer Control  

You can [`read`](crate::Reg::read) this register and get [`sniff_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sniff_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sniff_ctrl`]
module"]
pub type SNIFF_CTRL = crate::Reg<sniff_ctrl::SNIFF_CTRL_SPEC>;
#[doc = "Sniffer Control"]
pub mod sniff_ctrl;
#[doc = "SNIFF_DATA (rw) register accessor: Data accumulator for sniff hardware  

You can [`read`](crate::Reg::read) this register and get [`sniff_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sniff_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sniff_data`]
module"]
pub type SNIFF_DATA = crate::Reg<sniff_data::SNIFF_DATA_SPEC>;
#[doc = "Data accumulator for sniff hardware"]
pub mod sniff_data;
#[doc = "FIFO_LEVELS (rw) register accessor: Debug RAF, WAF, TDF levels  

You can [`read`](crate::Reg::read) this register and get [`fifo_levels::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_levels::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fifo_levels`]
module"]
pub type FIFO_LEVELS = crate::Reg<fifo_levels::FIFO_LEVELS_SPEC>;
#[doc = "Debug RAF, WAF, TDF levels"]
pub mod fifo_levels;
#[doc = "CHAN_ABORT (rw) register accessor: Abort an in-progress transfer sequence on one or more channels  

You can [`read`](crate::Reg::read) this register and get [`chan_abort::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chan_abort::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chan_abort`]
module"]
pub type CHAN_ABORT = crate::Reg<chan_abort::CHAN_ABORT_SPEC>;
#[doc = "Abort an in-progress transfer sequence on one or more channels"]
pub mod chan_abort;
#[doc = "N_CHANNELS (rw) register accessor: The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area.  

You can [`read`](crate::Reg::read) this register and get [`n_channels::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`n_channels::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@n_channels`]
module"]
pub type N_CHANNELS = crate::Reg<n_channels::N_CHANNELS_SPEC>;
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
pub mod n_channels;
#[doc = "SECCFG_CH0 (rw) register accessor: Security configuration for channel 0. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch0`]
module"]
pub type SECCFG_CH0 = crate::Reg<seccfg_ch0::SECCFG_CH0_SPEC>;
#[doc = "Security configuration for channel 0. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch0;
#[doc = "SECCFG_CH1 (rw) register accessor: Security configuration for channel 1. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch1`]
module"]
pub type SECCFG_CH1 = crate::Reg<seccfg_ch1::SECCFG_CH1_SPEC>;
#[doc = "Security configuration for channel 1. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch1;
#[doc = "SECCFG_CH2 (rw) register accessor: Security configuration for channel 2. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch2`]
module"]
pub type SECCFG_CH2 = crate::Reg<seccfg_ch2::SECCFG_CH2_SPEC>;
#[doc = "Security configuration for channel 2. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch2;
#[doc = "SECCFG_CH3 (rw) register accessor: Security configuration for channel 3. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch3`]
module"]
pub type SECCFG_CH3 = crate::Reg<seccfg_ch3::SECCFG_CH3_SPEC>;
#[doc = "Security configuration for channel 3. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch3;
#[doc = "SECCFG_CH4 (rw) register accessor: Security configuration for channel 4. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch4`]
module"]
pub type SECCFG_CH4 = crate::Reg<seccfg_ch4::SECCFG_CH4_SPEC>;
#[doc = "Security configuration for channel 4. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch4;
#[doc = "SECCFG_CH5 (rw) register accessor: Security configuration for channel 5. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch5`]
module"]
pub type SECCFG_CH5 = crate::Reg<seccfg_ch5::SECCFG_CH5_SPEC>;
#[doc = "Security configuration for channel 5. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch5;
#[doc = "SECCFG_CH6 (rw) register accessor: Security configuration for channel 6. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch6`]
module"]
pub type SECCFG_CH6 = crate::Reg<seccfg_ch6::SECCFG_CH6_SPEC>;
#[doc = "Security configuration for channel 6. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch6;
#[doc = "SECCFG_CH7 (rw) register accessor: Security configuration for channel 7. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch7`]
module"]
pub type SECCFG_CH7 = crate::Reg<seccfg_ch7::SECCFG_CH7_SPEC>;
#[doc = "Security configuration for channel 7. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch7;
#[doc = "SECCFG_CH8 (rw) register accessor: Security configuration for channel 8. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch8`]
module"]
pub type SECCFG_CH8 = crate::Reg<seccfg_ch8::SECCFG_CH8_SPEC>;
#[doc = "Security configuration for channel 8. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch8;
#[doc = "SECCFG_CH9 (rw) register accessor: Security configuration for channel 9. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch9`]
module"]
pub type SECCFG_CH9 = crate::Reg<seccfg_ch9::SECCFG_CH9_SPEC>;
#[doc = "Security configuration for channel 9. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch9;
#[doc = "SECCFG_CH10 (rw) register accessor: Security configuration for channel 10. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch10`]
module"]
pub type SECCFG_CH10 = crate::Reg<seccfg_ch10::SECCFG_CH10_SPEC>;
#[doc = "Security configuration for channel 10. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch10;
#[doc = "SECCFG_CH11 (rw) register accessor: Security configuration for channel 11. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch11`]
module"]
pub type SECCFG_CH11 = crate::Reg<seccfg_ch11::SECCFG_CH11_SPEC>;
#[doc = "Security configuration for channel 11. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch11;
#[doc = "SECCFG_CH12 (rw) register accessor: Security configuration for channel 12. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch12`]
module"]
pub type SECCFG_CH12 = crate::Reg<seccfg_ch12::SECCFG_CH12_SPEC>;
#[doc = "Security configuration for channel 12. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch12;
#[doc = "SECCFG_CH13 (rw) register accessor: Security configuration for channel 13. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch13`]
module"]
pub type SECCFG_CH13 = crate::Reg<seccfg_ch13::SECCFG_CH13_SPEC>;
#[doc = "Security configuration for channel 13. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch13;
#[doc = "SECCFG_CH14 (rw) register accessor: Security configuration for channel 14. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch14`]
module"]
pub type SECCFG_CH14 = crate::Reg<seccfg_ch14::SECCFG_CH14_SPEC>;
#[doc = "Security configuration for channel 14. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch14;
#[doc = "SECCFG_CH15 (rw) register accessor: Security configuration for channel 15. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch15`]
module"]
pub type SECCFG_CH15 = crate::Reg<seccfg_ch15::SECCFG_CH15_SPEC>;
#[doc = "Security configuration for channel 15. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch15;
#[doc = "SECCFG_IRQ0 (rw) register accessor: Security configuration for IRQ 0. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_irq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_irq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_irq0`]
module"]
pub type SECCFG_IRQ0 = crate::Reg<seccfg_irq0::SECCFG_IRQ0_SPEC>;
#[doc = "Security configuration for IRQ 0. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
pub mod seccfg_irq0;
#[doc = "SECCFG_IRQ1 (rw) register accessor: Security configuration for IRQ 1. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_irq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_irq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_irq1`]
module"]
pub type SECCFG_IRQ1 = crate::Reg<seccfg_irq1::SECCFG_IRQ1_SPEC>;
#[doc = "Security configuration for IRQ 1. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
pub mod seccfg_irq1;
#[doc = "SECCFG_IRQ2 (rw) register accessor: Security configuration for IRQ 2. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_irq2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_irq2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_irq2`]
module"]
pub type SECCFG_IRQ2 = crate::Reg<seccfg_irq2::SECCFG_IRQ2_SPEC>;
#[doc = "Security configuration for IRQ 2. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
pub mod seccfg_irq2;
#[doc = "SECCFG_IRQ3 (rw) register accessor: Security configuration for IRQ 3. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_irq3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_irq3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_irq3`]
module"]
pub type SECCFG_IRQ3 = crate::Reg<seccfg_irq3::SECCFG_IRQ3_SPEC>;
#[doc = "Security configuration for IRQ 3. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
pub mod seccfg_irq3;
#[doc = "SECCFG_MISC (rw) register accessor: Miscellaneous security configuration  

You can [`read`](crate::Reg::read) this register and get [`seccfg_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_misc`]
module"]
pub type SECCFG_MISC = crate::Reg<seccfg_misc::SECCFG_MISC_SPEC>;
#[doc = "Miscellaneous security configuration"]
pub mod seccfg_misc;
#[doc = "MPU_CTRL (rw) register accessor: Control register for DMA MPU. Accessible only from a Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`mpu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_ctrl`]
module"]
pub type MPU_CTRL = crate::Reg<mpu_ctrl::MPU_CTRL_SPEC>;
#[doc = "Control register for DMA MPU. Accessible only from a Privileged context."]
pub mod mpu_ctrl;
#[doc = "MPU_BAR0 (rw) register accessor: Base address register for MPU region 0. Writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`mpu_bar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_bar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_bar0`]
module"]
pub type MPU_BAR0 = crate::Reg<mpu_bar0::MPU_BAR0_SPEC>;
#[doc = "Base address register for MPU region 0. Writable only from a Secure, Privileged context."]
pub mod mpu_bar0;
#[doc = "MPU_LAR0 (rw) register accessor: Limit address register for MPU region 0. Writable only from a Secure, Privileged context, with the exception of the P bit.  

You can [`read`](crate::Reg::read) this register and get [`mpu_lar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_lar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_lar0`]
module"]
pub type MPU_LAR0 = crate::Reg<mpu_lar0::MPU_LAR0_SPEC>;
#[doc = "Limit address register for MPU region 0. Writable only from a Secure, Privileged context, with the exception of the P bit."]
pub mod mpu_lar0;
#[doc = "MPU_BAR1 (rw) register accessor: Base address register for MPU region 1. Writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`mpu_bar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_bar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_bar1`]
module"]
pub type MPU_BAR1 = crate::Reg<mpu_bar1::MPU_BAR1_SPEC>;
#[doc = "Base address register for MPU region 1. Writable only from a Secure, Privileged context."]
pub mod mpu_bar1;
#[doc = "MPU_LAR1 (rw) register accessor: Limit address register for MPU region 1. Writable only from a Secure, Privileged context, with the exception of the P bit.  

You can [`read`](crate::Reg::read) this register and get [`mpu_lar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_lar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_lar1`]
module"]
pub type MPU_LAR1 = crate::Reg<mpu_lar1::MPU_LAR1_SPEC>;
#[doc = "Limit address register for MPU region 1. Writable only from a Secure, Privileged context, with the exception of the P bit."]
pub mod mpu_lar1;
#[doc = "MPU_BAR2 (rw) register accessor: Base address register for MPU region 2. Writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`mpu_bar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_bar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_bar2`]
module"]
pub type MPU_BAR2 = crate::Reg<mpu_bar2::MPU_BAR2_SPEC>;
#[doc = "Base address register for MPU region 2. Writable only from a Secure, Privileged context."]
pub mod mpu_bar2;
#[doc = "MPU_LAR2 (rw) register accessor: Limit address register for MPU region 2. Writable only from a Secure, Privileged context, with the exception of the P bit.  

You can [`read`](crate::Reg::read) this register and get [`mpu_lar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_lar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_lar2`]
module"]
pub type MPU_LAR2 = crate::Reg<mpu_lar2::MPU_LAR2_SPEC>;
#[doc = "Limit address register for MPU region 2. Writable only from a Secure, Privileged context, with the exception of the P bit."]
pub mod mpu_lar2;
#[doc = "MPU_BAR3 (rw) register accessor: Base address register for MPU region 3. Writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`mpu_bar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_bar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_bar3`]
module"]
pub type MPU_BAR3 = crate::Reg<mpu_bar3::MPU_BAR3_SPEC>;
#[doc = "Base address register for MPU region 3. Writable only from a Secure, Privileged context."]
pub mod mpu_bar3;
#[doc = "MPU_LAR3 (rw) register accessor: Limit address register for MPU region 3. Writable only from a Secure, Privileged context, with the exception of the P bit.  

You can [`read`](crate::Reg::read) this register and get [`mpu_lar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_lar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_lar3`]
module"]
pub type MPU_LAR3 = crate::Reg<mpu_lar3::MPU_LAR3_SPEC>;
#[doc = "Limit address register for MPU region 3. Writable only from a Secure, Privileged context, with the exception of the P bit."]
pub mod mpu_lar3;
#[doc = "MPU_BAR4 (rw) register accessor: Base address register for MPU region 4. Writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`mpu_bar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_bar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_bar4`]
module"]
pub type MPU_BAR4 = crate::Reg<mpu_bar4::MPU_BAR4_SPEC>;
#[doc = "Base address register for MPU region 4. Writable only from a Secure, Privileged context."]
pub mod mpu_bar4;
#[doc = "MPU_LAR4 (rw) register accessor: Limit address register for MPU region 4. Writable only from a Secure, Privileged context, with the exception of the P bit.  

You can [`read`](crate::Reg::read) this register and get [`mpu_lar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_lar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_lar4`]
module"]
pub type MPU_LAR4 = crate::Reg<mpu_lar4::MPU_LAR4_SPEC>;
#[doc = "Limit address register for MPU region 4. Writable only from a Secure, Privileged context, with the exception of the P bit."]
pub mod mpu_lar4;
#[doc = "MPU_BAR5 (rw) register accessor: Base address register for MPU region 5. Writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`mpu_bar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_bar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_bar5`]
module"]
pub type MPU_BAR5 = crate::Reg<mpu_bar5::MPU_BAR5_SPEC>;
#[doc = "Base address register for MPU region 5. Writable only from a Secure, Privileged context."]
pub mod mpu_bar5;
#[doc = "MPU_LAR5 (rw) register accessor: Limit address register for MPU region 5. Writable only from a Secure, Privileged context, with the exception of the P bit.  

You can [`read`](crate::Reg::read) this register and get [`mpu_lar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_lar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_lar5`]
module"]
pub type MPU_LAR5 = crate::Reg<mpu_lar5::MPU_LAR5_SPEC>;
#[doc = "Limit address register for MPU region 5. Writable only from a Secure, Privileged context, with the exception of the P bit."]
pub mod mpu_lar5;
#[doc = "MPU_BAR6 (rw) register accessor: Base address register for MPU region 6. Writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`mpu_bar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_bar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_bar6`]
module"]
pub type MPU_BAR6 = crate::Reg<mpu_bar6::MPU_BAR6_SPEC>;
#[doc = "Base address register for MPU region 6. Writable only from a Secure, Privileged context."]
pub mod mpu_bar6;
#[doc = "MPU_LAR6 (rw) register accessor: Limit address register for MPU region 6. Writable only from a Secure, Privileged context, with the exception of the P bit.  

You can [`read`](crate::Reg::read) this register and get [`mpu_lar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_lar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_lar6`]
module"]
pub type MPU_LAR6 = crate::Reg<mpu_lar6::MPU_LAR6_SPEC>;
#[doc = "Limit address register for MPU region 6. Writable only from a Secure, Privileged context, with the exception of the P bit."]
pub mod mpu_lar6;
#[doc = "MPU_BAR7 (rw) register accessor: Base address register for MPU region 7. Writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`mpu_bar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_bar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_bar7`]
module"]
pub type MPU_BAR7 = crate::Reg<mpu_bar7::MPU_BAR7_SPEC>;
#[doc = "Base address register for MPU region 7. Writable only from a Secure, Privileged context."]
pub mod mpu_bar7;
#[doc = "MPU_LAR7 (rw) register accessor: Limit address register for MPU region 7. Writable only from a Secure, Privileged context, with the exception of the P bit.  

You can [`read`](crate::Reg::read) this register and get [`mpu_lar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_lar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_lar7`]
module"]
pub type MPU_LAR7 = crate::Reg<mpu_lar7::MPU_LAR7_SPEC>;
#[doc = "Limit address register for MPU region 7. Writable only from a Secure, Privileged context, with the exception of the P bit."]
pub mod mpu_lar7;
#[doc = "CH0_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch0_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch0_dbg_ctdreq`]
module"]
pub type CH0_DBG_CTDREQ = crate::Reg<ch0_dbg_ctdreq::CH0_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch0_dbg_ctdreq;
#[doc = "CH0_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch0_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch0_dbg_tcr`]
module"]
pub type CH0_DBG_TCR = crate::Reg<ch0_dbg_tcr::CH0_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch0_dbg_tcr;
#[doc = "CH1_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch1_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch1_dbg_ctdreq`]
module"]
pub type CH1_DBG_CTDREQ = crate::Reg<ch1_dbg_ctdreq::CH1_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch1_dbg_ctdreq;
#[doc = "CH1_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch1_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch1_dbg_tcr`]
module"]
pub type CH1_DBG_TCR = crate::Reg<ch1_dbg_tcr::CH1_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch1_dbg_tcr;
#[doc = "CH2_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch2_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch2_dbg_ctdreq`]
module"]
pub type CH2_DBG_CTDREQ = crate::Reg<ch2_dbg_ctdreq::CH2_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch2_dbg_ctdreq;
#[doc = "CH2_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch2_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch2_dbg_tcr`]
module"]
pub type CH2_DBG_TCR = crate::Reg<ch2_dbg_tcr::CH2_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch2_dbg_tcr;
#[doc = "CH3_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch3_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch3_dbg_ctdreq`]
module"]
pub type CH3_DBG_CTDREQ = crate::Reg<ch3_dbg_ctdreq::CH3_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch3_dbg_ctdreq;
#[doc = "CH3_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch3_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch3_dbg_tcr`]
module"]
pub type CH3_DBG_TCR = crate::Reg<ch3_dbg_tcr::CH3_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch3_dbg_tcr;
#[doc = "CH4_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch4_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch4_dbg_ctdreq`]
module"]
pub type CH4_DBG_CTDREQ = crate::Reg<ch4_dbg_ctdreq::CH4_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch4_dbg_ctdreq;
#[doc = "CH4_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch4_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch4_dbg_tcr`]
module"]
pub type CH4_DBG_TCR = crate::Reg<ch4_dbg_tcr::CH4_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch4_dbg_tcr;
#[doc = "CH5_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch5_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch5_dbg_ctdreq`]
module"]
pub type CH5_DBG_CTDREQ = crate::Reg<ch5_dbg_ctdreq::CH5_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch5_dbg_ctdreq;
#[doc = "CH5_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch5_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch5_dbg_tcr`]
module"]
pub type CH5_DBG_TCR = crate::Reg<ch5_dbg_tcr::CH5_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch5_dbg_tcr;
#[doc = "CH6_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch6_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch6_dbg_ctdreq`]
module"]
pub type CH6_DBG_CTDREQ = crate::Reg<ch6_dbg_ctdreq::CH6_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch6_dbg_ctdreq;
#[doc = "CH6_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch6_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch6_dbg_tcr`]
module"]
pub type CH6_DBG_TCR = crate::Reg<ch6_dbg_tcr::CH6_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch6_dbg_tcr;
#[doc = "CH7_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch7_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch7_dbg_ctdreq`]
module"]
pub type CH7_DBG_CTDREQ = crate::Reg<ch7_dbg_ctdreq::CH7_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch7_dbg_ctdreq;
#[doc = "CH7_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch7_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch7_dbg_tcr`]
module"]
pub type CH7_DBG_TCR = crate::Reg<ch7_dbg_tcr::CH7_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch7_dbg_tcr;
#[doc = "CH8_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch8_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch8_dbg_ctdreq`]
module"]
pub type CH8_DBG_CTDREQ = crate::Reg<ch8_dbg_ctdreq::CH8_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch8_dbg_ctdreq;
#[doc = "CH8_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch8_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch8_dbg_tcr`]
module"]
pub type CH8_DBG_TCR = crate::Reg<ch8_dbg_tcr::CH8_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch8_dbg_tcr;
#[doc = "CH9_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch9_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch9_dbg_ctdreq`]
module"]
pub type CH9_DBG_CTDREQ = crate::Reg<ch9_dbg_ctdreq::CH9_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch9_dbg_ctdreq;
#[doc = "CH9_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch9_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch9_dbg_tcr`]
module"]
pub type CH9_DBG_TCR = crate::Reg<ch9_dbg_tcr::CH9_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch9_dbg_tcr;
#[doc = "CH10_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch10_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch10_dbg_ctdreq`]
module"]
pub type CH10_DBG_CTDREQ = crate::Reg<ch10_dbg_ctdreq::CH10_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch10_dbg_ctdreq;
#[doc = "CH10_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch10_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch10_dbg_tcr`]
module"]
pub type CH10_DBG_TCR = crate::Reg<ch10_dbg_tcr::CH10_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch10_dbg_tcr;
#[doc = "CH11_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch11_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch11_dbg_ctdreq`]
module"]
pub type CH11_DBG_CTDREQ = crate::Reg<ch11_dbg_ctdreq::CH11_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch11_dbg_ctdreq;
#[doc = "CH11_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch11_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch11_dbg_tcr`]
module"]
pub type CH11_DBG_TCR = crate::Reg<ch11_dbg_tcr::CH11_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch11_dbg_tcr;
#[doc = "CH12_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch12_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch12_dbg_ctdreq`]
module"]
pub type CH12_DBG_CTDREQ = crate::Reg<ch12_dbg_ctdreq::CH12_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch12_dbg_ctdreq;
#[doc = "CH12_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch12_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch12_dbg_tcr`]
module"]
pub type CH12_DBG_TCR = crate::Reg<ch12_dbg_tcr::CH12_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch12_dbg_tcr;
#[doc = "CH13_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch13_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch13_dbg_ctdreq`]
module"]
pub type CH13_DBG_CTDREQ = crate::Reg<ch13_dbg_ctdreq::CH13_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch13_dbg_ctdreq;
#[doc = "CH13_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch13_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch13_dbg_tcr`]
module"]
pub type CH13_DBG_TCR = crate::Reg<ch13_dbg_tcr::CH13_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch13_dbg_tcr;
#[doc = "CH14_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch14_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch14_dbg_ctdreq`]
module"]
pub type CH14_DBG_CTDREQ = crate::Reg<ch14_dbg_ctdreq::CH14_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch14_dbg_ctdreq;
#[doc = "CH14_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch14_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch14_dbg_tcr`]
module"]
pub type CH14_DBG_TCR = crate::Reg<ch14_dbg_tcr::CH14_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch14_dbg_tcr;
#[doc = "CH15_DBG_CTDREQ (rw) register accessor: Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::Reg::read) this register and get [`ch15_dbg_ctdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_dbg_ctdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch15_dbg_ctdreq`]
module"]
pub type CH15_DBG_CTDREQ = crate::Reg<ch15_dbg_ctdreq::CH15_DBG_CTDREQ_SPEC>;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
pub mod ch15_dbg_ctdreq;
#[doc = "CH15_DBG_TCR (rw) register accessor: Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch15_dbg_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_dbg_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ch15_dbg_tcr`]
module"]
pub type CH15_DBG_TCR = crate::Reg<ch15_dbg_tcr::CH15_DBG_TCR_SPEC>;
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
pub mod ch15_dbg_tcr;
