#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    itm_stim0: ITM_STIM0,
    itm_stim1: ITM_STIM1,
    itm_stim2: ITM_STIM2,
    itm_stim3: ITM_STIM3,
    itm_stim4: ITM_STIM4,
    itm_stim5: ITM_STIM5,
    itm_stim6: ITM_STIM6,
    itm_stim7: ITM_STIM7,
    itm_stim8: ITM_STIM8,
    itm_stim9: ITM_STIM9,
    itm_stim10: ITM_STIM10,
    itm_stim11: ITM_STIM11,
    itm_stim12: ITM_STIM12,
    itm_stim13: ITM_STIM13,
    itm_stim14: ITM_STIM14,
    itm_stim15: ITM_STIM15,
    itm_stim16: ITM_STIM16,
    itm_stim17: ITM_STIM17,
    itm_stim18: ITM_STIM18,
    itm_stim19: ITM_STIM19,
    itm_stim20: ITM_STIM20,
    itm_stim21: ITM_STIM21,
    itm_stim22: ITM_STIM22,
    itm_stim23: ITM_STIM23,
    itm_stim24: ITM_STIM24,
    itm_stim25: ITM_STIM25,
    itm_stim26: ITM_STIM26,
    itm_stim27: ITM_STIM27,
    itm_stim28: ITM_STIM28,
    itm_stim29: ITM_STIM29,
    itm_stim30: ITM_STIM30,
    itm_stim31: ITM_STIM31,
    _reserved32: [u8; 0x0d80],
    itm_ter0: ITM_TER0,
    _reserved33: [u8; 0x3c],
    itm_tpr: ITM_TPR,
    _reserved34: [u8; 0x3c],
    itm_tcr: ITM_TCR,
    _reserved35: [u8; 0x6c],
    int_atready: INT_ATREADY,
    _reserved36: [u8; 0x04],
    int_atvalid: INT_ATVALID,
    _reserved37: [u8; 0x04],
    itm_itctrl: ITM_ITCTRL,
    _reserved38: [u8; 0xb8],
    itm_devarch: ITM_DEVARCH,
    _reserved39: [u8; 0x0c],
    itm_devtype: ITM_DEVTYPE,
    itm_pidr4: ITM_PIDR4,
    itm_pidr5: ITM_PIDR5,
    itm_pidr6: ITM_PIDR6,
    itm_pidr7: ITM_PIDR7,
    itm_pidr0: ITM_PIDR0,
    itm_pidr1: ITM_PIDR1,
    itm_pidr2: ITM_PIDR2,
    itm_pidr3: ITM_PIDR3,
    itm_cidr0: ITM_CIDR0,
    itm_cidr1: ITM_CIDR1,
    itm_cidr2: ITM_CIDR2,
    itm_cidr3: ITM_CIDR3,
    dwt_ctrl: DWT_CTRL,
    dwt_cyccnt: DWT_CYCCNT,
    _reserved54: [u8; 0x04],
    dwt_exccnt: DWT_EXCCNT,
    _reserved55: [u8; 0x04],
    dwt_lsucnt: DWT_LSUCNT,
    dwt_foldcnt: DWT_FOLDCNT,
    _reserved57: [u8; 0x04],
    dwt_comp0: DWT_COMP0,
    _reserved58: [u8; 0x04],
    dwt_function0: DWT_FUNCTION0,
    _reserved59: [u8; 0x04],
    dwt_comp1: DWT_COMP1,
    _reserved60: [u8; 0x04],
    dwt_function1: DWT_FUNCTION1,
    _reserved61: [u8; 0x04],
    dwt_comp2: DWT_COMP2,
    _reserved62: [u8; 0x04],
    dwt_function2: DWT_FUNCTION2,
    _reserved63: [u8; 0x04],
    dwt_comp3: DWT_COMP3,
    _reserved64: [u8; 0x04],
    dwt_function3: DWT_FUNCTION3,
    _reserved65: [u8; 0x0f60],
    dwt_devarch: DWT_DEVARCH,
    _reserved66: [u8; 0x0c],
    dwt_devtype: DWT_DEVTYPE,
    dwt_pidr4: DWT_PIDR4,
    dwt_pidr5: DWT_PIDR5,
    dwt_pidr6: DWT_PIDR6,
    dwt_pidr7: DWT_PIDR7,
    dwt_pidr0: DWT_PIDR0,
    dwt_pidr1: DWT_PIDR1,
    dwt_pidr2: DWT_PIDR2,
    dwt_pidr3: DWT_PIDR3,
    dwt_cidr0: DWT_CIDR0,
    dwt_cidr1: DWT_CIDR1,
    dwt_cidr2: DWT_CIDR2,
    dwt_cidr3: DWT_CIDR3,
    fp_ctrl: FP_CTRL,
    fp_remap: FP_REMAP,
    fp_comp0: FP_COMP0,
    fp_comp1: FP_COMP1,
    fp_comp2: FP_COMP2,
    fp_comp3: FP_COMP3,
    fp_comp4: FP_COMP4,
    fp_comp5: FP_COMP5,
    fp_comp6: FP_COMP6,
    fp_comp7: FP_COMP7,
    _reserved89: [u8; 0x0f94],
    fp_devarch: FP_DEVARCH,
    _reserved90: [u8; 0x0c],
    fp_devtype: FP_DEVTYPE,
    fp_pidr4: FP_PIDR4,
    fp_pidr5: FP_PIDR5,
    fp_pidr6: FP_PIDR6,
    fp_pidr7: FP_PIDR7,
    fp_pidr0: FP_PIDR0,
    fp_pidr1: FP_PIDR1,
    fp_pidr2: FP_PIDR2,
    fp_pidr3: FP_PIDR3,
    fp_cidr0: FP_CIDR0,
    fp_cidr1: FP_CIDR1,
    fp_cidr2: FP_CIDR2,
    fp_cidr3: FP_CIDR3,
    _reserved103: [u8; 0xb004],
    ictr: ICTR,
    actlr: ACTLR,
    _reserved105: [u8; 0x04],
    syst_csr: SYST_CSR,
    syst_rvr: SYST_RVR,
    syst_cvr: SYST_CVR,
    syst_calib: SYST_CALIB,
    _reserved109: [u8; 0xe0],
    nvic_iser0: NVIC_ISER0,
    nvic_iser1: NVIC_ISER1,
    _reserved111: [u8; 0x78],
    nvic_icer0: NVIC_ICER0,
    nvic_icer1: NVIC_ICER1,
    _reserved113: [u8; 0x78],
    nvic_ispr0: NVIC_ISPR0,
    nvic_ispr1: NVIC_ISPR1,
    _reserved115: [u8; 0x78],
    nvic_icpr0: NVIC_ICPR0,
    nvic_icpr1: NVIC_ICPR1,
    _reserved117: [u8; 0x78],
    nvic_iabr0: NVIC_IABR0,
    nvic_iabr1: NVIC_IABR1,
    _reserved119: [u8; 0x78],
    nvic_itns0: NVIC_ITNS0,
    nvic_itns1: NVIC_ITNS1,
    _reserved121: [u8; 0x78],
    nvic_ipr0: NVIC_IPR0,
    nvic_ipr1: NVIC_IPR1,
    nvic_ipr2: NVIC_IPR2,
    nvic_ipr3: NVIC_IPR3,
    nvic_ipr4: NVIC_IPR4,
    nvic_ipr5: NVIC_IPR5,
    nvic_ipr6: NVIC_IPR6,
    nvic_ipr7: NVIC_IPR7,
    nvic_ipr8: NVIC_IPR8,
    nvic_ipr9: NVIC_IPR9,
    nvic_ipr10: NVIC_IPR10,
    nvic_ipr11: NVIC_IPR11,
    nvic_ipr12: NVIC_IPR12,
    nvic_ipr13: NVIC_IPR13,
    nvic_ipr14: NVIC_IPR14,
    nvic_ipr15: NVIC_IPR15,
    _reserved137: [u8; 0x08c0],
    cpuid: CPUID,
    icsr: ICSR,
    vtor: VTOR,
    aircr: AIRCR,
    scr: SCR,
    ccr: CCR,
    shpr1: SHPR1,
    shpr2: SHPR2,
    shpr3: SHPR3,
    shcsr: SHCSR,
    cfsr: CFSR,
    hfsr: HFSR,
    dfsr: DFSR,
    mmfar: MMFAR,
    bfar: BFAR,
    _reserved152: [u8; 0x04],
    id_pfr0: ID_PFR0,
    id_pfr1: ID_PFR1,
    id_dfr0: ID_DFR0,
    id_afr0: ID_AFR0,
    id_mmfr0: ID_MMFR0,
    id_mmfr1: ID_MMFR1,
    id_mmfr2: ID_MMFR2,
    id_mmfr3: ID_MMFR3,
    id_isar0: ID_ISAR0,
    id_isar1: ID_ISAR1,
    id_isar2: ID_ISAR2,
    id_isar3: ID_ISAR3,
    id_isar4: ID_ISAR4,
    id_isar5: ID_ISAR5,
    _reserved166: [u8; 0x04],
    ctr: CTR,
    _reserved167: [u8; 0x08],
    cpacr: CPACR,
    nsacr: NSACR,
    mpu_type: MPU_TYPE,
    mpu_ctrl: MPU_CTRL,
    mpu_rnr: MPU_RNR,
    mpu_rbar: MPU_RBAR,
    mpu_rlar: MPU_RLAR,
    mpu_rbar_a1: MPU_RBAR_A1,
    mpu_rlar_a1: MPU_RLAR_A1,
    mpu_rbar_a2: MPU_RBAR_A2,
    mpu_rlar_a2: MPU_RLAR_A2,
    mpu_rbar_a3: MPU_RBAR_A3,
    mpu_rlar_a3: MPU_RLAR_A3,
    _reserved180: [u8; 0x04],
    mpu_mair0: MPU_MAIR0,
    mpu_mair1: MPU_MAIR1,
    _reserved182: [u8; 0x08],
    sau_ctrl: SAU_CTRL,
    sau_type: SAU_TYPE,
    sau_rnr: SAU_RNR,
    sau_rbar: SAU_RBAR,
    sau_rlar: SAU_RLAR,
    sfsr: SFSR,
    sfar: SFAR,
    _reserved189: [u8; 0x04],
    dhcsr: DHCSR,
    dcrsr: DCRSR,
    dcrdr: DCRDR,
    demcr: DEMCR,
    _reserved193: [u8; 0x08],
    dscsr: DSCSR,
    _reserved194: [u8; 0xf4],
    stir: STIR,
    _reserved195: [u8; 0x30],
    fpccr: FPCCR,
    fpcar: FPCAR,
    fpdscr: FPDSCR,
    mvfr0: MVFR0,
    mvfr1: MVFR1,
    mvfr2: MVFR2,
    _reserved201: [u8; 0x70],
    ddevarch: DDEVARCH,
    _reserved202: [u8; 0x0c],
    ddevtype: DDEVTYPE,
    dpidr4: DPIDR4,
    dpidr5: DPIDR5,
    dpidr6: DPIDR6,
    dpidr7: DPIDR7,
    dpidr0: DPIDR0,
    dpidr1: DPIDR1,
    dpidr2: DPIDR2,
    dpidr3: DPIDR3,
    dcidr0: DCIDR0,
    dcidr1: DCIDR1,
    dcidr2: DCIDR2,
    dcidr3: DCIDR3,
    _reserved215: [u8; 0x0003_2004],
    trcprgctlr: TRCPRGCTLR,
    _reserved216: [u8; 0x04],
    trcstatr: TRCSTATR,
    trcconfigr: TRCCONFIGR,
    _reserved218: [u8; 0x0c],
    trceventctl0r: TRCEVENTCTL0R,
    trceventctl1r: TRCEVENTCTL1R,
    _reserved220: [u8; 0x04],
    trcstallctlr: TRCSTALLCTLR,
    trctsctlr: TRCTSCTLR,
    trcsyncpr: TRCSYNCPR,
    trcccctlr: TRCCCCTLR,
    _reserved224: [u8; 0x44],
    trcvictlr: TRCVICTLR,
    _reserved225: [u8; 0xbc],
    trccntrldvr0: TRCCNTRLDVR0,
    _reserved226: [u8; 0x3c],
    trcidr8: TRCIDR8,
    trcidr9: TRCIDR9,
    trcidr10: TRCIDR10,
    trcidr11: TRCIDR11,
    trcidr12: TRCIDR12,
    trcidr13: TRCIDR13,
    _reserved232: [u8; 0x28],
    trcimspec: TRCIMSPEC,
    _reserved233: [u8; 0x1c],
    trcidr0: TRCIDR0,
    trcidr1: TRCIDR1,
    trcidr2: TRCIDR2,
    trcidr3: TRCIDR3,
    trcidr4: TRCIDR4,
    trcidr5: TRCIDR5,
    trcidr6: TRCIDR6,
    trcidr7: TRCIDR7,
    _reserved241: [u8; 0x08],
    trcrsctlr2: TRCRSCTLR2,
    trcrsctlr3: TRCRSCTLR3,
    _reserved243: [u8; 0x90],
    trcsscsr: TRCSSCSR,
    _reserved244: [u8; 0x1c],
    trcsspcicr: TRCSSPCICR,
    _reserved245: [u8; 0x4c],
    trcpdcr: TRCPDCR,
    trcpdsr: TRCPDSR,
    _reserved247: [u8; 0x0bcc],
    trcitatbidr: TRCITATBIDR,
    _reserved248: [u8; 0x0c],
    trcitiatbinr: TRCITIATBINR,
    _reserved249: [u8; 0x04],
    trcitiatboutr: TRCITIATBOUTR,
    _reserved250: [u8; 0xa0],
    trcclaimset: TRCCLAIMSET,
    trcclaimclr: TRCCLAIMCLR,
    _reserved252: [u8; 0x10],
    trcauthstatus: TRCAUTHSTATUS,
    trcdevarch: TRCDEVARCH,
    _reserved254: [u8; 0x08],
    trcdevid: TRCDEVID,
    trcdevtype: TRCDEVTYPE,
    trcpidr4: TRCPIDR4,
    trcpidr5: TRCPIDR5,
    trcpidr6: TRCPIDR6,
    trcpidr7: TRCPIDR7,
    trcpidr0: TRCPIDR0,
    trcpidr1: TRCPIDR1,
    trcpidr2: TRCPIDR2,
    trcpidr3: TRCPIDR3,
    trccidr0: TRCCIDR0,
    trccidr1: TRCCIDR1,
    trccidr2: TRCCIDR2,
    trccidr3: TRCCIDR3,
    cticontrol: CTICONTROL,
    _reserved269: [u8; 0x0c],
    ctiintack: CTIINTACK,
    ctiappset: CTIAPPSET,
    ctiappclear: CTIAPPCLEAR,
    ctiapppulse: CTIAPPPULSE,
    ctiinen0: CTIINEN0,
    ctiinen1: CTIINEN1,
    ctiinen2: CTIINEN2,
    ctiinen3: CTIINEN3,
    ctiinen4: CTIINEN4,
    ctiinen5: CTIINEN5,
    ctiinen6: CTIINEN6,
    ctiinen7: CTIINEN7,
    _reserved281: [u8; 0x60],
    ctiouten0: CTIOUTEN0,
    ctiouten1: CTIOUTEN1,
    ctiouten2: CTIOUTEN2,
    ctiouten3: CTIOUTEN3,
    ctiouten4: CTIOUTEN4,
    ctiouten5: CTIOUTEN5,
    ctiouten6: CTIOUTEN6,
    ctiouten7: CTIOUTEN7,
    _reserved289: [u8; 0x70],
    ctitriginstatus: CTITRIGINSTATUS,
    ctitrigoutstatus: CTITRIGOUTSTATUS,
    ctichinstatus: CTICHINSTATUS,
    _reserved292: [u8; 0x04],
    ctigate: CTIGATE,
    asicctl: ASICCTL,
    _reserved294: [u8; 0x0d9c],
    itchout: ITCHOUT,
    ittrigout: ITTRIGOUT,
    _reserved296: [u8; 0x08],
    itchin: ITCHIN,
    _reserved297: [u8; 0x08],
    itctrl: ITCTRL,
    _reserved298: [u8; 0xb8],
    devarch: DEVARCH,
    _reserved299: [u8; 0x08],
    devid: DEVID,
    devtype: DEVTYPE,
    pidr4: PIDR4,
    pidr5: PIDR5,
    pidr6: PIDR6,
    pidr7: PIDR7,
    pidr0: PIDR0,
    pidr1: PIDR1,
    pidr2: PIDR2,
    pidr3: PIDR3,
    cidr0: CIDR0,
    cidr1: CIDR1,
    cidr2: CIDR2,
    cidr3: CIDR3,
}
impl RegisterBlock {
    #[doc = "0x00 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim0(&self) -> &ITM_STIM0 {
        &self.itm_stim0
    }
    #[doc = "0x04 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim1(&self) -> &ITM_STIM1 {
        &self.itm_stim1
    }
    #[doc = "0x08 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim2(&self) -> &ITM_STIM2 {
        &self.itm_stim2
    }
    #[doc = "0x0c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim3(&self) -> &ITM_STIM3 {
        &self.itm_stim3
    }
    #[doc = "0x10 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim4(&self) -> &ITM_STIM4 {
        &self.itm_stim4
    }
    #[doc = "0x14 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim5(&self) -> &ITM_STIM5 {
        &self.itm_stim5
    }
    #[doc = "0x18 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim6(&self) -> &ITM_STIM6 {
        &self.itm_stim6
    }
    #[doc = "0x1c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim7(&self) -> &ITM_STIM7 {
        &self.itm_stim7
    }
    #[doc = "0x20 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim8(&self) -> &ITM_STIM8 {
        &self.itm_stim8
    }
    #[doc = "0x24 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim9(&self) -> &ITM_STIM9 {
        &self.itm_stim9
    }
    #[doc = "0x28 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim10(&self) -> &ITM_STIM10 {
        &self.itm_stim10
    }
    #[doc = "0x2c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim11(&self) -> &ITM_STIM11 {
        &self.itm_stim11
    }
    #[doc = "0x30 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim12(&self) -> &ITM_STIM12 {
        &self.itm_stim12
    }
    #[doc = "0x34 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim13(&self) -> &ITM_STIM13 {
        &self.itm_stim13
    }
    #[doc = "0x38 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim14(&self) -> &ITM_STIM14 {
        &self.itm_stim14
    }
    #[doc = "0x3c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim15(&self) -> &ITM_STIM15 {
        &self.itm_stim15
    }
    #[doc = "0x40 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim16(&self) -> &ITM_STIM16 {
        &self.itm_stim16
    }
    #[doc = "0x44 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim17(&self) -> &ITM_STIM17 {
        &self.itm_stim17
    }
    #[doc = "0x48 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim18(&self) -> &ITM_STIM18 {
        &self.itm_stim18
    }
    #[doc = "0x4c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim19(&self) -> &ITM_STIM19 {
        &self.itm_stim19
    }
    #[doc = "0x50 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim20(&self) -> &ITM_STIM20 {
        &self.itm_stim20
    }
    #[doc = "0x54 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim21(&self) -> &ITM_STIM21 {
        &self.itm_stim21
    }
    #[doc = "0x58 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim22(&self) -> &ITM_STIM22 {
        &self.itm_stim22
    }
    #[doc = "0x5c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim23(&self) -> &ITM_STIM23 {
        &self.itm_stim23
    }
    #[doc = "0x60 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim24(&self) -> &ITM_STIM24 {
        &self.itm_stim24
    }
    #[doc = "0x64 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim25(&self) -> &ITM_STIM25 {
        &self.itm_stim25
    }
    #[doc = "0x68 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim26(&self) -> &ITM_STIM26 {
        &self.itm_stim26
    }
    #[doc = "0x6c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim27(&self) -> &ITM_STIM27 {
        &self.itm_stim27
    }
    #[doc = "0x70 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim28(&self) -> &ITM_STIM28 {
        &self.itm_stim28
    }
    #[doc = "0x74 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim29(&self) -> &ITM_STIM29 {
        &self.itm_stim29
    }
    #[doc = "0x78 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim30(&self) -> &ITM_STIM30 {
        &self.itm_stim30
    }
    #[doc = "0x7c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn itm_stim31(&self) -> &ITM_STIM31 {
        &self.itm_stim31
    }
    #[doc = "0xe00 - Provide an individual enable bit for each ITM_STIM register"]
    #[inline(always)]
    pub const fn itm_ter0(&self) -> &ITM_TER0 {
        &self.itm_ter0
    }
    #[doc = "0xe40 - Controls which stimulus ports can be accessed by unprivileged code"]
    #[inline(always)]
    pub const fn itm_tpr(&self) -> &ITM_TPR {
        &self.itm_tpr
    }
    #[doc = "0xe80 - Configures and controls transfers through the ITM interface"]
    #[inline(always)]
    pub const fn itm_tcr(&self) -> &ITM_TCR {
        &self.itm_tcr
    }
    #[doc = "0xef0 - Integration Mode: Read ATB Ready"]
    #[inline(always)]
    pub const fn int_atready(&self) -> &INT_ATREADY {
        &self.int_atready
    }
    #[doc = "0xef8 - Integration Mode: Write ATB Valid"]
    #[inline(always)]
    pub const fn int_atvalid(&self) -> &INT_ATVALID {
        &self.int_atvalid
    }
    #[doc = "0xf00 - Integration Mode Control Register"]
    #[inline(always)]
    pub const fn itm_itctrl(&self) -> &ITM_ITCTRL {
        &self.itm_itctrl
    }
    #[doc = "0xfbc - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_devarch(&self) -> &ITM_DEVARCH {
        &self.itm_devarch
    }
    #[doc = "0xfcc - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_devtype(&self) -> &ITM_DEVTYPE {
        &self.itm_devtype
    }
    #[doc = "0xfd0 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_pidr4(&self) -> &ITM_PIDR4 {
        &self.itm_pidr4
    }
    #[doc = "0xfd4 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_pidr5(&self) -> &ITM_PIDR5 {
        &self.itm_pidr5
    }
    #[doc = "0xfd8 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_pidr6(&self) -> &ITM_PIDR6 {
        &self.itm_pidr6
    }
    #[doc = "0xfdc - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_pidr7(&self) -> &ITM_PIDR7 {
        &self.itm_pidr7
    }
    #[doc = "0xfe0 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_pidr0(&self) -> &ITM_PIDR0 {
        &self.itm_pidr0
    }
    #[doc = "0xfe4 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_pidr1(&self) -> &ITM_PIDR1 {
        &self.itm_pidr1
    }
    #[doc = "0xfe8 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_pidr2(&self) -> &ITM_PIDR2 {
        &self.itm_pidr2
    }
    #[doc = "0xfec - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_pidr3(&self) -> &ITM_PIDR3 {
        &self.itm_pidr3
    }
    #[doc = "0xff0 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_cidr0(&self) -> &ITM_CIDR0 {
        &self.itm_cidr0
    }
    #[doc = "0xff4 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_cidr1(&self) -> &ITM_CIDR1 {
        &self.itm_cidr1
    }
    #[doc = "0xff8 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_cidr2(&self) -> &ITM_CIDR2 {
        &self.itm_cidr2
    }
    #[doc = "0xffc - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn itm_cidr3(&self) -> &ITM_CIDR3 {
        &self.itm_cidr3
    }
    #[doc = "0x1000 - Provides configuration and status information for the DWT unit, and used to control features of the unit"]
    #[inline(always)]
    pub const fn dwt_ctrl(&self) -> &DWT_CTRL {
        &self.dwt_ctrl
    }
    #[doc = "0x1004 - Shows or sets the value of the processor cycle counter, CYCCNT"]
    #[inline(always)]
    pub const fn dwt_cyccnt(&self) -> &DWT_CYCCNT {
        &self.dwt_cyccnt
    }
    #[doc = "0x100c - Counts the total cycles spent in exception processing"]
    #[inline(always)]
    pub const fn dwt_exccnt(&self) -> &DWT_EXCCNT {
        &self.dwt_exccnt
    }
    #[doc = "0x1014 - Increments on the additional cycles required to execute all load or store instructions"]
    #[inline(always)]
    pub const fn dwt_lsucnt(&self) -> &DWT_LSUCNT {
        &self.dwt_lsucnt
    }
    #[doc = "0x1018 - Increments on the additional cycles required to execute all load or store instructions"]
    #[inline(always)]
    pub const fn dwt_foldcnt(&self) -> &DWT_FOLDCNT {
        &self.dwt_foldcnt
    }
    #[doc = "0x1020 - Provides a reference value for use by watchpoint comparator 0"]
    #[inline(always)]
    pub const fn dwt_comp0(&self) -> &DWT_COMP0 {
        &self.dwt_comp0
    }
    #[doc = "0x1028 - Controls the operation of watchpoint comparator 0"]
    #[inline(always)]
    pub const fn dwt_function0(&self) -> &DWT_FUNCTION0 {
        &self.dwt_function0
    }
    #[doc = "0x1030 - Provides a reference value for use by watchpoint comparator 1"]
    #[inline(always)]
    pub const fn dwt_comp1(&self) -> &DWT_COMP1 {
        &self.dwt_comp1
    }
    #[doc = "0x1038 - Controls the operation of watchpoint comparator 1"]
    #[inline(always)]
    pub const fn dwt_function1(&self) -> &DWT_FUNCTION1 {
        &self.dwt_function1
    }
    #[doc = "0x1040 - Provides a reference value for use by watchpoint comparator 2"]
    #[inline(always)]
    pub const fn dwt_comp2(&self) -> &DWT_COMP2 {
        &self.dwt_comp2
    }
    #[doc = "0x1048 - Controls the operation of watchpoint comparator 2"]
    #[inline(always)]
    pub const fn dwt_function2(&self) -> &DWT_FUNCTION2 {
        &self.dwt_function2
    }
    #[doc = "0x1050 - Provides a reference value for use by watchpoint comparator 3"]
    #[inline(always)]
    pub const fn dwt_comp3(&self) -> &DWT_COMP3 {
        &self.dwt_comp3
    }
    #[doc = "0x1058 - Controls the operation of watchpoint comparator 3"]
    #[inline(always)]
    pub const fn dwt_function3(&self) -> &DWT_FUNCTION3 {
        &self.dwt_function3
    }
    #[doc = "0x1fbc - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_devarch(&self) -> &DWT_DEVARCH {
        &self.dwt_devarch
    }
    #[doc = "0x1fcc - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_devtype(&self) -> &DWT_DEVTYPE {
        &self.dwt_devtype
    }
    #[doc = "0x1fd0 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_pidr4(&self) -> &DWT_PIDR4 {
        &self.dwt_pidr4
    }
    #[doc = "0x1fd4 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_pidr5(&self) -> &DWT_PIDR5 {
        &self.dwt_pidr5
    }
    #[doc = "0x1fd8 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_pidr6(&self) -> &DWT_PIDR6 {
        &self.dwt_pidr6
    }
    #[doc = "0x1fdc - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_pidr7(&self) -> &DWT_PIDR7 {
        &self.dwt_pidr7
    }
    #[doc = "0x1fe0 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_pidr0(&self) -> &DWT_PIDR0 {
        &self.dwt_pidr0
    }
    #[doc = "0x1fe4 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_pidr1(&self) -> &DWT_PIDR1 {
        &self.dwt_pidr1
    }
    #[doc = "0x1fe8 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_pidr2(&self) -> &DWT_PIDR2 {
        &self.dwt_pidr2
    }
    #[doc = "0x1fec - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_pidr3(&self) -> &DWT_PIDR3 {
        &self.dwt_pidr3
    }
    #[doc = "0x1ff0 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_cidr0(&self) -> &DWT_CIDR0 {
        &self.dwt_cidr0
    }
    #[doc = "0x1ff4 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_cidr1(&self) -> &DWT_CIDR1 {
        &self.dwt_cidr1
    }
    #[doc = "0x1ff8 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_cidr2(&self) -> &DWT_CIDR2 {
        &self.dwt_cidr2
    }
    #[doc = "0x1ffc - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn dwt_cidr3(&self) -> &DWT_CIDR3 {
        &self.dwt_cidr3
    }
    #[doc = "0x2000 - Provides FPB implementation information, and the global enable for the FPB unit"]
    #[inline(always)]
    pub const fn fp_ctrl(&self) -> &FP_CTRL {
        &self.fp_ctrl
    }
    #[doc = "0x2004 - Indicates whether the implementation supports Flash Patch remap and, if it does, holds the target address for remap"]
    #[inline(always)]
    pub const fn fp_remap(&self) -> &FP_REMAP {
        &self.fp_remap
    }
    #[doc = "0x2008 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    #[inline(always)]
    pub const fn fp_comp0(&self) -> &FP_COMP0 {
        &self.fp_comp0
    }
    #[doc = "0x200c - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    #[inline(always)]
    pub const fn fp_comp1(&self) -> &FP_COMP1 {
        &self.fp_comp1
    }
    #[doc = "0x2010 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    #[inline(always)]
    pub const fn fp_comp2(&self) -> &FP_COMP2 {
        &self.fp_comp2
    }
    #[doc = "0x2014 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    #[inline(always)]
    pub const fn fp_comp3(&self) -> &FP_COMP3 {
        &self.fp_comp3
    }
    #[doc = "0x2018 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    #[inline(always)]
    pub const fn fp_comp4(&self) -> &FP_COMP4 {
        &self.fp_comp4
    }
    #[doc = "0x201c - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    #[inline(always)]
    pub const fn fp_comp5(&self) -> &FP_COMP5 {
        &self.fp_comp5
    }
    #[doc = "0x2020 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    #[inline(always)]
    pub const fn fp_comp6(&self) -> &FP_COMP6 {
        &self.fp_comp6
    }
    #[doc = "0x2024 - Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
    #[inline(always)]
    pub const fn fp_comp7(&self) -> &FP_COMP7 {
        &self.fp_comp7
    }
    #[doc = "0x2fbc - Provides CoreSight discovery information for the FPB"]
    #[inline(always)]
    pub const fn fp_devarch(&self) -> &FP_DEVARCH {
        &self.fp_devarch
    }
    #[doc = "0x2fcc - Provides CoreSight discovery information for the FPB"]
    #[inline(always)]
    pub const fn fp_devtype(&self) -> &FP_DEVTYPE {
        &self.fp_devtype
    }
    #[doc = "0x2fd0 - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_pidr4(&self) -> &FP_PIDR4 {
        &self.fp_pidr4
    }
    #[doc = "0x2fd4 - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_pidr5(&self) -> &FP_PIDR5 {
        &self.fp_pidr5
    }
    #[doc = "0x2fd8 - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_pidr6(&self) -> &FP_PIDR6 {
        &self.fp_pidr6
    }
    #[doc = "0x2fdc - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_pidr7(&self) -> &FP_PIDR7 {
        &self.fp_pidr7
    }
    #[doc = "0x2fe0 - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_pidr0(&self) -> &FP_PIDR0 {
        &self.fp_pidr0
    }
    #[doc = "0x2fe4 - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_pidr1(&self) -> &FP_PIDR1 {
        &self.fp_pidr1
    }
    #[doc = "0x2fe8 - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_pidr2(&self) -> &FP_PIDR2 {
        &self.fp_pidr2
    }
    #[doc = "0x2fec - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_pidr3(&self) -> &FP_PIDR3 {
        &self.fp_pidr3
    }
    #[doc = "0x2ff0 - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_cidr0(&self) -> &FP_CIDR0 {
        &self.fp_cidr0
    }
    #[doc = "0x2ff4 - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_cidr1(&self) -> &FP_CIDR1 {
        &self.fp_cidr1
    }
    #[doc = "0x2ff8 - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_cidr2(&self) -> &FP_CIDR2 {
        &self.fp_cidr2
    }
    #[doc = "0x2ffc - Provides CoreSight discovery information for the FP"]
    #[inline(always)]
    pub const fn fp_cidr3(&self) -> &FP_CIDR3 {
        &self.fp_cidr3
    }
    #[doc = "0xe004 - Provides information about the interrupt controller"]
    #[inline(always)]
    pub const fn ictr(&self) -> &ICTR {
        &self.ictr
    }
    #[doc = "0xe008 - Provides IMPLEMENTATION DEFINED configuration and control options"]
    #[inline(always)]
    pub const fn actlr(&self) -> &ACTLR {
        &self.actlr
    }
    #[doc = "0xe010 - Use the SysTick Control and Status Register to enable the SysTick features."]
    #[inline(always)]
    pub const fn syst_csr(&self) -> &SYST_CSR {
        &self.syst_csr
    }
    #[doc = "0xe014 - Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN. To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99."]
    #[inline(always)]
    pub const fn syst_rvr(&self) -> &SYST_RVR {
        &self.syst_rvr
    }
    #[doc = "0xe018 - Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
    #[inline(always)]
    pub const fn syst_cvr(&self) -> &SYST_CVR {
        &self.syst_cvr
    }
    #[doc = "0xe01c - Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
    #[inline(always)]
    pub const fn syst_calib(&self) -> &SYST_CALIB {
        &self.syst_calib
    }
    #[doc = "0xe100 - Enables or reads the enabled state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn nvic_iser0(&self) -> &NVIC_ISER0 {
        &self.nvic_iser0
    }
    #[doc = "0xe104 - Enables or reads the enabled state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn nvic_iser1(&self) -> &NVIC_ISER1 {
        &self.nvic_iser1
    }
    #[doc = "0xe180 - Clears or reads the enabled state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn nvic_icer0(&self) -> &NVIC_ICER0 {
        &self.nvic_icer0
    }
    #[doc = "0xe184 - Clears or reads the enabled state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn nvic_icer1(&self) -> &NVIC_ICER1 {
        &self.nvic_icer1
    }
    #[doc = "0xe200 - Enables or reads the pending state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn nvic_ispr0(&self) -> &NVIC_ISPR0 {
        &self.nvic_ispr0
    }
    #[doc = "0xe204 - Enables or reads the pending state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn nvic_ispr1(&self) -> &NVIC_ISPR1 {
        &self.nvic_ispr1
    }
    #[doc = "0xe280 - Clears or reads the pending state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn nvic_icpr0(&self) -> &NVIC_ICPR0 {
        &self.nvic_icpr0
    }
    #[doc = "0xe284 - Clears or reads the pending state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn nvic_icpr1(&self) -> &NVIC_ICPR1 {
        &self.nvic_icpr1
    }
    #[doc = "0xe300 - For each group of 32 interrupts, shows the active state of each interrupt"]
    #[inline(always)]
    pub const fn nvic_iabr0(&self) -> &NVIC_IABR0 {
        &self.nvic_iabr0
    }
    #[doc = "0xe304 - For each group of 32 interrupts, shows the active state of each interrupt"]
    #[inline(always)]
    pub const fn nvic_iabr1(&self) -> &NVIC_IABR1 {
        &self.nvic_iabr1
    }
    #[doc = "0xe380 - For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
    #[inline(always)]
    pub const fn nvic_itns0(&self) -> &NVIC_ITNS0 {
        &self.nvic_itns0
    }
    #[doc = "0xe384 - For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
    #[inline(always)]
    pub const fn nvic_itns1(&self) -> &NVIC_ITNS1 {
        &self.nvic_itns1
    }
    #[doc = "0xe400 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr0(&self) -> &NVIC_IPR0 {
        &self.nvic_ipr0
    }
    #[doc = "0xe404 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr1(&self) -> &NVIC_IPR1 {
        &self.nvic_ipr1
    }
    #[doc = "0xe408 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr2(&self) -> &NVIC_IPR2 {
        &self.nvic_ipr2
    }
    #[doc = "0xe40c - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr3(&self) -> &NVIC_IPR3 {
        &self.nvic_ipr3
    }
    #[doc = "0xe410 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr4(&self) -> &NVIC_IPR4 {
        &self.nvic_ipr4
    }
    #[doc = "0xe414 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr5(&self) -> &NVIC_IPR5 {
        &self.nvic_ipr5
    }
    #[doc = "0xe418 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr6(&self) -> &NVIC_IPR6 {
        &self.nvic_ipr6
    }
    #[doc = "0xe41c - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr7(&self) -> &NVIC_IPR7 {
        &self.nvic_ipr7
    }
    #[doc = "0xe420 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr8(&self) -> &NVIC_IPR8 {
        &self.nvic_ipr8
    }
    #[doc = "0xe424 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr9(&self) -> &NVIC_IPR9 {
        &self.nvic_ipr9
    }
    #[doc = "0xe428 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr10(&self) -> &NVIC_IPR10 {
        &self.nvic_ipr10
    }
    #[doc = "0xe42c - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr11(&self) -> &NVIC_IPR11 {
        &self.nvic_ipr11
    }
    #[doc = "0xe430 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr12(&self) -> &NVIC_IPR12 {
        &self.nvic_ipr12
    }
    #[doc = "0xe434 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr13(&self) -> &NVIC_IPR13 {
        &self.nvic_ipr13
    }
    #[doc = "0xe438 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr14(&self) -> &NVIC_IPR14 {
        &self.nvic_ipr14
    }
    #[doc = "0xe43c - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn nvic_ipr15(&self) -> &NVIC_IPR15 {
        &self.nvic_ipr15
    }
    #[doc = "0xed00 - Provides identification information for the PE, including an implementer code for the device and a device ID number"]
    #[inline(always)]
    pub const fn cpuid(&self) -> &CPUID {
        &self.cpuid
    }
    #[doc = "0xed04 - Controls and provides status information for NMI, PendSV, SysTick and interrupts"]
    #[inline(always)]
    pub const fn icsr(&self) -> &ICSR {
        &self.icsr
    }
    #[doc = "0xed08 - The VTOR indicates the offset of the vector table base address from memory address 0x00000000."]
    #[inline(always)]
    pub const fn vtor(&self) -> &VTOR {
        &self.vtor
    }
    #[doc = "0xed0c - Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset."]
    #[inline(always)]
    pub const fn aircr(&self) -> &AIRCR {
        &self.aircr
    }
    #[doc = "0xed10 - System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states."]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    #[doc = "0xed14 - Sets or returns configuration and control data"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0xed18 - Sets or returns priority for system handlers 4 - 7"]
    #[inline(always)]
    pub const fn shpr1(&self) -> &SHPR1 {
        &self.shpr1
    }
    #[doc = "0xed1c - Sets or returns priority for system handlers 8 - 11"]
    #[inline(always)]
    pub const fn shpr2(&self) -> &SHPR2 {
        &self.shpr2
    }
    #[doc = "0xed20 - Sets or returns priority for system handlers 12 - 15"]
    #[inline(always)]
    pub const fn shpr3(&self) -> &SHPR3 {
        &self.shpr3
    }
    #[doc = "0xed24 - Provides access to the active and pending status of system exceptions"]
    #[inline(always)]
    pub const fn shcsr(&self) -> &SHCSR {
        &self.shcsr
    }
    #[doc = "0xed28 - Contains the three Configurable Fault Status Registers. 31:16 UFSR: Provides information on UsageFault exceptions 15:8 BFSR: Provides information on BusFault exceptions 7:0 MMFSR: Provides information on MemManage exceptions"]
    #[inline(always)]
    pub const fn cfsr(&self) -> &CFSR {
        &self.cfsr
    }
    #[doc = "0xed2c - Shows the cause of any HardFaults"]
    #[inline(always)]
    pub const fn hfsr(&self) -> &HFSR {
        &self.hfsr
    }
    #[doc = "0xed30 - Shows which debug event occurred"]
    #[inline(always)]
    pub const fn dfsr(&self) -> &DFSR {
        &self.dfsr
    }
    #[doc = "0xed34 - Shows the address of the memory location that caused an MPU fault"]
    #[inline(always)]
    pub const fn mmfar(&self) -> &MMFAR {
        &self.mmfar
    }
    #[doc = "0xed38 - Shows the address associated with a precise data access BusFault"]
    #[inline(always)]
    pub const fn bfar(&self) -> &BFAR {
        &self.bfar
    }
    #[doc = "0xed40 - Gives top-level information about the instruction set supported by the PE"]
    #[inline(always)]
    pub const fn id_pfr0(&self) -> &ID_PFR0 {
        &self.id_pfr0
    }
    #[doc = "0xed44 - Gives information about the programmers' model and Extensions support"]
    #[inline(always)]
    pub const fn id_pfr1(&self) -> &ID_PFR1 {
        &self.id_pfr1
    }
    #[doc = "0xed48 - Provides top level information about the debug system"]
    #[inline(always)]
    pub const fn id_dfr0(&self) -> &ID_DFR0 {
        &self.id_dfr0
    }
    #[doc = "0xed4c - Provides information about the IMPLEMENTATION DEFINED features of the PE"]
    #[inline(always)]
    pub const fn id_afr0(&self) -> &ID_AFR0 {
        &self.id_afr0
    }
    #[doc = "0xed50 - Provides information about the implemented memory model and memory management support"]
    #[inline(always)]
    pub const fn id_mmfr0(&self) -> &ID_MMFR0 {
        &self.id_mmfr0
    }
    #[doc = "0xed54 - Provides information about the implemented memory model and memory management support"]
    #[inline(always)]
    pub const fn id_mmfr1(&self) -> &ID_MMFR1 {
        &self.id_mmfr1
    }
    #[doc = "0xed58 - Provides information about the implemented memory model and memory management support"]
    #[inline(always)]
    pub const fn id_mmfr2(&self) -> &ID_MMFR2 {
        &self.id_mmfr2
    }
    #[doc = "0xed5c - Provides information about the implemented memory model and memory management support"]
    #[inline(always)]
    pub const fn id_mmfr3(&self) -> &ID_MMFR3 {
        &self.id_mmfr3
    }
    #[doc = "0xed60 - Provides information about the instruction set implemented by the PE"]
    #[inline(always)]
    pub const fn id_isar0(&self) -> &ID_ISAR0 {
        &self.id_isar0
    }
    #[doc = "0xed64 - Provides information about the instruction set implemented by the PE"]
    #[inline(always)]
    pub const fn id_isar1(&self) -> &ID_ISAR1 {
        &self.id_isar1
    }
    #[doc = "0xed68 - Provides information about the instruction set implemented by the PE"]
    #[inline(always)]
    pub const fn id_isar2(&self) -> &ID_ISAR2 {
        &self.id_isar2
    }
    #[doc = "0xed6c - Provides information about the instruction set implemented by the PE"]
    #[inline(always)]
    pub const fn id_isar3(&self) -> &ID_ISAR3 {
        &self.id_isar3
    }
    #[doc = "0xed70 - Provides information about the instruction set implemented by the PE"]
    #[inline(always)]
    pub const fn id_isar4(&self) -> &ID_ISAR4 {
        &self.id_isar4
    }
    #[doc = "0xed74 - Provides information about the instruction set implemented by the PE"]
    #[inline(always)]
    pub const fn id_isar5(&self) -> &ID_ISAR5 {
        &self.id_isar5
    }
    #[doc = "0xed7c - Provides information about the architecture of the caches. CTR is RES0 if CLIDR is zero."]
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        &self.ctr
    }
    #[doc = "0xed88 - Specifies the access privileges for coprocessors and the FP Extension"]
    #[inline(always)]
    pub const fn cpacr(&self) -> &CPACR {
        &self.cpacr
    }
    #[doc = "0xed8c - Defines the Non-secure access permissions for both the FP Extension and coprocessors CP0 to CP7"]
    #[inline(always)]
    pub const fn nsacr(&self) -> &NSACR {
        &self.nsacr
    }
    #[doc = "0xed90 - The MPU Type Register indicates how many regions the MPU `FTSSS supports"]
    #[inline(always)]
    pub const fn mpu_type(&self) -> &MPU_TYPE {
        &self.mpu_type
    }
    #[doc = "0xed94 - Enables the MPU and, when the MPU is enabled, controls whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
    #[inline(always)]
    pub const fn mpu_ctrl(&self) -> &MPU_CTRL {
        &self.mpu_ctrl
    }
    #[doc = "0xed98 - Selects the region currently accessed by MPU_RBAR and MPU_RLAR"]
    #[inline(always)]
    pub const fn mpu_rnr(&self) -> &MPU_RNR {
        &self.mpu_rnr
    }
    #[doc = "0xed9c - Provides indirect read and write access to the base address of the currently selected MPU region `FTSSS"]
    #[inline(always)]
    pub const fn mpu_rbar(&self) -> &MPU_RBAR {
        &self.mpu_rbar
    }
    #[doc = "0xeda0 - Provides indirect read and write access to the limit address of the currently selected MPU region `FTSSS"]
    #[inline(always)]
    pub const fn mpu_rlar(&self) -> &MPU_RLAR {
        &self.mpu_rlar
    }
    #[doc = "0xeda4 - Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\]) `FTSSS"]
    #[inline(always)]
    pub const fn mpu_rbar_a1(&self) -> &MPU_RBAR_A1 {
        &self.mpu_rbar_a1
    }
    #[doc = "0xeda8 - Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\]) `FTSSS"]
    #[inline(always)]
    pub const fn mpu_rlar_a1(&self) -> &MPU_RLAR_A1 {
        &self.mpu_rlar_a1
    }
    #[doc = "0xedac - Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\]) `FTSSS"]
    #[inline(always)]
    pub const fn mpu_rbar_a2(&self) -> &MPU_RBAR_A2 {
        &self.mpu_rbar_a2
    }
    #[doc = "0xedb0 - Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\]) `FTSSS"]
    #[inline(always)]
    pub const fn mpu_rlar_a2(&self) -> &MPU_RLAR_A2 {
        &self.mpu_rlar_a2
    }
    #[doc = "0xedb4 - Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\]) `FTSSS"]
    #[inline(always)]
    pub const fn mpu_rbar_a3(&self) -> &MPU_RBAR_A3 {
        &self.mpu_rbar_a3
    }
    #[doc = "0xedb8 - Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\]) `FTSSS"]
    #[inline(always)]
    pub const fn mpu_rlar_a3(&self) -> &MPU_RLAR_A3 {
        &self.mpu_rlar_a3
    }
    #[doc = "0xedc0 - Along with MPU_MAIR1, provides the memory attribute encodings corresponding to the AttrIndex values"]
    #[inline(always)]
    pub const fn mpu_mair0(&self) -> &MPU_MAIR0 {
        &self.mpu_mair0
    }
    #[doc = "0xedc4 - Along with MPU_MAIR0, provides the memory attribute encodings corresponding to the AttrIndex values"]
    #[inline(always)]
    pub const fn mpu_mair1(&self) -> &MPU_MAIR1 {
        &self.mpu_mair1
    }
    #[doc = "0xedd0 - Allows enabling of the Security Attribution Unit"]
    #[inline(always)]
    pub const fn sau_ctrl(&self) -> &SAU_CTRL {
        &self.sau_ctrl
    }
    #[doc = "0xedd4 - Indicates the number of regions implemented by the Security Attribution Unit"]
    #[inline(always)]
    pub const fn sau_type(&self) -> &SAU_TYPE {
        &self.sau_type
    }
    #[doc = "0xedd8 - Selects the region currently accessed by SAU_RBAR and SAU_RLAR"]
    #[inline(always)]
    pub const fn sau_rnr(&self) -> &SAU_RNR {
        &self.sau_rnr
    }
    #[doc = "0xeddc - Provides indirect read and write access to the base address of the currently selected SAU region"]
    #[inline(always)]
    pub const fn sau_rbar(&self) -> &SAU_RBAR {
        &self.sau_rbar
    }
    #[doc = "0xede0 - Provides indirect read and write access to the limit address of the currently selected SAU region"]
    #[inline(always)]
    pub const fn sau_rlar(&self) -> &SAU_RLAR {
        &self.sau_rlar
    }
    #[doc = "0xede4 - Provides information about any security related faults"]
    #[inline(always)]
    pub const fn sfsr(&self) -> &SFSR {
        &self.sfsr
    }
    #[doc = "0xede8 - Shows the address of the memory location that caused a Security violation"]
    #[inline(always)]
    pub const fn sfar(&self) -> &SFAR {
        &self.sfar
    }
    #[doc = "0xedf0 - Controls halting debug"]
    #[inline(always)]
    pub const fn dhcsr(&self) -> &DHCSR {
        &self.dhcsr
    }
    #[doc = "0xedf4 - With the DCRDR, provides debug access to the general-purpose registers, special-purpose registers, and the FP extension registers. A write to the DCRSR specifies the register to transfer, whether the transfer is a read or write, and starts the transfer"]
    #[inline(always)]
    pub const fn dcrsr(&self) -> &DCRSR {
        &self.dcrsr
    }
    #[doc = "0xedf8 - With the DCRSR, provides debug access to the general-purpose registers, special-purpose registers, and the FP Extension registers. If the Main Extension is implemented, it can also be used for message passing between an external debugger and a debug agent running on the PE"]
    #[inline(always)]
    pub const fn dcrdr(&self) -> &DCRDR {
        &self.dcrdr
    }
    #[doc = "0xedfc - Manages vector catch behavior and DebugMonitor handling when debugging"]
    #[inline(always)]
    pub const fn demcr(&self) -> &DEMCR {
        &self.demcr
    }
    #[doc = "0xee08 - Provides control and status information for Secure debug"]
    #[inline(always)]
    pub const fn dscsr(&self) -> &DSCSR {
        &self.dscsr
    }
    #[doc = "0xef00 - Provides a mechanism for software to generate an interrupt"]
    #[inline(always)]
    pub const fn stir(&self) -> &STIR {
        &self.stir
    }
    #[doc = "0xef34 - Holds control data for the Floating-point extension"]
    #[inline(always)]
    pub const fn fpccr(&self) -> &FPCCR {
        &self.fpccr
    }
    #[doc = "0xef38 - Holds the location of the unpopulated floating-point register space allocated on an exception stack frame"]
    #[inline(always)]
    pub const fn fpcar(&self) -> &FPCAR {
        &self.fpcar
    }
    #[doc = "0xef3c - Holds the default values for the floating-point status control data that the PE assigns to the FPSCR when it creates a new floating-point context"]
    #[inline(always)]
    pub const fn fpdscr(&self) -> &FPDSCR {
        &self.fpdscr
    }
    #[doc = "0xef40 - Describes the features provided by the Floating-point Extension"]
    #[inline(always)]
    pub const fn mvfr0(&self) -> &MVFR0 {
        &self.mvfr0
    }
    #[doc = "0xef44 - Describes the features provided by the Floating-point Extension"]
    #[inline(always)]
    pub const fn mvfr1(&self) -> &MVFR1 {
        &self.mvfr1
    }
    #[doc = "0xef48 - Describes the features provided by the Floating-point Extension"]
    #[inline(always)]
    pub const fn mvfr2(&self) -> &MVFR2 {
        &self.mvfr2
    }
    #[doc = "0xefbc - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn ddevarch(&self) -> &DDEVARCH {
        &self.ddevarch
    }
    #[doc = "0xefcc - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn ddevtype(&self) -> &DDEVTYPE {
        &self.ddevtype
    }
    #[doc = "0xefd0 - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dpidr4(&self) -> &DPIDR4 {
        &self.dpidr4
    }
    #[doc = "0xefd4 - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dpidr5(&self) -> &DPIDR5 {
        &self.dpidr5
    }
    #[doc = "0xefd8 - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dpidr6(&self) -> &DPIDR6 {
        &self.dpidr6
    }
    #[doc = "0xefdc - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dpidr7(&self) -> &DPIDR7 {
        &self.dpidr7
    }
    #[doc = "0xefe0 - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dpidr0(&self) -> &DPIDR0 {
        &self.dpidr0
    }
    #[doc = "0xefe4 - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dpidr1(&self) -> &DPIDR1 {
        &self.dpidr1
    }
    #[doc = "0xefe8 - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dpidr2(&self) -> &DPIDR2 {
        &self.dpidr2
    }
    #[doc = "0xefec - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dpidr3(&self) -> &DPIDR3 {
        &self.dpidr3
    }
    #[doc = "0xeff0 - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dcidr0(&self) -> &DCIDR0 {
        &self.dcidr0
    }
    #[doc = "0xeff4 - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dcidr1(&self) -> &DCIDR1 {
        &self.dcidr1
    }
    #[doc = "0xeff8 - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dcidr2(&self) -> &DCIDR2 {
        &self.dcidr2
    }
    #[doc = "0xeffc - Provides CoreSight discovery information for the SCS"]
    #[inline(always)]
    pub const fn dcidr3(&self) -> &DCIDR3 {
        &self.dcidr3
    }
    #[doc = "0x41004 - Programming Control Register"]
    #[inline(always)]
    pub const fn trcprgctlr(&self) -> &TRCPRGCTLR {
        &self.trcprgctlr
    }
    #[doc = "0x4100c - The TRCSTATR indicates the ETM-Teal status"]
    #[inline(always)]
    pub const fn trcstatr(&self) -> &TRCSTATR {
        &self.trcstatr
    }
    #[doc = "0x41010 - The TRCCONFIGR sets the basic tracing options for the trace unit"]
    #[inline(always)]
    pub const fn trcconfigr(&self) -> &TRCCONFIGR {
        &self.trcconfigr
    }
    #[doc = "0x41020 - The TRCEVENTCTL0R controls the tracing of events in the trace stream. The events also drive the ETM-Teal external outputs."]
    #[inline(always)]
    pub const fn trceventctl0r(&self) -> &TRCEVENTCTL0R {
        &self.trceventctl0r
    }
    #[doc = "0x41024 - The TRCEVENTCTL1R controls how the events selected by TRCEVENTCTL0R behave"]
    #[inline(always)]
    pub const fn trceventctl1r(&self) -> &TRCEVENTCTL1R {
        &self.trceventctl1r
    }
    #[doc = "0x4102c - The TRCSTALLCTLR enables ETM-Teal to stall the processor if the ETM-Teal FIFO goes over the programmed level to minimize risk of overflow"]
    #[inline(always)]
    pub const fn trcstallctlr(&self) -> &TRCSTALLCTLR {
        &self.trcstallctlr
    }
    #[doc = "0x41030 - The TRCTSCTLR controls the insertion of global timestamps into the trace stream. A timestamp is always inserted into the instruction trace stream"]
    #[inline(always)]
    pub const fn trctsctlr(&self) -> &TRCTSCTLR {
        &self.trctsctlr
    }
    #[doc = "0x41034 - The TRCSYNCPR specifies the period of trace synchronization of the trace streams. TRCSYNCPR defines a number of bytes of trace between requests for trace synchronization. This value is always a power of two"]
    #[inline(always)]
    pub const fn trcsyncpr(&self) -> &TRCSYNCPR {
        &self.trcsyncpr
    }
    #[doc = "0x41038 - The TRCCCCTLR sets the threshold value for instruction trace cycle counting. The threshold represents the minimum interval between cycle count trace packets"]
    #[inline(always)]
    pub const fn trcccctlr(&self) -> &TRCCCCTLR {
        &self.trcccctlr
    }
    #[doc = "0x41080 - The TRCVICTLR controls instruction trace filtering"]
    #[inline(always)]
    pub const fn trcvictlr(&self) -> &TRCVICTLR {
        &self.trcvictlr
    }
    #[doc = "0x41140 - The TRCCNTRLDVR defines the reload value for the reduced function counter"]
    #[inline(always)]
    pub const fn trccntrldvr0(&self) -> &TRCCNTRLDVR0 {
        &self.trccntrldvr0
    }
    #[doc = "0x41180 - TRCIDR8"]
    #[inline(always)]
    pub const fn trcidr8(&self) -> &TRCIDR8 {
        &self.trcidr8
    }
    #[doc = "0x41184 - TRCIDR9"]
    #[inline(always)]
    pub const fn trcidr9(&self) -> &TRCIDR9 {
        &self.trcidr9
    }
    #[doc = "0x41188 - TRCIDR10"]
    #[inline(always)]
    pub const fn trcidr10(&self) -> &TRCIDR10 {
        &self.trcidr10
    }
    #[doc = "0x4118c - TRCIDR11"]
    #[inline(always)]
    pub const fn trcidr11(&self) -> &TRCIDR11 {
        &self.trcidr11
    }
    #[doc = "0x41190 - TRCIDR12"]
    #[inline(always)]
    pub const fn trcidr12(&self) -> &TRCIDR12 {
        &self.trcidr12
    }
    #[doc = "0x41194 - TRCIDR13"]
    #[inline(always)]
    pub const fn trcidr13(&self) -> &TRCIDR13 {
        &self.trcidr13
    }
    #[doc = "0x411c0 - The TRCIMSPEC shows the presence of any IMPLEMENTATION SPECIFIC features, and enables any features that are provided"]
    #[inline(always)]
    pub const fn trcimspec(&self) -> &TRCIMSPEC {
        &self.trcimspec
    }
    #[doc = "0x411e0 - TRCIDR0"]
    #[inline(always)]
    pub const fn trcidr0(&self) -> &TRCIDR0 {
        &self.trcidr0
    }
    #[doc = "0x411e4 - TRCIDR1"]
    #[inline(always)]
    pub const fn trcidr1(&self) -> &TRCIDR1 {
        &self.trcidr1
    }
    #[doc = "0x411e8 - TRCIDR2"]
    #[inline(always)]
    pub const fn trcidr2(&self) -> &TRCIDR2 {
        &self.trcidr2
    }
    #[doc = "0x411ec - TRCIDR3"]
    #[inline(always)]
    pub const fn trcidr3(&self) -> &TRCIDR3 {
        &self.trcidr3
    }
    #[doc = "0x411f0 - TRCIDR4"]
    #[inline(always)]
    pub const fn trcidr4(&self) -> &TRCIDR4 {
        &self.trcidr4
    }
    #[doc = "0x411f4 - TRCIDR5"]
    #[inline(always)]
    pub const fn trcidr5(&self) -> &TRCIDR5 {
        &self.trcidr5
    }
    #[doc = "0x411f8 - TRCIDR6"]
    #[inline(always)]
    pub const fn trcidr6(&self) -> &TRCIDR6 {
        &self.trcidr6
    }
    #[doc = "0x411fc - TRCIDR7"]
    #[inline(always)]
    pub const fn trcidr7(&self) -> &TRCIDR7 {
        &self.trcidr7
    }
    #[doc = "0x41208 - The TRCRSCTLR controls the trace resources"]
    #[inline(always)]
    pub const fn trcrsctlr2(&self) -> &TRCRSCTLR2 {
        &self.trcrsctlr2
    }
    #[doc = "0x4120c - The TRCRSCTLR controls the trace resources"]
    #[inline(always)]
    pub const fn trcrsctlr3(&self) -> &TRCRSCTLR3 {
        &self.trcrsctlr3
    }
    #[doc = "0x412a0 - Controls the corresponding single-shot comparator resource"]
    #[inline(always)]
    pub const fn trcsscsr(&self) -> &TRCSSCSR {
        &self.trcsscsr
    }
    #[doc = "0x412c0 - Selects the PE comparator inputs for Single-shot control"]
    #[inline(always)]
    pub const fn trcsspcicr(&self) -> &TRCSSPCICR {
        &self.trcsspcicr
    }
    #[doc = "0x41310 - Requests the system to provide power to the trace unit"]
    #[inline(always)]
    pub const fn trcpdcr(&self) -> &TRCPDCR {
        &self.trcpdcr
    }
    #[doc = "0x41314 - Returns the following information about the trace unit: - OS Lock status. - Core power domain status. - Power interruption status"]
    #[inline(always)]
    pub const fn trcpdsr(&self) -> &TRCPDSR {
        &self.trcpdsr
    }
    #[doc = "0x41ee4 - Trace Integration ATB Identification Register"]
    #[inline(always)]
    pub const fn trcitatbidr(&self) -> &TRCITATBIDR {
        &self.trcitatbidr
    }
    #[doc = "0x41ef4 - Trace Integration Instruction ATB In Register"]
    #[inline(always)]
    pub const fn trcitiatbinr(&self) -> &TRCITIATBINR {
        &self.trcitiatbinr
    }
    #[doc = "0x41efc - Trace Integration Instruction ATB Out Register"]
    #[inline(always)]
    pub const fn trcitiatboutr(&self) -> &TRCITIATBOUTR {
        &self.trcitiatboutr
    }
    #[doc = "0x41fa0 - Claim Tag Set Register"]
    #[inline(always)]
    pub const fn trcclaimset(&self) -> &TRCCLAIMSET {
        &self.trcclaimset
    }
    #[doc = "0x41fa4 - Claim Tag Clear Register"]
    #[inline(always)]
    pub const fn trcclaimclr(&self) -> &TRCCLAIMCLR {
        &self.trcclaimclr
    }
    #[doc = "0x41fb8 - Returns the level of tracing that the trace unit can support"]
    #[inline(always)]
    pub const fn trcauthstatus(&self) -> &TRCAUTHSTATUS {
        &self.trcauthstatus
    }
    #[doc = "0x41fbc - TRCDEVARCH"]
    #[inline(always)]
    pub const fn trcdevarch(&self) -> &TRCDEVARCH {
        &self.trcdevarch
    }
    #[doc = "0x41fc8 - TRCDEVID"]
    #[inline(always)]
    pub const fn trcdevid(&self) -> &TRCDEVID {
        &self.trcdevid
    }
    #[doc = "0x41fcc - TRCDEVTYPE"]
    #[inline(always)]
    pub const fn trcdevtype(&self) -> &TRCDEVTYPE {
        &self.trcdevtype
    }
    #[doc = "0x41fd0 - TRCPIDR4"]
    #[inline(always)]
    pub const fn trcpidr4(&self) -> &TRCPIDR4 {
        &self.trcpidr4
    }
    #[doc = "0x41fd4 - TRCPIDR5"]
    #[inline(always)]
    pub const fn trcpidr5(&self) -> &TRCPIDR5 {
        &self.trcpidr5
    }
    #[doc = "0x41fd8 - TRCPIDR6"]
    #[inline(always)]
    pub const fn trcpidr6(&self) -> &TRCPIDR6 {
        &self.trcpidr6
    }
    #[doc = "0x41fdc - TRCPIDR7"]
    #[inline(always)]
    pub const fn trcpidr7(&self) -> &TRCPIDR7 {
        &self.trcpidr7
    }
    #[doc = "0x41fe0 - TRCPIDR0"]
    #[inline(always)]
    pub const fn trcpidr0(&self) -> &TRCPIDR0 {
        &self.trcpidr0
    }
    #[doc = "0x41fe4 - TRCPIDR1"]
    #[inline(always)]
    pub const fn trcpidr1(&self) -> &TRCPIDR1 {
        &self.trcpidr1
    }
    #[doc = "0x41fe8 - TRCPIDR2"]
    #[inline(always)]
    pub const fn trcpidr2(&self) -> &TRCPIDR2 {
        &self.trcpidr2
    }
    #[doc = "0x41fec - TRCPIDR3"]
    #[inline(always)]
    pub const fn trcpidr3(&self) -> &TRCPIDR3 {
        &self.trcpidr3
    }
    #[doc = "0x41ff0 - TRCCIDR0"]
    #[inline(always)]
    pub const fn trccidr0(&self) -> &TRCCIDR0 {
        &self.trccidr0
    }
    #[doc = "0x41ff4 - TRCCIDR1"]
    #[inline(always)]
    pub const fn trccidr1(&self) -> &TRCCIDR1 {
        &self.trccidr1
    }
    #[doc = "0x41ff8 - TRCCIDR2"]
    #[inline(always)]
    pub const fn trccidr2(&self) -> &TRCCIDR2 {
        &self.trccidr2
    }
    #[doc = "0x41ffc - TRCCIDR3"]
    #[inline(always)]
    pub const fn trccidr3(&self) -> &TRCCIDR3 {
        &self.trccidr3
    }
    #[doc = "0x42000 - CTI Control Register"]
    #[inline(always)]
    pub const fn cticontrol(&self) -> &CTICONTROL {
        &self.cticontrol
    }
    #[doc = "0x42010 - CTI Interrupt Acknowledge Register"]
    #[inline(always)]
    pub const fn ctiintack(&self) -> &CTIINTACK {
        &self.ctiintack
    }
    #[doc = "0x42014 - CTI Application Trigger Set Register"]
    #[inline(always)]
    pub const fn ctiappset(&self) -> &CTIAPPSET {
        &self.ctiappset
    }
    #[doc = "0x42018 - CTI Application Trigger Clear Register"]
    #[inline(always)]
    pub const fn ctiappclear(&self) -> &CTIAPPCLEAR {
        &self.ctiappclear
    }
    #[doc = "0x4201c - CTI Application Pulse Register"]
    #[inline(always)]
    pub const fn ctiapppulse(&self) -> &CTIAPPPULSE {
        &self.ctiapppulse
    }
    #[doc = "0x42020 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiinen0(&self) -> &CTIINEN0 {
        &self.ctiinen0
    }
    #[doc = "0x42024 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiinen1(&self) -> &CTIINEN1 {
        &self.ctiinen1
    }
    #[doc = "0x42028 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiinen2(&self) -> &CTIINEN2 {
        &self.ctiinen2
    }
    #[doc = "0x4202c - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiinen3(&self) -> &CTIINEN3 {
        &self.ctiinen3
    }
    #[doc = "0x42030 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiinen4(&self) -> &CTIINEN4 {
        &self.ctiinen4
    }
    #[doc = "0x42034 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiinen5(&self) -> &CTIINEN5 {
        &self.ctiinen5
    }
    #[doc = "0x42038 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiinen6(&self) -> &CTIINEN6 {
        &self.ctiinen6
    }
    #[doc = "0x4203c - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiinen7(&self) -> &CTIINEN7 {
        &self.ctiinen7
    }
    #[doc = "0x420a0 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiouten0(&self) -> &CTIOUTEN0 {
        &self.ctiouten0
    }
    #[doc = "0x420a4 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiouten1(&self) -> &CTIOUTEN1 {
        &self.ctiouten1
    }
    #[doc = "0x420a8 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiouten2(&self) -> &CTIOUTEN2 {
        &self.ctiouten2
    }
    #[doc = "0x420ac - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiouten3(&self) -> &CTIOUTEN3 {
        &self.ctiouten3
    }
    #[doc = "0x420b0 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiouten4(&self) -> &CTIOUTEN4 {
        &self.ctiouten4
    }
    #[doc = "0x420b4 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiouten5(&self) -> &CTIOUTEN5 {
        &self.ctiouten5
    }
    #[doc = "0x420b8 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiouten6(&self) -> &CTIOUTEN6 {
        &self.ctiouten6
    }
    #[doc = "0x420bc - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctiouten7(&self) -> &CTIOUTEN7 {
        &self.ctiouten7
    }
    #[doc = "0x42130 - CTI Trigger to Channel Enable Registers"]
    #[inline(always)]
    pub const fn ctitriginstatus(&self) -> &CTITRIGINSTATUS {
        &self.ctitriginstatus
    }
    #[doc = "0x42134 - CTI Trigger In Status Register"]
    #[inline(always)]
    pub const fn ctitrigoutstatus(&self) -> &CTITRIGOUTSTATUS {
        &self.ctitrigoutstatus
    }
    #[doc = "0x42138 - CTI Channel In Status Register"]
    #[inline(always)]
    pub const fn ctichinstatus(&self) -> &CTICHINSTATUS {
        &self.ctichinstatus
    }
    #[doc = "0x42140 - Enable CTI Channel Gate register"]
    #[inline(always)]
    pub const fn ctigate(&self) -> &CTIGATE {
        &self.ctigate
    }
    #[doc = "0x42144 - External Multiplexer Control register"]
    #[inline(always)]
    pub const fn asicctl(&self) -> &ASICCTL {
        &self.asicctl
    }
    #[doc = "0x42ee4 - Integration Test Channel Output register"]
    #[inline(always)]
    pub const fn itchout(&self) -> &ITCHOUT {
        &self.itchout
    }
    #[doc = "0x42ee8 - Integration Test Trigger Output register"]
    #[inline(always)]
    pub const fn ittrigout(&self) -> &ITTRIGOUT {
        &self.ittrigout
    }
    #[doc = "0x42ef4 - Integration Test Channel Input register"]
    #[inline(always)]
    pub const fn itchin(&self) -> &ITCHIN {
        &self.itchin
    }
    #[doc = "0x42f00 - Integration Mode Control register"]
    #[inline(always)]
    pub const fn itctrl(&self) -> &ITCTRL {
        &self.itctrl
    }
    #[doc = "0x42fbc - Device Architecture register"]
    #[inline(always)]
    pub const fn devarch(&self) -> &DEVARCH {
        &self.devarch
    }
    #[doc = "0x42fc8 - Device Configuration register"]
    #[inline(always)]
    pub const fn devid(&self) -> &DEVID {
        &self.devid
    }
    #[doc = "0x42fcc - Device Type Identifier register"]
    #[inline(always)]
    pub const fn devtype(&self) -> &DEVTYPE {
        &self.devtype
    }
    #[doc = "0x42fd0 - CoreSight Peripheral ID4"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &PIDR4 {
        &self.pidr4
    }
    #[doc = "0x42fd4 - CoreSight Peripheral ID5"]
    #[inline(always)]
    pub const fn pidr5(&self) -> &PIDR5 {
        &self.pidr5
    }
    #[doc = "0x42fd8 - CoreSight Peripheral ID6"]
    #[inline(always)]
    pub const fn pidr6(&self) -> &PIDR6 {
        &self.pidr6
    }
    #[doc = "0x42fdc - CoreSight Peripheral ID7"]
    #[inline(always)]
    pub const fn pidr7(&self) -> &PIDR7 {
        &self.pidr7
    }
    #[doc = "0x42fe0 - CoreSight Peripheral ID0"]
    #[inline(always)]
    pub const fn pidr0(&self) -> &PIDR0 {
        &self.pidr0
    }
    #[doc = "0x42fe4 - CoreSight Peripheral ID1"]
    #[inline(always)]
    pub const fn pidr1(&self) -> &PIDR1 {
        &self.pidr1
    }
    #[doc = "0x42fe8 - CoreSight Peripheral ID2"]
    #[inline(always)]
    pub const fn pidr2(&self) -> &PIDR2 {
        &self.pidr2
    }
    #[doc = "0x42fec - CoreSight Peripheral ID3"]
    #[inline(always)]
    pub const fn pidr3(&self) -> &PIDR3 {
        &self.pidr3
    }
    #[doc = "0x42ff0 - CoreSight Component ID0"]
    #[inline(always)]
    pub const fn cidr0(&self) -> &CIDR0 {
        &self.cidr0
    }
    #[doc = "0x42ff4 - CoreSight Component ID1"]
    #[inline(always)]
    pub const fn cidr1(&self) -> &CIDR1 {
        &self.cidr1
    }
    #[doc = "0x42ff8 - CoreSight Component ID2"]
    #[inline(always)]
    pub const fn cidr2(&self) -> &CIDR2 {
        &self.cidr2
    }
    #[doc = "0x42ffc - CoreSight Component ID3"]
    #[inline(always)]
    pub const fn cidr3(&self) -> &CIDR3 {
        &self.cidr3
    }
}
#[doc = "ITM_STIM0 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim0`]
module"]
pub type ITM_STIM0 = crate::Reg<itm_stim0::ITM_STIM0_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim0;
#[doc = "ITM_STIM1 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim1`]
module"]
pub type ITM_STIM1 = crate::Reg<itm_stim1::ITM_STIM1_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim1;
#[doc = "ITM_STIM2 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim2`]
module"]
pub type ITM_STIM2 = crate::Reg<itm_stim2::ITM_STIM2_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim2;
#[doc = "ITM_STIM3 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim3`]
module"]
pub type ITM_STIM3 = crate::Reg<itm_stim3::ITM_STIM3_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim3;
#[doc = "ITM_STIM4 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim4`]
module"]
pub type ITM_STIM4 = crate::Reg<itm_stim4::ITM_STIM4_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim4;
#[doc = "ITM_STIM5 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim5`]
module"]
pub type ITM_STIM5 = crate::Reg<itm_stim5::ITM_STIM5_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim5;
#[doc = "ITM_STIM6 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim6`]
module"]
pub type ITM_STIM6 = crate::Reg<itm_stim6::ITM_STIM6_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim6;
#[doc = "ITM_STIM7 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim7`]
module"]
pub type ITM_STIM7 = crate::Reg<itm_stim7::ITM_STIM7_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim7;
#[doc = "ITM_STIM8 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim8`]
module"]
pub type ITM_STIM8 = crate::Reg<itm_stim8::ITM_STIM8_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim8;
#[doc = "ITM_STIM9 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim9`]
module"]
pub type ITM_STIM9 = crate::Reg<itm_stim9::ITM_STIM9_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim9;
#[doc = "ITM_STIM10 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim10`]
module"]
pub type ITM_STIM10 = crate::Reg<itm_stim10::ITM_STIM10_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim10;
#[doc = "ITM_STIM11 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim11`]
module"]
pub type ITM_STIM11 = crate::Reg<itm_stim11::ITM_STIM11_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim11;
#[doc = "ITM_STIM12 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim12`]
module"]
pub type ITM_STIM12 = crate::Reg<itm_stim12::ITM_STIM12_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim12;
#[doc = "ITM_STIM13 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim13`]
module"]
pub type ITM_STIM13 = crate::Reg<itm_stim13::ITM_STIM13_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim13;
#[doc = "ITM_STIM14 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim14`]
module"]
pub type ITM_STIM14 = crate::Reg<itm_stim14::ITM_STIM14_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim14;
#[doc = "ITM_STIM15 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim15`]
module"]
pub type ITM_STIM15 = crate::Reg<itm_stim15::ITM_STIM15_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim15;
#[doc = "ITM_STIM16 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim16`]
module"]
pub type ITM_STIM16 = crate::Reg<itm_stim16::ITM_STIM16_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim16;
#[doc = "ITM_STIM17 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim17`]
module"]
pub type ITM_STIM17 = crate::Reg<itm_stim17::ITM_STIM17_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim17;
#[doc = "ITM_STIM18 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim18`]
module"]
pub type ITM_STIM18 = crate::Reg<itm_stim18::ITM_STIM18_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim18;
#[doc = "ITM_STIM19 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim19`]
module"]
pub type ITM_STIM19 = crate::Reg<itm_stim19::ITM_STIM19_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim19;
#[doc = "ITM_STIM20 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim20`]
module"]
pub type ITM_STIM20 = crate::Reg<itm_stim20::ITM_STIM20_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim20;
#[doc = "ITM_STIM21 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim21`]
module"]
pub type ITM_STIM21 = crate::Reg<itm_stim21::ITM_STIM21_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim21;
#[doc = "ITM_STIM22 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim22`]
module"]
pub type ITM_STIM22 = crate::Reg<itm_stim22::ITM_STIM22_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim22;
#[doc = "ITM_STIM23 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim23`]
module"]
pub type ITM_STIM23 = crate::Reg<itm_stim23::ITM_STIM23_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim23;
#[doc = "ITM_STIM24 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim24`]
module"]
pub type ITM_STIM24 = crate::Reg<itm_stim24::ITM_STIM24_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim24;
#[doc = "ITM_STIM25 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim25`]
module"]
pub type ITM_STIM25 = crate::Reg<itm_stim25::ITM_STIM25_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim25;
#[doc = "ITM_STIM26 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim26`]
module"]
pub type ITM_STIM26 = crate::Reg<itm_stim26::ITM_STIM26_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim26;
#[doc = "ITM_STIM27 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim27`]
module"]
pub type ITM_STIM27 = crate::Reg<itm_stim27::ITM_STIM27_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim27;
#[doc = "ITM_STIM28 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim28`]
module"]
pub type ITM_STIM28 = crate::Reg<itm_stim28::ITM_STIM28_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim28;
#[doc = "ITM_STIM29 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim29`]
module"]
pub type ITM_STIM29 = crate::Reg<itm_stim29::ITM_STIM29_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim29;
#[doc = "ITM_STIM30 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim30`]
module"]
pub type ITM_STIM30 = crate::Reg<itm_stim30::ITM_STIM30_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim30;
#[doc = "ITM_STIM31 (rw) register accessor: Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_stim31`]
module"]
pub type ITM_STIM31 = crate::Reg<itm_stim31::ITM_STIM31_SPEC>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod itm_stim31;
#[doc = "ITM_TER0 (rw) register accessor: Provide an individual enable bit for each ITM_STIM register  

