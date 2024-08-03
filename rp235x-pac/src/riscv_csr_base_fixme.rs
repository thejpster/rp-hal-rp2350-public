#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0300],
    _reserved_0_mie: [u8; 0x0a],
    menvcfg: MENVCFG,
    _reserved2: [u8; 0x02],
    mstatush: MSTATUSH,
    _reserved3: [u8; 0x06],
    menvcfgh: MENVCFGH,
    _reserved4: [u8; 0x02],
    _reserved_4_mip: [u8; 0x28],
    _reserved5: [u8; 0x58],
    _reserved_5_pmpcfg0: [u8; 0x07],
    _reserved6: [u8; 0x09],
    _reserved_6_pmpaddr0: [u8; 0x13],
    _reserved7: [u8; 0x03dd],
    _reserved_7_tdata1: [u8; 0x06],
    _reserved8: [u8; 0x0a],
    _reserved_8_dpc: [u8; 0x05],
    _reserved9: [u8; 0x034b],
    _reserved_9_mcycle: [u8; 0x23],
    _reserved10: [u8; 0x5d],
    _reserved_10_mcycleh: [u8; 0x23],
    _reserved11: [u8; 0x2d],
    pmpcfgm0: PMPCFGM0,
    _reserved12: [u8; 0x0c],
    _reserved_12_meiea: [u8; 0x09],
    _reserved13: [u8; 0x07],
    msleep: MSLEEP,
    _reserved14: [u8; 0x0b],
    _reserved_14_cycle: [u8; 0x07],
    _reserved15: [u8; 0x7a],
    _reserved_15_cycleh: [u8; 0x06],
    _reserved16: [u8; 0x028b],
    _reserved_16_mimpid: [u8; 0x08],
}
impl RegisterBlock {
    #[doc = "0x300 - Machine status register"]
    #[inline(always)]
    pub const fn mstatus(&self) -> &MSTATUS {
        unsafe { &*(self as *const Self).cast::<u8>().add(768).cast() }
    }
    #[doc = "0x301 - Summary of ISA extension support  

 On RP2350, Hazard3's full `-march` string is: `rv32ima_zicsr_zifencei_zba_zbb_zbs_zbkb_zca_zcb_zcmp`  

 Note Zca is equivalent to the C extension in this case; all instructions from the RISC-V C extension relevant to a 32-bit non-floating-point processor are supported. On older toolchains which do not support the Zc extensions, the appropriate `-march` string is: `rv32imac_zicsr_zifencei_zba_zbb_zbs_zbkb`  

 In addition the following custom extensions are configured: Xh3bm, Xh3power, Xh3irq, Xh3pmpm"]
    #[inline(always)]
    pub const fn misa(&self) -> &MISA {
        unsafe { &*(self as *const Self).cast::<u8>().add(769).cast() }
    }
    #[doc = "0x302 - Machine exception delegation register. Not implemented, as no S-mode support."]
    #[inline(always)]
    pub const fn medeleg(&self) -> &MEDELEG {
        unsafe { &*(self as *const Self).cast::<u8>().add(770).cast() }
    }
    #[doc = "0x303 - Machine interrupt delegation register. Not implemented, as no S-mode support."]
    #[inline(always)]
    pub const fn mideleg(&self) -> &MIDELEG {
        unsafe { &*(self as *const Self).cast::<u8>().add(771).cast() }
    }
    #[doc = "0x304 - Machine interrupt enable register"]
    #[inline(always)]
    pub const fn mie(&self) -> &MIE {
        unsafe { &*(self as *const Self).cast::<u8>().add(772).cast() }
    }
    #[doc = "0x305 - Machine trap handler base address."]
    #[inline(always)]
    pub const fn mtvec(&self) -> &MTVEC {
        unsafe { &*(self as *const Self).cast::<u8>().add(773).cast() }
    }
    #[doc = "0x306 - Counter enable. Control access to counters from U-mode. Not to be confused with mcountinhibit."]
    #[inline(always)]
    pub const fn mcounteren(&self) -> &MCOUNTEREN {
        unsafe { &*(self as *const Self).cast::<u8>().add(774).cast() }
    }
    #[doc = "0x30a - Machine environment configuration register, low half"]
    #[inline(always)]
    pub const fn menvcfg(&self) -> &MENVCFG {
        &self.menvcfg
    }
    #[doc = "0x310 - High half of mstatus, hardwired to 0."]
    #[inline(always)]
    pub const fn mstatush(&self) -> &MSTATUSH {
        &self.mstatush
    }
    #[doc = "0x31a - Machine environment configuration register, high half  

