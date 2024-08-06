#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpuid: CPUID,
    gpio_in: GPIO_IN,
    gpio_hi_in: GPIO_HI_IN,
    _reserved3: [u8; 0x04],
    gpio_out: GPIO_OUT,
    gpio_hi_out: GPIO_HI_OUT,
    gpio_out_set: GPIO_OUT_SET,
    gpio_hi_out_set: GPIO_HI_OUT_SET,
    gpio_out_clr: GPIO_OUT_CLR,
    gpio_hi_out_clr: GPIO_HI_OUT_CLR,
    gpio_out_xor: GPIO_OUT_XOR,
    gpio_hi_out_xor: GPIO_HI_OUT_XOR,
    gpio_oe: GPIO_OE,
    gpio_hi_oe: GPIO_HI_OE,
    gpio_oe_set: GPIO_OE_SET,
    gpio_hi_oe_set: GPIO_HI_OE_SET,
    gpio_oe_clr: GPIO_OE_CLR,
    gpio_hi_oe_clr: GPIO_HI_OE_CLR,
    gpio_oe_xor: GPIO_OE_XOR,
    gpio_hi_oe_xor: GPIO_HI_OE_XOR,
    fifo_st: FIFO_ST,
    fifo_wr: FIFO_WR,
    fifo_rd: FIFO_RD,
    spinlock_st: SPINLOCK_ST,
    _reserved23: [u8; 0x20],
    interp0_accum0: INTERP0_ACCUM0,
    interp0_accum1: INTERP0_ACCUM1,
    interp0_base0: INTERP0_BASE0,
    interp0_base1: INTERP0_BASE1,
    interp0_base2: INTERP0_BASE2,
    interp0_pop_lane0: INTERP0_POP_LANE0,
    interp0_pop_lane1: INTERP0_POP_LANE1,
    interp0_pop_full: INTERP0_POP_FULL,
    interp0_peek_lane0: INTERP0_PEEK_LANE0,
    interp0_peek_lane1: INTERP0_PEEK_LANE1,
    interp0_peek_full: INTERP0_PEEK_FULL,
    interp0_ctrl_lane0: INTERP0_CTRL_LANE0,
    interp0_ctrl_lane1: INTERP0_CTRL_LANE1,
    interp0_accum0_add: INTERP0_ACCUM0_ADD,
    interp0_accum1_add: INTERP0_ACCUM1_ADD,
    interp0_base_1and0: INTERP0_BASE_1AND0,
    interp1_accum0: INTERP1_ACCUM0,
    interp1_accum1: INTERP1_ACCUM1,
    interp1_base0: INTERP1_BASE0,
    interp1_base1: INTERP1_BASE1,
    interp1_base2: INTERP1_BASE2,
    interp1_pop_lane0: INTERP1_POP_LANE0,
    interp1_pop_lane1: INTERP1_POP_LANE1,
    interp1_pop_full: INTERP1_POP_FULL,
    interp1_peek_lane0: INTERP1_PEEK_LANE0,
    interp1_peek_lane1: INTERP1_PEEK_LANE1,
    interp1_peek_full: INTERP1_PEEK_FULL,
    interp1_ctrl_lane0: INTERP1_CTRL_LANE0,
    interp1_ctrl_lane1: INTERP1_CTRL_LANE1,
    interp1_accum0_add: INTERP1_ACCUM0_ADD,
    interp1_accum1_add: INTERP1_ACCUM1_ADD,
    interp1_base_1and0: INTERP1_BASE_1AND0,
    spinlock: [SPINLOCK; 32],
    doorbell_out_set: DOORBELL_OUT_SET,
    doorbell_out_clr: DOORBELL_OUT_CLR,
    doorbell_in_set: DOORBELL_IN_SET,
    doorbell_in_clr: DOORBELL_IN_CLR,
    peri_nonsec: PERI_NONSEC,
    _reserved61: [u8; 0x0c],
    riscv_softirq: RISCV_SOFTIRQ,
    mtime_ctrl: MTIME_CTRL,
    _reserved63: [u8; 0x08],
    mtime: MTIME,
    mtimeh: MTIMEH,
    mtimecmp: MTIMECMP,
    mtimecmph: MTIMECMPH,
    tmds_ctrl: TMDS_CTRL,
    tmds_wdata: TMDS_WDATA,
    tmds_peek_single: TMDS_PEEK_SINGLE,
    tmds_pop_single: TMDS_POP_SINGLE,
    tmds_peek_double_l0: TMDS_PEEK_DOUBLE_L0,
    tmds_pop_double_l0: TMDS_POP_DOUBLE_L0,
    tmds_peek_double_l1: TMDS_PEEK_DOUBLE_L1,
    tmds_pop_double_l1: TMDS_POP_DOUBLE_L1,
    tmds_peek_double_l2: TMDS_PEEK_DOUBLE_L2,
    tmds_pop_double_l2: TMDS_POP_DOUBLE_L2,
}
impl RegisterBlock {
    #[doc = "0x00 - Processor core identifier"]
    #[inline(always)]
    pub const fn cpuid(&self) -> &CPUID {
        &self.cpuid
    }
    #[doc = "0x04 - Input value for GPIO0...31. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero."]
    #[inline(always)]
    pub const fn gpio_in(&self) -> &GPIO_IN {
        &self.gpio_in
    }
    #[doc = "0x08 - Input value on GPIO32...47, QSPI IOs and USB pins In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero."]
    #[inline(always)]
    pub const fn gpio_hi_in(&self) -> &GPIO_HI_IN {
        &self.gpio_hi_in
    }
    #[doc = "0x10 - GPIO0...31 output value"]
    #[inline(always)]
    pub const fn gpio_out(&self) -> &GPIO_OUT {
        &self.gpio_out
    }
    #[doc = "0x14 - Output value for GPIO32...47, QSPI IOs and USB pins. Write to set output level (1/0 -> high/low). Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register."]
    #[inline(always)]
    pub const fn gpio_hi_out(&self) -> &GPIO_HI_OUT {
        &self.gpio_hi_out
    }
    #[doc = "0x18 - GPIO0...31 output value set"]
    #[inline(always)]
    pub const fn gpio_out_set(&self) -> &GPIO_OUT_SET {
        &self.gpio_out_set
    }
    #[doc = "0x1c - Output value set for GPIO32..47, QSPI IOs and USB pins. Perform an atomic bit-set on GPIO_HI_OUT, i.e. `GPIO_HI_OUT |= wdata`"]
    #[inline(always)]
    pub const fn gpio_hi_out_set(&self) -> &GPIO_HI_OUT_SET {
        &self.gpio_hi_out_set
    }
    #[doc = "0x20 - GPIO0...31 output value clear"]
    #[inline(always)]
    pub const fn gpio_out_clr(&self) -> &GPIO_OUT_CLR {
        &self.gpio_out_clr
    }
    #[doc = "0x24 - Output value clear for GPIO32..47, QSPI IOs and USB pins. Perform an atomic bit-clear on GPIO_HI_OUT, i.e. `GPIO_HI_OUT &amp;= ~wdata`"]
    #[inline(always)]
    pub const fn gpio_hi_out_clr(&self) -> &GPIO_HI_OUT_CLR {
        &self.gpio_hi_out_clr
    }
    #[doc = "0x28 - GPIO0...31 output value XOR"]
    #[inline(always)]
    pub const fn gpio_out_xor(&self) -> &GPIO_OUT_XOR {
        &self.gpio_out_xor
    }
    #[doc = "0x2c - Output value XOR for GPIO32..47, QSPI IOs and USB pins. Perform an atomic bitwise XOR on GPIO_HI_OUT, i.e. `GPIO_HI_OUT ^= wdata`"]
    #[inline(always)]
    pub const fn gpio_hi_out_xor(&self) -> &GPIO_HI_OUT_XOR {
        &self.gpio_hi_out_xor
    }
    #[doc = "0x30 - GPIO0...31 output enable"]
    #[inline(always)]
    pub const fn gpio_oe(&self) -> &GPIO_OE {
        &self.gpio_oe
    }
    #[doc = "0x34 - Output enable value for GPIO32...47, QSPI IOs and USB pins. Write output enable (1/0 -> output/input). Reading back gives the last value written. If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register."]
    #[inline(always)]
    pub const fn gpio_hi_oe(&self) -> &GPIO_HI_OE {
        &self.gpio_hi_oe
    }
    #[doc = "0x38 - GPIO0...31 output enable set"]
    #[inline(always)]
    pub const fn gpio_oe_set(&self) -> &GPIO_OE_SET {
        &self.gpio_oe_set
    }
    #[doc = "0x3c - Output enable set for GPIO32...47, QSPI IOs and USB pins. Perform an atomic bit-set on GPIO_HI_OE, i.e. `GPIO_HI_OE |= wdata`"]
    #[inline(always)]
    pub const fn gpio_hi_oe_set(&self) -> &GPIO_HI_OE_SET {
        &self.gpio_hi_oe_set
    }
    #[doc = "0x40 - GPIO0...31 output enable clear"]
    #[inline(always)]
    pub const fn gpio_oe_clr(&self) -> &GPIO_OE_CLR {
        &self.gpio_oe_clr
    }
    #[doc = "0x44 - Output enable clear for GPIO32...47, QSPI IOs and USB pins. Perform an atomic bit-clear on GPIO_HI_OE, i.e. `GPIO_HI_OE &amp;= ~wdata`"]
    #[inline(always)]
    pub const fn gpio_hi_oe_clr(&self) -> &GPIO_HI_OE_CLR {
        &self.gpio_hi_oe_clr
    }
    #[doc = "0x48 - GPIO0...31 output enable XOR"]
    #[inline(always)]
    pub const fn gpio_oe_xor(&self) -> &GPIO_OE_XOR {
        &self.gpio_oe_xor
    }
    #[doc = "0x4c - Output enable XOR for GPIO32...47, QSPI IOs and USB pins. Perform an atomic bitwise XOR on GPIO_HI_OE, i.e. `GPIO_HI_OE ^= wdata`"]
    #[inline(always)]
    pub const fn gpio_hi_oe_xor(&self) -> &GPIO_HI_OE_XOR {
        &self.gpio_hi_oe_xor
    }
    #[doc = "0x50 - Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
    #[inline(always)]
    pub const fn fifo_st(&self) -> &FIFO_ST {
        &self.fifo_st
    }
    #[doc = "0x54 - Write access to this core's TX FIFO"]
    #[inline(always)]
    pub const fn fifo_wr(&self) -> &FIFO_WR {
        &self.fifo_wr
    }
    #[doc = "0x58 - Read access to this core's RX FIFO"]
    #[inline(always)]
    pub const fn fifo_rd(&self) -> &FIFO_RD {
        &self.fifo_rd
    }
    #[doc = "0x5c - Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging."]
    #[inline(always)]
    pub const fn spinlock_st(&self) -> &SPINLOCK_ST {
        &self.spinlock_st
    }
    #[doc = "0x80 - Read/write access to accumulator 0"]
    #[inline(always)]
    pub const fn interp0_accum0(&self) -> &INTERP0_ACCUM0 {
        &self.interp0_accum0
    }
    #[doc = "0x84 - Read/write access to accumulator 1"]
    #[inline(always)]
    pub const fn interp0_accum1(&self) -> &INTERP0_ACCUM1 {
        &self.interp0_accum1
    }
    #[doc = "0x88 - Read/write access to BASE0 register."]
    #[inline(always)]
    pub const fn interp0_base0(&self) -> &INTERP0_BASE0 {
        &self.interp0_base0
    }
    #[doc = "0x8c - Read/write access to BASE1 register."]
    #[inline(always)]
    pub const fn interp0_base1(&self) -> &INTERP0_BASE1 {
        &self.interp0_base1
    }
    #[doc = "0x90 - Read/write access to BASE2 register."]
    #[inline(always)]
    pub const fn interp0_base2(&self) -> &INTERP0_BASE2 {
        &self.interp0_base2
    }
    #[doc = "0x94 - Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp0_pop_lane0(&self) -> &INTERP0_POP_LANE0 {
        &self.interp0_pop_lane0
    }
    #[doc = "0x98 - Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp0_pop_lane1(&self) -> &INTERP0_POP_LANE1 {
        &self.interp0_pop_lane1
    }
    #[doc = "0x9c - Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp0_pop_full(&self) -> &INTERP0_POP_FULL {
        &self.interp0_pop_full
    }
    #[doc = "0xa0 - Read LANE0 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp0_peek_lane0(&self) -> &INTERP0_PEEK_LANE0 {
        &self.interp0_peek_lane0
    }
    #[doc = "0xa4 - Read LANE1 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp0_peek_lane1(&self) -> &INTERP0_PEEK_LANE1 {
        &self.interp0_peek_lane1
    }
    #[doc = "0xa8 - Read FULL result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp0_peek_full(&self) -> &INTERP0_PEEK_FULL {
        &self.interp0_peek_full
    }
    #[doc = "0xac - Control register for lane 0"]
    #[inline(always)]
    pub const fn interp0_ctrl_lane0(&self) -> &INTERP0_CTRL_LANE0 {
        &self.interp0_ctrl_lane0
    }
    #[doc = "0xb0 - Control register for lane 1"]
    #[inline(always)]
    pub const fn interp0_ctrl_lane1(&self) -> &INTERP0_CTRL_LANE1 {
        &self.interp0_ctrl_lane1
    }
    #[doc = "0xb4 - Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    #[inline(always)]
    pub const fn interp0_accum0_add(&self) -> &INTERP0_ACCUM0_ADD {
        &self.interp0_accum0_add
    }
    #[doc = "0xb8 - Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    #[inline(always)]
    pub const fn interp0_accum1_add(&self) -> &INTERP0_ACCUM1_ADD {
        &self.interp0_accum1_add
    }
    #[doc = "0xbc - On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    #[inline(always)]
    pub const fn interp0_base_1and0(&self) -> &INTERP0_BASE_1AND0 {
        &self.interp0_base_1and0
    }
    #[doc = "0xc0 - Read/write access to accumulator 0"]
    #[inline(always)]
    pub const fn interp1_accum0(&self) -> &INTERP1_ACCUM0 {
        &self.interp1_accum0
    }
    #[doc = "0xc4 - Read/write access to accumulator 1"]
    #[inline(always)]
    pub const fn interp1_accum1(&self) -> &INTERP1_ACCUM1 {
        &self.interp1_accum1
    }
    #[doc = "0xc8 - Read/write access to BASE0 register."]
    #[inline(always)]
    pub const fn interp1_base0(&self) -> &INTERP1_BASE0 {
        &self.interp1_base0
    }
    #[doc = "0xcc - Read/write access to BASE1 register."]
    #[inline(always)]
    pub const fn interp1_base1(&self) -> &INTERP1_BASE1 {
        &self.interp1_base1
    }
    #[doc = "0xd0 - Read/write access to BASE2 register."]
    #[inline(always)]
    pub const fn interp1_base2(&self) -> &INTERP1_BASE2 {
        &self.interp1_base2
    }
    #[doc = "0xd4 - Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp1_pop_lane0(&self) -> &INTERP1_POP_LANE0 {
        &self.interp1_pop_lane0
    }
    #[doc = "0xd8 - Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp1_pop_lane1(&self) -> &INTERP1_POP_LANE1 {
        &self.interp1_pop_lane1
    }
    #[doc = "0xdc - Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn interp1_pop_full(&self) -> &INTERP1_POP_FULL {
        &self.interp1_pop_full
    }
    #[doc = "0xe0 - Read LANE0 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp1_peek_lane0(&self) -> &INTERP1_PEEK_LANE0 {
        &self.interp1_peek_lane0
    }
    #[doc = "0xe4 - Read LANE1 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp1_peek_lane1(&self) -> &INTERP1_PEEK_LANE1 {
        &self.interp1_peek_lane1
    }
    #[doc = "0xe8 - Read FULL result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn interp1_peek_full(&self) -> &INTERP1_PEEK_FULL {
        &self.interp1_peek_full
    }
    #[doc = "0xec - Control register for lane 0"]
    #[inline(always)]
    pub const fn interp1_ctrl_lane0(&self) -> &INTERP1_CTRL_LANE0 {
        &self.interp1_ctrl_lane0
    }
    #[doc = "0xf0 - Control register for lane 1"]
    #[inline(always)]
    pub const fn interp1_ctrl_lane1(&self) -> &INTERP1_CTRL_LANE1 {
        &self.interp1_ctrl_lane1
    }
    #[doc = "0xf4 - Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    #[inline(always)]
    pub const fn interp1_accum0_add(&self) -> &INTERP1_ACCUM0_ADD {
        &self.interp1_accum0_add
    }
    #[doc = "0xf8 - Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    #[inline(always)]
    pub const fn interp1_accum1_add(&self) -> &INTERP1_ACCUM1_ADD {
        &self.interp1_accum1_add
    }
    #[doc = "0xfc - On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    #[inline(always)]
    pub const fn interp1_base_1and0(&self) -> &INTERP1_BASE_1AND0 {
        &self.interp1_base_1and0
    }
    #[doc = "0x100..0x180 - Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 &lt;&lt; lock number."]
    #[inline(always)]
    pub const fn spinlock(&self, n: usize) -> &SPINLOCK {
        &self.spinlock[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x180 - Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 &lt;&lt; lock number."]
    #[inline(always)]
    pub fn spinlock_iter(&self) -> impl Iterator<Item = &SPINLOCK> {
        self.spinlock.iter()
    }
    #[doc = "0x180 - Trigger a doorbell interrupt on the opposite core. Write 1 to a bit to set the corresponding bit in DOORBELL_IN on the opposite core. This raises the opposite core's doorbell interrupt. Read to get the status of the doorbells currently asserted on the opposite core. This is equivalent to that core reading its own DOORBELL_IN status."]
    #[inline(always)]
    pub const fn doorbell_out_set(&self) -> &DOORBELL_OUT_SET {
        &self.doorbell_out_set
    }
    #[doc = "0x184 - Clear doorbells which have been posted to the opposite core. This register is intended for debugging and initialisation purposes. Writing 1 to a bit in DOORBELL_OUT_CLR clears the corresponding bit in DOORBELL_IN on the opposite core. Clearing all bits will cause that core's doorbell interrupt to deassert. Since the usual order of events is for software to send events using DOORBELL_OUT_SET, and acknowledge incoming events by writing to DOORBELL_IN_CLR, this register should be used with caution to avoid race conditions. Reading returns the status of the doorbells currently asserted on the other core, i.e. is equivalent to that core reading its own DOORBELL_IN status."]
    #[inline(always)]
    pub const fn doorbell_out_clr(&self) -> &DOORBELL_OUT_CLR {
        &self.doorbell_out_clr
    }
    #[doc = "0x188 - Write 1s to trigger doorbell interrupts on this core. Read to get status of doorbells currently asserted on this core."]
    #[inline(always)]
    pub const fn doorbell_in_set(&self) -> &DOORBELL_IN_SET {
        &self.doorbell_in_set
    }
    #[doc = "0x18c - Check and acknowledge doorbells posted to this core. This core's doorbell interrupt is asserted when any bit in this register is 1. Write 1 to each bit to clear that bit. The doorbell interrupt deasserts once all bits are cleared. Read to get status of doorbells currently asserted on this core."]
    #[inline(always)]
    pub const fn doorbell_in_clr(&self) -> &DOORBELL_IN_CLR {
        &self.doorbell_in_clr
    }
    #[doc = "0x190 - Detach certain core-local peripherals from Secure SIO, and attach them to Non-secure SIO, so that Non-secure software can use them. Attempting to access one of these peripherals from the Secure SIO when it is attached to the Non-secure SIO, or vice versa, will generate a bus error. This register is per-core, and is only present on the Secure SIO. Most SIO hardware is duplicated across the Secure and Non-secure SIO, so is not listed in this register."]
    #[inline(always)]
    pub const fn peri_nonsec(&self) -> &PERI_NONSEC {
        &self.peri_nonsec
    }
    #[doc = "0x1a0 - Control the assertion of the standard software interrupt (MIP.MSIP) on the RISC-V cores. Unlike the RISC-V timer, this interrupt is not routed to a normal system-level interrupt line, so can not be used by the Arm cores. It is safe for both cores to write to this register on the same cycle. The set/clear effect is accumulated across both cores, and then applied. If a flag is both set and cleared on the same cycle, only the set takes effect."]
    #[inline(always)]
    pub const fn riscv_softirq(&self) -> &RISCV_SOFTIRQ {
        &self.riscv_softirq
    }
    #[doc = "0x1a4 - Control register for the RISC-V 64-bit Machine-mode timer. This timer is only present in the Secure SIO, so is only accessible to an Arm core in Secure mode or a RISC-V core in Machine mode. Note whilst this timer follows the RISC-V privileged specification, it is equally usable by the Arm cores. The interrupts are routed to normal system-level interrupt lines as well as to the MIP.MTIP inputs on the RISC-V cores."]
    #[inline(always)]
    pub const fn mtime_ctrl(&self) -> &MTIME_CTRL {
        &self.mtime_ctrl
    }
    #[doc = "0x1b0 - Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence."]
    #[inline(always)]
    pub const fn mtime(&self) -> &MTIME {
        &self.mtime
    }
    #[doc = "0x1b4 - Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence."]
    #[inline(always)]
    pub const fn mtimeh(&self) -> &MTIMEH {
        &self.mtimeh
    }
    #[doc = "0x1b8 - Low half of RISC-V Machine-mode timer comparator. This register is core-local, i.e., each core gets a copy of this register, with the comparison result routed to its own interrupt line. The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values."]
    #[inline(always)]
    pub const fn mtimecmp(&self) -> &MTIMECMP {
        &self.mtimecmp
    }
    #[doc = "0x1bc - High half of RISC-V Machine-mode timer comparator. This register is core-local. The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values."]
    #[inline(always)]
    pub const fn mtimecmph(&self) -> &MTIMECMPH {
        &self.mtimecmph
    }
    #[doc = "0x1c0 - Control register for TMDS encoder."]
    #[inline(always)]
    pub const fn tmds_ctrl(&self) -> &TMDS_CTRL {
        &self.tmds_ctrl
    }
    #[doc = "0x1c4 - Write-only access to the TMDS colour data register."]
    #[inline(always)]
    pub const fn tmds_wdata(&self) -> &TMDS_WDATA {
        &self.tmds_wdata
    }
    #[doc = "0x1c8 - Get the encoding of one pixel's worth of colour data, packed into a 32-bit value (3x10-bit symbols). The PEEK alias does not shift the colour register when read, but still advances the running DC balance state of each encoder. This is useful for pixel doubling."]
    #[inline(always)]
    pub const fn tmds_peek_single(&self) -> &TMDS_PEEK_SINGLE {
        &self.tmds_peek_single
    }
    #[doc = "0x1cc - Get the encoding of one pixel's worth of colour data, packed into a 32-bit value. The packing is 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane. This format is intended for shifting out with the HSTX peripheral on RP2350. The POP alias shifts the colour register when read, as well as advancing the running DC balance state of each encoder."]
    #[inline(always)]
    pub const fn tmds_pop_single(&self) -> &TMDS_POP_SINGLE {
        &self.tmds_pop_single
    }
    #[doc = "0x1d0 - Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 0 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane."]
    #[inline(always)]
    pub const fn tmds_peek_double_l0(&self) -> &TMDS_PEEK_DOUBLE_L0 {
        &self.tmds_peek_double_l0
    }
    #[doc = "0x1d4 - Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT."]
    #[inline(always)]
    pub const fn tmds_pop_double_l0(&self) -> &TMDS_POP_DOUBLE_L0 {
        &self.tmds_pop_double_l0
    }
    #[doc = "0x1d8 - Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 1 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane."]
    #[inline(always)]
    pub const fn tmds_peek_double_l1(&self) -> &TMDS_PEEK_DOUBLE_L1 {
        &self.tmds_peek_double_l1
    }
    #[doc = "0x1dc - Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT."]
    #[inline(always)]
    pub const fn tmds_pop_double_l1(&self) -> &TMDS_POP_DOUBLE_L1 {
        &self.tmds_pop_double_l1
    }
    #[doc = "0x1e0 - Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 2 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane."]
    #[inline(always)]
    pub const fn tmds_peek_double_l2(&self) -> &TMDS_PEEK_DOUBLE_L2 {
        &self.tmds_peek_double_l2
    }
    #[doc = "0x1e4 - Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT."]
    #[inline(always)]
    pub const fn tmds_pop_double_l2(&self) -> &TMDS_POP_DOUBLE_L2 {
        &self.tmds_pop_double_l2
    }
}
#[doc = "CPUID (rw) register accessor: Processor core identifier  

You can [`read`](crate::Reg::read) this register and get [`cpuid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cpuid`]
module"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "Processor core identifier"]
pub mod cpuid;
#[doc = "GPIO_IN (rw) register accessor: Input value for GPIO0...31. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero.  

You can [`read`](crate::Reg::read) this register and get [`gpio_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_in`]
module"]
pub type GPIO_IN = crate::Reg<gpio_in::GPIO_IN_SPEC>;
#[doc = "Input value for GPIO0...31. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero."]
pub mod gpio_in;
#[doc = "GPIO_HI_IN (rw) register accessor: Input value on GPIO32...47, QSPI IOs and USB pins In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero.  

You can [`read`](crate::Reg::read) this register and get [`gpio_hi_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hi_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_in`]
module"]
pub type GPIO_HI_IN = crate::Reg<gpio_hi_in::GPIO_HI_IN_SPEC>;
#[doc = "Input value on GPIO32...47, QSPI IOs and USB pins In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero."]
pub mod gpio_hi_in;
#[doc = "GPIO_OUT (rw) register accessor: GPIO0...31 output value  

You can [`read`](crate::Reg::read) this register and get [`gpio_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_out`]
module"]
pub type GPIO_OUT = crate::Reg<gpio_out::GPIO_OUT_SPEC>;
#[doc = "GPIO0...31 output value"]
pub mod gpio_out;
#[doc = "GPIO_HI_OUT (rw) register accessor: Output value for GPIO32...47, QSPI IOs and USB pins. Write to set output level (1/0 -> high/low). Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.  

You can [`read`](crate::Reg::read) this register and get [`gpio_hi_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hi_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_out`]
module"]
pub type GPIO_HI_OUT = crate::Reg<gpio_hi_out::GPIO_HI_OUT_SPEC>;
#[doc = "Output value for GPIO32...47, QSPI IOs and USB pins. Write to set output level (1/0 -> high/low). Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register."]
pub mod gpio_hi_out;
#[doc = "GPIO_OUT_SET (rw) register accessor: GPIO0...31 output value set  

You can [`read`](crate::Reg::read) this register and get [`gpio_out_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_out_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_out_set`]
module"]
pub type GPIO_OUT_SET = crate::Reg<gpio_out_set::GPIO_OUT_SET_SPEC>;
#[doc = "GPIO0...31 output value set"]
pub mod gpio_out_set;
#[doc = "GPIO_HI_OUT_SET (rw) register accessor: Output value set for GPIO32..47, QSPI IOs and USB pins. Perform an atomic bit-set on GPIO_HI_OUT, i.e. `GPIO_HI_OUT |= wdata`  

You can [`read`](crate::Reg::read) this register and get [`gpio_hi_out_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hi_out_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_out_set`]
module"]
pub type GPIO_HI_OUT_SET = crate::Reg<gpio_hi_out_set::GPIO_HI_OUT_SET_SPEC>;
#[doc = "Output value set for GPIO32..47, QSPI IOs and USB pins. Perform an atomic bit-set on GPIO_HI_OUT, i.e. `GPIO_HI_OUT |= wdata`"]
pub mod gpio_hi_out_set;
#[doc = "GPIO_OUT_CLR (rw) register accessor: GPIO0...31 output value clear  

You can [`read`](crate::Reg::read) this register and get [`gpio_out_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_out_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_out_clr`]
module"]
pub type GPIO_OUT_CLR = crate::Reg<gpio_out_clr::GPIO_OUT_CLR_SPEC>;
#[doc = "GPIO0...31 output value clear"]
pub mod gpio_out_clr;
#[doc = "GPIO_HI_OUT_CLR (rw) register accessor: Output value clear for GPIO32..47, QSPI IOs and USB pins. Perform an atomic bit-clear on GPIO_HI_OUT, i.e. `GPIO_HI_OUT &amp;= ~wdata`  

You can [`read`](crate::Reg::read) this register and get [`gpio_hi_out_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hi_out_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_out_clr`]
module"]
pub type GPIO_HI_OUT_CLR = crate::Reg<gpio_hi_out_clr::GPIO_HI_OUT_CLR_SPEC>;
#[doc = "Output value clear for GPIO32..47, QSPI IOs and USB pins. Perform an atomic bit-clear on GPIO_HI_OUT, i.e. `GPIO_HI_OUT &amp;= ~wdata`"]
pub mod gpio_hi_out_clr;
#[doc = "GPIO_OUT_XOR (rw) register accessor: GPIO0...31 output value XOR  

You can [`read`](crate::Reg::read) this register and get [`gpio_out_xor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_out_xor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_out_xor`]
module"]
pub type GPIO_OUT_XOR = crate::Reg<gpio_out_xor::GPIO_OUT_XOR_SPEC>;
#[doc = "GPIO0...31 output value XOR"]
pub mod gpio_out_xor;
#[doc = "GPIO_HI_OUT_XOR (rw) register accessor: Output value XOR for GPIO32..47, QSPI IOs and USB pins. Perform an atomic bitwise XOR on GPIO_HI_OUT, i.e. `GPIO_HI_OUT ^= wdata`  

You can [`read`](crate::Reg::read) this register and get [`gpio_hi_out_xor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hi_out_xor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_out_xor`]
module"]
pub type GPIO_HI_OUT_XOR = crate::Reg<gpio_hi_out_xor::GPIO_HI_OUT_XOR_SPEC>;
#[doc = "Output value XOR for GPIO32..47, QSPI IOs and USB pins. Perform an atomic bitwise XOR on GPIO_HI_OUT, i.e. `GPIO_HI_OUT ^= wdata`"]
pub mod gpio_hi_out_xor;
#[doc = "GPIO_OE (rw) register accessor: GPIO0...31 output enable  

You can [`read`](crate::Reg::read) this register and get [`gpio_oe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_oe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_oe`]
module"]
pub type GPIO_OE = crate::Reg<gpio_oe::GPIO_OE_SPEC>;
#[doc = "GPIO0...31 output enable"]
pub mod gpio_oe;
#[doc = "GPIO_HI_OE (rw) register accessor: Output enable value for GPIO32...47, QSPI IOs and USB pins. Write output enable (1/0 -> output/input). Reading back gives the last value written. If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.  

You can [`read`](crate::Reg::read) this register and get [`gpio_hi_oe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hi_oe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_oe`]
module"]
pub type GPIO_HI_OE = crate::Reg<gpio_hi_oe::GPIO_HI_OE_SPEC>;
#[doc = "Output enable value for GPIO32...47, QSPI IOs and USB pins. Write output enable (1/0 -> output/input). Reading back gives the last value written. If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register."]
pub mod gpio_hi_oe;
#[doc = "GPIO_OE_SET (rw) register accessor: GPIO0...31 output enable set  

You can [`read`](crate::Reg::read) this register and get [`gpio_oe_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_oe_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_oe_set`]
module"]
pub type GPIO_OE_SET = crate::Reg<gpio_oe_set::GPIO_OE_SET_SPEC>;
#[doc = "GPIO0...31 output enable set"]
pub mod gpio_oe_set;
#[doc = "GPIO_HI_OE_SET (rw) register accessor: Output enable set for GPIO32...47, QSPI IOs and USB pins. Perform an atomic bit-set on GPIO_HI_OE, i.e. `GPIO_HI_OE |= wdata`  

You can [`read`](crate::Reg::read) this register and get [`gpio_hi_oe_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hi_oe_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_oe_set`]
module"]
pub type GPIO_HI_OE_SET = crate::Reg<gpio_hi_oe_set::GPIO_HI_OE_SET_SPEC>;
#[doc = "Output enable set for GPIO32...47, QSPI IOs and USB pins. Perform an atomic bit-set on GPIO_HI_OE, i.e. `GPIO_HI_OE |= wdata`"]
pub mod gpio_hi_oe_set;
#[doc = "GPIO_OE_CLR (rw) register accessor: GPIO0...31 output enable clear  

You can [`read`](crate::Reg::read) this register and get [`gpio_oe_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_oe_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_oe_clr`]
module"]
pub type GPIO_OE_CLR = crate::Reg<gpio_oe_clr::GPIO_OE_CLR_SPEC>;
#[doc = "GPIO0...31 output enable clear"]
pub mod gpio_oe_clr;
#[doc = "GPIO_HI_OE_CLR (rw) register accessor: Output enable clear for GPIO32...47, QSPI IOs and USB pins. Perform an atomic bit-clear on GPIO_HI_OE, i.e. `GPIO_HI_OE &amp;= ~wdata`  

You can [`read`](crate::Reg::read) this register and get [`gpio_hi_oe_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hi_oe_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_oe_clr`]
module"]
pub type GPIO_HI_OE_CLR = crate::Reg<gpio_hi_oe_clr::GPIO_HI_OE_CLR_SPEC>;
#[doc = "Output enable clear for GPIO32...47, QSPI IOs and USB pins. Perform an atomic bit-clear on GPIO_HI_OE, i.e. `GPIO_HI_OE &amp;= ~wdata`"]
pub mod gpio_hi_oe_clr;
#[doc = "GPIO_OE_XOR (rw) register accessor: GPIO0...31 output enable XOR  

You can [`read`](crate::Reg::read) this register and get [`gpio_oe_xor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_oe_xor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_oe_xor`]
module"]
pub type GPIO_OE_XOR = crate::Reg<gpio_oe_xor::GPIO_OE_XOR_SPEC>;
#[doc = "GPIO0...31 output enable XOR"]
pub mod gpio_oe_xor;
#[doc = "GPIO_HI_OE_XOR (rw) register accessor: Output enable XOR for GPIO32...47, QSPI IOs and USB pins. Perform an atomic bitwise XOR on GPIO_HI_OE, i.e. `GPIO_HI_OE ^= wdata`  

You can [`read`](crate::Reg::read) this register and get [`gpio_hi_oe_xor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_hi_oe_xor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_hi_oe_xor`]
module"]
pub type GPIO_HI_OE_XOR = crate::Reg<gpio_hi_oe_xor::GPIO_HI_OE_XOR_SPEC>;
#[doc = "Output enable XOR for GPIO32...47, QSPI IOs and USB pins. Perform an atomic bitwise XOR on GPIO_HI_OE, i.e. `GPIO_HI_OE ^= wdata`"]
pub mod gpio_hi_oe_xor;
#[doc = "FIFO_ST (rw) register accessor: Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register.  

You can [`read`](crate::Reg::read) this register and get [`fifo_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fifo_st`]
module"]
pub type FIFO_ST = crate::Reg<fifo_st::FIFO_ST_SPEC>;
#[doc = "Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
pub mod fifo_st;
#[doc = "FIFO_WR (rw) register accessor: Write access to this core's TX FIFO  

You can [`read`](crate::Reg::read) this register and get [`fifo_wr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_wr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fifo_wr`]
module"]
pub type FIFO_WR = crate::Reg<fifo_wr::FIFO_WR_SPEC>;
#[doc = "Write access to this core's TX FIFO"]
pub mod fifo_wr;
#[doc = "FIFO_RD (rw) register accessor: Read access to this core's RX FIFO  

You can [`read`](crate::Reg::read) this register and get [`fifo_rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fifo_rd`]
module"]
pub type FIFO_RD = crate::Reg<fifo_rd::FIFO_RD_SPEC>;
#[doc = "Read access to this core's RX FIFO"]
pub mod fifo_rd;
#[doc = "SPINLOCK_ST (rw) register accessor: Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging.  

You can [`read`](crate::Reg::read) this register and get [`spinlock_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spinlock_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@spinlock_st`]
module"]
pub type SPINLOCK_ST = crate::Reg<spinlock_st::SPINLOCK_ST_SPEC>;
#[doc = "Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging."]
pub mod spinlock_st;
#[doc = "INTERP0_ACCUM0 (rw) register accessor: Read/write access to accumulator 0  

You can [`read`](crate::Reg::read) this register and get [`interp0_accum0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_accum0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_accum0`]
module"]
pub type INTERP0_ACCUM0 = crate::Reg<interp0_accum0::INTERP0_ACCUM0_SPEC>;
#[doc = "Read/write access to accumulator 0"]
pub mod interp0_accum0;
#[doc = "INTERP0_ACCUM1 (rw) register accessor: Read/write access to accumulator 1  

You can [`read`](crate::Reg::read) this register and get [`interp0_accum1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_accum1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_accum1`]
module"]
pub type INTERP0_ACCUM1 = crate::Reg<interp0_accum1::INTERP0_ACCUM1_SPEC>;
#[doc = "Read/write access to accumulator 1"]
pub mod interp0_accum1;
#[doc = "INTERP0_BASE0 (rw) register accessor: Read/write access to BASE0 register.  

You can [`read`](crate::Reg::read) this register and get [`interp0_base0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_base0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_base0`]
module"]
pub type INTERP0_BASE0 = crate::Reg<interp0_base0::INTERP0_BASE0_SPEC>;
#[doc = "Read/write access to BASE0 register."]
pub mod interp0_base0;
#[doc = "INTERP0_BASE1 (rw) register accessor: Read/write access to BASE1 register.  

You can [`read`](crate::Reg::read) this register and get [`interp0_base1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_base1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_base1`]
module"]
pub type INTERP0_BASE1 = crate::Reg<interp0_base1::INTERP0_BASE1_SPEC>;
#[doc = "Read/write access to BASE1 register."]
pub mod interp0_base1;
#[doc = "INTERP0_BASE2 (rw) register accessor: Read/write access to BASE2 register.  

You can [`read`](crate::Reg::read) this register and get [`interp0_base2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_base2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_base2`]
module"]
pub type INTERP0_BASE2 = crate::Reg<interp0_base2::INTERP0_BASE2_SPEC>;
#[doc = "Read/write access to BASE2 register."]
pub mod interp0_base2;
#[doc = "INTERP0_POP_LANE0 (rw) register accessor: Read LANE0 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::Reg::read) this register and get [`interp0_pop_lane0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_pop_lane0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_pop_lane0`]
module"]
pub type INTERP0_POP_LANE0 = crate::Reg<interp0_pop_lane0::INTERP0_POP_LANE0_SPEC>;
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_lane0;
#[doc = "INTERP0_POP_LANE1 (rw) register accessor: Read LANE1 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::Reg::read) this register and get [`interp0_pop_lane1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_pop_lane1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_pop_lane1`]
module"]
pub type INTERP0_POP_LANE1 = crate::Reg<interp0_pop_lane1::INTERP0_POP_LANE1_SPEC>;
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_lane1;
#[doc = "INTERP0_POP_FULL (rw) register accessor: Read FULL result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::Reg::read) this register and get [`interp0_pop_full::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_pop_full::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_pop_full`]
module"]
pub type INTERP0_POP_FULL = crate::Reg<interp0_pop_full::INTERP0_POP_FULL_SPEC>;
#[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp0_pop_full;
#[doc = "INTERP0_PEEK_LANE0 (rw) register accessor: Read LANE0 result, without altering any internal state (PEEK).  

You can [`read`](crate::Reg::read) this register and get [`interp0_peek_lane0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_peek_lane0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_peek_lane0`]
module"]
pub type INTERP0_PEEK_LANE0 = crate::Reg<interp0_peek_lane0::INTERP0_PEEK_LANE0_SPEC>;
#[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
pub mod interp0_peek_lane0;
#[doc = "INTERP0_PEEK_LANE1 (rw) register accessor: Read LANE1 result, without altering any internal state (PEEK).  

You can [`read`](crate::Reg::read) this register and get [`interp0_peek_lane1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_peek_lane1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_peek_lane1`]
module"]
pub type INTERP0_PEEK_LANE1 = crate::Reg<interp0_peek_lane1::INTERP0_PEEK_LANE1_SPEC>;
#[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
pub mod interp0_peek_lane1;
#[doc = "INTERP0_PEEK_FULL (rw) register accessor: Read FULL result, without altering any internal state (PEEK).  

You can [`read`](crate::Reg::read) this register and get [`interp0_peek_full::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_peek_full::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_peek_full`]
module"]
pub type INTERP0_PEEK_FULL = crate::Reg<interp0_peek_full::INTERP0_PEEK_FULL_SPEC>;
#[doc = "Read FULL result, without altering any internal state (PEEK)."]
pub mod interp0_peek_full;
#[doc = "INTERP0_CTRL_LANE0 (rw) register accessor: Control register for lane 0  

You can [`read`](crate::Reg::read) this register and get [`interp0_ctrl_lane0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_ctrl_lane0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_ctrl_lane0`]
module"]
pub type INTERP0_CTRL_LANE0 = crate::Reg<interp0_ctrl_lane0::INTERP0_CTRL_LANE0_SPEC>;
#[doc = "Control register for lane 0"]
pub mod interp0_ctrl_lane0;
#[doc = "INTERP0_CTRL_LANE1 (rw) register accessor: Control register for lane 1  

You can [`read`](crate::Reg::read) this register and get [`interp0_ctrl_lane1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_ctrl_lane1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_ctrl_lane1`]
module"]
pub type INTERP0_CTRL_LANE1 = crate::Reg<interp0_ctrl_lane1::INTERP0_CTRL_LANE1_SPEC>;
#[doc = "Control register for lane 1"]
pub mod interp0_ctrl_lane1;
#[doc = "INTERP0_ACCUM0_ADD (rw) register accessor: Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added).  

You can [`read`](crate::Reg::read) this register and get [`interp0_accum0_add::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_accum0_add::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_accum0_add`]
module"]
pub type INTERP0_ACCUM0_ADD = crate::Reg<interp0_accum0_add::INTERP0_ACCUM0_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
pub mod interp0_accum0_add;
#[doc = "INTERP0_ACCUM1_ADD (rw) register accessor: Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added).  

You can [`read`](crate::Reg::read) this register and get [`interp0_accum1_add::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_accum1_add::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_accum1_add`]
module"]
pub type INTERP0_ACCUM1_ADD = crate::Reg<interp0_accum1_add::INTERP0_ACCUM1_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
pub mod interp0_accum1_add;
#[doc = "INTERP0_BASE_1AND0 (rw) register accessor: On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.  

You can [`read`](crate::Reg::read) this register and get [`interp0_base_1and0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_base_1and0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp0_base_1and0`]
module"]
pub type INTERP0_BASE_1AND0 = crate::Reg<interp0_base_1and0::INTERP0_BASE_1AND0_SPEC>;
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
pub mod interp0_base_1and0;
#[doc = "INTERP1_ACCUM0 (rw) register accessor: Read/write access to accumulator 0  

You can [`read`](crate::Reg::read) this register and get [`interp1_accum0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_accum0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_accum0`]
module"]
pub type INTERP1_ACCUM0 = crate::Reg<interp1_accum0::INTERP1_ACCUM0_SPEC>;
#[doc = "Read/write access to accumulator 0"]
pub mod interp1_accum0;
#[doc = "INTERP1_ACCUM1 (rw) register accessor: Read/write access to accumulator 1  

You can [`read`](crate::Reg::read) this register and get [`interp1_accum1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_accum1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_accum1`]
module"]
pub type INTERP1_ACCUM1 = crate::Reg<interp1_accum1::INTERP1_ACCUM1_SPEC>;
#[doc = "Read/write access to accumulator 1"]
pub mod interp1_accum1;
#[doc = "INTERP1_BASE0 (rw) register accessor: Read/write access to BASE0 register.  

You can [`read`](crate::Reg::read) this register and get [`interp1_base0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_base0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_base0`]
module"]
pub type INTERP1_BASE0 = crate::Reg<interp1_base0::INTERP1_BASE0_SPEC>;
#[doc = "Read/write access to BASE0 register."]
pub mod interp1_base0;
#[doc = "INTERP1_BASE1 (rw) register accessor: Read/write access to BASE1 register.  

You can [`read`](crate::Reg::read) this register and get [`interp1_base1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_base1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_base1`]
module"]
pub type INTERP1_BASE1 = crate::Reg<interp1_base1::INTERP1_BASE1_SPEC>;
#[doc = "Read/write access to BASE1 register."]
pub mod interp1_base1;
#[doc = "INTERP1_BASE2 (rw) register accessor: Read/write access to BASE2 register.  

You can [`read`](crate::Reg::read) this register and get [`interp1_base2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_base2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_base2`]
module"]
pub type INTERP1_BASE2 = crate::Reg<interp1_base2::INTERP1_BASE2_SPEC>;
#[doc = "Read/write access to BASE2 register."]
pub mod interp1_base2;
#[doc = "INTERP1_POP_LANE0 (rw) register accessor: Read LANE0 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::Reg::read) this register and get [`interp1_pop_lane0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_pop_lane0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_pop_lane0`]
module"]
pub type INTERP1_POP_LANE0 = crate::Reg<interp1_pop_lane0::INTERP1_POP_LANE0_SPEC>;
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_lane0;
#[doc = "INTERP1_POP_LANE1 (rw) register accessor: Read LANE1 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::Reg::read) this register and get [`interp1_pop_lane1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_pop_lane1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_pop_lane1`]
module"]
pub type INTERP1_POP_LANE1 = crate::Reg<interp1_pop_lane1::INTERP1_POP_LANE1_SPEC>;
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_lane1;
#[doc = "INTERP1_POP_FULL (rw) register accessor: Read FULL result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::Reg::read) this register and get [`interp1_pop_full::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_pop_full::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_pop_full`]
module"]
pub type INTERP1_POP_FULL = crate::Reg<interp1_pop_full::INTERP1_POP_FULL_SPEC>;
#[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
pub mod interp1_pop_full;
#[doc = "INTERP1_PEEK_LANE0 (rw) register accessor: Read LANE0 result, without altering any internal state (PEEK).  

You can [`read`](crate::Reg::read) this register and get [`interp1_peek_lane0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_peek_lane0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_peek_lane0`]
module"]
pub type INTERP1_PEEK_LANE0 = crate::Reg<interp1_peek_lane0::INTERP1_PEEK_LANE0_SPEC>;
#[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
pub mod interp1_peek_lane0;
#[doc = "INTERP1_PEEK_LANE1 (rw) register accessor: Read LANE1 result, without altering any internal state (PEEK).  

You can [`read`](crate::Reg::read) this register and get [`interp1_peek_lane1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_peek_lane1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_peek_lane1`]
module"]
pub type INTERP1_PEEK_LANE1 = crate::Reg<interp1_peek_lane1::INTERP1_PEEK_LANE1_SPEC>;
#[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
pub mod interp1_peek_lane1;
#[doc = "INTERP1_PEEK_FULL (rw) register accessor: Read FULL result, without altering any internal state (PEEK).  

You can [`read`](crate::Reg::read) this register and get [`interp1_peek_full::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_peek_full::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_peek_full`]
module"]
pub type INTERP1_PEEK_FULL = crate::Reg<interp1_peek_full::INTERP1_PEEK_FULL_SPEC>;
#[doc = "Read FULL result, without altering any internal state (PEEK)."]
pub mod interp1_peek_full;
#[doc = "INTERP1_CTRL_LANE0 (rw) register accessor: Control register for lane 0  

You can [`read`](crate::Reg::read) this register and get [`interp1_ctrl_lane0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_ctrl_lane0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_ctrl_lane0`]
module"]
pub type INTERP1_CTRL_LANE0 = crate::Reg<interp1_ctrl_lane0::INTERP1_CTRL_LANE0_SPEC>;
#[doc = "Control register for lane 0"]
pub mod interp1_ctrl_lane0;
#[doc = "INTERP1_CTRL_LANE1 (rw) register accessor: Control register for lane 1  

You can [`read`](crate::Reg::read) this register and get [`interp1_ctrl_lane1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_ctrl_lane1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_ctrl_lane1`]
module"]
pub type INTERP1_CTRL_LANE1 = crate::Reg<interp1_ctrl_lane1::INTERP1_CTRL_LANE1_SPEC>;
#[doc = "Control register for lane 1"]
pub mod interp1_ctrl_lane1;
#[doc = "INTERP1_ACCUM0_ADD (rw) register accessor: Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added).  

You can [`read`](crate::Reg::read) this register and get [`interp1_accum0_add::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_accum0_add::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_accum0_add`]
module"]
pub type INTERP1_ACCUM0_ADD = crate::Reg<interp1_accum0_add::INTERP1_ACCUM0_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
pub mod interp1_accum0_add;
#[doc = "INTERP1_ACCUM1_ADD (rw) register accessor: Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added).  

You can [`read`](crate::Reg::read) this register and get [`interp1_accum1_add::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_accum1_add::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_accum1_add`]
module"]
pub type INTERP1_ACCUM1_ADD = crate::Reg<interp1_accum1_add::INTERP1_ACCUM1_ADD_SPEC>;
#[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
pub mod interp1_accum1_add;
#[doc = "INTERP1_BASE_1AND0 (rw) register accessor: On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.  

You can [`read`](crate::Reg::read) this register and get [`interp1_base_1and0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_base_1and0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@interp1_base_1and0`]
module"]
pub type INTERP1_BASE_1AND0 = crate::Reg<interp1_base_1and0::INTERP1_BASE_1AND0_SPEC>;
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
pub mod interp1_base_1and0;
#[doc = "SPINLOCK (rw) register accessor: Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 &lt;&lt; lock number.  

You can [`read`](crate::Reg::read) this register and get [`spinlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spinlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@spinlock`]
module"]
pub type SPINLOCK = crate::Reg<spinlock::SPINLOCK_SPEC>;
#[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 &lt;&lt; lock number."]
pub mod spinlock;
#[doc = "DOORBELL_OUT_SET (rw) register accessor: Trigger a doorbell interrupt on the opposite core. Write 1 to a bit to set the corresponding bit in DOORBELL_IN on the opposite core. This raises the opposite core's doorbell interrupt. Read to get the status of the doorbells currently asserted on the opposite core. This is equivalent to that core reading its own DOORBELL_IN status.  

You can [`read`](crate::Reg::read) this register and get [`doorbell_out_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doorbell_out_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@doorbell_out_set`]
module"]
pub type DOORBELL_OUT_SET = crate::Reg<doorbell_out_set::DOORBELL_OUT_SET_SPEC>;
#[doc = "Trigger a doorbell interrupt on the opposite core. Write 1 to a bit to set the corresponding bit in DOORBELL_IN on the opposite core. This raises the opposite core's doorbell interrupt. Read to get the status of the doorbells currently asserted on the opposite core. This is equivalent to that core reading its own DOORBELL_IN status."]
pub mod doorbell_out_set;
#[doc = "DOORBELL_OUT_CLR (rw) register accessor: Clear doorbells which have been posted to the opposite core. This register is intended for debugging and initialisation purposes. Writing 1 to a bit in DOORBELL_OUT_CLR clears the corresponding bit in DOORBELL_IN on the opposite core. Clearing all bits will cause that core's doorbell interrupt to deassert. Since the usual order of events is for software to send events using DOORBELL_OUT_SET, and acknowledge incoming events by writing to DOORBELL_IN_CLR, this register should be used with caution to avoid race conditions. Reading returns the status of the doorbells currently asserted on the other core, i.e. is equivalent to that core reading its own DOORBELL_IN status.  

You can [`read`](crate::Reg::read) this register and get [`doorbell_out_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doorbell_out_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@doorbell_out_clr`]
module"]
pub type DOORBELL_OUT_CLR = crate::Reg<doorbell_out_clr::DOORBELL_OUT_CLR_SPEC>;
#[doc = "Clear doorbells which have been posted to the opposite core. This register is intended for debugging and initialisation purposes. Writing 1 to a bit in DOORBELL_OUT_CLR clears the corresponding bit in DOORBELL_IN on the opposite core. Clearing all bits will cause that core's doorbell interrupt to deassert. Since the usual order of events is for software to send events using DOORBELL_OUT_SET, and acknowledge incoming events by writing to DOORBELL_IN_CLR, this register should be used with caution to avoid race conditions. Reading returns the status of the doorbells currently asserted on the other core, i.e. is equivalent to that core reading its own DOORBELL_IN status."]
pub mod doorbell_out_clr;
#[doc = "DOORBELL_IN_SET (rw) register accessor: Write 1s to trigger doorbell interrupts on this core. Read to get status of doorbells currently asserted on this core.  

You can [`read`](crate::Reg::read) this register and get [`doorbell_in_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doorbell_in_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@doorbell_in_set`]
module"]
pub type DOORBELL_IN_SET = crate::Reg<doorbell_in_set::DOORBELL_IN_SET_SPEC>;
#[doc = "Write 1s to trigger doorbell interrupts on this core. Read to get status of doorbells currently asserted on this core."]
pub mod doorbell_in_set;
#[doc = "DOORBELL_IN_CLR (rw) register accessor: Check and acknowledge doorbells posted to this core. This core's doorbell interrupt is asserted when any bit in this register is 1. Write 1 to each bit to clear that bit. The doorbell interrupt deasserts once all bits are cleared. Read to get status of doorbells currently asserted on this core.  

You can [`read`](crate::Reg::read) this register and get [`doorbell_in_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doorbell_in_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@doorbell_in_clr`]
module"]
pub type DOORBELL_IN_CLR = crate::Reg<doorbell_in_clr::DOORBELL_IN_CLR_SPEC>;
#[doc = "Check and acknowledge doorbells posted to this core. This core's doorbell interrupt is asserted when any bit in this register is 1. Write 1 to each bit to clear that bit. The doorbell interrupt deasserts once all bits are cleared. Read to get status of doorbells currently asserted on this core."]
pub mod doorbell_in_clr;
#[doc = "PERI_NONSEC (rw) register accessor: Detach certain core-local peripherals from Secure SIO, and attach them to Non-secure SIO, so that Non-secure software can use them. Attempting to access one of these peripherals from the Secure SIO when it is attached to the Non-secure SIO, or vice versa, will generate a bus error. This register is per-core, and is only present on the Secure SIO. Most SIO hardware is duplicated across the Secure and Non-secure SIO, so is not listed in this register.  

You can [`read`](crate::Reg::read) this register and get [`peri_nonsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_nonsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@peri_nonsec`]
module"]
pub type PERI_NONSEC = crate::Reg<peri_nonsec::PERI_NONSEC_SPEC>;
#[doc = "Detach certain core-local peripherals from Secure SIO, and attach them to Non-secure SIO, so that Non-secure software can use them. Attempting to access one of these peripherals from the Secure SIO when it is attached to the Non-secure SIO, or vice versa, will generate a bus error. This register is per-core, and is only present on the Secure SIO. Most SIO hardware is duplicated across the Secure and Non-secure SIO, so is not listed in this register."]
pub mod peri_nonsec;
#[doc = "RISCV_SOFTIRQ (rw) register accessor: Control the assertion of the standard software interrupt (MIP.MSIP) on the RISC-V cores. Unlike the RISC-V timer, this interrupt is not routed to a normal system-level interrupt line, so can not be used by the Arm cores. It is safe for both cores to write to this register on the same cycle. The set/clear effect is accumulated across both cores, and then applied. If a flag is both set and cleared on the same cycle, only the set takes effect.  

You can [`read`](crate::Reg::read) this register and get [`riscv_softirq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`riscv_softirq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@riscv_softirq`]
module"]
pub type RISCV_SOFTIRQ = crate::Reg<riscv_softirq::RISCV_SOFTIRQ_SPEC>;
#[doc = "Control the assertion of the standard software interrupt (MIP.MSIP) on the RISC-V cores. Unlike the RISC-V timer, this interrupt is not routed to a normal system-level interrupt line, so can not be used by the Arm cores. It is safe for both cores to write to this register on the same cycle. The set/clear effect is accumulated across both cores, and then applied. If a flag is both set and cleared on the same cycle, only the set takes effect."]
pub mod riscv_softirq;
#[doc = "MTIME_CTRL (rw) register accessor: Control register for the RISC-V 64-bit Machine-mode timer. This timer is only present in the Secure SIO, so is only accessible to an Arm core in Secure mode or a RISC-V core in Machine mode. Note whilst this timer follows the RISC-V privileged specification, it is equally usable by the Arm cores. The interrupts are routed to normal system-level interrupt lines as well as to the MIP.MTIP inputs on the RISC-V cores.  

You can [`read`](crate::Reg::read) this register and get [`mtime_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mtime_ctrl`]
module"]
pub type MTIME_CTRL = crate::Reg<mtime_ctrl::MTIME_CTRL_SPEC>;
#[doc = "Control register for the RISC-V 64-bit Machine-mode timer. This timer is only present in the Secure SIO, so is only accessible to an Arm core in Secure mode or a RISC-V core in Machine mode. Note whilst this timer follows the RISC-V privileged specification, it is equally usable by the Arm cores. The interrupts are routed to normal system-level interrupt lines as well as to the MIP.MTIP inputs on the RISC-V cores."]
pub mod mtime_ctrl;
#[doc = "MTIME (rw) register accessor: Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence.  

You can [`read`](crate::Reg::read) this register and get [`mtime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mtime`]
module"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence."]
pub mod mtime;
#[doc = "MTIMEH (rw) register accessor: Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence.  

You can [`read`](crate::Reg::read) this register and get [`mtimeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mtimeh`]
module"]
pub type MTIMEH = crate::Reg<mtimeh::MTIMEH_SPEC>;
#[doc = "Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence."]
pub mod mtimeh;
#[doc = "MTIMECMP (rw) register accessor: Low half of RISC-V Machine-mode timer comparator. This register is core-local, i.e., each core gets a copy of this register, with the comparison result routed to its own interrupt line. The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values.  

You can [`read`](crate::Reg::read) this register and get [`mtimecmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mtimecmp`]
module"]
pub type MTIMECMP = crate::Reg<mtimecmp::MTIMECMP_SPEC>;
#[doc = "Low half of RISC-V Machine-mode timer comparator. This register is core-local, i.e., each core gets a copy of this register, with the comparison result routed to its own interrupt line. The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values."]
pub mod mtimecmp;
#[doc = "MTIMECMPH (rw) register accessor: High half of RISC-V Machine-mode timer comparator. This register is core-local. The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values.  

You can [`read`](crate::Reg::read) this register and get [`mtimecmph::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmph::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mtimecmph`]
module"]
pub type MTIMECMPH = crate::Reg<mtimecmph::MTIMECMPH_SPEC>;
#[doc = "High half of RISC-V Machine-mode timer comparator. This register is core-local. The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values."]
pub mod mtimecmph;
#[doc = "TMDS_CTRL (rw) register accessor: Control register for TMDS encoder.  

You can [`read`](crate::Reg::read) this register and get [`tmds_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tmds_ctrl`]
module"]
pub type TMDS_CTRL = crate::Reg<tmds_ctrl::TMDS_CTRL_SPEC>;
#[doc = "Control register for TMDS encoder."]
pub mod tmds_ctrl;
#[doc = "TMDS_WDATA (rw) register accessor: Write-only access to the TMDS colour data register.  

You can [`read`](crate::Reg::read) this register and get [`tmds_wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tmds_wdata`]
module"]
pub type TMDS_WDATA = crate::Reg<tmds_wdata::TMDS_WDATA_SPEC>;
#[doc = "Write-only access to the TMDS colour data register."]
pub mod tmds_wdata;
#[doc = "TMDS_PEEK_SINGLE (rw) register accessor: Get the encoding of one pixel's worth of colour data, packed into a 32-bit value (3x10-bit symbols). The PEEK alias does not shift the colour register when read, but still advances the running DC balance state of each encoder. This is useful for pixel doubling.  

You can [`read`](crate::Reg::read) this register and get [`tmds_peek_single::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_peek_single::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tmds_peek_single`]
module"]
pub type TMDS_PEEK_SINGLE = crate::Reg<tmds_peek_single::TMDS_PEEK_SINGLE_SPEC>;
#[doc = "Get the encoding of one pixel's worth of colour data, packed into a 32-bit value (3x10-bit symbols). The PEEK alias does not shift the colour register when read, but still advances the running DC balance state of each encoder. This is useful for pixel doubling."]
pub mod tmds_peek_single;
#[doc = "TMDS_POP_SINGLE (rw) register accessor: Get the encoding of one pixel's worth of colour data, packed into a 32-bit value. The packing is 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane. This format is intended for shifting out with the HSTX peripheral on RP2350. The POP alias shifts the colour register when read, as well as advancing the running DC balance state of each encoder.  

You can [`read`](crate::Reg::read) this register and get [`tmds_pop_single::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_pop_single::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tmds_pop_single`]
module"]
pub type TMDS_POP_SINGLE = crate::Reg<tmds_pop_single::TMDS_POP_SINGLE_SPEC>;
#[doc = "Get the encoding of one pixel's worth of colour data, packed into a 32-bit value. The packing is 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane. This format is intended for shifting out with the HSTX peripheral on RP2350. The POP alias shifts the colour register when read, as well as advancing the running DC balance state of each encoder."]
pub mod tmds_pop_single;
#[doc = "TMDS_PEEK_DOUBLE_L0 (rw) register accessor: Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 0 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane.  

You can [`read`](crate::Reg::read) this register and get [`tmds_peek_double_l0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_peek_double_l0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tmds_peek_double_l0`]
module"]
pub type TMDS_PEEK_DOUBLE_L0 = crate::Reg<tmds_peek_double_l0::TMDS_PEEK_DOUBLE_L0_SPEC>;
#[doc = "Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 0 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane."]
pub mod tmds_peek_double_l0;
#[doc = "TMDS_POP_DOUBLE_L0 (rw) register accessor: Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.  

You can [`read`](crate::Reg::read) this register and get [`tmds_pop_double_l0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_pop_double_l0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tmds_pop_double_l0`]
module"]
pub type TMDS_POP_DOUBLE_L0 = crate::Reg<tmds_pop_double_l0::TMDS_POP_DOUBLE_L0_SPEC>;
#[doc = "Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT."]
pub mod tmds_pop_double_l0;
#[doc = "TMDS_PEEK_DOUBLE_L1 (rw) register accessor: Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 1 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane.  

You can [`read`](crate::Reg::read) this register and get [`tmds_peek_double_l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_peek_double_l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tmds_peek_double_l1`]
module"]
pub type TMDS_PEEK_DOUBLE_L1 = crate::Reg<tmds_peek_double_l1::TMDS_PEEK_DOUBLE_L1_SPEC>;
#[doc = "Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 1 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane."]
pub mod tmds_peek_double_l1;
#[doc = "TMDS_POP_DOUBLE_L1 (rw) register accessor: Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.  

You can [`read`](crate::Reg::read) this register and get [`tmds_pop_double_l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_pop_double_l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tmds_pop_double_l1`]
module"]
pub type TMDS_POP_DOUBLE_L1 = crate::Reg<tmds_pop_double_l1::TMDS_POP_DOUBLE_L1_SPEC>;
#[doc = "Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT."]
pub mod tmds_pop_double_l1;
#[doc = "TMDS_PEEK_DOUBLE_L2 (rw) register accessor: Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 2 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane.  

You can [`read`](crate::Reg::read) this register and get [`tmds_peek_double_l2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_peek_double_l2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tmds_peek_double_l2`]
module"]
pub type TMDS_PEEK_DOUBLE_L2 = crate::Reg<tmds_peek_double_l2::TMDS_PEEK_DOUBLE_L2_SPEC>;
#[doc = "Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 2 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane."]
pub mod tmds_peek_double_l2;
#[doc = "TMDS_POP_DOUBLE_L2 (rw) register accessor: Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.  

You can [`read`](crate::Reg::read) this register and get [`tmds_pop_double_l2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_pop_double_l2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tmds_pop_double_l2`]
module"]
pub type TMDS_POP_DOUBLE_L2 = crate::Reg<tmds_pop_double_l2::TMDS_POP_DOUBLE_L2_SPEC>;
#[doc = "Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT."]
pub mod tmds_pop_double_l2;