You can [`read`](crate::Reg::read) this register and get [`itm_ter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_ter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_ter0`]
module"]
pub type ITM_TER0 = crate::Reg<itm_ter0::ITM_TER0_SPEC>;
#[doc = "Provide an individual enable bit for each ITM_STIM register"]
pub mod itm_ter0;
#[doc = "ITM_TPR (rw) register accessor: Controls which stimulus ports can be accessed by unprivileged code  

You can [`read`](crate::Reg::read) this register and get [`itm_tpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_tpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_tpr`]
module"]
pub type ITM_TPR = crate::Reg<itm_tpr::ITM_TPR_SPEC>;
#[doc = "Controls which stimulus ports can be accessed by unprivileged code"]
pub mod itm_tpr;
#[doc = "ITM_TCR (rw) register accessor: Configures and controls transfers through the ITM interface  

You can [`read`](crate::Reg::read) this register and get [`itm_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_tcr`]
module"]
pub type ITM_TCR = crate::Reg<itm_tcr::ITM_TCR_SPEC>;
#[doc = "Configures and controls transfers through the ITM interface"]
pub mod itm_tcr;
#[doc = "INT_ATREADY (rw) register accessor: Integration Mode: Read ATB Ready  

You can [`read`](crate::Reg::read) this register and get [`int_atready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_atready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@int_atready`]
module"]
pub type INT_ATREADY = crate::Reg<int_atready::INT_ATREADY_SPEC>;
#[doc = "Integration Mode: Read ATB Ready"]
pub mod int_atready;
#[doc = "INT_ATVALID (rw) register accessor: Integration Mode: Write ATB Valid  