 This register is fully reserved, as Hazard3 does not implement the relevant extensions. It is implemented as hardwired-0."]
    #[inline(always)]
    pub const fn menvcfgh(&self) -> &MENVCFGH {
        &self.menvcfgh
    }
    #[doc = "0x320 - Count inhibit register for `mcycle`/`minstret`"]
    #[inline(always)]
    pub const fn mcountinhibit(&self) -> &MCOUNTINHIBIT {
        unsafe { &*(self as *const Self).cast::<u8>().add(800).cast() }
    }
    #[doc = "0x323 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent3(&self) -> &MHPMEVENT3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(803).cast() }
    }
    #[doc = "0x324 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent4(&self) -> &MHPMEVENT4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(804).cast() }
    }
    #[doc = "0x325 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent5(&self) -> &MHPMEVENT5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(805).cast() }
    }
    #[doc = "0x326 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent6(&self) -> &MHPMEVENT6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(806).cast() }
    }
    #[doc = "0x327 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent7(&self) -> &MHPMEVENT7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(807).cast() }
    }
    #[doc = "0x328 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent8(&self) -> &MHPMEVENT8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(808).cast() }
    }
    #[doc = "0x329 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent9(&self) -> &MHPMEVENT9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(809).cast() }
    }
    #[doc = "0x32a - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent10(&self) -> &MHPMEVENT10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(810).cast() }
    }
    #[doc = "0x32b - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent11(&self) -> &MHPMEVENT11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(811).cast() }
    }
    #[doc = "0x32c - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent12(&self) -> &MHPMEVENT12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(812).cast() }
    }
    #[doc = "0x32d - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent13(&self) -> &MHPMEVENT13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(813).cast() }
    }
    #[doc = "0x32e - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent14(&self) -> &MHPMEVENT14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(814).cast() }
    }
    #[doc = "0x32f - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent15(&self) -> &MHPMEVENT15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(815).cast() }
    }
    #[doc = "0x330 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent16(&self) -> &MHPMEVENT16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(816).cast() }
    }
    #[doc = "0x331 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent17(&self) -> &MHPMEVENT17 {
        unsafe { &*(self as *const Self).cast::<u8>().add(817).cast() }
    }
    #[doc = "0x332 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent18(&self) -> &MHPMEVENT18 {
        unsafe { &*(self as *const Self).cast::<u8>().add(818).cast() }
    }
    #[doc = "0x333 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent19(&self) -> &MHPMEVENT19 {
        unsafe { &*(self as *const Self).cast::<u8>().add(819).cast() }
    }
    #[doc = "0x334 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent20(&self) -> &MHPMEVENT20 {
        unsafe { &*(self as *const Self).cast::<u8>().add(820).cast() }
    }
    #[doc = "0x335 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent21(&self) -> &MHPMEVENT21 {
        unsafe { &*(self as *const Self).cast::<u8>().add(821).cast() }
    }
    #[doc = "0x336 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent22(&self) -> &MHPMEVENT22 {
        unsafe { &*(self as *const Self).cast::<u8>().add(822).cast() }
    }
    #[doc = "0x337 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent23(&self) -> &MHPMEVENT23 {
        unsafe { &*(self as *const Self).cast::<u8>().add(823).cast() }
    }
    #[doc = "0x338 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent24(&self) -> &MHPMEVENT24 {
        unsafe { &*(self as *const Self).cast::<u8>().add(824).cast() }
    }
    #[doc = "0x339 - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent25(&self) -> &MHPMEVENT25 {
        unsafe { &*(self as *const Self).cast::<u8>().add(825).cast() }
    }
    #[doc = "0x33a - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent26(&self) -> &MHPMEVENT26 {
        unsafe { &*(self as *const Self).cast::<u8>().add(826).cast() }
    }
    #[doc = "0x33b - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent27(&self) -> &MHPMEVENT27 {
        unsafe { &*(self as *const Self).cast::<u8>().add(827).cast() }
    }
    #[doc = "0x33c - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent28(&self) -> &MHPMEVENT28 {
        unsafe { &*(self as *const Self).cast::<u8>().add(828).cast() }
    }
    #[doc = "0x33d - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent29(&self) -> &MHPMEVENT29 {
        unsafe { &*(self as *const Self).cast::<u8>().add(829).cast() }
    }
    #[doc = "0x33e - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent30(&self) -> &MHPMEVENT30 {
        unsafe { &*(self as *const Self).cast::<u8>().add(830).cast() }
    }
    #[doc = "0x33f - Extended performance event selector, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmevent31(&self) -> &MHPMEVENT31 {
        unsafe { &*(self as *const Self).cast::<u8>().add(831).cast() }
    }
    #[doc = "0x340 - Scratch register for machine trap handlers.  

 32-bit read/write register with no specific hardware function. Software may use this to do a fast save/restore of a core register in a trap handler."]
    #[inline(always)]
    pub const fn mscratch(&self) -> &MSCRATCH {
        unsafe { &*(self as *const Self).cast::<u8>().add(832).cast() }
    }
    #[doc = "0x341 - Machine exception program counter.  

 When entering a trap, the current value of the program counter is recorded here. When executing an `mret`, the processor jumps to `mepc`. Can also be read and written by software."]
    #[inline(always)]
    pub const fn mepc(&self) -> &MEPC {
        unsafe { &*(self as *const Self).cast::<u8>().add(833).cast() }
    }
    #[doc = "0x342 - Machine trap cause. Set when entering a trap to indicate the reason for the trap. Readable and writable by software."]
    #[inline(always)]
    pub const fn mcause(&self) -> &MCAUSE {
        unsafe { &*(self as *const Self).cast::<u8>().add(834).cast() }
    }
    #[doc = "0x343 - Machine bad address or instruction. Hardwired to zero."]
    #[inline(always)]
    pub const fn mtval(&self) -> &MTVAL {
        unsafe { &*(self as *const Self).cast::<u8>().add(835).cast() }
    }
    #[doc = "0x344 - Machine interrupt pending"]
    #[inline(always)]
    pub const fn mip(&self) -> &MIP {
        unsafe { &*(self as *const Self).cast::<u8>().add(836).cast() }
    }
    #[doc = "0x3a0 - Physical memory protection configuration for regions 0 through 3"]
    #[inline(always)]
    pub const fn pmpcfg0(&self) -> &PMPCFG0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(928).cast() }
    }
    #[doc = "0x3a1 - Physical memory protection configuration for regions 4 through 7"]
    #[inline(always)]
    pub const fn pmpcfg1(&self) -> &PMPCFG1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(929).cast() }
    }
    #[doc = "0x3a2 - Physical memory protection configuration for regions 8 through 11"]
    #[inline(always)]
    pub const fn pmpcfg2(&self) -> &PMPCFG2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(930).cast() }
    }
    #[doc = "0x3a3 - Physical memory protection configuration for regions 12 through 15"]
    #[inline(always)]
    pub const fn pmpcfg3(&self) -> &PMPCFG3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(931).cast() }
    }
    #[doc = "0x3b0 - Physical memory protection address for region 0. Note all PMP addresses are in units of four bytes."]
    #[inline(always)]
    pub const fn pmpaddr0(&self) -> &PMPADDR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(944).cast() }
    }
    #[doc = "0x3b1 - Physical memory protection address for region 1. Note all PMP addresses are in units of four bytes."]
    #[inline(always)]
    pub const fn pmpaddr1(&self) -> &PMPADDR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(945).cast() }
    }
    #[doc = "0x3b2 - Physical memory protection address for region 2. Note all PMP addresses are in units of four bytes."]
    #[inline(always)]
    pub const fn pmpaddr2(&self) -> &PMPADDR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(946).cast() }
    }
    #[doc = "0x3b3 - Physical memory protection address for region 3. Note all PMP addresses are in units of four bytes."]
    #[inline(always)]
    pub const fn pmpaddr3(&self) -> &PMPADDR3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(947).cast() }
    }
    #[doc = "0x3b4 - Physical memory protection address for region 4. Note all PMP addresses are in units of four bytes."]
    #[inline(always)]
    pub const fn pmpaddr4(&self) -> &PMPADDR4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(948).cast() }
    }
    #[doc = "0x3b5 - Physical memory protection address for region 5. Note all PMP addresses are in units of four bytes."]
    #[inline(always)]
    pub const fn pmpaddr5(&self) -> &PMPADDR5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(949).cast() }
    }
    #[doc = "0x3b6 - Physical memory protection address for region 6. Note all PMP addresses are in units of four bytes."]
    #[inline(always)]
    pub const fn pmpaddr6(&self) -> &PMPADDR6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(950).cast() }
    }
    #[doc = "0x3b7 - Physical memory protection address for region 7. Note all PMP addresses are in units of four bytes."]
    #[inline(always)]
    pub const fn pmpaddr7(&self) -> &PMPADDR7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(951).cast() }
    }
    #[doc = "0x3b8 - Physical memory protection address for region 8. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0x00000000` through `0x0fffffff`, which contains the boot ROM. This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL."]
    #[inline(always)]
    pub const fn pmpaddr8(&self) -> &PMPADDR8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(952).cast() }
    }
    #[doc = "0x3b9 - Physical memory protection address for region 9. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0x40000000` through `0x5fffffff`, which contains the system peripherals. This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL."]
    #[inline(always)]
    pub const fn pmpaddr9(&self) -> &PMPADDR9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(953).cast() }
    }
    #[doc = "0x3ba - Physical memory protection address for region 10. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0xd0000000` through `0xdfffffff`, which contains the core-local peripherals (SIO). This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL."]
    #[inline(always)]
    pub const fn pmpaddr10(&self) -> &PMPADDR10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(954).cast() }
    }
    #[doc = "0x3bb - Physical memory protection address for region 11. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented."]
    #[inline(always)]
    pub const fn pmpaddr11(&self) -> &PMPADDR11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(955).cast() }
    }
    #[doc = "0x3bc - Physical memory protection address for region 12. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented."]
    #[inline(always)]
    pub const fn pmpaddr12(&self) -> &PMPADDR12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(956).cast() }
    }
    #[doc = "0x3bd - Physical memory protection address for region 13. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented."]
    #[inline(always)]
    pub const fn pmpaddr13(&self) -> &PMPADDR13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(957).cast() }
    }
    #[doc = "0x3be - Physical memory protection address for region 14. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented."]
    #[inline(always)]
    pub const fn pmpaddr14(&self) -> &PMPADDR14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(958).cast() }
    }
    #[doc = "0x3bf - Physical memory protection address for region 15. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented."]
    #[inline(always)]
    pub const fn pmpaddr15(&self) -> &PMPADDR15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(959).cast() }
    }
    #[doc = "0x7a0 - Select trigger to be configured via `tdata1`/`tdata2`  

 On RP2350, four instruction address triggers are implemented, so only the two LSBs of this register are writable."]
    #[inline(always)]
    pub const fn tselect(&self) -> &TSELECT {
        unsafe { &*(self as *const Self).cast::<u8>().add(1952).cast() }
    }
    #[doc = "0x7a1 - Trigger configuration data 1  

 Hazard 3 only supports address/data match triggers (type=2) so this register description includes the `mcontrol` fields for this type.  

 More precisely, Hazard3 only supports exact instruction address match triggers (hardware breakpoints) so many of this register's fields are hardwired."]
    #[inline(always)]
    pub const fn tdata1(&self) -> &TDATA1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1953).cast() }
    }
    #[doc = "0x7a2 - Trigger configuration data 2  

 Contains the address for instruction address triggers (hardware breakpoints)"]
    #[inline(always)]
    pub const fn tdata2(&self) -> &TDATA2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1954).cast() }
    }
    #[doc = "0x7b0 - Debug control and status register. Access outside of Debug Mode will cause an illegal instruction exception."]
    #[inline(always)]
    pub const fn dcsr(&self) -> &DCSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(1968).cast() }
    }
    #[doc = "0x7b1 - Debug program counter. When entering Debug Mode, `dpc` samples the current program counter, e.g. the address of an `ebreak` which caused Debug Mode entry. When leaving debug mode, the processor jumps to `dpc`. The host may read/write this register whilst in Debug Mode."]
    #[inline(always)]
    pub const fn dpc(&self) -> &DPC {
        unsafe { &*(self as *const Self).cast::<u8>().add(1969).cast() }
    }
    #[doc = "0xb00 - Machine-mode cycle counter, low half  
 Counts up once per cycle, when `mcountinhibit.cy` is 0. Disabled by default to save power."]
    #[inline(always)]
    pub const fn mcycle(&self) -> &MCYCLE {
        unsafe { &*(self as *const Self).cast::<u8>().add(2816).cast() }
    }
    #[doc = "0xb02 - Machine-mode instruction retire counter, low half  
 Counts up once per instruction, when `mcountinhibit.ir` is 0. Disabled by default to save power."]
    #[inline(always)]
    pub const fn minstret(&self) -> &MINSTRET {
        unsafe { &*(self as *const Self).cast::<u8>().add(2818).cast() }
    }
    #[doc = "0xb03 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter3(&self) -> &MHPMCOUNTER3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2819).cast() }
    }
    #[doc = "0xb04 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter4(&self) -> &MHPMCOUNTER4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2820).cast() }
    }
    #[doc = "0xb05 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter5(&self) -> &MHPMCOUNTER5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2821).cast() }
    }
    #[doc = "0xb06 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter6(&self) -> &MHPMCOUNTER6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2822).cast() }
    }
    #[doc = "0xb07 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter7(&self) -> &MHPMCOUNTER7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2823).cast() }
    }
    #[doc = "0xb08 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter8(&self) -> &MHPMCOUNTER8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2824).cast() }
    }
    #[doc = "0xb09 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter9(&self) -> &MHPMCOUNTER9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2825).cast() }
    }
    #[doc = "0xb0a - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter10(&self) -> &MHPMCOUNTER10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2826).cast() }
    }
    #[doc = "0xb0b - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter11(&self) -> &MHPMCOUNTER11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2827).cast() }
    }
    #[doc = "0xb0c - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter12(&self) -> &MHPMCOUNTER12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2828).cast() }
    }
    #[doc = "0xb0d - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter13(&self) -> &MHPMCOUNTER13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2829).cast() }
    }
    #[doc = "0xb0e - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter14(&self) -> &MHPMCOUNTER14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2830).cast() }
    }
    #[doc = "0xb0f - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter15(&self) -> &MHPMCOUNTER15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2831).cast() }
    }
    #[doc = "0xb10 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter16(&self) -> &MHPMCOUNTER16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2832).cast() }
    }
    #[doc = "0xb11 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter17(&self) -> &MHPMCOUNTER17 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2833).cast() }
    }
    #[doc = "0xb12 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter18(&self) -> &MHPMCOUNTER18 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2834).cast() }
    }
    #[doc = "0xb13 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter19(&self) -> &MHPMCOUNTER19 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2835).cast() }
    }
    #[doc = "0xb14 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter20(&self) -> &MHPMCOUNTER20 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2836).cast() }
    }
    #[doc = "0xb15 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter21(&self) -> &MHPMCOUNTER21 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2837).cast() }
    }
    #[doc = "0xb16 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter22(&self) -> &MHPMCOUNTER22 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2838).cast() }
    }
    #[doc = "0xb17 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter23(&self) -> &MHPMCOUNTER23 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2839).cast() }
    }
    #[doc = "0xb18 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter24(&self) -> &MHPMCOUNTER24 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2840).cast() }
    }
    #[doc = "0xb19 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter25(&self) -> &MHPMCOUNTER25 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2841).cast() }
    }
    #[doc = "0xb1a - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter26(&self) -> &MHPMCOUNTER26 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2842).cast() }
    }
    #[doc = "0xb1b - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter27(&self) -> &MHPMCOUNTER27 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2843).cast() }
    }
    #[doc = "0xb1c - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter28(&self) -> &MHPMCOUNTER28 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2844).cast() }
    }
    #[doc = "0xb1d - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter29(&self) -> &MHPMCOUNTER29 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2845).cast() }
    }
    #[doc = "0xb1e - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter30(&self) -> &MHPMCOUNTER30 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2846).cast() }
    }
    #[doc = "0xb1f - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter31(&self) -> &MHPMCOUNTER31 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2847).cast() }
    }
    #[doc = "0xb80 - Machine-mode cycle counter, high half  
 Counts up once per 1 &lt;&lt; 32 cycles, when `mcountinhibit.cy` is 0. Disabled by default to save power."]
    #[inline(always)]
    pub const fn mcycleh(&self) -> &MCYCLEH {
        unsafe { &*(self as *const Self).cast::<u8>().add(2944).cast() }
    }
    #[doc = "0xb82 - Machine-mode instruction retire counter, low half  
 Counts up once per 1 &lt;&lt; 32 instructions, when `mcountinhibit.ir` is 0. Disabled by default to save power."]
    #[inline(always)]
    pub const fn minstreth(&self) -> &MINSTRETH {
        unsafe { &*(self as *const Self).cast::<u8>().add(2946).cast() }
    }
    #[doc = "0xb83 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter3h(&self) -> &MHPMCOUNTER3H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2947).cast() }
    }
    #[doc = "0xb84 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter4h(&self) -> &MHPMCOUNTER4H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2948).cast() }
    }
    #[doc = "0xb85 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter5h(&self) -> &MHPMCOUNTER5H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2949).cast() }
    }
    #[doc = "0xb86 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter6h(&self) -> &MHPMCOUNTER6H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2950).cast() }
    }
    #[doc = "0xb87 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter7h(&self) -> &MHPMCOUNTER7H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2951).cast() }
    }
    #[doc = "0xb88 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter8h(&self) -> &MHPMCOUNTER8H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2952).cast() }
    }
    #[doc = "0xb89 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter9h(&self) -> &MHPMCOUNTER9H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2953).cast() }
    }
    #[doc = "0xb8a - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter10h(&self) -> &MHPMCOUNTER10H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2954).cast() }
    }
    #[doc = "0xb8b - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter11h(&self) -> &MHPMCOUNTER11H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2955).cast() }
    }
    #[doc = "0xb8c - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter12h(&self) -> &MHPMCOUNTER12H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2956).cast() }
    }
    #[doc = "0xb8d - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter13h(&self) -> &MHPMCOUNTER13H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2957).cast() }
    }
    #[doc = "0xb8e - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter14h(&self) -> &MHPMCOUNTER14H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2958).cast() }
    }
    #[doc = "0xb8f - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter15h(&self) -> &MHPMCOUNTER15H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2959).cast() }
    }
    #[doc = "0xb90 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter16h(&self) -> &MHPMCOUNTER16H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2960).cast() }
    }
    #[doc = "0xb91 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter17h(&self) -> &MHPMCOUNTER17H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2961).cast() }
    }
    #[doc = "0xb92 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter18h(&self) -> &MHPMCOUNTER18H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2962).cast() }
    }
    #[doc = "0xb93 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter19h(&self) -> &MHPMCOUNTER19H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2963).cast() }
    }
    #[doc = "0xb94 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter20h(&self) -> &MHPMCOUNTER20H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2964).cast() }
    }
    #[doc = "0xb95 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter21h(&self) -> &MHPMCOUNTER21H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2965).cast() }
    }
    #[doc = "0xb96 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter22h(&self) -> &MHPMCOUNTER22H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2966).cast() }
    }
    #[doc = "0xb97 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter23h(&self) -> &MHPMCOUNTER23H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2967).cast() }
    }
    #[doc = "0xb98 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter24h(&self) -> &MHPMCOUNTER24H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2968).cast() }
    }
    #[doc = "0xb99 - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter25h(&self) -> &MHPMCOUNTER25H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2969).cast() }
    }
    #[doc = "0xb9a - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter26h(&self) -> &MHPMCOUNTER26H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2970).cast() }
    }
    #[doc = "0xb9b - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter27h(&self) -> &MHPMCOUNTER27H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2971).cast() }
    }
    #[doc = "0xb9c - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter28h(&self) -> &MHPMCOUNTER28H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2972).cast() }
    }
    #[doc = "0xb9d - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter29h(&self) -> &MHPMCOUNTER29H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2973).cast() }
    }
    #[doc = "0xb9e - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter30h(&self) -> &MHPMCOUNTER30H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2974).cast() }
    }
    #[doc = "0xb9f - Extended performance counter, hardwired to 0."]
    #[inline(always)]
    pub const fn mhpmcounter31h(&self) -> &MHPMCOUNTER31H {
        unsafe { &*(self as *const Self).cast::<u8>().add(2975).cast() }
    }
    #[doc = "0xbd0 - PMP M-mode configuration. One bit per PMP region. Setting a bit makes the corresponding region apply to M-mode (like the `pmpcfg.L` bit) but does not lock the region.  

 PMP is useful for non-security-related purposes, such as stack guarding and peripheral emulation. This extension allows M-mode to freely use any currently unlocked regions for its own purposes, without the inconvenience of having to lock them.  

 Note that this does not grant any new capabilities to M-mode, since in the base standard it is already possible to apply unlocked regions to M-mode by locking them. In general, PMP regions should be locked in ascending region number order so they can't be subsequently overridden by currently unlocked regions.  

 Note also that this is not the same as the rule locking bypass bit in the ePMP extension, which does not permit locked and unlocked M-mode regions to coexist.  

 This is a Hazard3 custom CSR."]
    #[inline(always)]
    pub const fn pmpcfgm0(&self) -> &PMPCFGM0 {
        &self.pmpcfgm0
    }
    #[doc = "0xbe0 - External interrupt enable array.  

 The array contains a read-write bit for each external interrupt request: a `1` bit indicates that interrupt is currently enabled. At reset, all external interrupts are disabled.  

 If enabled, an external interrupt can cause assertion of the standard RISC-V machine external interrupt pending flag (`mip.meip`), and therefore cause the processor to enter the external interrupt vector. See `meipa`.  

 There are up to 512 external interrupts. The upper half of this register contains a 16-bit window into the full 512-bit vector. The window is indexed by the 5 LSBs of the write data."]
    #[inline(always)]
    pub const fn meiea(&self) -> &MEIEA {
        unsafe { &*(self as *const Self).cast::<u8>().add(3040).cast() }
    }
    #[doc = "0xbe1 - External interrupt pending array  

 Contains a read-only bit for each external interrupt request. Similarly to `meiea`, this register is a window into an array of up to 512 external interrupt flags. The status appears in the upper 16 bits of the value read from `meipa`, and the lower 5 bits of the value _written_ by the same CSR instruction (or 0 if no write takes place) select a 16-bit window of the full interrupt pending array.  

 A `1` bit indicates that interrupt is currently asserted. IRQs are assumed to be level-sensitive, and the relevant `meipa` bit is cleared by servicing the requestor so that it deasserts its interrupt request.  

 When any interrupt of sufficient priority is both set in `meipa` and enabled in `meiea`, the standard RISC-V external interrupt pending bit `mip.meip` is asserted. In other words, `meipa` is filtered by `meiea` to generate the standard `mip.meip` flag."]
    #[inline(always)]
    pub const fn meipa(&self) -> &MEIPA {
        unsafe { &*(self as *const Self).cast::<u8>().add(3041).cast() }
    }
    #[doc = "0xbe2 - External interrupt force array  

 Contains a read-write bit for every interrupt request. Writing a 1 to a bit in the interrupt force array causes the corresponding bit to become pending in `meipa`. Software can use this feature to manually trigger a particular interrupt.  

 There are no restrictions on using `meifa` inside of an interrupt. The more useful case here is to schedule some lower-priority handler from within a high-priority interrupt, so that it will execute before the core returns to the foreground code. Implementers may wish to reserve some external IRQs with their external inputs tied to 0 for this purpose.  

 Bits can be cleared by software, and are cleared automatically by hardware upon a read of `meinext` which returns the corresponding IRQ number in `meinext.irq` with `mienext.noirq` clear (no matter whether `meinext.update` is written).  

 `meifa` implements the same array window indexing scheme as `meiea` and `meipa`."]
    #[inline(always)]
    pub const fn meifa(&self) -> &MEIFA {
        unsafe { &*(self as *const Self).cast::<u8>().add(3042).cast() }
    }
    #[doc = "0xbe3 - External interrupt priority array  

 Each interrupt has an (up to) 4-bit priority value associated with it, and each access to this register reads and/or writes a 16-bit window containing four such priority values. When less than 16 priority levels are available, the LSBs of the priority fields are hardwired to 0.  

 When an interrupt's priority is lower than the current preemption priority `meicontext.preempt`, it is treated as not being pending for the purposes of `mip.meip`. The pending bit in `meipa` will still assert, but the machine external interrupt pending bit `mip.meip` will not, so the processor will ignore this interrupt. See `meicontext`."]
    #[inline(always)]
    pub const fn meipra(&self) -> &MEIPRA {
        unsafe { &*(self as *const Self).cast::<u8>().add(3043).cast() }
    }
    #[doc = "0xbe4 - Get next external interrupt  

 Contains the index of the highest-priority external interrupt which is both asserted in `meipa` and enabled in `meiea`, left-shifted by 2 so that it can be used to index an array of 32-bit function pointers. If there is no such interrupt, the MSB is set.  

 When multiple interrupts of the same priority are both pending and enabled, the lowest-numbered wins. Interrupts with priority less than `meicontext.ppreempt` -- the _previous_ preemption priority -- are treated as though they are not pending. This is to ensure that a preempting interrupt frame does not service interrupts which may be in progress in the frame that was preempted."]
    #[inline(always)]
    pub const fn meinext(&self) -> &MEINEXT {
        unsafe { &*(self as *const Self).cast::<u8>().add(3044).cast() }
    }
    #[doc = "0xbe5 - External interrupt context register  

 Configures the priority level for interrupt preemption, and helps software track which interrupt it is currently in. The latter is useful when a common interrupt service routine handles interrupt requests from multiple instances of the same peripheral.  

 A three-level stack of preemption priorities is maintained in the `preempt`, `ppreempt` and `pppreempt` fields. The priority stack is saved when hardware enters the external interrupt vector, and restored by an `mret` instruction if `meicontext.mreteirq` is set.  

 The top entry of the priority stack, `preempt`, is used by hardware to ensure that only higher-priority interrupts can preempt the current interrupt. The next entry, `ppreempt`, is used to avoid servicing interrupts which may already be in progress in a frame that was preempted. The third entry, `pppreempt`, has no hardware effect, but ensures that `preempt` and `ppreempt` can be correctly saved/restored across arbitary levels of preemption."]
    #[inline(always)]
    pub const fn meicontext(&self) -> &MEICONTEXT {
        unsafe { &*(self as *const Self).cast::<u8>().add(3045).cast() }
    }
    #[doc = "0xbf0 - M-mode sleep control register"]
    #[inline(always)]
    pub const fn msleep(&self) -> &MSLEEP {
        &self.msleep
    }
    #[doc = "0xbff - The Debug Module's DATA0 register is mapped into Hazard3's CSR space so that the Debug Module can exchange data with the core by executing CSR access instructions (this is used to implement the Abstract Access Register command). Only accessible in Debug Mode."]
    #[inline(always)]
    pub const fn dmdata0(&self) -> &DMDATA0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3071).cast() }
    }
    #[doc = "0xc00 - Read-only U-mode alias of mcycle, accessible when `mcounteren.cy` is set"]
    #[inline(always)]
    pub const fn cycle(&self) -> &CYCLE {
        unsafe { &*(self as *const Self).cast::<u8>().add(3072).cast() }
    }
    #[doc = "0xc02 - Read-only U-mode alias of minstret, accessible when `mcounteren.ir` is set"]
    #[inline(always)]
    pub const fn instret(&self) -> &INSTRET {
        unsafe { &*(self as *const Self).cast::<u8>().add(3074).cast() }
    }
    #[doc = "0xc80 - Read-only U-mode alias of mcycleh, accessible when `mcounteren.cy` is set"]
    #[inline(always)]
    pub const fn cycleh(&self) -> &CYCLEH {
        unsafe { &*(self as *const Self).cast::<u8>().add(3200).cast() }
    }
    #[doc = "0xc82 - Read-only U-mode alias of minstreth, accessible when `mcounteren.ir` is set"]
    #[inline(always)]
    pub const fn instreth(&self) -> &INSTRETH {
        unsafe { &*(self as *const Self).cast::<u8>().add(3202).cast() }
    }
    #[doc = "0xf11 - Vendor ID"]
    #[inline(always)]
    pub const fn mvendorid(&self) -> &MVENDORID {
        unsafe { &*(self as *const Self).cast::<u8>().add(3857).cast() }
    }
    #[doc = "0xf12 - Architecture ID (Hazard3)"]
    #[inline(always)]
    pub const fn marchid(&self) -> &MARCHID {
        unsafe { &*(self as *const Self).cast::<u8>().add(3858).cast() }
    }
    #[doc = "0xf13 - Implementation ID"]
    #[inline(always)]
    pub const fn mimpid(&self) -> &MIMPID {
        unsafe { &*(self as *const Self).cast::<u8>().add(3859).cast() }
    }
    #[doc = "0xf14 - Hardware thread ID  
 On RP2350, core 0 has a hart ID of 0, and core 1 has a hart ID of 1."]
    #[inline(always)]
    pub const fn mhartid(&self) -> &MHARTID {
        unsafe { &*(self as *const Self).cast::<u8>().add(3860).cast() }
    }
    #[doc = "0xf15 - Pointer to configuration data structure (hardwired to 0)"]
    #[inline(always)]
    pub const fn mconfigptr(&self) -> &MCONFIGPTR {
        unsafe { &*(self as *const Self).cast::<u8>().add(3861).cast() }
    }
}
#[doc = "MSTATUS (rw) register accessor: Machine status register  