You can [`read`](crate::Reg::read) this register and get [`int_atvalid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_atvalid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@int_atvalid`]
module"]
pub type INT_ATVALID = crate::Reg<int_atvalid::INT_ATVALID_SPEC>;
#[doc = "Integration Mode: Write ATB Valid"]
pub mod int_atvalid;
#[doc = "ITM_ITCTRL (rw) register accessor: Integration Mode Control Register  

You can [`read`](crate::Reg::read) this register and get [`itm_itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_itctrl`]
module"]
pub type ITM_ITCTRL = crate::Reg<itm_itctrl::ITM_ITCTRL_SPEC>;
#[doc = "Integration Mode Control Register"]
pub mod itm_itctrl;
#[doc = "ITM_DEVARCH (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_devarch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_devarch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_devarch`]
module"]
pub type ITM_DEVARCH = crate::Reg<itm_devarch::ITM_DEVARCH_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_devarch;
#[doc = "ITM_DEVTYPE (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_devtype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_devtype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_devtype`]
module"]
pub type ITM_DEVTYPE = crate::Reg<itm_devtype::ITM_DEVTYPE_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_devtype;
#[doc = "ITM_PIDR4 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_pidr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_pidr4`]
module"]
pub type ITM_PIDR4 = crate::Reg<itm_pidr4::ITM_PIDR4_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_pidr4;
#[doc = "ITM_PIDR5 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_pidr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_pidr5`]
module"]
pub type ITM_PIDR5 = crate::Reg<itm_pidr5::ITM_PIDR5_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_pidr5;
#[doc = "ITM_PIDR6 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_pidr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_pidr6`]
module"]
pub type ITM_PIDR6 = crate::Reg<itm_pidr6::ITM_PIDR6_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_pidr6;
#[doc = "ITM_PIDR7 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_pidr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_pidr7`]
module"]
pub type ITM_PIDR7 = crate::Reg<itm_pidr7::ITM_PIDR7_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_pidr7;
#[doc = "ITM_PIDR0 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_pidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_pidr0`]
module"]
pub type ITM_PIDR0 = crate::Reg<itm_pidr0::ITM_PIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_pidr0;
#[doc = "ITM_PIDR1 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_pidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_pidr1`]
module"]
pub type ITM_PIDR1 = crate::Reg<itm_pidr1::ITM_PIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_pidr1;
#[doc = "ITM_PIDR2 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_pidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_pidr2`]
module"]
pub type ITM_PIDR2 = crate::Reg<itm_pidr2::ITM_PIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_pidr2;
#[doc = "ITM_PIDR3 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_pidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_pidr3`]
module"]
pub type ITM_PIDR3 = crate::Reg<itm_pidr3::ITM_PIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_pidr3;
#[doc = "ITM_CIDR0 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_cidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_cidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_cidr0`]
module"]
pub type ITM_CIDR0 = crate::Reg<itm_cidr0::ITM_CIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_cidr0;
#[doc = "ITM_CIDR1 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_cidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_cidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_cidr1`]
module"]
pub type ITM_CIDR1 = crate::Reg<itm_cidr1::ITM_CIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_cidr1;
#[doc = "ITM_CIDR2 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_cidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_cidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_cidr2`]
module"]
pub type ITM_CIDR2 = crate::Reg<itm_cidr2::ITM_CIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_cidr2;
#[doc = "ITM_CIDR3 (rw) register accessor: Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_cidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_cidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itm_cidr3`]
module"]
pub type ITM_CIDR3 = crate::Reg<itm_cidr3::ITM_CIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod itm_cidr3;
#[doc = "DWT_CTRL (rw) register accessor: Provides configuration and status information for the DWT unit, and used to control features of the unit  

You can [`read`](crate::Reg::read) this register and get [`dwt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_ctrl`]
module"]
pub type DWT_CTRL = crate::Reg<dwt_ctrl::DWT_CTRL_SPEC>;
#[doc = "Provides configuration and status information for the DWT unit, and used to control features of the unit"]
pub mod dwt_ctrl;
#[doc = "DWT_CYCCNT (rw) register accessor: Shows or sets the value of the processor cycle counter, CYCCNT  

You can [`read`](crate::Reg::read) this register and get [`dwt_cyccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_cyccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_cyccnt`]
module"]
pub type DWT_CYCCNT = crate::Reg<dwt_cyccnt::DWT_CYCCNT_SPEC>;
#[doc = "Shows or sets the value of the processor cycle counter, CYCCNT"]
pub mod dwt_cyccnt;
#[doc = "DWT_EXCCNT (rw) register accessor: Counts the total cycles spent in exception processing  

You can [`read`](crate::Reg::read) this register and get [`dwt_exccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_exccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_exccnt`]
module"]
pub type DWT_EXCCNT = crate::Reg<dwt_exccnt::DWT_EXCCNT_SPEC>;
#[doc = "Counts the total cycles spent in exception processing"]
pub mod dwt_exccnt;
#[doc = "DWT_LSUCNT (rw) register accessor: Increments on the additional cycles required to execute all load or store instructions  

You can [`read`](crate::Reg::read) this register and get [`dwt_lsucnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_lsucnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_lsucnt`]
module"]
pub type DWT_LSUCNT = crate::Reg<dwt_lsucnt::DWT_LSUCNT_SPEC>;
#[doc = "Increments on the additional cycles required to execute all load or store instructions"]
pub mod dwt_lsucnt;
#[doc = "DWT_FOLDCNT (rw) register accessor: Increments on the additional cycles required to execute all load or store instructions  