You can [`read`](crate::Reg::read) this register and get [`mstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mstatus`]
module"]
pub type MSTATUS = crate::Reg<mstatus::MSTATUS_SPEC>;
#[doc = "Machine status register"]
pub mod mstatus;
#[doc = "MISA (r) register accessor: Summary of ISA extension support  

 On RP2350, Hazard3's full `-march` string is: `rv32ima_zicsr_zifencei_zba_zbb_zbs_zbkb_zca_zcb_zcmp`  

 Note Zca is equivalent to the C extension in this case; all instructions from the RISC-V C extension relevant to a 32-bit non-floating-point processor are supported. On older toolchains which do not support the Zc extensions, the appropriate `-march` string is: `rv32imac_zicsr_zifencei_zba_zbb_zbs_zbkb`  

 In addition the following custom extensions are configured: Xh3bm, Xh3power, Xh3irq, Xh3pmpm  

You can [`read`](crate::Reg::read) this register and get [`misa::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@misa`]
module"]
pub type MISA = crate::Reg<misa::MISA_SPEC>;
#[doc = "Summary of ISA extension support  

 On RP2350, Hazard3's full `-march` string is: `rv32ima_zicsr_zifencei_zba_zbb_zbs_zbkb_zca_zcb_zcmp`  

 Note Zca is equivalent to the C extension in this case; all instructions from the RISC-V C extension relevant to a 32-bit non-floating-point processor are supported. On older toolchains which do not support the Zc extensions, the appropriate `-march` string is: `rv32imac_zicsr_zifencei_zba_zbb_zbs_zbkb`  

 In addition the following custom extensions are configured: Xh3bm, Xh3power, Xh3irq, Xh3pmpm"]