You can [`read`](crate::Reg::read) this register and get [`dwt_foldcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_foldcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_foldcnt`]
module"]
pub type DWT_FOLDCNT = crate::Reg<dwt_foldcnt::DWT_FOLDCNT_SPEC>;
#[doc = "Increments on the additional cycles required to execute all load or store instructions"]
pub mod dwt_foldcnt;
#[doc = "DWT_COMP0 (rw) register accessor: Provides a reference value for use by watchpoint comparator 0  

You can [`read`](crate::Reg::read) this register and get [`dwt_comp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_comp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_comp0`]
module"]
pub type DWT_COMP0 = crate::Reg<dwt_comp0::DWT_COMP0_SPEC>;
#[doc = "Provides a reference value for use by watchpoint comparator 0"]
pub mod dwt_comp0;
#[doc = "DWT_FUNCTION0 (rw) register accessor: Controls the operation of watchpoint comparator 0  

You can [`read`](crate::Reg::read) this register and get [`dwt_function0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_function0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_function0`]
module"]
pub type DWT_FUNCTION0 = crate::Reg<dwt_function0::DWT_FUNCTION0_SPEC>;
#[doc = "Controls the operation of watchpoint comparator 0"]
pub mod dwt_function0;
#[doc = "DWT_COMP1 (rw) register accessor: Provides a reference value for use by watchpoint comparator 1  

You can [`read`](crate::Reg::read) this register and get [`dwt_comp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_comp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_comp1`]
module"]
pub type DWT_COMP1 = crate::Reg<dwt_comp1::DWT_COMP1_SPEC>;
#[doc = "Provides a reference value for use by watchpoint comparator 1"]
pub mod dwt_comp1;
#[doc = "DWT_FUNCTION1 (rw) register accessor: Controls the operation of watchpoint comparator 1  

You can [`read`](crate::Reg::read) this register and get [`dwt_function1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_function1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_function1`]
module"]
pub type DWT_FUNCTION1 = crate::Reg<dwt_function1::DWT_FUNCTION1_SPEC>;
#[doc = "Controls the operation of watchpoint comparator 1"]
pub mod dwt_function1;
#[doc = "DWT_COMP2 (rw) register accessor: Provides a reference value for use by watchpoint comparator 2  

You can [`read`](crate::Reg::read) this register and get [`dwt_comp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_comp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_comp2`]
module"]
pub type DWT_COMP2 = crate::Reg<dwt_comp2::DWT_COMP2_SPEC>;
#[doc = "Provides a reference value for use by watchpoint comparator 2"]
pub mod dwt_comp2;
#[doc = "DWT_FUNCTION2 (rw) register accessor: Controls the operation of watchpoint comparator 2  

You can [`read`](crate::Reg::read) this register and get [`dwt_function2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_function2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_function2`]
module"]
pub type DWT_FUNCTION2 = crate::Reg<dwt_function2::DWT_FUNCTION2_SPEC>;
#[doc = "Controls the operation of watchpoint comparator 2"]
pub mod dwt_function2;
#[doc = "DWT_COMP3 (rw) register accessor: Provides a reference value for use by watchpoint comparator 3  

You can [`read`](crate::Reg::read) this register and get [`dwt_comp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_comp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_comp3`]
module"]
pub type DWT_COMP3 = crate::Reg<dwt_comp3::DWT_COMP3_SPEC>;
#[doc = "Provides a reference value for use by watchpoint comparator 3"]
pub mod dwt_comp3;
#[doc = "DWT_FUNCTION3 (rw) register accessor: Controls the operation of watchpoint comparator 3  

You can [`read`](crate::Reg::read) this register and get [`dwt_function3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_function3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_function3`]
module"]
pub type DWT_FUNCTION3 = crate::Reg<dwt_function3::DWT_FUNCTION3_SPEC>;
#[doc = "Controls the operation of watchpoint comparator 3"]
pub mod dwt_function3;
#[doc = "DWT_DEVARCH (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_devarch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_devarch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_devarch`]
module"]
pub type DWT_DEVARCH = crate::Reg<dwt_devarch::DWT_DEVARCH_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_devarch;
#[doc = "DWT_DEVTYPE (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_devtype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_devtype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_devtype`]
module"]
pub type DWT_DEVTYPE = crate::Reg<dwt_devtype::DWT_DEVTYPE_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_devtype;
#[doc = "DWT_PIDR4 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_pidr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_pidr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_pidr4`]
module"]
pub type DWT_PIDR4 = crate::Reg<dwt_pidr4::DWT_PIDR4_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_pidr4;
#[doc = "DWT_PIDR5 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_pidr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_pidr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_pidr5`]
module"]
pub type DWT_PIDR5 = crate::Reg<dwt_pidr5::DWT_PIDR5_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_pidr5;
#[doc = "DWT_PIDR6 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_pidr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_pidr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_pidr6`]
module"]
pub type DWT_PIDR6 = crate::Reg<dwt_pidr6::DWT_PIDR6_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_pidr6;
#[doc = "DWT_PIDR7 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_pidr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_pidr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_pidr7`]
module"]
pub type DWT_PIDR7 = crate::Reg<dwt_pidr7::DWT_PIDR7_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_pidr7;
#[doc = "DWT_PIDR0 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_pidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_pidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_pidr0`]
module"]
pub type DWT_PIDR0 = crate::Reg<dwt_pidr0::DWT_PIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_pidr0;
#[doc = "DWT_PIDR1 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_pidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_pidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_pidr1`]
module"]
pub type DWT_PIDR1 = crate::Reg<dwt_pidr1::DWT_PIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_pidr1;
#[doc = "DWT_PIDR2 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_pidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_pidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_pidr2`]
module"]
pub type DWT_PIDR2 = crate::Reg<dwt_pidr2::DWT_PIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_pidr2;
#[doc = "DWT_PIDR3 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_pidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_pidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_pidr3`]
module"]
pub type DWT_PIDR3 = crate::Reg<dwt_pidr3::DWT_PIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_pidr3;
#[doc = "DWT_CIDR0 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_cidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_cidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_cidr0`]
module"]
pub type DWT_CIDR0 = crate::Reg<dwt_cidr0::DWT_CIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_cidr0;
#[doc = "DWT_CIDR1 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_cidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_cidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_cidr1`]
module"]
pub type DWT_CIDR1 = crate::Reg<dwt_cidr1::DWT_CIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_cidr1;
#[doc = "DWT_CIDR2 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_cidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_cidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_cidr2`]
module"]
pub type DWT_CIDR2 = crate::Reg<dwt_cidr2::DWT_CIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_cidr2;
#[doc = "DWT_CIDR3 (rw) register accessor: Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_cidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_cidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dwt_cidr3`]
module"]
pub type DWT_CIDR3 = crate::Reg<dwt_cidr3::DWT_CIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod dwt_cidr3;
#[doc = "FP_CTRL (rw) register accessor: Provides FPB implementation information, and the global enable for the FPB unit  

You can [`read`](crate::Reg::read) this register and get [`fp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_ctrl`]
module"]
pub type FP_CTRL = crate::Reg<fp_ctrl::FP_CTRL_SPEC>;
#[doc = "Provides FPB implementation information, and the global enable for the FPB unit"]
pub mod fp_ctrl;
#[doc = "FP_REMAP (rw) register accessor: Indicates whether the implementation supports Flash Patch remap and, if it does, holds the target address for remap  

You can [`read`](crate::Reg::read) this register and get [`fp_remap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_remap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_remap`]
module"]
pub type FP_REMAP = crate::Reg<fp_remap::FP_REMAP_SPEC>;
#[doc = "Indicates whether the implementation supports Flash Patch remap and, if it does, holds the target address for remap"]
pub mod fp_remap;
#[doc = "FP_COMP0 (rw) register accessor: Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator  

You can [`read`](crate::Reg::read) this register and get [`fp_comp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_comp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_comp0`]
module"]
pub type FP_COMP0 = crate::Reg<fp_comp0::FP_COMP0_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod fp_comp0;
#[doc = "FP_COMP1 (rw) register accessor: Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator  

You can [`read`](crate::Reg::read) this register and get [`fp_comp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_comp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_comp1`]
module"]
pub type FP_COMP1 = crate::Reg<fp_comp1::FP_COMP1_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod fp_comp1;
#[doc = "FP_COMP2 (rw) register accessor: Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator  

You can [`read`](crate::Reg::read) this register and get [`fp_comp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_comp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_comp2`]
module"]
pub type FP_COMP2 = crate::Reg<fp_comp2::FP_COMP2_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod fp_comp2;
#[doc = "FP_COMP3 (rw) register accessor: Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator  

You can [`read`](crate::Reg::read) this register and get [`fp_comp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_comp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_comp3`]
module"]
pub type FP_COMP3 = crate::Reg<fp_comp3::FP_COMP3_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod fp_comp3;
#[doc = "FP_COMP4 (rw) register accessor: Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator  

You can [`read`](crate::Reg::read) this register and get [`fp_comp4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_comp4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_comp4`]
module"]
pub type FP_COMP4 = crate::Reg<fp_comp4::FP_COMP4_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod fp_comp4;
#[doc = "FP_COMP5 (rw) register accessor: Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator  

You can [`read`](crate::Reg::read) this register and get [`fp_comp5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_comp5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_comp5`]
module"]
pub type FP_COMP5 = crate::Reg<fp_comp5::FP_COMP5_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod fp_comp5;
#[doc = "FP_COMP6 (rw) register accessor: Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator  

You can [`read`](crate::Reg::read) this register and get [`fp_comp6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_comp6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_comp6`]
module"]
pub type FP_COMP6 = crate::Reg<fp_comp6::FP_COMP6_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod fp_comp6;
#[doc = "FP_COMP7 (rw) register accessor: Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator  

You can [`read`](crate::Reg::read) this register and get [`fp_comp7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_comp7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_comp7`]
module"]
pub type FP_COMP7 = crate::Reg<fp_comp7::FP_COMP7_SPEC>;
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator"]
pub mod fp_comp7;
#[doc = "FP_DEVARCH (rw) register accessor: Provides CoreSight discovery information for the FPB  

You can [`read`](crate::Reg::read) this register and get [`fp_devarch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_devarch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_devarch`]
module"]
pub type FP_DEVARCH = crate::Reg<fp_devarch::FP_DEVARCH_SPEC>;
#[doc = "Provides CoreSight discovery information for the FPB"]
pub mod fp_devarch;
#[doc = "FP_DEVTYPE (rw) register accessor: Provides CoreSight discovery information for the FPB  

You can [`read`](crate::Reg::read) this register and get [`fp_devtype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_devtype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_devtype`]
module"]
pub type FP_DEVTYPE = crate::Reg<fp_devtype::FP_DEVTYPE_SPEC>;
#[doc = "Provides CoreSight discovery information for the FPB"]
pub mod fp_devtype;
#[doc = "FP_PIDR4 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_pidr4`]
module"]
pub type FP_PIDR4 = crate::Reg<fp_pidr4::FP_PIDR4_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_pidr4;
#[doc = "FP_PIDR5 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_pidr5`]
module"]
pub type FP_PIDR5 = crate::Reg<fp_pidr5::FP_PIDR5_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_pidr5;
#[doc = "FP_PIDR6 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_pidr6`]
module"]
pub type FP_PIDR6 = crate::Reg<fp_pidr6::FP_PIDR6_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_pidr6;
#[doc = "FP_PIDR7 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_pidr7`]
module"]
pub type FP_PIDR7 = crate::Reg<fp_pidr7::FP_PIDR7_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_pidr7;
#[doc = "FP_PIDR0 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_pidr0`]
module"]
pub type FP_PIDR0 = crate::Reg<fp_pidr0::FP_PIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_pidr0;
#[doc = "FP_PIDR1 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_pidr1`]
module"]
pub type FP_PIDR1 = crate::Reg<fp_pidr1::FP_PIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_pidr1;
#[doc = "FP_PIDR2 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_pidr2`]
module"]
pub type FP_PIDR2 = crate::Reg<fp_pidr2::FP_PIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_pidr2;
#[doc = "FP_PIDR3 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_pidr3`]
module"]
pub type FP_PIDR3 = crate::Reg<fp_pidr3::FP_PIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_pidr3;
#[doc = "FP_CIDR0 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_cidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_cidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_cidr0`]
module"]
pub type FP_CIDR0 = crate::Reg<fp_cidr0::FP_CIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_cidr0;
#[doc = "FP_CIDR1 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_cidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_cidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_cidr1`]
module"]
pub type FP_CIDR1 = crate::Reg<fp_cidr1::FP_CIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_cidr1;
#[doc = "FP_CIDR2 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_cidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_cidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_cidr2`]
module"]
pub type FP_CIDR2 = crate::Reg<fp_cidr2::FP_CIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_cidr2;
#[doc = "FP_CIDR3 (rw) register accessor: Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_cidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_cidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fp_cidr3`]
module"]
pub type FP_CIDR3 = crate::Reg<fp_cidr3::FP_CIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the FP"]
pub mod fp_cidr3;
#[doc = "ICTR (rw) register accessor: Provides information about the interrupt controller  

You can [`read`](crate::Reg::read) this register and get [`ictr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ictr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ictr`]
module"]
pub type ICTR = crate::Reg<ictr::ICTR_SPEC>;
#[doc = "Provides information about the interrupt controller"]
pub mod ictr;
#[doc = "ACTLR (rw) register accessor: Provides IMPLEMENTATION DEFINED configuration and control options  

You can [`read`](crate::Reg::read) this register and get [`actlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@actlr`]
module"]
pub type ACTLR = crate::Reg<actlr::ACTLR_SPEC>;
#[doc = "Provides IMPLEMENTATION DEFINED configuration and control options"]
pub mod actlr;
#[doc = "SYST_CSR (rw) register accessor: Use the SysTick Control and Status Register to enable the SysTick features.  

You can [`read`](crate::Reg::read) this register and get [`syst_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@syst_csr`]
module"]
pub type SYST_CSR = crate::Reg<syst_csr::SYST_CSR_SPEC>;
#[doc = "Use the SysTick Control and Status Register to enable the SysTick features."]
pub mod syst_csr;
#[doc = "SYST_RVR (rw) register accessor: Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN. To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99.  

You can [`read`](crate::Reg::read) this register and get [`syst_rvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_rvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@syst_rvr`]
module"]
pub type SYST_RVR = crate::Reg<syst_rvr::SYST_RVR_SPEC>;
#[doc = "Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN. To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99."]
pub mod syst_rvr;
#[doc = "SYST_CVR (rw) register accessor: Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN.  

You can [`read`](crate::Reg::read) this register and get [`syst_cvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_cvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@syst_cvr`]
module"]
pub type SYST_CVR = crate::Reg<syst_cvr::SYST_CVR_SPEC>;
#[doc = "Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
pub mod syst_cvr;
#[doc = "SYST_CALIB (rw) register accessor: Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply.  

You can [`read`](crate::Reg::read) this register and get [`syst_calib::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_calib::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@syst_calib`]
module"]
pub type SYST_CALIB = crate::Reg<syst_calib::SYST_CALIB_SPEC>;
#[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
pub mod syst_calib;
#[doc = "NVIC_ISER0 (rw) register accessor: Enables or reads the enabled state of each group of 32 interrupts  

You can [`read`](crate::Reg::read) this register and get [`nvic_iser0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_iser0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_iser0`]
module"]
pub type NVIC_ISER0 = crate::Reg<nvic_iser0::NVIC_ISER0_SPEC>;
#[doc = "Enables or reads the enabled state of each group of 32 interrupts"]
pub mod nvic_iser0;
#[doc = "NVIC_ISER1 (rw) register accessor: Enables or reads the enabled state of each group of 32 interrupts  

You can [`read`](crate::Reg::read) this register and get [`nvic_iser1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_iser1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_iser1`]
module"]
pub type NVIC_ISER1 = crate::Reg<nvic_iser1::NVIC_ISER1_SPEC>;
#[doc = "Enables or reads the enabled state of each group of 32 interrupts"]
pub mod nvic_iser1;
#[doc = "NVIC_ICER0 (rw) register accessor: Clears or reads the enabled state of each group of 32 interrupts  

You can [`read`](crate::Reg::read) this register and get [`nvic_icer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_icer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_icer0`]
module"]
pub type NVIC_ICER0 = crate::Reg<nvic_icer0::NVIC_ICER0_SPEC>;
#[doc = "Clears or reads the enabled state of each group of 32 interrupts"]
pub mod nvic_icer0;
#[doc = "NVIC_ICER1 (rw) register accessor: Clears or reads the enabled state of each group of 32 interrupts  

You can [`read`](crate::Reg::read) this register and get [`nvic_icer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_icer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_icer1`]
module"]
pub type NVIC_ICER1 = crate::Reg<nvic_icer1::NVIC_ICER1_SPEC>;
#[doc = "Clears or reads the enabled state of each group of 32 interrupts"]
pub mod nvic_icer1;
#[doc = "NVIC_ISPR0 (rw) register accessor: Enables or reads the pending state of each group of 32 interrupts  

You can [`read`](crate::Reg::read) this register and get [`nvic_ispr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ispr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ispr0`]
module"]
pub type NVIC_ISPR0 = crate::Reg<nvic_ispr0::NVIC_ISPR0_SPEC>;
#[doc = "Enables or reads the pending state of each group of 32 interrupts"]
pub mod nvic_ispr0;
#[doc = "NVIC_ISPR1 (rw) register accessor: Enables or reads the pending state of each group of 32 interrupts  

You can [`read`](crate::Reg::read) this register and get [`nvic_ispr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ispr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ispr1`]
module"]
pub type NVIC_ISPR1 = crate::Reg<nvic_ispr1::NVIC_ISPR1_SPEC>;
#[doc = "Enables or reads the pending state of each group of 32 interrupts"]
pub mod nvic_ispr1;
#[doc = "NVIC_ICPR0 (rw) register accessor: Clears or reads the pending state of each group of 32 interrupts  

You can [`read`](crate::Reg::read) this register and get [`nvic_icpr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_icpr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_icpr0`]
module"]
pub type NVIC_ICPR0 = crate::Reg<nvic_icpr0::NVIC_ICPR0_SPEC>;
#[doc = "Clears or reads the pending state of each group of 32 interrupts"]
pub mod nvic_icpr0;
#[doc = "NVIC_ICPR1 (rw) register accessor: Clears or reads the pending state of each group of 32 interrupts  

You can [`read`](crate::Reg::read) this register and get [`nvic_icpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_icpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_icpr1`]
module"]
pub type NVIC_ICPR1 = crate::Reg<nvic_icpr1::NVIC_ICPR1_SPEC>;
#[doc = "Clears or reads the pending state of each group of 32 interrupts"]
pub mod nvic_icpr1;
#[doc = "NVIC_IABR0 (rw) register accessor: For each group of 32 interrupts, shows the active state of each interrupt  

You can [`read`](crate::Reg::read) this register and get [`nvic_iabr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_iabr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_iabr0`]
module"]
pub type NVIC_IABR0 = crate::Reg<nvic_iabr0::NVIC_IABR0_SPEC>;
#[doc = "For each group of 32 interrupts, shows the active state of each interrupt"]
pub mod nvic_iabr0;
#[doc = "NVIC_IABR1 (rw) register accessor: For each group of 32 interrupts, shows the active state of each interrupt  

You can [`read`](crate::Reg::read) this register and get [`nvic_iabr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_iabr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_iabr1`]
module"]
pub type NVIC_IABR1 = crate::Reg<nvic_iabr1::NVIC_IABR1_SPEC>;
#[doc = "For each group of 32 interrupts, shows the active state of each interrupt"]
pub mod nvic_iabr1;
#[doc = "NVIC_ITNS0 (rw) register accessor: For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state  

You can [`read`](crate::Reg::read) this register and get [`nvic_itns0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_itns0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_itns0`]
module"]
pub type NVIC_ITNS0 = crate::Reg<nvic_itns0::NVIC_ITNS0_SPEC>;
#[doc = "For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
pub mod nvic_itns0;
#[doc = "NVIC_ITNS1 (rw) register accessor: For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state  