pub mod misa;
#[doc = "MEDELEG (rw) register accessor: Machine exception delegation register. Not implemented, as no S-mode support.  

You can [`read`](crate::Reg::read) this register and get [`medeleg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`medeleg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@medeleg`]
module"]
pub type MEDELEG = crate::Reg<medeleg::MEDELEG_SPEC>;
#[doc = "Machine exception delegation register. Not implemented, as no S-mode support."]
pub mod medeleg;
#[doc = "MIDELEG (rw) register accessor: Machine interrupt delegation register. Not implemented, as no S-mode support.  

You can [`read`](crate::Reg::read) this register and get [`mideleg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mideleg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mideleg`]
module"]
pub type MIDELEG = crate::Reg<mideleg::MIDELEG_SPEC>;
#[doc = "Machine interrupt delegation register. Not implemented, as no S-mode support."]
pub mod mideleg;
#[doc = "MIE (rw) register accessor: Machine interrupt enable register  

You can [`read`](crate::Reg::read) this register and get [`mie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mie`]
module"]
pub type MIE = crate::Reg<mie::MIE_SPEC>;
#[doc = "Machine interrupt enable register"]
pub mod mie;
#[doc = "MTVEC (rw) register accessor: Machine trap handler base address.  

You can [`read`](crate::Reg::read) this register and get [`mtvec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtvec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mtvec`]
module"]
pub type MTVEC = crate::Reg<mtvec::MTVEC_SPEC>;
#[doc = "Machine trap handler base address."]
pub mod mtvec;
#[doc = "MCOUNTEREN (rw) register accessor: Counter enable. Control access to counters from U-mode. Not to be confused with mcountinhibit.  

You can [`read`](crate::Reg::read) this register and get [`mcounteren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcounteren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mcounteren`]
module"]
pub type MCOUNTEREN = crate::Reg<mcounteren::MCOUNTEREN_SPEC>;
#[doc = "Counter enable. Control access to counters from U-mode. Not to be confused with mcountinhibit."]
pub mod mcounteren;
#[doc = "MENVCFG (r) register accessor: Machine environment configuration register, low half  

You can [`read`](crate::Reg::read) this register and get [`menvcfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@menvcfg`]
module"]
pub type MENVCFG = crate::Reg<menvcfg::MENVCFG_SPEC>;
#[doc = "Machine environment configuration register, low half"]
pub mod menvcfg;
#[doc = "MSTATUSH (r) register accessor: High half of mstatus, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mstatush::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mstatush`]
module"]
pub type MSTATUSH = crate::Reg<mstatush::MSTATUSH_SPEC>;
#[doc = "High half of mstatus, hardwired to 0."]
pub mod mstatush;
#[doc = "MENVCFGH (rw) register accessor: Machine environment configuration register, high half  

 This register is fully reserved, as Hazard3 does not implement the relevant extensions. It is implemented as hardwired-0.  

You can [`read`](crate::Reg::read) this register and get [`menvcfgh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`menvcfgh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@menvcfgh`]
module"]
pub type MENVCFGH = crate::Reg<menvcfgh::MENVCFGH_SPEC>;
#[doc = "Machine environment configuration register, high half  

 This register is fully reserved, as Hazard3 does not implement the relevant extensions. It is implemented as hardwired-0."]
pub mod menvcfgh;
#[doc = "MCOUNTINHIBIT (rw) register accessor: Count inhibit register for `mcycle`/`minstret`  

You can [`read`](crate::Reg::read) this register and get [`mcountinhibit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcountinhibit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mcountinhibit`]
module"]
pub type MCOUNTINHIBIT = crate::Reg<mcountinhibit::MCOUNTINHIBIT_SPEC>;
#[doc = "Count inhibit register for `mcycle`/`minstret`"]
pub mod mcountinhibit;
#[doc = "MHPMEVENT3 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent3`]
module"]
pub type MHPMEVENT3 = crate::Reg<mhpmevent3::MHPMEVENT3_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent3;
#[doc = "MHPMEVENT4 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent4`]
module"]
pub type MHPMEVENT4 = crate::Reg<mhpmevent4::MHPMEVENT4_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent4;
#[doc = "MHPMEVENT5 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent5`]
module"]
pub type MHPMEVENT5 = crate::Reg<mhpmevent5::MHPMEVENT5_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent5;
#[doc = "MHPMEVENT6 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent6`]
module"]
pub type MHPMEVENT6 = crate::Reg<mhpmevent6::MHPMEVENT6_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent6;
#[doc = "MHPMEVENT7 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent7`]
module"]
pub type MHPMEVENT7 = crate::Reg<mhpmevent7::MHPMEVENT7_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent7;
#[doc = "MHPMEVENT8 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent8`]
module"]
pub type MHPMEVENT8 = crate::Reg<mhpmevent8::MHPMEVENT8_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent8;
#[doc = "MHPMEVENT9 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent9`]
module"]
pub type MHPMEVENT9 = crate::Reg<mhpmevent9::MHPMEVENT9_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent9;
#[doc = "MHPMEVENT10 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent10`]
module"]
pub type MHPMEVENT10 = crate::Reg<mhpmevent10::MHPMEVENT10_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent10;
#[doc = "MHPMEVENT11 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent11`]
module"]
pub type MHPMEVENT11 = crate::Reg<mhpmevent11::MHPMEVENT11_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent11;
#[doc = "MHPMEVENT12 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent12`]
module"]
pub type MHPMEVENT12 = crate::Reg<mhpmevent12::MHPMEVENT12_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent12;
#[doc = "MHPMEVENT13 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent13`]
module"]
pub type MHPMEVENT13 = crate::Reg<mhpmevent13::MHPMEVENT13_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent13;
#[doc = "MHPMEVENT14 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent14`]
module"]
pub type MHPMEVENT14 = crate::Reg<mhpmevent14::MHPMEVENT14_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent14;
#[doc = "MHPMEVENT15 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent15`]
module"]
pub type MHPMEVENT15 = crate::Reg<mhpmevent15::MHPMEVENT15_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent15;
#[doc = "MHPMEVENT16 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent16`]
module"]
pub type MHPMEVENT16 = crate::Reg<mhpmevent16::MHPMEVENT16_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent16;
#[doc = "MHPMEVENT17 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent17`]
module"]
pub type MHPMEVENT17 = crate::Reg<mhpmevent17::MHPMEVENT17_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent17;
#[doc = "MHPMEVENT18 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent18`]
module"]
pub type MHPMEVENT18 = crate::Reg<mhpmevent18::MHPMEVENT18_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent18;
#[doc = "MHPMEVENT19 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent19`]
module"]
pub type MHPMEVENT19 = crate::Reg<mhpmevent19::MHPMEVENT19_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent19;
#[doc = "MHPMEVENT20 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent20`]
module"]
pub type MHPMEVENT20 = crate::Reg<mhpmevent20::MHPMEVENT20_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent20;
#[doc = "MHPMEVENT21 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent21`]
module"]
pub type MHPMEVENT21 = crate::Reg<mhpmevent21::MHPMEVENT21_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent21;
#[doc = "MHPMEVENT22 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent22`]
module"]
pub type MHPMEVENT22 = crate::Reg<mhpmevent22::MHPMEVENT22_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent22;
#[doc = "MHPMEVENT23 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent23`]
module"]
pub type MHPMEVENT23 = crate::Reg<mhpmevent23::MHPMEVENT23_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent23;
#[doc = "MHPMEVENT24 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent24`]
module"]
pub type MHPMEVENT24 = crate::Reg<mhpmevent24::MHPMEVENT24_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent24;
#[doc = "MHPMEVENT25 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent25`]
module"]
pub type MHPMEVENT25 = crate::Reg<mhpmevent25::MHPMEVENT25_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent25;
#[doc = "MHPMEVENT26 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent26`]
module"]
pub type MHPMEVENT26 = crate::Reg<mhpmevent26::MHPMEVENT26_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent26;
#[doc = "MHPMEVENT27 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent27`]
module"]
pub type MHPMEVENT27 = crate::Reg<mhpmevent27::MHPMEVENT27_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent27;
#[doc = "MHPMEVENT28 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent28`]
module"]
pub type MHPMEVENT28 = crate::Reg<mhpmevent28::MHPMEVENT28_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent28;
#[doc = "MHPMEVENT29 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent29`]
module"]
pub type MHPMEVENT29 = crate::Reg<mhpmevent29::MHPMEVENT29_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent29;
#[doc = "MHPMEVENT30 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent30`]
module"]
pub type MHPMEVENT30 = crate::Reg<mhpmevent30::MHPMEVENT30_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent30;
#[doc = "MHPMEVENT31 (r) register accessor: Extended performance event selector, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmevent31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmevent31`]
module"]
pub type MHPMEVENT31 = crate::Reg<mhpmevent31::MHPMEVENT31_SPEC>;
#[doc = "Extended performance event selector, hardwired to 0."]
pub mod mhpmevent31;
#[doc = "MSCRATCH (rw) register accessor: Scratch register for machine trap handlers.  

 32-bit read/write register with no specific hardware function. Software may use this to do a fast save/restore of a core register in a trap handler.  

You can [`read`](crate::Reg::read) this register and get [`mscratch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mscratch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mscratch`]
module"]
pub type MSCRATCH = crate::Reg<mscratch::MSCRATCH_SPEC>;
#[doc = "Scratch register for machine trap handlers.  

 32-bit read/write register with no specific hardware function. Software may use this to do a fast save/restore of a core register in a trap handler."]
pub mod mscratch;
#[doc = "MEPC (rw) register accessor: Machine exception program counter.  

 When entering a trap, the current value of the program counter is recorded here. When executing an `mret`, the processor jumps to `mepc`. Can also be read and written by software.  

You can [`read`](crate::Reg::read) this register and get [`mepc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mepc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mepc`]
module"]
pub type MEPC = crate::Reg<mepc::MEPC_SPEC>;
#[doc = "Machine exception program counter.  

 When entering a trap, the current value of the program counter is recorded here. When executing an `mret`, the processor jumps to `mepc`. Can also be read and written by software."]
pub mod mepc;
#[doc = "MCAUSE (rw) register accessor: Machine trap cause. Set when entering a trap to indicate the reason for the trap. Readable and writable by software.  

You can [`read`](crate::Reg::read) this register and get [`mcause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mcause`]
module"]
pub type MCAUSE = crate::Reg<mcause::MCAUSE_SPEC>;
#[doc = "Machine trap cause. Set when entering a trap to indicate the reason for the trap. Readable and writable by software."]
pub mod mcause;
#[doc = "MTVAL (r) register accessor: Machine bad address or instruction. Hardwired to zero.  

You can [`read`](crate::Reg::read) this register and get [`mtval::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mtval`]
module"]
pub type MTVAL = crate::Reg<mtval::MTVAL_SPEC>;
#[doc = "Machine bad address or instruction. Hardwired to zero."]
pub mod mtval;
#[doc = "MIP (rw) register accessor: Machine interrupt pending  

You can [`read`](crate::Reg::read) this register and get [`mip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mip`]
module"]
pub type MIP = crate::Reg<mip::MIP_SPEC>;
#[doc = "Machine interrupt pending"]
pub mod mip;
#[doc = "PMPCFG0 (rw) register accessor: Physical memory protection configuration for regions 0 through 3  

You can [`read`](crate::Reg::read) this register and get [`pmpcfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpcfg0`]
module"]
pub type PMPCFG0 = crate::Reg<pmpcfg0::PMPCFG0_SPEC>;
#[doc = "Physical memory protection configuration for regions 0 through 3"]
pub mod pmpcfg0;
#[doc = "PMPCFG1 (rw) register accessor: Physical memory protection configuration for regions 4 through 7  

You can [`read`](crate::Reg::read) this register and get [`pmpcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpcfg1`]
module"]
pub type PMPCFG1 = crate::Reg<pmpcfg1::PMPCFG1_SPEC>;
#[doc = "Physical memory protection configuration for regions 4 through 7"]
pub mod pmpcfg1;
#[doc = "PMPCFG2 (r) register accessor: Physical memory protection configuration for regions 8 through 11  

You can [`read`](crate::Reg::read) this register and get [`pmpcfg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpcfg2`]
module"]
pub type PMPCFG2 = crate::Reg<pmpcfg2::PMPCFG2_SPEC>;
#[doc = "Physical memory protection configuration for regions 8 through 11"]
pub mod pmpcfg2;
#[doc = "PMPCFG3 (r) register accessor: Physical memory protection configuration for regions 12 through 15  

You can [`read`](crate::Reg::read) this register and get [`pmpcfg3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpcfg3`]
module"]
pub type PMPCFG3 = crate::Reg<pmpcfg3::PMPCFG3_SPEC>;
#[doc = "Physical memory protection configuration for regions 12 through 15"]
pub mod pmpcfg3;
#[doc = "PMPADDR0 (rw) register accessor: Physical memory protection address for region 0. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr0`]
module"]
pub type PMPADDR0 = crate::Reg<pmpaddr0::PMPADDR0_SPEC>;
#[doc = "Physical memory protection address for region 0. Note all PMP addresses are in units of four bytes."]
pub mod pmpaddr0;
#[doc = "PMPADDR1 (rw) register accessor: Physical memory protection address for region 1. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr1`]
module"]
pub type PMPADDR1 = crate::Reg<pmpaddr1::PMPADDR1_SPEC>;
#[doc = "Physical memory protection address for region 1. Note all PMP addresses are in units of four bytes."]
pub mod pmpaddr1;
#[doc = "PMPADDR2 (rw) register accessor: Physical memory protection address for region 2. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr2`]
module"]
pub type PMPADDR2 = crate::Reg<pmpaddr2::PMPADDR2_SPEC>;
#[doc = "Physical memory protection address for region 2. Note all PMP addresses are in units of four bytes."]
pub mod pmpaddr2;
#[doc = "PMPADDR3 (rw) register accessor: Physical memory protection address for region 3. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr3`]
module"]
pub type PMPADDR3 = crate::Reg<pmpaddr3::PMPADDR3_SPEC>;
#[doc = "Physical memory protection address for region 3. Note all PMP addresses are in units of four bytes."]
pub mod pmpaddr3;
#[doc = "PMPADDR4 (rw) register accessor: Physical memory protection address for region 4. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr4`]
module"]
pub type PMPADDR4 = crate::Reg<pmpaddr4::PMPADDR4_SPEC>;
#[doc = "Physical memory protection address for region 4. Note all PMP addresses are in units of four bytes."]
pub mod pmpaddr4;
#[doc = "PMPADDR5 (rw) register accessor: Physical memory protection address for region 5. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr5`]
module"]
pub type PMPADDR5 = crate::Reg<pmpaddr5::PMPADDR5_SPEC>;
#[doc = "Physical memory protection address for region 5. Note all PMP addresses are in units of four bytes."]
pub mod pmpaddr5;
#[doc = "PMPADDR6 (rw) register accessor: Physical memory protection address for region 6. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr6`]
module"]
pub type PMPADDR6 = crate::Reg<pmpaddr6::PMPADDR6_SPEC>;
#[doc = "Physical memory protection address for region 6. Note all PMP addresses are in units of four bytes."]
pub mod pmpaddr6;
#[doc = "PMPADDR7 (rw) register accessor: Physical memory protection address for region 7. Note all PMP addresses are in units of four bytes.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpaddr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr7`]
module"]
pub type PMPADDR7 = crate::Reg<pmpaddr7::PMPADDR7_SPEC>;
#[doc = "Physical memory protection address for region 7. Note all PMP addresses are in units of four bytes."]
pub mod pmpaddr7;
#[doc = "PMPADDR8 (r) register accessor: Physical memory protection address for region 8. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0x00000000` through `0x0fffffff`, which contains the boot ROM. This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr8`]
module"]
pub type PMPADDR8 = crate::Reg<pmpaddr8::PMPADDR8_SPEC>;
#[doc = "Physical memory protection address for region 8. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0x00000000` through `0x0fffffff`, which contains the boot ROM. This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL."]
pub mod pmpaddr8;
#[doc = "PMPADDR9 (r) register accessor: Physical memory protection address for region 9. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0x40000000` through `0x5fffffff`, which contains the system peripherals. This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr9`]
module"]
pub type PMPADDR9 = crate::Reg<pmpaddr9::PMPADDR9_SPEC>;
#[doc = "Physical memory protection address for region 9. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0x40000000` through `0x5fffffff`, which contains the system peripherals. This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL."]
pub mod pmpaddr9;
#[doc = "PMPADDR10 (r) register accessor: Physical memory protection address for region 10. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0xd0000000` through `0xdfffffff`, which contains the core-local peripherals (SIO). This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr10`]
module"]
pub type PMPADDR10 = crate::Reg<pmpaddr10::PMPADDR10_SPEC>;
#[doc = "Physical memory protection address for region 10. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0xd0000000` through `0xdfffffff`, which contains the core-local peripherals (SIO). This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL."]
pub mod pmpaddr10;
#[doc = "PMPADDR11 (r) register accessor: Physical memory protection address for region 11. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr11`]
module"]
pub type PMPADDR11 = crate::Reg<pmpaddr11::PMPADDR11_SPEC>;
#[doc = "Physical memory protection address for region 11. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented."]
pub mod pmpaddr11;
#[doc = "PMPADDR12 (r) register accessor: Physical memory protection address for region 12. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr12`]
module"]
pub type PMPADDR12 = crate::Reg<pmpaddr12::PMPADDR12_SPEC>;
#[doc = "Physical memory protection address for region 12. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented."]
pub mod pmpaddr12;
#[doc = "PMPADDR13 (r) register accessor: Physical memory protection address for region 13. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr13`]
module"]
pub type PMPADDR13 = crate::Reg<pmpaddr13::PMPADDR13_SPEC>;
#[doc = "Physical memory protection address for region 13. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented."]
pub mod pmpaddr13;
#[doc = "PMPADDR14 (r) register accessor: Physical memory protection address for region 14. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr14`]
module"]
pub type PMPADDR14 = crate::Reg<pmpaddr14::PMPADDR14_SPEC>;
#[doc = "Physical memory protection address for region 14. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented."]
pub mod pmpaddr14;
#[doc = "PMPADDR15 (r) register accessor: Physical memory protection address for region 15. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpaddr15`]
module"]
pub type PMPADDR15 = crate::Reg<pmpaddr15::PMPADDR15_SPEC>;
#[doc = "Physical memory protection address for region 15. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented."]
pub mod pmpaddr15;
#[doc = "TSELECT (rw) register accessor: Select trigger to be configured via `tdata1`/`tdata2`  

 On RP2350, four instruction address triggers are implemented, so only the two LSBs of this register are writable.  

You can [`read`](crate::Reg::read) this register and get [`tselect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tselect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tselect`]
module"]
pub type TSELECT = crate::Reg<tselect::TSELECT_SPEC>;
#[doc = "Select trigger to be configured via `tdata1`/`tdata2`  

 On RP2350, four instruction address triggers are implemented, so only the two LSBs of this register are writable."]