You can [`read`](crate::Reg::read) this register and get [`nvic_itns1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_itns1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_itns1`]
module"]
pub type NVIC_ITNS1 = crate::Reg<nvic_itns1::NVIC_ITNS1_SPEC>;
#[doc = "For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
pub mod nvic_itns1;
#[doc = "NVIC_IPR0 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr0`]
module"]
pub type NVIC_IPR0 = crate::Reg<nvic_ipr0::NVIC_IPR0_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr0;
#[doc = "NVIC_IPR1 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr1`]
module"]
pub type NVIC_IPR1 = crate::Reg<nvic_ipr1::NVIC_IPR1_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr1;
#[doc = "NVIC_IPR2 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr2`]
module"]
pub type NVIC_IPR2 = crate::Reg<nvic_ipr2::NVIC_IPR2_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr2;
#[doc = "NVIC_IPR3 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr3`]
module"]
pub type NVIC_IPR3 = crate::Reg<nvic_ipr3::NVIC_IPR3_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr3;
#[doc = "NVIC_IPR4 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr4`]
module"]
pub type NVIC_IPR4 = crate::Reg<nvic_ipr4::NVIC_IPR4_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr4;
#[doc = "NVIC_IPR5 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr5`]
module"]
pub type NVIC_IPR5 = crate::Reg<nvic_ipr5::NVIC_IPR5_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr5;
#[doc = "NVIC_IPR6 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr6`]
module"]
pub type NVIC_IPR6 = crate::Reg<nvic_ipr6::NVIC_IPR6_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr6;
#[doc = "NVIC_IPR7 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr7`]
module"]
pub type NVIC_IPR7 = crate::Reg<nvic_ipr7::NVIC_IPR7_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr7;
#[doc = "NVIC_IPR8 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr8`]
module"]
pub type NVIC_IPR8 = crate::Reg<nvic_ipr8::NVIC_IPR8_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr8;
#[doc = "NVIC_IPR9 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr9`]
module"]
pub type NVIC_IPR9 = crate::Reg<nvic_ipr9::NVIC_IPR9_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr9;
#[doc = "NVIC_IPR10 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr10`]
module"]
pub type NVIC_IPR10 = crate::Reg<nvic_ipr10::NVIC_IPR10_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr10;
#[doc = "NVIC_IPR11 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr11`]
module"]
pub type NVIC_IPR11 = crate::Reg<nvic_ipr11::NVIC_IPR11_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr11;
#[doc = "NVIC_IPR12 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr12`]
module"]
pub type NVIC_IPR12 = crate::Reg<nvic_ipr12::NVIC_IPR12_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr12;
#[doc = "NVIC_IPR13 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr13`]
module"]
pub type NVIC_IPR13 = crate::Reg<nvic_ipr13::NVIC_IPR13_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr13;
#[doc = "NVIC_IPR14 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr14`]
module"]
pub type NVIC_IPR14 = crate::Reg<nvic_ipr14::NVIC_IPR14_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr14;
#[doc = "NVIC_IPR15 (rw) register accessor: Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nvic_ipr15`]
module"]
pub type NVIC_IPR15 = crate::Reg<nvic_ipr15::NVIC_IPR15_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod nvic_ipr15;
#[doc = "CPUID (rw) register accessor: Provides identification information for the PE, including an implementer code for the device and a device ID number  

You can [`read`](crate::Reg::read) this register and get [`cpuid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cpuid`]
module"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "Provides identification information for the PE, including an implementer code for the device and a device ID number"]
pub mod cpuid;
#[doc = "ICSR (rw) register accessor: Controls and provides status information for NMI, PendSV, SysTick and interrupts  

You can [`read`](crate::Reg::read) this register and get [`icsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@icsr`]
module"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "Controls and provides status information for NMI, PendSV, SysTick and interrupts"]
pub mod icsr;
#[doc = "VTOR (rw) register accessor: The VTOR indicates the offset of the vector table base address from memory address 0x00000000.  

You can [`read`](crate::Reg::read) this register and get [`vtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@vtor`]
module"]
pub type VTOR = crate::Reg<vtor::VTOR_SPEC>;
#[doc = "The VTOR indicates the offset of the vector table base address from memory address 0x00000000."]
pub mod vtor;
#[doc = "AIRCR (rw) register accessor: Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset.  

You can [`read`](crate::Reg::read) this register and get [`aircr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aircr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@aircr`]
module"]
pub type AIRCR = crate::Reg<aircr::AIRCR_SPEC>;
#[doc = "Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset."]
pub mod aircr;
#[doc = "SCR (rw) register accessor: System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states.  

You can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states."]
pub mod scr;
#[doc = "CCR (rw) register accessor: Sets or returns configuration and control data  

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Sets or returns configuration and control data"]
pub mod ccr;
#[doc = "SHPR1 (rw) register accessor: Sets or returns priority for system handlers 4 - 7  

You can [`read`](crate::Reg::read) this register and get [`shpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@shpr1`]
module"]
pub type SHPR1 = crate::Reg<shpr1::SHPR1_SPEC>;
#[doc = "Sets or returns priority for system handlers 4 - 7"]
pub mod shpr1;
#[doc = "SHPR2 (rw) register accessor: Sets or returns priority for system handlers 8 - 11  

You can [`read`](crate::Reg::read) this register and get [`shpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@shpr2`]
module"]
pub type SHPR2 = crate::Reg<shpr2::SHPR2_SPEC>;
#[doc = "Sets or returns priority for system handlers 8 - 11"]
pub mod shpr2;
#[doc = "SHPR3 (rw) register accessor: Sets or returns priority for system handlers 12 - 15  

You can [`read`](crate::Reg::read) this register and get [`shpr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@shpr3`]
module"]
pub type SHPR3 = crate::Reg<shpr3::SHPR3_SPEC>;
#[doc = "Sets or returns priority for system handlers 12 - 15"]
pub mod shpr3;
#[doc = "SHCSR (rw) register accessor: Provides access to the active and pending status of system exceptions  

You can [`read`](crate::Reg::read) this register and get [`shcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@shcsr`]
module"]
pub type SHCSR = crate::Reg<shcsr::SHCSR_SPEC>;
#[doc = "Provides access to the active and pending status of system exceptions"]
pub mod shcsr;
#[doc = "CFSR (rw) register accessor: Contains the three Configurable Fault Status Registers. 31:16 UFSR: Provides information on UsageFault exceptions 15:8 BFSR: Provides information on BusFault exceptions 7:0 MMFSR: Provides information on MemManage exceptions  

You can [`read`](crate::Reg::read) this register and get [`cfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cfsr`]
module"]
pub type CFSR = crate::Reg<cfsr::CFSR_SPEC>;
#[doc = "Contains the three Configurable Fault Status Registers. 31:16 UFSR: Provides information on UsageFault exceptions 15:8 BFSR: Provides information on BusFault exceptions 7:0 MMFSR: Provides information on MemManage exceptions"]
pub mod cfsr;
#[doc = "HFSR (rw) register accessor: Shows the cause of any HardFaults  

You can [`read`](crate::Reg::read) this register and get [`hfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@hfsr`]
module"]
pub type HFSR = crate::Reg<hfsr::HFSR_SPEC>;
#[doc = "Shows the cause of any HardFaults"]
pub mod hfsr;
#[doc = "DFSR (rw) register accessor: Shows which debug event occurred  

You can [`read`](crate::Reg::read) this register and get [`dfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dfsr`]
module"]
pub type DFSR = crate::Reg<dfsr::DFSR_SPEC>;
#[doc = "Shows which debug event occurred"]
pub mod dfsr;
#[doc = "MMFAR (rw) register accessor: Shows the address of the memory location that caused an MPU fault  

You can [`read`](crate::Reg::read) this register and get [`mmfar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmfar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mmfar`]
module"]
pub type MMFAR = crate::Reg<mmfar::MMFAR_SPEC>;
#[doc = "Shows the address of the memory location that caused an MPU fault"]
pub mod mmfar;
#[doc = "BFAR (rw) register accessor: Shows the address associated with a precise data access BusFault  

You can [`read`](crate::Reg::read) this register and get [`bfar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bfar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bfar`]
module"]
pub type BFAR = crate::Reg<bfar::BFAR_SPEC>;
#[doc = "Shows the address associated with a precise data access BusFault"]
pub mod bfar;
#[doc = "ID_PFR0 (rw) register accessor: Gives top-level information about the instruction set supported by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_pfr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_pfr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_pfr0`]
module"]
pub type ID_PFR0 = crate::Reg<id_pfr0::ID_PFR0_SPEC>;
#[doc = "Gives top-level information about the instruction set supported by the PE"]
pub mod id_pfr0;
#[doc = "ID_PFR1 (rw) register accessor: Gives information about the programmers' model and Extensions support  

You can [`read`](crate::Reg::read) this register and get [`id_pfr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_pfr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_pfr1`]
module"]
pub type ID_PFR1 = crate::Reg<id_pfr1::ID_PFR1_SPEC>;
#[doc = "Gives information about the programmers' model and Extensions support"]
pub mod id_pfr1;
#[doc = "ID_DFR0 (rw) register accessor: Provides top level information about the debug system  

You can [`read`](crate::Reg::read) this register and get [`id_dfr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_dfr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_dfr0`]
module"]
pub type ID_DFR0 = crate::Reg<id_dfr0::ID_DFR0_SPEC>;
#[doc = "Provides top level information about the debug system"]
pub mod id_dfr0;
#[doc = "ID_AFR0 (rw) register accessor: Provides information about the IMPLEMENTATION DEFINED features of the PE  

You can [`read`](crate::Reg::read) this register and get [`id_afr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_afr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_afr0`]
module"]
pub type ID_AFR0 = crate::Reg<id_afr0::ID_AFR0_SPEC>;
#[doc = "Provides information about the IMPLEMENTATION DEFINED features of the PE"]
pub mod id_afr0;
#[doc = "ID_MMFR0 (rw) register accessor: Provides information about the implemented memory model and memory management support  

You can [`read`](crate::Reg::read) this register and get [`id_mmfr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_mmfr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_mmfr0`]
module"]
pub type ID_MMFR0 = crate::Reg<id_mmfr0::ID_MMFR0_SPEC>;
#[doc = "Provides information about the implemented memory model and memory management support"]
pub mod id_mmfr0;
#[doc = "ID_MMFR1 (rw) register accessor: Provides information about the implemented memory model and memory management support  

You can [`read`](crate::Reg::read) this register and get [`id_mmfr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_mmfr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_mmfr1`]
module"]
pub type ID_MMFR1 = crate::Reg<id_mmfr1::ID_MMFR1_SPEC>;
#[doc = "Provides information about the implemented memory model and memory management support"]
pub mod id_mmfr1;
#[doc = "ID_MMFR2 (rw) register accessor: Provides information about the implemented memory model and memory management support  

You can [`read`](crate::Reg::read) this register and get [`id_mmfr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_mmfr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_mmfr2`]
module"]
pub type ID_MMFR2 = crate::Reg<id_mmfr2::ID_MMFR2_SPEC>;
#[doc = "Provides information about the implemented memory model and memory management support"]
pub mod id_mmfr2;
#[doc = "ID_MMFR3 (rw) register accessor: Provides information about the implemented memory model and memory management support  

You can [`read`](crate::Reg::read) this register and get [`id_mmfr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_mmfr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_mmfr3`]
module"]
pub type ID_MMFR3 = crate::Reg<id_mmfr3::ID_MMFR3_SPEC>;
#[doc = "Provides information about the implemented memory model and memory management support"]
pub mod id_mmfr3;
#[doc = "ID_ISAR0 (rw) register accessor: Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_isar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_isar0`]
module"]
pub type ID_ISAR0 = crate::Reg<id_isar0::ID_ISAR0_SPEC>;
#[doc = "Provides information about the instruction set implemented by the PE"]
pub mod id_isar0;
#[doc = "ID_ISAR1 (rw) register accessor: Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_isar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_isar1`]
module"]
pub type ID_ISAR1 = crate::Reg<id_isar1::ID_ISAR1_SPEC>;
#[doc = "Provides information about the instruction set implemented by the PE"]
pub mod id_isar1;
#[doc = "ID_ISAR2 (rw) register accessor: Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_isar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_isar2`]
module"]
pub type ID_ISAR2 = crate::Reg<id_isar2::ID_ISAR2_SPEC>;
#[doc = "Provides information about the instruction set implemented by the PE"]
pub mod id_isar2;
#[doc = "ID_ISAR3 (rw) register accessor: Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_isar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_isar3`]
module"]
pub type ID_ISAR3 = crate::Reg<id_isar3::ID_ISAR3_SPEC>;
#[doc = "Provides information about the instruction set implemented by the PE"]
pub mod id_isar3;
#[doc = "ID_ISAR4 (rw) register accessor: Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_isar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_isar4`]
module"]
pub type ID_ISAR4 = crate::Reg<id_isar4::ID_ISAR4_SPEC>;
#[doc = "Provides information about the instruction set implemented by the PE"]
pub mod id_isar4;
#[doc = "ID_ISAR5 (rw) register accessor: Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_isar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@id_isar5`]
module"]
pub type ID_ISAR5 = crate::Reg<id_isar5::ID_ISAR5_SPEC>;
#[doc = "Provides information about the instruction set implemented by the PE"]
pub mod id_isar5;
#[doc = "CTR (rw) register accessor: Provides information about the architecture of the caches. CTR is RES0 if CLIDR is zero.  

You can [`read`](crate::Reg::read) this register and get [`ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctr`]
module"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "Provides information about the architecture of the caches. CTR is RES0 if CLIDR is zero."]
pub mod ctr;
#[doc = "CPACR (rw) register accessor: Specifies the access privileges for coprocessors and the FP Extension  

You can [`read`](crate::Reg::read) this register and get [`cpacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cpacr`]
module"]
pub type CPACR = crate::Reg<cpacr::CPACR_SPEC>;
#[doc = "Specifies the access privileges for coprocessors and the FP Extension"]
pub mod cpacr;
#[doc = "NSACR (rw) register accessor: Defines the Non-secure access permissions for both the FP Extension and coprocessors CP0 to CP7  

You can [`read`](crate::Reg::read) this register and get [`nsacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nsacr`]
module"]
pub type NSACR = crate::Reg<nsacr::NSACR_SPEC>;
#[doc = "Defines the Non-secure access permissions for both the FP Extension and coprocessors CP0 to CP7"]
pub mod nsacr;
#[doc = "MPU_TYPE (rw) register accessor: The MPU Type Register indicates how many regions the MPU `FTSSS supports  

You can [`read`](crate::Reg::read) this register and get [`mpu_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_type`]
module"]
pub type MPU_TYPE = crate::Reg<mpu_type::MPU_TYPE_SPEC>;
#[doc = "The MPU Type Register indicates how many regions the MPU `FTSSS supports"]
pub mod mpu_type;
#[doc = "MPU_CTRL (rw) register accessor: Enables the MPU and, when the MPU is enabled, controls whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1  

You can [`read`](crate::Reg::read) this register and get [`mpu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_ctrl`]
module"]
pub type MPU_CTRL = crate::Reg<mpu_ctrl::MPU_CTRL_SPEC>;
#[doc = "Enables the MPU and, when the MPU is enabled, controls whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults, NMIs, and exception handlers when FAULTMASK is set to 1"]
pub mod mpu_ctrl;
#[doc = "MPU_RNR (rw) register accessor: Selects the region currently accessed by MPU_RBAR and MPU_RLAR  

You can [`read`](crate::Reg::read) this register and get [`mpu_rnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rnr`]
module"]
pub type MPU_RNR = crate::Reg<mpu_rnr::MPU_RNR_SPEC>;
#[doc = "Selects the region currently accessed by MPU_RBAR and MPU_RLAR"]
pub mod mpu_rnr;
#[doc = "MPU_RBAR (rw) register accessor: Provides indirect read and write access to the base address of the currently selected MPU region `FTSSS  

You can [`read`](crate::Reg::read) this register and get [`mpu_rbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rbar`]
module"]
pub type MPU_RBAR = crate::Reg<mpu_rbar::MPU_RBAR_SPEC>;
#[doc = "Provides indirect read and write access to the base address of the currently selected MPU region `FTSSS"]
pub mod mpu_rbar;
#[doc = "MPU_RLAR (rw) register accessor: Provides indirect read and write access to the limit address of the currently selected MPU region `FTSSS  

You can [`read`](crate::Reg::read) this register and get [`mpu_rlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rlar`]
module"]
pub type MPU_RLAR = crate::Reg<mpu_rlar::MPU_RLAR_SPEC>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region `FTSSS"]
pub mod mpu_rlar;
#[doc = "MPU_RBAR_A1 (rw) register accessor: Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\]) `FTSSS  

You can [`read`](crate::Reg::read) this register and get [`mpu_rbar_a1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rbar_a1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rbar_a1`]
module"]
pub type MPU_RBAR_A1 = crate::Reg<mpu_rbar_a1::MPU_RBAR_A1_SPEC>;
#[doc = "Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\]) `FTSSS"]
pub mod mpu_rbar_a1;
#[doc = "MPU_RLAR_A1 (rw) register accessor: Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\]) `FTSSS  

You can [`read`](crate::Reg::read) this register and get [`mpu_rlar_a1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rlar_a1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rlar_a1`]
module"]
pub type MPU_RLAR_A1 = crate::Reg<mpu_rlar_a1::MPU_RLAR_A1_SPEC>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(1\\[1:0\\]) `FTSSS"]
pub mod mpu_rlar_a1;
#[doc = "MPU_RBAR_A2 (rw) register accessor: Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\]) `FTSSS  

You can [`read`](crate::Reg::read) this register and get [`mpu_rbar_a2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rbar_a2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rbar_a2`]
module"]
pub type MPU_RBAR_A2 = crate::Reg<mpu_rbar_a2::MPU_RBAR_A2_SPEC>;
#[doc = "Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\]) `FTSSS"]
pub mod mpu_rbar_a2;
#[doc = "MPU_RLAR_A2 (rw) register accessor: Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\]) `FTSSS  

You can [`read`](crate::Reg::read) this register and get [`mpu_rlar_a2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rlar_a2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rlar_a2`]
module"]
pub type MPU_RLAR_A2 = crate::Reg<mpu_rlar_a2::MPU_RLAR_A2_SPEC>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\]) `FTSSS"]
pub mod mpu_rlar_a2;
#[doc = "MPU_RBAR_A3 (rw) register accessor: Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\]) `FTSSS  

You can [`read`](crate::Reg::read) this register and get [`mpu_rbar_a3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rbar_a3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rbar_a3`]
module"]
pub type MPU_RBAR_A3 = crate::Reg<mpu_rbar_a3::MPU_RBAR_A3_SPEC>;
#[doc = "Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\]) `FTSSS"]
pub mod mpu_rbar_a3;
#[doc = "MPU_RLAR_A3 (rw) register accessor: Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\]) `FTSSS  

You can [`read`](crate::Reg::read) this register and get [`mpu_rlar_a3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rlar_a3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_rlar_a3`]
module"]
pub type MPU_RLAR_A3 = crate::Reg<mpu_rlar_a3::MPU_RLAR_A3_SPEC>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(3\\[1:0\\]) `FTSSS"]
pub mod mpu_rlar_a3;
#[doc = "MPU_MAIR0 (rw) register accessor: Along with MPU_MAIR1, provides the memory attribute encodings corresponding to the AttrIndex values  

You can [`read`](crate::Reg::read) this register and get [`mpu_mair0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_mair0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_mair0`]
module"]
pub type MPU_MAIR0 = crate::Reg<mpu_mair0::MPU_MAIR0_SPEC>;
#[doc = "Along with MPU_MAIR1, provides the memory attribute encodings corresponding to the AttrIndex values"]
pub mod mpu_mair0;
#[doc = "MPU_MAIR1 (rw) register accessor: Along with MPU_MAIR0, provides the memory attribute encodings corresponding to the AttrIndex values  

You can [`read`](crate::Reg::read) this register and get [`mpu_mair1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_mair1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_mair1`]
module"]
pub type MPU_MAIR1 = crate::Reg<mpu_mair1::MPU_MAIR1_SPEC>;
#[doc = "Along with MPU_MAIR0, provides the memory attribute encodings corresponding to the AttrIndex values"]
pub mod mpu_mair1;
#[doc = "SAU_CTRL (rw) register accessor: Allows enabling of the Security Attribution Unit  

You can [`read`](crate::Reg::read) this register and get [`sau_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sau_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sau_ctrl`]
module"]
pub type SAU_CTRL = crate::Reg<sau_ctrl::SAU_CTRL_SPEC>;
#[doc = "Allows enabling of the Security Attribution Unit"]
pub mod sau_ctrl;
#[doc = "SAU_TYPE (rw) register accessor: Indicates the number of regions implemented by the Security Attribution Unit  

You can [`read`](crate::Reg::read) this register and get [`sau_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sau_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sau_type`]
module"]
pub type SAU_TYPE = crate::Reg<sau_type::SAU_TYPE_SPEC>;
#[doc = "Indicates the number of regions implemented by the Security Attribution Unit"]
pub mod sau_type;
#[doc = "SAU_RNR (rw) register accessor: Selects the region currently accessed by SAU_RBAR and SAU_RLAR  

You can [`read`](crate::Reg::read) this register and get [`sau_rnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sau_rnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sau_rnr`]
module"]
pub type SAU_RNR = crate::Reg<sau_rnr::SAU_RNR_SPEC>;
#[doc = "Selects the region currently accessed by SAU_RBAR and SAU_RLAR"]
pub mod sau_rnr;
#[doc = "SAU_RBAR (rw) register accessor: Provides indirect read and write access to the base address of the currently selected SAU region  

You can [`read`](crate::Reg::read) this register and get [`sau_rbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sau_rbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sau_rbar`]
module"]
pub type SAU_RBAR = crate::Reg<sau_rbar::SAU_RBAR_SPEC>;
#[doc = "Provides indirect read and write access to the base address of the currently selected SAU region"]
pub mod sau_rbar;
#[doc = "SAU_RLAR (rw) register accessor: Provides indirect read and write access to the limit address of the currently selected SAU region  

You can [`read`](crate::Reg::read) this register and get [`sau_rlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sau_rlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sau_rlar`]
module"]
pub type SAU_RLAR = crate::Reg<sau_rlar::SAU_RLAR_SPEC>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected SAU region"]
pub mod sau_rlar;
#[doc = "SFSR (rw) register accessor: Provides information about any security related faults  

You can [`read`](crate::Reg::read) this register and get [`sfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sfsr`]
module"]
pub type SFSR = crate::Reg<sfsr::SFSR_SPEC>;
#[doc = "Provides information about any security related faults"]
pub mod sfsr;
#[doc = "SFAR (rw) register accessor: Shows the address of the memory location that caused a Security violation  

You can [`read`](crate::Reg::read) this register and get [`sfar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sfar`]
module"]
pub type SFAR = crate::Reg<sfar::SFAR_SPEC>;
#[doc = "Shows the address of the memory location that caused a Security violation"]
pub mod sfar;
#[doc = "DHCSR (rw) register accessor: Controls halting debug  

You can [`read`](crate::Reg::read) this register and get [`dhcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dhcsr`]
module"]
pub type DHCSR = crate::Reg<dhcsr::DHCSR_SPEC>;
#[doc = "Controls halting debug"]
pub mod dhcsr;
#[doc = "DCRSR (rw) register accessor: With the DCRDR, provides debug access to the general-purpose registers, special-purpose registers, and the FP extension registers. A write to the DCRSR specifies the register to transfer, whether the transfer is a read or write, and starts the transfer  

You can [`read`](crate::Reg::read) this register and get [`dcrsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dcrsr`]
module"]
pub type DCRSR = crate::Reg<dcrsr::DCRSR_SPEC>;
#[doc = "With the DCRDR, provides debug access to the general-purpose registers, special-purpose registers, and the FP extension registers. A write to the DCRSR specifies the register to transfer, whether the transfer is a read or write, and starts the transfer"]
pub mod dcrsr;
#[doc = "DCRDR (rw) register accessor: With the DCRSR, provides debug access to the general-purpose registers, special-purpose registers, and the FP Extension registers. If the Main Extension is implemented, it can also be used for message passing between an external debugger and a debug agent running on the PE  

You can [`read`](crate::Reg::read) this register and get [`dcrdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dcrdr`]
module"]
pub type DCRDR = crate::Reg<dcrdr::DCRDR_SPEC>;
#[doc = "With the DCRSR, provides debug access to the general-purpose registers, special-purpose registers, and the FP Extension registers. If the Main Extension is implemented, it can also be used for message passing between an external debugger and a debug agent running on the PE"]
pub mod dcrdr;
#[doc = "DEMCR (rw) register accessor: Manages vector catch behavior and DebugMonitor handling when debugging  

You can [`read`](crate::Reg::read) this register and get [`demcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@demcr`]
module"]
pub type DEMCR = crate::Reg<demcr::DEMCR_SPEC>;
#[doc = "Manages vector catch behavior and DebugMonitor handling when debugging"]
pub mod demcr;
#[doc = "DSCSR (rw) register accessor: Provides control and status information for Secure debug  

You can [`read`](crate::Reg::read) this register and get [`dscsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dscsr`]
module"]
pub type DSCSR = crate::Reg<dscsr::DSCSR_SPEC>;
#[doc = "Provides control and status information for Secure debug"]
pub mod dscsr;
#[doc = "STIR (rw) register accessor: Provides a mechanism for software to generate an interrupt  

You can [`read`](crate::Reg::read) this register and get [`stir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@stir`]
module"]
pub type STIR = crate::Reg<stir::STIR_SPEC>;
#[doc = "Provides a mechanism for software to generate an interrupt"]
pub mod stir;
#[doc = "FPCCR (rw) register accessor: Holds control data for the Floating-point extension  

You can [`read`](crate::Reg::read) this register and get [`fpccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fpccr`]
module"]
pub type FPCCR = crate::Reg<fpccr::FPCCR_SPEC>;
#[doc = "Holds control data for the Floating-point extension"]
pub mod fpccr;
#[doc = "FPCAR (rw) register accessor: Holds the location of the unpopulated floating-point register space allocated on an exception stack frame  

You can [`read`](crate::Reg::read) this register and get [`fpcar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpcar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fpcar`]
module"]
pub type FPCAR = crate::Reg<fpcar::FPCAR_SPEC>;
#[doc = "Holds the location of the unpopulated floating-point register space allocated on an exception stack frame"]
pub mod fpcar;
#[doc = "FPDSCR (rw) register accessor: Holds the default values for the floating-point status control data that the PE assigns to the FPSCR when it creates a new floating-point context  

You can [`read`](crate::Reg::read) this register and get [`fpdscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpdscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@fpdscr`]
module"]
pub type FPDSCR = crate::Reg<fpdscr::FPDSCR_SPEC>;
#[doc = "Holds the default values for the floating-point status control data that the PE assigns to the FPSCR when it creates a new floating-point context"]
pub mod fpdscr;
#[doc = "MVFR0 (rw) register accessor: Describes the features provided by the Floating-point Extension  

You can [`read`](crate::Reg::read) this register and get [`mvfr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mvfr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mvfr0`]
module"]
pub type MVFR0 = crate::Reg<mvfr0::MVFR0_SPEC>;
#[doc = "Describes the features provided by the Floating-point Extension"]
pub mod mvfr0;
#[doc = "MVFR1 (rw) register accessor: Describes the features provided by the Floating-point Extension  

You can [`read`](crate::Reg::read) this register and get [`mvfr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mvfr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mvfr1`]
module"]
pub type MVFR1 = crate::Reg<mvfr1::MVFR1_SPEC>;
#[doc = "Describes the features provided by the Floating-point Extension"]
pub mod mvfr1;
#[doc = "MVFR2 (rw) register accessor: Describes the features provided by the Floating-point Extension  

You can [`read`](crate::Reg::read) this register and get [`mvfr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mvfr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mvfr2`]
module"]
pub type MVFR2 = crate::Reg<mvfr2::MVFR2_SPEC>;
#[doc = "Describes the features provided by the Floating-point Extension"]
pub mod mvfr2;
#[doc = "DDEVARCH (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`ddevarch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddevarch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ddevarch`]
module"]
pub type DDEVARCH = crate::Reg<ddevarch::DDEVARCH_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod ddevarch;
#[doc = "DDEVTYPE (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`ddevtype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddevtype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ddevtype`]
module"]
pub type DDEVTYPE = crate::Reg<ddevtype::DDEVTYPE_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod ddevtype;
#[doc = "DPIDR4 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dpidr4`]
module"]
pub type DPIDR4 = crate::Reg<dpidr4::DPIDR4_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dpidr4;
#[doc = "DPIDR5 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dpidr5`]
module"]
pub type DPIDR5 = crate::Reg<dpidr5::DPIDR5_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dpidr5;
#[doc = "DPIDR6 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dpidr6`]
module"]
pub type DPIDR6 = crate::Reg<dpidr6::DPIDR6_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dpidr6;
#[doc = "DPIDR7 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dpidr7`]
module"]
pub type DPIDR7 = crate::Reg<dpidr7::DPIDR7_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dpidr7;
#[doc = "DPIDR0 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dpidr0`]
module"]
pub type DPIDR0 = crate::Reg<dpidr0::DPIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dpidr0;
#[doc = "DPIDR1 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dpidr1`]
module"]
pub type DPIDR1 = crate::Reg<dpidr1::DPIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dpidr1;
#[doc = "DPIDR2 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dpidr2`]
module"]
pub type DPIDR2 = crate::Reg<dpidr2::DPIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dpidr2;
#[doc = "DPIDR3 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dpidr3`]
module"]
pub type DPIDR3 = crate::Reg<dpidr3::DPIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dpidr3;
#[doc = "DCIDR0 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dcidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dcidr0`]
module"]
pub type DCIDR0 = crate::Reg<dcidr0::DCIDR0_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dcidr0;
#[doc = "DCIDR1 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dcidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dcidr1`]
module"]
pub type DCIDR1 = crate::Reg<dcidr1::DCIDR1_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dcidr1;
#[doc = "DCIDR2 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dcidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dcidr2`]
module"]
pub type DCIDR2 = crate::Reg<dcidr2::DCIDR2_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dcidr2;
#[doc = "DCIDR3 (rw) register accessor: Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dcidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dcidr3`]
module"]
pub type DCIDR3 = crate::Reg<dcidr3::DCIDR3_SPEC>;
#[doc = "Provides CoreSight discovery information for the SCS"]
pub mod dcidr3;
#[doc = "TRCPRGCTLR (rw) register accessor: Programming Control Register  

You can [`read`](crate::Reg::read) this register and get [`trcprgctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcprgctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcprgctlr`]
module"]
pub type TRCPRGCTLR = crate::Reg<trcprgctlr::TRCPRGCTLR_SPEC>;
#[doc = "Programming Control Register"]
pub mod trcprgctlr;
#[doc = "TRCSTATR (rw) register accessor: The TRCSTATR indicates the ETM-Teal status  

You can [`read`](crate::Reg::read) this register and get [`trcstatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcstatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcstatr`]
module"]
pub type TRCSTATR = crate::Reg<trcstatr::TRCSTATR_SPEC>;
#[doc = "The TRCSTATR indicates the ETM-Teal status"]
pub mod trcstatr;
#[doc = "TRCCONFIGR (rw) register accessor: The TRCCONFIGR sets the basic tracing options for the trace unit  

You can [`read`](crate::Reg::read) this register and get [`trcconfigr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcconfigr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcconfigr`]
module"]
pub type TRCCONFIGR = crate::Reg<trcconfigr::TRCCONFIGR_SPEC>;
#[doc = "The TRCCONFIGR sets the basic tracing options for the trace unit"]
pub mod trcconfigr;
#[doc = "TRCEVENTCTL0R (rw) register accessor: The TRCEVENTCTL0R controls the tracing of events in the trace stream. The events also drive the ETM-Teal external outputs.  

You can [`read`](crate::Reg::read) this register and get [`trceventctl0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trceventctl0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trceventctl0r`]
module"]
pub type TRCEVENTCTL0R = crate::Reg<trceventctl0r::TRCEVENTCTL0R_SPEC>;
#[doc = "The TRCEVENTCTL0R controls the tracing of events in the trace stream. The events also drive the ETM-Teal external outputs."]
pub mod trceventctl0r;
#[doc = "TRCEVENTCTL1R (rw) register accessor: The TRCEVENTCTL1R controls how the events selected by TRCEVENTCTL0R behave  

You can [`read`](crate::Reg::read) this register and get [`trceventctl1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trceventctl1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trceventctl1r`]
module"]
pub type TRCEVENTCTL1R = crate::Reg<trceventctl1r::TRCEVENTCTL1R_SPEC>;
#[doc = "The TRCEVENTCTL1R controls how the events selected by TRCEVENTCTL0R behave"]
pub mod trceventctl1r;
#[doc = "TRCSTALLCTLR (rw) register accessor: The TRCSTALLCTLR enables ETM-Teal to stall the processor if the ETM-Teal FIFO goes over the programmed level to minimize risk of overflow  

You can [`read`](crate::Reg::read) this register and get [`trcstallctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcstallctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcstallctlr`]
module"]
pub type TRCSTALLCTLR = crate::Reg<trcstallctlr::TRCSTALLCTLR_SPEC>;
#[doc = "The TRCSTALLCTLR enables ETM-Teal to stall the processor if the ETM-Teal FIFO goes over the programmed level to minimize risk of overflow"]
pub mod trcstallctlr;
#[doc = "TRCTSCTLR (rw) register accessor: The TRCTSCTLR controls the insertion of global timestamps into the trace stream. A timestamp is always inserted into the instruction trace stream  

You can [`read`](crate::Reg::read) this register and get [`trctsctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trctsctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trctsctlr`]
module"]
pub type TRCTSCTLR = crate::Reg<trctsctlr::TRCTSCTLR_SPEC>;
#[doc = "The TRCTSCTLR controls the insertion of global timestamps into the trace stream. A timestamp is always inserted into the instruction trace stream"]
pub mod trctsctlr;
#[doc = "TRCSYNCPR (rw) register accessor: The TRCSYNCPR specifies the period of trace synchronization of the trace streams. TRCSYNCPR defines a number of bytes of trace between requests for trace synchronization. This value is always a power of two  

You can [`read`](crate::Reg::read) this register and get [`trcsyncpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcsyncpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcsyncpr`]
module"]
pub type TRCSYNCPR = crate::Reg<trcsyncpr::TRCSYNCPR_SPEC>;
#[doc = "The TRCSYNCPR specifies the period of trace synchronization of the trace streams. TRCSYNCPR defines a number of bytes of trace between requests for trace synchronization. This value is always a power of two"]
pub mod trcsyncpr;
#[doc = "TRCCCCTLR (rw) register accessor: The TRCCCCTLR sets the threshold value for instruction trace cycle counting. The threshold represents the minimum interval between cycle count trace packets  

You can [`read`](crate::Reg::read) this register and get [`trcccctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcccctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcccctlr`]
module"]
pub type TRCCCCTLR = crate::Reg<trcccctlr::TRCCCCTLR_SPEC>;
#[doc = "The TRCCCCTLR sets the threshold value for instruction trace cycle counting. The threshold represents the minimum interval between cycle count trace packets"]
pub mod trcccctlr;
#[doc = "TRCVICTLR (rw) register accessor: The TRCVICTLR controls instruction trace filtering  

You can [`read`](crate::Reg::read) this register and get [`trcvictlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvictlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcvictlr`]
module"]
pub type TRCVICTLR = crate::Reg<trcvictlr::TRCVICTLR_SPEC>;
#[doc = "The TRCVICTLR controls instruction trace filtering"]
pub mod trcvictlr;
#[doc = "TRCCNTRLDVR0 (rw) register accessor: The TRCCNTRLDVR defines the reload value for the reduced function counter  

You can [`read`](crate::Reg::read) this register and get [`trccntrldvr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccntrldvr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trccntrldvr0`]
module"]
pub type TRCCNTRLDVR0 = crate::Reg<trccntrldvr0::TRCCNTRLDVR0_SPEC>;
#[doc = "The TRCCNTRLDVR defines the reload value for the reduced function counter"]
pub mod trccntrldvr0;
#[doc = "TRCIDR8 (rw) register accessor: TRCIDR8  

You can [`read`](crate::Reg::read) this register and get [`trcidr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr8`]
module"]
pub type TRCIDR8 = crate::Reg<trcidr8::TRCIDR8_SPEC>;
#[doc = "TRCIDR8"]
pub mod trcidr8;
#[doc = "TRCIDR9 (rw) register accessor: TRCIDR9  

You can [`read`](crate::Reg::read) this register and get [`trcidr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr9`]
module"]
pub type TRCIDR9 = crate::Reg<trcidr9::TRCIDR9_SPEC>;
#[doc = "TRCIDR9"]
pub mod trcidr9;
#[doc = "TRCIDR10 (rw) register accessor: TRCIDR10  

You can [`read`](crate::Reg::read) this register and get [`trcidr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr10`]
module"]
pub type TRCIDR10 = crate::Reg<trcidr10::TRCIDR10_SPEC>;
#[doc = "TRCIDR10"]
pub mod trcidr10;
#[doc = "TRCIDR11 (rw) register accessor: TRCIDR11  

You can [`read`](crate::Reg::read) this register and get [`trcidr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr11`]
module"]
pub type TRCIDR11 = crate::Reg<trcidr11::TRCIDR11_SPEC>;
#[doc = "TRCIDR11"]
pub mod trcidr11;
#[doc = "TRCIDR12 (rw) register accessor: TRCIDR12  

You can [`read`](crate::Reg::read) this register and get [`trcidr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr12`]
module"]
pub type TRCIDR12 = crate::Reg<trcidr12::TRCIDR12_SPEC>;
#[doc = "TRCIDR12"]
pub mod trcidr12;
#[doc = "TRCIDR13 (rw) register accessor: TRCIDR13  

You can [`read`](crate::Reg::read) this register and get [`trcidr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr13`]
module"]
pub type TRCIDR13 = crate::Reg<trcidr13::TRCIDR13_SPEC>;
#[doc = "TRCIDR13"]
pub mod trcidr13;
#[doc = "TRCIMSPEC (rw) register accessor: The TRCIMSPEC shows the presence of any IMPLEMENTATION SPECIFIC features, and enables any features that are provided  

You can [`read`](crate::Reg::read) this register and get [`trcimspec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcimspec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcimspec`]
module"]
pub type TRCIMSPEC = crate::Reg<trcimspec::TRCIMSPEC_SPEC>;
#[doc = "The TRCIMSPEC shows the presence of any IMPLEMENTATION SPECIFIC features, and enables any features that are provided"]
pub mod trcimspec;
#[doc = "TRCIDR0 (rw) register accessor: TRCIDR0  

You can [`read`](crate::Reg::read) this register and get [`trcidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr0`]
module"]
pub type TRCIDR0 = crate::Reg<trcidr0::TRCIDR0_SPEC>;
#[doc = "TRCIDR0"]
pub mod trcidr0;
#[doc = "TRCIDR1 (rw) register accessor: TRCIDR1  

You can [`read`](crate::Reg::read) this register and get [`trcidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr1`]
module"]
pub type TRCIDR1 = crate::Reg<trcidr1::TRCIDR1_SPEC>;
#[doc = "TRCIDR1"]
pub mod trcidr1;
#[doc = "TRCIDR2 (rw) register accessor: TRCIDR2  

You can [`read`](crate::Reg::read) this register and get [`trcidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr2`]
module"]
pub type TRCIDR2 = crate::Reg<trcidr2::TRCIDR2_SPEC>;
#[doc = "TRCIDR2"]
pub mod trcidr2;
#[doc = "TRCIDR3 (rw) register accessor: TRCIDR3  

You can [`read`](crate::Reg::read) this register and get [`trcidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr3`]
module"]
pub type TRCIDR3 = crate::Reg<trcidr3::TRCIDR3_SPEC>;
#[doc = "TRCIDR3"]
pub mod trcidr3;
#[doc = "TRCIDR4 (rw) register accessor: TRCIDR4  

You can [`read`](crate::Reg::read) this register and get [`trcidr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr4`]
module"]
pub type TRCIDR4 = crate::Reg<trcidr4::TRCIDR4_SPEC>;
#[doc = "TRCIDR4"]
pub mod trcidr4;
#[doc = "TRCIDR5 (rw) register accessor: TRCIDR5  

You can [`read`](crate::Reg::read) this register and get [`trcidr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr5`]
module"]
pub type TRCIDR5 = crate::Reg<trcidr5::TRCIDR5_SPEC>;
#[doc = "TRCIDR5"]
pub mod trcidr5;
#[doc = "TRCIDR6 (rw) register accessor: TRCIDR6  

You can [`read`](crate::Reg::read) this register and get [`trcidr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr6`]
module"]
pub type TRCIDR6 = crate::Reg<trcidr6::TRCIDR6_SPEC>;
#[doc = "TRCIDR6"]
pub mod trcidr6;
#[doc = "TRCIDR7 (rw) register accessor: TRCIDR7  

You can [`read`](crate::Reg::read) this register and get [`trcidr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcidr7`]
module"]
pub type TRCIDR7 = crate::Reg<trcidr7::TRCIDR7_SPEC>;
#[doc = "TRCIDR7"]
pub mod trcidr7;
#[doc = "TRCRSCTLR2 (rw) register accessor: The TRCRSCTLR controls the trace resources  

You can [`read`](crate::Reg::read) this register and get [`trcrsctlr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcrsctlr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcrsctlr2`]
module"]
pub type TRCRSCTLR2 = crate::Reg<trcrsctlr2::TRCRSCTLR2_SPEC>;
#[doc = "The TRCRSCTLR controls the trace resources"]
pub mod trcrsctlr2;
#[doc = "TRCRSCTLR3 (rw) register accessor: The TRCRSCTLR controls the trace resources  

You can [`read`](crate::Reg::read) this register and get [`trcrsctlr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcrsctlr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcrsctlr3`]
module"]
pub type TRCRSCTLR3 = crate::Reg<trcrsctlr3::TRCRSCTLR3_SPEC>;
#[doc = "The TRCRSCTLR controls the trace resources"]
pub mod trcrsctlr3;
#[doc = "TRCSSCSR (rw) register accessor: Controls the corresponding single-shot comparator resource  

You can [`read`](crate::Reg::read) this register and get [`trcsscsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcsscsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcsscsr`]
module"]
pub type TRCSSCSR = crate::Reg<trcsscsr::TRCSSCSR_SPEC>;
#[doc = "Controls the corresponding single-shot comparator resource"]
pub mod trcsscsr;
#[doc = "TRCSSPCICR (rw) register accessor: Selects the PE comparator inputs for Single-shot control  