pub mod tselect;
#[doc = "TDATA1 (rw) register accessor: Trigger configuration data 1  

 Hazard 3 only supports address/data match triggers (type=2) so this register description includes the `mcontrol` fields for this type.  

 More precisely, Hazard3 only supports exact instruction address match triggers (hardware breakpoints) so many of this register's fields are hardwired.  

You can [`read`](crate::Reg::read) this register and get [`tdata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tdata1`]
module"]
pub type TDATA1 = crate::Reg<tdata1::TDATA1_SPEC>;
#[doc = "Trigger configuration data 1  

 Hazard 3 only supports address/data match triggers (type=2) so this register description includes the `mcontrol` fields for this type.  

 More precisely, Hazard3 only supports exact instruction address match triggers (hardware breakpoints) so many of this register's fields are hardwired."]
pub mod tdata1;
#[doc = "TDATA2 (rw) register accessor: Trigger configuration data 2  

 Contains the address for instruction address triggers (hardware breakpoints)  

You can [`read`](crate::Reg::read) this register and get [`tdata2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdata2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@tdata2`]
module"]
pub type TDATA2 = crate::Reg<tdata2::TDATA2_SPEC>;
#[doc = "Trigger configuration data 2  

 Contains the address for instruction address triggers (hardware breakpoints)"]
pub mod tdata2;
#[doc = "DCSR (rw) register accessor: Debug control and status register. Access outside of Debug Mode will cause an illegal instruction exception.  

You can [`read`](crate::Reg::read) this register and get [`dcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dcsr`]
module"]
pub type DCSR = crate::Reg<dcsr::DCSR_SPEC>;
#[doc = "Debug control and status register. Access outside of Debug Mode will cause an illegal instruction exception."]
pub mod dcsr;
#[doc = "DPC (rw) register accessor: Debug program counter. When entering Debug Mode, `dpc` samples the current program counter, e.g. the address of an `ebreak` which caused Debug Mode entry. When leaving debug mode, the processor jumps to `dpc`. The host may read/write this register whilst in Debug Mode.  

You can [`read`](crate::Reg::read) this register and get [`dpc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dpc`]
module"]
pub type DPC = crate::Reg<dpc::DPC_SPEC>;
#[doc = "Debug program counter. When entering Debug Mode, `dpc` samples the current program counter, e.g. the address of an `ebreak` which caused Debug Mode entry. When leaving debug mode, the processor jumps to `dpc`. The host may read/write this register whilst in Debug Mode."]
pub mod dpc;
#[doc = "MCYCLE (rw) register accessor: Machine-mode cycle counter, low half  
 Counts up once per cycle, when `mcountinhibit.cy` is 0. Disabled by default to save power.  

You can [`read`](crate::Reg::read) this register and get [`mcycle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcycle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mcycle`]
module"]
pub type MCYCLE = crate::Reg<mcycle::MCYCLE_SPEC>;
#[doc = "Machine-mode cycle counter, low half  
 Counts up once per cycle, when `mcountinhibit.cy` is 0. Disabled by default to save power."]
pub mod mcycle;
#[doc = "MINSTRET (rw) register accessor: Machine-mode instruction retire counter, low half  
 Counts up once per instruction, when `mcountinhibit.ir` is 0. Disabled by default to save power.  

You can [`read`](crate::Reg::read) this register and get [`minstret::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`minstret::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@minstret`]
module"]
pub type MINSTRET = crate::Reg<minstret::MINSTRET_SPEC>;
#[doc = "Machine-mode instruction retire counter, low half  
 Counts up once per instruction, when `mcountinhibit.ir` is 0. Disabled by default to save power."]
pub mod minstret;
#[doc = "MHPMCOUNTER3 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter3`]
module"]
pub type MHPMCOUNTER3 = crate::Reg<mhpmcounter3::MHPMCOUNTER3_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter3;
#[doc = "MHPMCOUNTER4 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter4`]
module"]
pub type MHPMCOUNTER4 = crate::Reg<mhpmcounter4::MHPMCOUNTER4_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter4;
#[doc = "MHPMCOUNTER5 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter5`]
module"]
pub type MHPMCOUNTER5 = crate::Reg<mhpmcounter5::MHPMCOUNTER5_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter5;
#[doc = "MHPMCOUNTER6 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter6`]
module"]
pub type MHPMCOUNTER6 = crate::Reg<mhpmcounter6::MHPMCOUNTER6_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter6;
#[doc = "MHPMCOUNTER7 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter7`]
module"]
pub type MHPMCOUNTER7 = crate::Reg<mhpmcounter7::MHPMCOUNTER7_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter7;
#[doc = "MHPMCOUNTER8 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter8`]
module"]
pub type MHPMCOUNTER8 = crate::Reg<mhpmcounter8::MHPMCOUNTER8_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter8;
#[doc = "MHPMCOUNTER9 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter9`]
module"]
pub type MHPMCOUNTER9 = crate::Reg<mhpmcounter9::MHPMCOUNTER9_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter9;
#[doc = "MHPMCOUNTER10 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter10`]
module"]
pub type MHPMCOUNTER10 = crate::Reg<mhpmcounter10::MHPMCOUNTER10_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter10;
#[doc = "MHPMCOUNTER11 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter11`]
module"]
pub type MHPMCOUNTER11 = crate::Reg<mhpmcounter11::MHPMCOUNTER11_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter11;
#[doc = "MHPMCOUNTER12 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter12`]
module"]
pub type MHPMCOUNTER12 = crate::Reg<mhpmcounter12::MHPMCOUNTER12_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter12;
#[doc = "MHPMCOUNTER13 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter13`]
module"]
pub type MHPMCOUNTER13 = crate::Reg<mhpmcounter13::MHPMCOUNTER13_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter13;
#[doc = "MHPMCOUNTER14 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter14`]
module"]
pub type MHPMCOUNTER14 = crate::Reg<mhpmcounter14::MHPMCOUNTER14_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter14;
#[doc = "MHPMCOUNTER15 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter15`]
module"]
pub type MHPMCOUNTER15 = crate::Reg<mhpmcounter15::MHPMCOUNTER15_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter15;
#[doc = "MHPMCOUNTER16 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter16`]
module"]
pub type MHPMCOUNTER16 = crate::Reg<mhpmcounter16::MHPMCOUNTER16_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter16;
#[doc = "MHPMCOUNTER17 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter17`]
module"]
pub type MHPMCOUNTER17 = crate::Reg<mhpmcounter17::MHPMCOUNTER17_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter17;
#[doc = "MHPMCOUNTER18 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter18`]
module"]
pub type MHPMCOUNTER18 = crate::Reg<mhpmcounter18::MHPMCOUNTER18_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter18;
#[doc = "MHPMCOUNTER19 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter19`]
module"]
pub type MHPMCOUNTER19 = crate::Reg<mhpmcounter19::MHPMCOUNTER19_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter19;
#[doc = "MHPMCOUNTER20 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter20`]
module"]
pub type MHPMCOUNTER20 = crate::Reg<mhpmcounter20::MHPMCOUNTER20_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter20;
#[doc = "MHPMCOUNTER21 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter21`]
module"]
pub type MHPMCOUNTER21 = crate::Reg<mhpmcounter21::MHPMCOUNTER21_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter21;
#[doc = "MHPMCOUNTER22 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter22`]
module"]
pub type MHPMCOUNTER22 = crate::Reg<mhpmcounter22::MHPMCOUNTER22_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter22;
#[doc = "MHPMCOUNTER23 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter23`]
module"]
pub type MHPMCOUNTER23 = crate::Reg<mhpmcounter23::MHPMCOUNTER23_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter23;
#[doc = "MHPMCOUNTER24 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter24`]
module"]
pub type MHPMCOUNTER24 = crate::Reg<mhpmcounter24::MHPMCOUNTER24_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter24;
#[doc = "MHPMCOUNTER25 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter25`]
module"]
pub type MHPMCOUNTER25 = crate::Reg<mhpmcounter25::MHPMCOUNTER25_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter25;
#[doc = "MHPMCOUNTER26 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter26`]
module"]
pub type MHPMCOUNTER26 = crate::Reg<mhpmcounter26::MHPMCOUNTER26_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter26;
#[doc = "MHPMCOUNTER27 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter27`]
module"]
pub type MHPMCOUNTER27 = crate::Reg<mhpmcounter27::MHPMCOUNTER27_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter27;
#[doc = "MHPMCOUNTER28 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter28`]
module"]
pub type MHPMCOUNTER28 = crate::Reg<mhpmcounter28::MHPMCOUNTER28_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter28;
#[doc = "MHPMCOUNTER29 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter29`]
module"]
pub type MHPMCOUNTER29 = crate::Reg<mhpmcounter29::MHPMCOUNTER29_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter29;
#[doc = "MHPMCOUNTER30 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter30`]
module"]
pub type MHPMCOUNTER30 = crate::Reg<mhpmcounter30::MHPMCOUNTER30_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter30;
#[doc = "MHPMCOUNTER31 (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter31`]
module"]
pub type MHPMCOUNTER31 = crate::Reg<mhpmcounter31::MHPMCOUNTER31_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter31;
#[doc = "MCYCLEH (rw) register accessor: Machine-mode cycle counter, high half  
 Counts up once per 1 &lt;&lt; 32 cycles, when `mcountinhibit.cy` is 0. Disabled by default to save power.  

You can [`read`](crate::Reg::read) this register and get [`mcycleh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcycleh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mcycleh`]
module"]
pub type MCYCLEH = crate::Reg<mcycleh::MCYCLEH_SPEC>;
#[doc = "Machine-mode cycle counter, high half  
 Counts up once per 1 &lt;&lt; 32 cycles, when `mcountinhibit.cy` is 0. Disabled by default to save power."]
pub mod mcycleh;
#[doc = "MINSTRETH (rw) register accessor: Machine-mode instruction retire counter, low half  
 Counts up once per 1 &lt;&lt; 32 instructions, when `mcountinhibit.ir` is 0. Disabled by default to save power.  

You can [`read`](crate::Reg::read) this register and get [`minstreth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`minstreth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@minstreth`]
module"]
pub type MINSTRETH = crate::Reg<minstreth::MINSTRETH_SPEC>;
#[doc = "Machine-mode instruction retire counter, low half  
 Counts up once per 1 &lt;&lt; 32 instructions, when `mcountinhibit.ir` is 0. Disabled by default to save power."]
pub mod minstreth;
#[doc = "MHPMCOUNTER3H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter3h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter3h`]
module"]
pub type MHPMCOUNTER3H = crate::Reg<mhpmcounter3h::MHPMCOUNTER3H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter3h;
#[doc = "MHPMCOUNTER4H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter4h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter4h`]
module"]
pub type MHPMCOUNTER4H = crate::Reg<mhpmcounter4h::MHPMCOUNTER4H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter4h;
#[doc = "MHPMCOUNTER5H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter5h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter5h`]
module"]
pub type MHPMCOUNTER5H = crate::Reg<mhpmcounter5h::MHPMCOUNTER5H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter5h;
#[doc = "MHPMCOUNTER6H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter6h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter6h`]
module"]
pub type MHPMCOUNTER6H = crate::Reg<mhpmcounter6h::MHPMCOUNTER6H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter6h;
#[doc = "MHPMCOUNTER7H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter7h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter7h`]
module"]
pub type MHPMCOUNTER7H = crate::Reg<mhpmcounter7h::MHPMCOUNTER7H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter7h;
#[doc = "MHPMCOUNTER8H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter8h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter8h`]
module"]
pub type MHPMCOUNTER8H = crate::Reg<mhpmcounter8h::MHPMCOUNTER8H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter8h;
#[doc = "MHPMCOUNTER9H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter9h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter9h`]
module"]
pub type MHPMCOUNTER9H = crate::Reg<mhpmcounter9h::MHPMCOUNTER9H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter9h;
#[doc = "MHPMCOUNTER10H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter10h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter10h`]
module"]
pub type MHPMCOUNTER10H = crate::Reg<mhpmcounter10h::MHPMCOUNTER10H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter10h;
#[doc = "MHPMCOUNTER11H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter11h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter11h`]
module"]
pub type MHPMCOUNTER11H = crate::Reg<mhpmcounter11h::MHPMCOUNTER11H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter11h;
#[doc = "MHPMCOUNTER12H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter12h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter12h`]
module"]
pub type MHPMCOUNTER12H = crate::Reg<mhpmcounter12h::MHPMCOUNTER12H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter12h;
#[doc = "MHPMCOUNTER13H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter13h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter13h`]
module"]
pub type MHPMCOUNTER13H = crate::Reg<mhpmcounter13h::MHPMCOUNTER13H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter13h;
#[doc = "MHPMCOUNTER14H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter14h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter14h`]
module"]
pub type MHPMCOUNTER14H = crate::Reg<mhpmcounter14h::MHPMCOUNTER14H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter14h;
#[doc = "MHPMCOUNTER15H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter15h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter15h`]
module"]
pub type MHPMCOUNTER15H = crate::Reg<mhpmcounter15h::MHPMCOUNTER15H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter15h;
#[doc = "MHPMCOUNTER16H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter16h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter16h`]
module"]
pub type MHPMCOUNTER16H = crate::Reg<mhpmcounter16h::MHPMCOUNTER16H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter16h;
#[doc = "MHPMCOUNTER17H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter17h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter17h`]
module"]
pub type MHPMCOUNTER17H = crate::Reg<mhpmcounter17h::MHPMCOUNTER17H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter17h;
#[doc = "MHPMCOUNTER18H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter18h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter18h`]
module"]
pub type MHPMCOUNTER18H = crate::Reg<mhpmcounter18h::MHPMCOUNTER18H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter18h;
#[doc = "MHPMCOUNTER19H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter19h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter19h`]
module"]
pub type MHPMCOUNTER19H = crate::Reg<mhpmcounter19h::MHPMCOUNTER19H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter19h;
#[doc = "MHPMCOUNTER20H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter20h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter20h`]
module"]
pub type MHPMCOUNTER20H = crate::Reg<mhpmcounter20h::MHPMCOUNTER20H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter20h;
#[doc = "MHPMCOUNTER21H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter21h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter21h`]
module"]
pub type MHPMCOUNTER21H = crate::Reg<mhpmcounter21h::MHPMCOUNTER21H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter21h;
#[doc = "MHPMCOUNTER22H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter22h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter22h`]
module"]
pub type MHPMCOUNTER22H = crate::Reg<mhpmcounter22h::MHPMCOUNTER22H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter22h;
#[doc = "MHPMCOUNTER23H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter23h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter23h`]
module"]
pub type MHPMCOUNTER23H = crate::Reg<mhpmcounter23h::MHPMCOUNTER23H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter23h;
#[doc = "MHPMCOUNTER24H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter24h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter24h`]
module"]
pub type MHPMCOUNTER24H = crate::Reg<mhpmcounter24h::MHPMCOUNTER24H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter24h;
#[doc = "MHPMCOUNTER25H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter25h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter25h`]
module"]
pub type MHPMCOUNTER25H = crate::Reg<mhpmcounter25h::MHPMCOUNTER25H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter25h;
#[doc = "MHPMCOUNTER26H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter26h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter26h`]
module"]
pub type MHPMCOUNTER26H = crate::Reg<mhpmcounter26h::MHPMCOUNTER26H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter26h;
#[doc = "MHPMCOUNTER27H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter27h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter27h`]
module"]
pub type MHPMCOUNTER27H = crate::Reg<mhpmcounter27h::MHPMCOUNTER27H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter27h;
#[doc = "MHPMCOUNTER28H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter28h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter28h`]
module"]
pub type MHPMCOUNTER28H = crate::Reg<mhpmcounter28h::MHPMCOUNTER28H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter28h;
#[doc = "MHPMCOUNTER29H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter29h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter29h`]
module"]
pub type MHPMCOUNTER29H = crate::Reg<mhpmcounter29h::MHPMCOUNTER29H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter29h;
#[doc = "MHPMCOUNTER30H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter30h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter30h`]
module"]
pub type MHPMCOUNTER30H = crate::Reg<mhpmcounter30h::MHPMCOUNTER30H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter30h;
#[doc = "MHPMCOUNTER31H (r) register accessor: Extended performance counter, hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`mhpmcounter31h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhpmcounter31h`]
module"]
pub type MHPMCOUNTER31H = crate::Reg<mhpmcounter31h::MHPMCOUNTER31H_SPEC>;
#[doc = "Extended performance counter, hardwired to 0."]
pub mod mhpmcounter31h;
#[doc = "PMPCFGM0 (rw) register accessor: PMP M-mode configuration. One bit per PMP region. Setting a bit makes the corresponding region apply to M-mode (like the `pmpcfg.L` bit) but does not lock the region.  

 PMP is useful for non-security-related purposes, such as stack guarding and peripheral emulation. This extension allows M-mode to freely use any currently unlocked regions for its own purposes, without the inconvenience of having to lock them.  

 Note that this does not grant any new capabilities to M-mode, since in the base standard it is already possible to apply unlocked regions to M-mode by locking them. In general, PMP regions should be locked in ascending region number order so they can't be subsequently overridden by currently unlocked regions.  

 Note also that this is not the same as the rule locking bypass bit in the ePMP extension, which does not permit locked and unlocked M-mode regions to coexist.  

 This is a Hazard3 custom CSR.  

You can [`read`](crate::Reg::read) this register and get [`pmpcfgm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpcfgm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pmpcfgm0`]
module"]
pub type PMPCFGM0 = crate::Reg<pmpcfgm0::PMPCFGM0_SPEC>;
#[doc = "PMP M-mode configuration. One bit per PMP region. Setting a bit makes the corresponding region apply to M-mode (like the `pmpcfg.L` bit) but does not lock the region.  

 PMP is useful for non-security-related purposes, such as stack guarding and peripheral emulation. This extension allows M-mode to freely use any currently unlocked regions for its own purposes, without the inconvenience of having to lock them.  

 Note that this does not grant any new capabilities to M-mode, since in the base standard it is already possible to apply unlocked regions to M-mode by locking them. In general, PMP regions should be locked in ascending region number order so they can't be subsequently overridden by currently unlocked regions.  

 Note also that this is not the same as the rule locking bypass bit in the ePMP extension, which does not permit locked and unlocked M-mode regions to coexist.  

 This is a Hazard3 custom CSR."]
pub mod pmpcfgm0;
#[doc = "MEIEA (rw) register accessor: External interrupt enable array.  

 The array contains a read-write bit for each external interrupt request: a `1` bit indicates that interrupt is currently enabled. At reset, all external interrupts are disabled.  

 If enabled, an external interrupt can cause assertion of the standard RISC-V machine external interrupt pending flag (`mip.meip`), and therefore cause the processor to enter the external interrupt vector. See `meipa`.  

 There are up to 512 external interrupts. The upper half of this register contains a 16-bit window into the full 512-bit vector. The window is indexed by the 5 LSBs of the write data.  

You can [`read`](crate::Reg::read) this register and get [`meiea::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meiea::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@meiea`]
module"]
pub type MEIEA = crate::Reg<meiea::MEIEA_SPEC>;
#[doc = "External interrupt enable array.  

 The array contains a read-write bit for each external interrupt request: a `1` bit indicates that interrupt is currently enabled. At reset, all external interrupts are disabled.  

 If enabled, an external interrupt can cause assertion of the standard RISC-V machine external interrupt pending flag (`mip.meip`), and therefore cause the processor to enter the external interrupt vector. See `meipa`.  

 There are up to 512 external interrupts. The upper half of this register contains a 16-bit window into the full 512-bit vector. The window is indexed by the 5 LSBs of the write data."]
pub mod meiea;
#[doc = "MEIPA (rw) register accessor: External interrupt pending array  

 Contains a read-only bit for each external interrupt request. Similarly to `meiea`, this register is a window into an array of up to 512 external interrupt flags. The status appears in the upper 16 bits of the value read from `meipa`, and the lower 5 bits of the value _written_ by the same CSR instruction (or 0 if no write takes place) select a 16-bit window of the full interrupt pending array.  

 A `1` bit indicates that interrupt is currently asserted. IRQs are assumed to be level-sensitive, and the relevant `meipa` bit is cleared by servicing the requestor so that it deasserts its interrupt request.  

 When any interrupt of sufficient priority is both set in `meipa` and enabled in `meiea`, the standard RISC-V external interrupt pending bit `mip.meip` is asserted. In other words, `meipa` is filtered by `meiea` to generate the standard `mip.meip` flag.  

You can [`read`](crate::Reg::read) this register and get [`meipa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meipa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@meipa`]
module"]
pub type MEIPA = crate::Reg<meipa::MEIPA_SPEC>;
#[doc = "External interrupt pending array  

 Contains a read-only bit for each external interrupt request. Similarly to `meiea`, this register is a window into an array of up to 512 external interrupt flags. The status appears in the upper 16 bits of the value read from `meipa`, and the lower 5 bits of the value _written_ by the same CSR instruction (or 0 if no write takes place) select a 16-bit window of the full interrupt pending array.  

 A `1` bit indicates that interrupt is currently asserted. IRQs are assumed to be level-sensitive, and the relevant `meipa` bit is cleared by servicing the requestor so that it deasserts its interrupt request.  

 When any interrupt of sufficient priority is both set in `meipa` and enabled in `meiea`, the standard RISC-V external interrupt pending bit `mip.meip` is asserted. In other words, `meipa` is filtered by `meiea` to generate the standard `mip.meip` flag."]