You can [`read`](crate::Reg::read) this register and get [`trcsspcicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcsspcicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcsspcicr`]
module"]
pub type TRCSSPCICR = crate::Reg<trcsspcicr::TRCSSPCICR_SPEC>;
#[doc = "Selects the PE comparator inputs for Single-shot control"]
pub mod trcsspcicr;
#[doc = "TRCPDCR (rw) register accessor: Requests the system to provide power to the trace unit  

You can [`read`](crate::Reg::read) this register and get [`trcpdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcpdcr`]
module"]
pub type TRCPDCR = crate::Reg<trcpdcr::TRCPDCR_SPEC>;
#[doc = "Requests the system to provide power to the trace unit"]
pub mod trcpdcr;
#[doc = "TRCPDSR (rw) register accessor: Returns the following information about the trace unit: - OS Lock status. - Core power domain status. - Power interruption status  

You can [`read`](crate::Reg::read) this register and get [`trcpdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcpdsr`]
module"]
pub type TRCPDSR = crate::Reg<trcpdsr::TRCPDSR_SPEC>;
#[doc = "Returns the following information about the trace unit: - OS Lock status. - Core power domain status. - Power interruption status"]
pub mod trcpdsr;
#[doc = "TRCITATBIDR (rw) register accessor: Trace Integration ATB Identification Register  

You can [`read`](crate::Reg::read) this register and get [`trcitatbidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitatbidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcitatbidr`]
module"]
pub type TRCITATBIDR = crate::Reg<trcitatbidr::TRCITATBIDR_SPEC>;
#[doc = "Trace Integration ATB Identification Register"]
pub mod trcitatbidr;
#[doc = "TRCITIATBINR (rw) register accessor: Trace Integration Instruction ATB In Register  

You can [`read`](crate::Reg::read) this register and get [`trcitiatbinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitiatbinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcitiatbinr`]
module"]
pub type TRCITIATBINR = crate::Reg<trcitiatbinr::TRCITIATBINR_SPEC>;
#[doc = "Trace Integration Instruction ATB In Register"]
pub mod trcitiatbinr;
#[doc = "TRCITIATBOUTR (rw) register accessor: Trace Integration Instruction ATB Out Register  

You can [`read`](crate::Reg::read) this register and get [`trcitiatboutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitiatboutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcitiatboutr`]
module"]
pub type TRCITIATBOUTR = crate::Reg<trcitiatboutr::TRCITIATBOUTR_SPEC>;
#[doc = "Trace Integration Instruction ATB Out Register"]
pub mod trcitiatboutr;
#[doc = "TRCCLAIMSET (rw) register accessor: Claim Tag Set Register  

You can [`read`](crate::Reg::read) this register and get [`trcclaimset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclaimset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcclaimset`]
module"]
pub type TRCCLAIMSET = crate::Reg<trcclaimset::TRCCLAIMSET_SPEC>;
#[doc = "Claim Tag Set Register"]
pub mod trcclaimset;
#[doc = "TRCCLAIMCLR (rw) register accessor: Claim Tag Clear Register  

You can [`read`](crate::Reg::read) this register and get [`trcclaimclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclaimclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcclaimclr`]
module"]
pub type TRCCLAIMCLR = crate::Reg<trcclaimclr::TRCCLAIMCLR_SPEC>;
#[doc = "Claim Tag Clear Register"]
pub mod trcclaimclr;
#[doc = "TRCAUTHSTATUS (rw) register accessor: Returns the level of tracing that the trace unit can support  

You can [`read`](crate::Reg::read) this register and get [`trcauthstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcauthstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcauthstatus`]
module"]
pub type TRCAUTHSTATUS = crate::Reg<trcauthstatus::TRCAUTHSTATUS_SPEC>;
#[doc = "Returns the level of tracing that the trace unit can support"]
pub mod trcauthstatus;
#[doc = "TRCDEVARCH (rw) register accessor: TRCDEVARCH  

You can [`read`](crate::Reg::read) this register and get [`trcdevarch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcdevarch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcdevarch`]
module"]
pub type TRCDEVARCH = crate::Reg<trcdevarch::TRCDEVARCH_SPEC>;
#[doc = "TRCDEVARCH"]
pub mod trcdevarch;
#[doc = "TRCDEVID (rw) register accessor: TRCDEVID  

You can [`read`](crate::Reg::read) this register and get [`trcdevid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcdevid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcdevid`]
module"]
pub type TRCDEVID = crate::Reg<trcdevid::TRCDEVID_SPEC>;
#[doc = "TRCDEVID"]
pub mod trcdevid;
#[doc = "TRCDEVTYPE (rw) register accessor: TRCDEVTYPE  

You can [`read`](crate::Reg::read) this register and get [`trcdevtype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcdevtype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcdevtype`]
module"]
pub type TRCDEVTYPE = crate::Reg<trcdevtype::TRCDEVTYPE_SPEC>;
#[doc = "TRCDEVTYPE"]
pub mod trcdevtype;
#[doc = "TRCPIDR4 (rw) register accessor: TRCPIDR4  

You can [`read`](crate::Reg::read) this register and get [`trcpidr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcpidr4`]
module"]
pub type TRCPIDR4 = crate::Reg<trcpidr4::TRCPIDR4_SPEC>;
#[doc = "TRCPIDR4"]
pub mod trcpidr4;
#[doc = "TRCPIDR5 (rw) register accessor: TRCPIDR5  

You can [`read`](crate::Reg::read) this register and get [`trcpidr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcpidr5`]
module"]
pub type TRCPIDR5 = crate::Reg<trcpidr5::TRCPIDR5_SPEC>;
#[doc = "TRCPIDR5"]
pub mod trcpidr5;
#[doc = "TRCPIDR6 (rw) register accessor: TRCPIDR6  

You can [`read`](crate::Reg::read) this register and get [`trcpidr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcpidr6`]
module"]
pub type TRCPIDR6 = crate::Reg<trcpidr6::TRCPIDR6_SPEC>;
#[doc = "TRCPIDR6"]
pub mod trcpidr6;
#[doc = "TRCPIDR7 (rw) register accessor: TRCPIDR7  

You can [`read`](crate::Reg::read) this register and get [`trcpidr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcpidr7`]
module"]
pub type TRCPIDR7 = crate::Reg<trcpidr7::TRCPIDR7_SPEC>;
#[doc = "TRCPIDR7"]
pub mod trcpidr7;
#[doc = "TRCPIDR0 (rw) register accessor: TRCPIDR0  

You can [`read`](crate::Reg::read) this register and get [`trcpidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcpidr0`]
module"]
pub type TRCPIDR0 = crate::Reg<trcpidr0::TRCPIDR0_SPEC>;
#[doc = "TRCPIDR0"]
pub mod trcpidr0;
#[doc = "TRCPIDR1 (rw) register accessor: TRCPIDR1  

You can [`read`](crate::Reg::read) this register and get [`trcpidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcpidr1`]
module"]
pub type TRCPIDR1 = crate::Reg<trcpidr1::TRCPIDR1_SPEC>;
#[doc = "TRCPIDR1"]
pub mod trcpidr1;
#[doc = "TRCPIDR2 (rw) register accessor: TRCPIDR2  

You can [`read`](crate::Reg::read) this register and get [`trcpidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcpidr2`]
module"]
pub type TRCPIDR2 = crate::Reg<trcpidr2::TRCPIDR2_SPEC>;
#[doc = "TRCPIDR2"]
pub mod trcpidr2;
#[doc = "TRCPIDR3 (rw) register accessor: TRCPIDR3  

You can [`read`](crate::Reg::read) this register and get [`trcpidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trcpidr3`]
module"]
pub type TRCPIDR3 = crate::Reg<trcpidr3::TRCPIDR3_SPEC>;
#[doc = "TRCPIDR3"]
pub mod trcpidr3;
#[doc = "TRCCIDR0 (rw) register accessor: TRCCIDR0  

You can [`read`](crate::Reg::read) this register and get [`trccidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trccidr0`]
module"]
pub type TRCCIDR0 = crate::Reg<trccidr0::TRCCIDR0_SPEC>;
#[doc = "TRCCIDR0"]
pub mod trccidr0;
#[doc = "TRCCIDR1 (rw) register accessor: TRCCIDR1  

You can [`read`](crate::Reg::read) this register and get [`trccidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trccidr1`]
module"]
pub type TRCCIDR1 = crate::Reg<trccidr1::TRCCIDR1_SPEC>;
#[doc = "TRCCIDR1"]
pub mod trccidr1;
#[doc = "TRCCIDR2 (rw) register accessor: TRCCIDR2  

You can [`read`](crate::Reg::read) this register and get [`trccidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trccidr2`]
module"]
pub type TRCCIDR2 = crate::Reg<trccidr2::TRCCIDR2_SPEC>;
#[doc = "TRCCIDR2"]
pub mod trccidr2;
#[doc = "TRCCIDR3 (rw) register accessor: TRCCIDR3  

You can [`read`](crate::Reg::read) this register and get [`trccidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trccidr3`]
module"]
pub type TRCCIDR3 = crate::Reg<trccidr3::TRCCIDR3_SPEC>;
#[doc = "TRCCIDR3"]
pub mod trccidr3;
#[doc = "CTICONTROL (rw) register accessor: CTI Control Register  

You can [`read`](crate::Reg::read) this register and get [`cticontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cticontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cticontrol`]
module"]
pub type CTICONTROL = crate::Reg<cticontrol::CTICONTROL_SPEC>;
#[doc = "CTI Control Register"]
pub mod cticontrol;
#[doc = "CTIINTACK (rw) register accessor: CTI Interrupt Acknowledge Register  

You can [`read`](crate::Reg::read) this register and get [`ctiintack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiintack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiintack`]
module"]
pub type CTIINTACK = crate::Reg<ctiintack::CTIINTACK_SPEC>;
#[doc = "CTI Interrupt Acknowledge Register"]
pub mod ctiintack;
#[doc = "CTIAPPSET (rw) register accessor: CTI Application Trigger Set Register  

You can [`read`](crate::Reg::read) this register and get [`ctiappset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiappset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiappset`]
module"]
pub type CTIAPPSET = crate::Reg<ctiappset::CTIAPPSET_SPEC>;
#[doc = "CTI Application Trigger Set Register"]
pub mod ctiappset;
#[doc = "CTIAPPCLEAR (rw) register accessor: CTI Application Trigger Clear Register  

You can [`read`](crate::Reg::read) this register and get [`ctiappclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiappclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiappclear`]
module"]
pub type CTIAPPCLEAR = crate::Reg<ctiappclear::CTIAPPCLEAR_SPEC>;
#[doc = "CTI Application Trigger Clear Register"]
pub mod ctiappclear;
#[doc = "CTIAPPPULSE (rw) register accessor: CTI Application Pulse Register  

You can [`read`](crate::Reg::read) this register and get [`ctiapppulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiapppulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiapppulse`]
module"]
pub type CTIAPPPULSE = crate::Reg<ctiapppulse::CTIAPPPULSE_SPEC>;
#[doc = "CTI Application Pulse Register"]
pub mod ctiapppulse;
#[doc = "CTIINEN0 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiinen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiinen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiinen0`]
module"]
pub type CTIINEN0 = crate::Reg<ctiinen0::CTIINEN0_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiinen0;
#[doc = "CTIINEN1 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiinen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiinen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiinen1`]
module"]
pub type CTIINEN1 = crate::Reg<ctiinen1::CTIINEN1_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiinen1;
#[doc = "CTIINEN2 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiinen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiinen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiinen2`]
module"]
pub type CTIINEN2 = crate::Reg<ctiinen2::CTIINEN2_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiinen2;
#[doc = "CTIINEN3 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiinen3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiinen3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiinen3`]
module"]
pub type CTIINEN3 = crate::Reg<ctiinen3::CTIINEN3_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiinen3;
#[doc = "CTIINEN4 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiinen4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiinen4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiinen4`]
module"]
pub type CTIINEN4 = crate::Reg<ctiinen4::CTIINEN4_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiinen4;
#[doc = "CTIINEN5 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiinen5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiinen5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiinen5`]
module"]
pub type CTIINEN5 = crate::Reg<ctiinen5::CTIINEN5_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiinen5;
#[doc = "CTIINEN6 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiinen6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiinen6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiinen6`]
module"]
pub type CTIINEN6 = crate::Reg<ctiinen6::CTIINEN6_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiinen6;
#[doc = "CTIINEN7 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiinen7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiinen7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiinen7`]
module"]
pub type CTIINEN7 = crate::Reg<ctiinen7::CTIINEN7_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiinen7;
#[doc = "CTIOUTEN0 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiouten0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiouten0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiouten0`]
module"]
pub type CTIOUTEN0 = crate::Reg<ctiouten0::CTIOUTEN0_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiouten0;
#[doc = "CTIOUTEN1 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiouten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiouten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiouten1`]
module"]
pub type CTIOUTEN1 = crate::Reg<ctiouten1::CTIOUTEN1_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiouten1;
#[doc = "CTIOUTEN2 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiouten2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiouten2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiouten2`]
module"]
pub type CTIOUTEN2 = crate::Reg<ctiouten2::CTIOUTEN2_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiouten2;
#[doc = "CTIOUTEN3 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiouten3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiouten3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiouten3`]
module"]
pub type CTIOUTEN3 = crate::Reg<ctiouten3::CTIOUTEN3_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiouten3;
#[doc = "CTIOUTEN4 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiouten4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiouten4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiouten4`]
module"]
pub type CTIOUTEN4 = crate::Reg<ctiouten4::CTIOUTEN4_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiouten4;
#[doc = "CTIOUTEN5 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiouten5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiouten5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiouten5`]
module"]
pub type CTIOUTEN5 = crate::Reg<ctiouten5::CTIOUTEN5_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiouten5;
#[doc = "CTIOUTEN6 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiouten6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiouten6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiouten6`]
module"]
pub type CTIOUTEN6 = crate::Reg<ctiouten6::CTIOUTEN6_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiouten6;
#[doc = "CTIOUTEN7 (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiouten7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiouten7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctiouten7`]
module"]
pub type CTIOUTEN7 = crate::Reg<ctiouten7::CTIOUTEN7_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctiouten7;
#[doc = "CTITRIGINSTATUS (rw) register accessor: CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctitriginstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctitriginstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctitriginstatus`]
module"]
pub type CTITRIGINSTATUS = crate::Reg<ctitriginstatus::CTITRIGINSTATUS_SPEC>;
#[doc = "CTI Trigger to Channel Enable Registers"]
pub mod ctitriginstatus;
#[doc = "CTITRIGOUTSTATUS (rw) register accessor: CTI Trigger In Status Register  

You can [`read`](crate::Reg::read) this register and get [`ctitrigoutstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctitrigoutstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctitrigoutstatus`]
module"]
pub type CTITRIGOUTSTATUS = crate::Reg<ctitrigoutstatus::CTITRIGOUTSTATUS_SPEC>;
#[doc = "CTI Trigger In Status Register"]
pub mod ctitrigoutstatus;
#[doc = "CTICHINSTATUS (rw) register accessor: CTI Channel In Status Register  

You can [`read`](crate::Reg::read) this register and get [`ctichinstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctichinstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctichinstatus`]
module"]
pub type CTICHINSTATUS = crate::Reg<ctichinstatus::CTICHINSTATUS_SPEC>;
#[doc = "CTI Channel In Status Register"]
pub mod ctichinstatus;
#[doc = "CTIGATE (rw) register accessor: Enable CTI Channel Gate register  

You can [`read`](crate::Reg::read) this register and get [`ctigate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctigate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctigate`]
module"]
pub type CTIGATE = crate::Reg<ctigate::CTIGATE_SPEC>;
#[doc = "Enable CTI Channel Gate register"]
pub mod ctigate;
#[doc = "ASICCTL (rw) register accessor: External Multiplexer Control register  

You can [`read`](crate::Reg::read) this register and get [`asicctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asicctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@asicctl`]
module"]
pub type ASICCTL = crate::Reg<asicctl::ASICCTL_SPEC>;
#[doc = "External Multiplexer Control register"]
pub mod asicctl;
#[doc = "ITCHOUT (rw) register accessor: Integration Test Channel Output register  

You can [`read`](crate::Reg::read) this register and get [`itchout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itchout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itchout`]
module"]
pub type ITCHOUT = crate::Reg<itchout::ITCHOUT_SPEC>;
#[doc = "Integration Test Channel Output register"]
pub mod itchout;
#[doc = "ITTRIGOUT (rw) register accessor: Integration Test Trigger Output register  

You can [`read`](crate::Reg::read) this register and get [`ittrigout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrigout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ittrigout`]
module"]
pub type ITTRIGOUT = crate::Reg<ittrigout::ITTRIGOUT_SPEC>;
#[doc = "Integration Test Trigger Output register"]
pub mod ittrigout;
#[doc = "ITCHIN (rw) register accessor: Integration Test Channel Input register  

You can [`read`](crate::Reg::read) this register and get [`itchin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itchin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itchin`]
module"]
pub type ITCHIN = crate::Reg<itchin::ITCHIN_SPEC>;
#[doc = "Integration Test Channel Input register"]
pub mod itchin;
#[doc = "ITCTRL (rw) register accessor: Integration Mode Control register  

You can [`read`](crate::Reg::read) this register and get [`itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@itctrl`]
module"]
pub type ITCTRL = crate::Reg<itctrl::ITCTRL_SPEC>;
#[doc = "Integration Mode Control register"]
pub mod itctrl;
#[doc = "DEVARCH (rw) register accessor: Device Architecture register  

You can [`read`](crate::Reg::read) this register and get [`devarch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devarch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@devarch`]
module"]
pub type DEVARCH = crate::Reg<devarch::DEVARCH_SPEC>;
#[doc = "Device Architecture register"]
pub mod devarch;
#[doc = "DEVID (rw) register accessor: Device Configuration register  

You can [`read`](crate::Reg::read) this register and get [`devid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@devid`]
module"]
pub type DEVID = crate::Reg<devid::DEVID_SPEC>;
#[doc = "Device Configuration register"]
pub mod devid;
#[doc = "DEVTYPE (rw) register accessor: Device Type Identifier register  

You can [`read`](crate::Reg::read) this register and get [`devtype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devtype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@devtype`]
module"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "Device Type Identifier register"]
pub mod devtype;
#[doc = "PIDR4 (rw) register accessor: CoreSight Peripheral ID4  

You can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pidr4`]
module"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
#[doc = "CoreSight Peripheral ID4"]
pub mod pidr4;
#[doc = "PIDR5 (rw) register accessor: CoreSight Peripheral ID5  

You can [`read`](crate::Reg::read) this register and get [`pidr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pidr5`]
module"]
pub type PIDR5 = crate::Reg<pidr5::PIDR5_SPEC>;
#[doc = "CoreSight Peripheral ID5"]
pub mod pidr5;
#[doc = "PIDR6 (rw) register accessor: CoreSight Peripheral ID6  

You can [`read`](crate::Reg::read) this register and get [`pidr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pidr6`]
module"]
pub type PIDR6 = crate::Reg<pidr6::PIDR6_SPEC>;
#[doc = "CoreSight Peripheral ID6"]
pub mod pidr6;
#[doc = "PIDR7 (rw) register accessor: CoreSight Peripheral ID7  

You can [`read`](crate::Reg::read) this register and get [`pidr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pidr7`]
module"]
pub type PIDR7 = crate::Reg<pidr7::PIDR7_SPEC>;
#[doc = "CoreSight Peripheral ID7"]
pub mod pidr7;
#[doc = "PIDR0 (rw) register accessor: CoreSight Peripheral ID0  

You can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pidr0`]
module"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
#[doc = "CoreSight Peripheral ID0"]
pub mod pidr0;
#[doc = "PIDR1 (rw) register accessor: CoreSight Peripheral ID1  

You can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pidr1`]
module"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
#[doc = "CoreSight Peripheral ID1"]
pub mod pidr1;
#[doc = "PIDR2 (rw) register accessor: CoreSight Peripheral ID2  

You can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pidr2`]
module"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
#[doc = "CoreSight Peripheral ID2"]
pub mod pidr2;
#[doc = "PIDR3 (rw) register accessor: CoreSight Peripheral ID3  

You can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@pidr3`]
module"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
#[doc = "CoreSight Peripheral ID3"]
pub mod pidr3;
#[doc = "CIDR0 (rw) register accessor: CoreSight Component ID0  

You can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cidr0`]
module"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "CoreSight Component ID0"]
pub mod cidr0;
#[doc = "CIDR1 (rw) register accessor: CoreSight Component ID1  

You can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cidr1`]
module"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "CoreSight Component ID1"]
pub mod cidr1;
#[doc = "CIDR2 (rw) register accessor: CoreSight Component ID2  

You can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cidr2`]
module"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "CoreSight Component ID2"]
pub mod cidr2;
#[doc = "CIDR3 (rw) register accessor: CoreSight Component ID3  

You can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cidr3`]
module"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "CoreSight Component ID3"]
pub mod cidr3;