pub mod meipa;
#[doc = "MEIFA (rw) register accessor: External interrupt force array  

 Contains a read-write bit for every interrupt request. Writing a 1 to a bit in the interrupt force array causes the corresponding bit to become pending in `meipa`. Software can use this feature to manually trigger a particular interrupt.  

 There are no restrictions on using `meifa` inside of an interrupt. The more useful case here is to schedule some lower-priority handler from within a high-priority interrupt, so that it will execute before the core returns to the foreground code. Implementers may wish to reserve some external IRQs with their external inputs tied to 0 for this purpose.  

 Bits can be cleared by software, and are cleared automatically by hardware upon a read of `meinext` which returns the corresponding IRQ number in `meinext.irq` with `mienext.noirq` clear (no matter whether `meinext.update` is written).  

 `meifa` implements the same array window indexing scheme as `meiea` and `meipa`.  

You can [`read`](crate::Reg::read) this register and get [`meifa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meifa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@meifa`]
module"]
pub type MEIFA = crate::Reg<meifa::MEIFA_SPEC>;
#[doc = "External interrupt force array  

 Contains a read-write bit for every interrupt request. Writing a 1 to a bit in the interrupt force array causes the corresponding bit to become pending in `meipa`. Software can use this feature to manually trigger a particular interrupt.  

 There are no restrictions on using `meifa` inside of an interrupt. The more useful case here is to schedule some lower-priority handler from within a high-priority interrupt, so that it will execute before the core returns to the foreground code. Implementers may wish to reserve some external IRQs with their external inputs tied to 0 for this purpose.  

 Bits can be cleared by software, and are cleared automatically by hardware upon a read of `meinext` which returns the corresponding IRQ number in `meinext.irq` with `mienext.noirq` clear (no matter whether `meinext.update` is written).  

 `meifa` implements the same array window indexing scheme as `meiea` and `meipa`."]
pub mod meifa;
#[doc = "MEIPRA (rw) register accessor: External interrupt priority array  

 Each interrupt has an (up to) 4-bit priority value associated with it, and each access to this register reads and/or writes a 16-bit window containing four such priority values. When less than 16 priority levels are available, the LSBs of the priority fields are hardwired to 0.  

 When an interrupt's priority is lower than the current preemption priority `meicontext.preempt`, it is treated as not being pending for the purposes of `mip.meip`. The pending bit in `meipa` will still assert, but the machine external interrupt pending bit `mip.meip` will not, so the processor will ignore this interrupt. See `meicontext`.  

You can [`read`](crate::Reg::read) this register and get [`meipra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meipra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@meipra`]
module"]
pub type MEIPRA = crate::Reg<meipra::MEIPRA_SPEC>;
#[doc = "External interrupt priority array  

 Each interrupt has an (up to) 4-bit priority value associated with it, and each access to this register reads and/or writes a 16-bit window containing four such priority values. When less than 16 priority levels are available, the LSBs of the priority fields are hardwired to 0.  

 When an interrupt's priority is lower than the current preemption priority `meicontext.preempt`, it is treated as not being pending for the purposes of `mip.meip`. The pending bit in `meipa` will still assert, but the machine external interrupt pending bit `mip.meip` will not, so the processor will ignore this interrupt. See `meicontext`."]
pub mod meipra;
#[doc = "MEINEXT (rw) register accessor: Get next external interrupt  

 Contains the index of the highest-priority external interrupt which is both asserted in `meipa` and enabled in `meiea`, left-shifted by 2 so that it can be used to index an array of 32-bit function pointers. If there is no such interrupt, the MSB is set.  

 When multiple interrupts of the same priority are both pending and enabled, the lowest-numbered wins. Interrupts with priority less than `meicontext.ppreempt` -- the _previous_ preemption priority -- are treated as though they are not pending. This is to ensure that a preempting interrupt frame does not service interrupts which may be in progress in the frame that was preempted.  

You can [`read`](crate::Reg::read) this register and get [`meinext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meinext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@meinext`]
module"]
pub type MEINEXT = crate::Reg<meinext::MEINEXT_SPEC>;
#[doc = "Get next external interrupt  

 Contains the index of the highest-priority external interrupt which is both asserted in `meipa` and enabled in `meiea`, left-shifted by 2 so that it can be used to index an array of 32-bit function pointers. If there is no such interrupt, the MSB is set.  

 When multiple interrupts of the same priority are both pending and enabled, the lowest-numbered wins. Interrupts with priority less than `meicontext.ppreempt` -- the _previous_ preemption priority -- are treated as though they are not pending. This is to ensure that a preempting interrupt frame does not service interrupts which may be in progress in the frame that was preempted."]
pub mod meinext;
#[doc = "MEICONTEXT (rw) register accessor: External interrupt context register  

 Configures the priority level for interrupt preemption, and helps software track which interrupt it is currently in. The latter is useful when a common interrupt service routine handles interrupt requests from multiple instances of the same peripheral.  

 A three-level stack of preemption priorities is maintained in the `preempt`, `ppreempt` and `pppreempt` fields. The priority stack is saved when hardware enters the external interrupt vector, and restored by an `mret` instruction if `meicontext.mreteirq` is set.  

 The top entry of the priority stack, `preempt`, is used by hardware to ensure that only higher-priority interrupts can preempt the current interrupt. The next entry, `ppreempt`, is used to avoid servicing interrupts which may already be in progress in a frame that was preempted. The third entry, `pppreempt`, has no hardware effect, but ensures that `preempt` and `ppreempt` can be correctly saved/restored across arbitary levels of preemption.  

You can [`read`](crate::Reg::read) this register and get [`meicontext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meicontext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@meicontext`]
module"]
pub type MEICONTEXT = crate::Reg<meicontext::MEICONTEXT_SPEC>;
#[doc = "External interrupt context register  

 Configures the priority level for interrupt preemption, and helps software track which interrupt it is currently in. The latter is useful when a common interrupt service routine handles interrupt requests from multiple instances of the same peripheral.  

 A three-level stack of preemption priorities is maintained in the `preempt`, `ppreempt` and `pppreempt` fields. The priority stack is saved when hardware enters the external interrupt vector, and restored by an `mret` instruction if `meicontext.mreteirq` is set.  

 The top entry of the priority stack, `preempt`, is used by hardware to ensure that only higher-priority interrupts can preempt the current interrupt. The next entry, `ppreempt`, is used to avoid servicing interrupts which may already be in progress in a frame that was preempted. The third entry, `pppreempt`, has no hardware effect, but ensures that `preempt` and `ppreempt` can be correctly saved/restored across arbitary levels of preemption."]
pub mod meicontext;
#[doc = "MSLEEP (rw) register accessor: M-mode sleep control register  

You can [`read`](crate::Reg::read) this register and get [`msleep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msleep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@msleep`]
module"]
pub type MSLEEP = crate::Reg<msleep::MSLEEP_SPEC>;
#[doc = "M-mode sleep control register"]
pub mod msleep;
#[doc = "DMDATA0 (rw) register accessor: The Debug Module's DATA0 register is mapped into Hazard3's CSR space so that the Debug Module can exchange data with the core by executing CSR access instructions (this is used to implement the Abstract Access Register command). Only accessible in Debug Mode.  

You can [`read`](crate::Reg::read) this register and get [`dmdata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dmdata0`]
module"]
pub type DMDATA0 = crate::Reg<dmdata0::DMDATA0_SPEC>;
#[doc = "The Debug Module's DATA0 register is mapped into Hazard3's CSR space so that the Debug Module can exchange data with the core by executing CSR access instructions (this is used to implement the Abstract Access Register command). Only accessible in Debug Mode."]
pub mod dmdata0;
#[doc = "CYCLE (r) register accessor: Read-only U-mode alias of mcycle, accessible when `mcounteren.cy` is set  

You can [`read`](crate::Reg::read) this register and get [`cycle::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cycle`]
module"]
pub type CYCLE = crate::Reg<cycle::CYCLE_SPEC>;
#[doc = "Read-only U-mode alias of mcycle, accessible when `mcounteren.cy` is set"]
pub mod cycle;
#[doc = "INSTRET (r) register accessor: Read-only U-mode alias of minstret, accessible when `mcounteren.ir` is set  

You can [`read`](crate::Reg::read) this register and get [`instret::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@instret`]
module"]
pub type INSTRET = crate::Reg<instret::INSTRET_SPEC>;
#[doc = "Read-only U-mode alias of minstret, accessible when `mcounteren.ir` is set"]
pub mod instret;
#[doc = "CYCLEH (r) register accessor: Read-only U-mode alias of mcycleh, accessible when `mcounteren.cy` is set  

You can [`read`](crate::Reg::read) this register and get [`cycleh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cycleh`]
module"]
pub type CYCLEH = crate::Reg<cycleh::CYCLEH_SPEC>;
#[doc = "Read-only U-mode alias of mcycleh, accessible when `mcounteren.cy` is set"]
pub mod cycleh;
#[doc = "INSTRETH (r) register accessor: Read-only U-mode alias of minstreth, accessible when `mcounteren.ir` is set  

You can [`read`](crate::Reg::read) this register and get [`instreth::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@instreth`]
module"]
pub type INSTRETH = crate::Reg<instreth::INSTRETH_SPEC>;
#[doc = "Read-only U-mode alias of minstreth, accessible when `mcounteren.ir` is set"]
pub mod instreth;
#[doc = "MVENDORID (r) register accessor: Vendor ID  

You can [`read`](crate::Reg::read) this register and get [`mvendorid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mvendorid`]
module"]
pub type MVENDORID = crate::Reg<mvendorid::MVENDORID_SPEC>;
#[doc = "Vendor ID"]
pub mod mvendorid;
#[doc = "MARCHID (r) register accessor: Architecture ID (Hazard3)  

You can [`read`](crate::Reg::read) this register and get [`marchid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@marchid`]
module"]
pub type MARCHID = crate::Reg<marchid::MARCHID_SPEC>;
#[doc = "Architecture ID (Hazard3)"]
pub mod marchid;
#[doc = "MIMPID (r) register accessor: Implementation ID  

You can [`read`](crate::Reg::read) this register and get [`mimpid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mimpid`]
module"]
pub type MIMPID = crate::Reg<mimpid::MIMPID_SPEC>;
#[doc = "Implementation ID"]
pub mod mimpid;
#[doc = "MHARTID (r) register accessor: Hardware thread ID  
 On RP2350, core 0 has a hart ID of 0, and core 1 has a hart ID of 1.  

You can [`read`](crate::Reg::read) this register and get [`mhartid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mhartid`]
module"]
pub type MHARTID = crate::Reg<mhartid::MHARTID_SPEC>;
#[doc = "Hardware thread ID  
 On RP2350, core 0 has a hart ID of 0, and core 1 has a hart ID of 1."]
pub mod mhartid;
#[doc = "MCONFIGPTR (r) register accessor: Pointer to configuration data structure (hardwired to 0)  

You can [`read`](crate::Reg::read) this register and get [`mconfigptr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mconfigptr`]
module"]
pub type MCONFIGPTR = crate::Reg<mconfigptr::MCONFIGPTR_SPEC>;
#[doc = "Pointer to configuration data structure (hardwired to 0)"]
pub mod mconfigptr;
