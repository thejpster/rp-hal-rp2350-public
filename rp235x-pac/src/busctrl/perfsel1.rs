#[doc = "Register `PERFSEL1` reader"]
pub type R = crate::R<PERFSEL1_SPEC>;
#[doc = "Register `PERFSEL1` writer"]
pub type W = crate::W<PERFSEL1_SPEC>;
#[doc = "Select an event for PERFCTR1. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters.  

Value on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERFSEL1_A {
    #[doc = "0: `0`"]
    SIOB_PROC1_STALL_UPSTREAM = 0,
    #[doc = "1: `1`"]
    SIOB_PROC1_STALL_DOWNSTREAM = 1,
    #[doc = "2: `10`"]
    SIOB_PROC1_ACCESS_CONTESTED = 2,
    #[doc = "3: `11`"]
    SIOB_PROC1_ACCESS = 3,
    #[doc = "4: `100`"]
    SIOB_PROC0_STALL_UPSTREAM = 4,
    #[doc = "5: `101`"]
    SIOB_PROC0_STALL_DOWNSTREAM = 5,
    #[doc = "6: `110`"]
    SIOB_PROC0_ACCESS_CONTESTED = 6,
    #[doc = "7: `111`"]
    SIOB_PROC0_ACCESS = 7,
    #[doc = "8: `1000`"]
    APB_STALL_UPSTREAM = 8,
    #[doc = "9: `1001`"]
    APB_STALL_DOWNSTREAM = 9,
    #[doc = "10: `1010`"]
    APB_ACCESS_CONTESTED = 10,
    #[doc = "11: `1011`"]
    APB_ACCESS = 11,
    #[doc = "12: `1100`"]
    FASTPERI_STALL_UPSTREAM = 12,
    #[doc = "13: `1101`"]
    FASTPERI_STALL_DOWNSTREAM = 13,
    #[doc = "14: `1110`"]
    FASTPERI_ACCESS_CONTESTED = 14,
    #[doc = "15: `1111`"]
    FASTPERI_ACCESS = 15,
    #[doc = "16: `10000`"]
    SRAM9_STALL_UPSTREAM = 16,
    #[doc = "17: `10001`"]
    SRAM9_STALL_DOWNSTREAM = 17,
    #[doc = "18: `10010`"]
    SRAM9_ACCESS_CONTESTED = 18,
    #[doc = "19: `10011`"]
    SRAM9_ACCESS = 19,
    #[doc = "20: `10100`"]
    SRAM8_STALL_UPSTREAM = 20,
    #[doc = "21: `10101`"]
    SRAM8_STALL_DOWNSTREAM = 21,
    #[doc = "22: `10110`"]
    SRAM8_ACCESS_CONTESTED = 22,
    #[doc = "23: `10111`"]
    SRAM8_ACCESS = 23,
    #[doc = "24: `11000`"]
    SRAM7_STALL_UPSTREAM = 24,
    #[doc = "25: `11001`"]
    SRAM7_STALL_DOWNSTREAM = 25,
    #[doc = "26: `11010`"]
    SRAM7_ACCESS_CONTESTED = 26,
    #[doc = "27: `11011`"]
    SRAM7_ACCESS = 27,
    #[doc = "28: `11100`"]
    SRAM6_STALL_UPSTREAM = 28,
    #[doc = "29: `11101`"]
    SRAM6_STALL_DOWNSTREAM = 29,
    #[doc = "30: `11110`"]
    SRAM6_ACCESS_CONTESTED = 30,
    #[doc = "31: `11111`"]
    SRAM6_ACCESS = 31,
    #[doc = "32: `100000`"]
    SRAM5_STALL_UPSTREAM = 32,
    #[doc = "33: `100001`"]
    SRAM5_STALL_DOWNSTREAM = 33,
    #[doc = "34: `100010`"]
    SRAM5_ACCESS_CONTESTED = 34,
    #[doc = "35: `100011`"]
    SRAM5_ACCESS = 35,
    #[doc = "36: `100100`"]
    SRAM4_STALL_UPSTREAM = 36,
    #[doc = "37: `100101`"]
    SRAM4_STALL_DOWNSTREAM = 37,
    #[doc = "38: `100110`"]
    SRAM4_ACCESS_CONTESTED = 38,
    #[doc = "39: `100111`"]
    SRAM4_ACCESS = 39,
    #[doc = "40: `101000`"]
    SRAM3_STALL_UPSTREAM = 40,
    #[doc = "41: `101001`"]
    SRAM3_STALL_DOWNSTREAM = 41,
    #[doc = "42: `101010`"]
    SRAM3_ACCESS_CONTESTED = 42,
    #[doc = "43: `101011`"]
    SRAM3_ACCESS = 43,
    #[doc = "44: `101100`"]
    SRAM2_STALL_UPSTREAM = 44,
    #[doc = "45: `101101`"]
    SRAM2_STALL_DOWNSTREAM = 45,
    #[doc = "46: `101110`"]
    SRAM2_ACCESS_CONTESTED = 46,
    #[doc = "47: `101111`"]
    SRAM2_ACCESS = 47,
    #[doc = "48: `110000`"]
    SRAM1_STALL_UPSTREAM = 48,
    #[doc = "49: `110001`"]
    SRAM1_STALL_DOWNSTREAM = 49,
    #[doc = "50: `110010`"]
    SRAM1_ACCESS_CONTESTED = 50,
    #[doc = "51: `110011`"]
    SRAM1_ACCESS = 51,
    #[doc = "52: `110100`"]
    SRAM0_STALL_UPSTREAM = 52,
    #[doc = "53: `110101`"]
    SRAM0_STALL_DOWNSTREAM = 53,
    #[doc = "54: `110110`"]
    SRAM0_ACCESS_CONTESTED = 54,
    #[doc = "55: `110111`"]
    SRAM0_ACCESS = 55,
    #[doc = "56: `111000`"]
    XIP_MAIN1_STALL_UPSTREAM = 56,
    #[doc = "57: `111001`"]
    XIP_MAIN1_STALL_DOWNSTREAM = 57,
    #[doc = "58: `111010`"]
    XIP_MAIN1_ACCESS_CONTESTED = 58,
    #[doc = "59: `111011`"]
    XIP_MAIN1_ACCESS = 59,
    #[doc = "60: `111100`"]
    XIP_MAIN0_STALL_UPSTREAM = 60,
    #[doc = "61: `111101`"]
    XIP_MAIN0_STALL_DOWNSTREAM = 61,
    #[doc = "62: `111110`"]
    XIP_MAIN0_ACCESS_CONTESTED = 62,
    #[doc = "63: `111111`"]
    XIP_MAIN0_ACCESS = 63,
    #[doc = "64: `1000000`"]
    ROM_STALL_UPSTREAM = 64,
    #[doc = "65: `1000001`"]
    ROM_STALL_DOWNSTREAM = 65,
    #[doc = "66: `1000010`"]
    ROM_ACCESS_CONTESTED = 66,
    #[doc = "67: `1000011`"]
    ROM_ACCESS = 67,
}
impl From<PERFSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: PERFSEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PERFSEL1_A {
    type Ux = u8;
}
impl crate::IsEnum for PERFSEL1_A {}
#[doc = "Field `PERFSEL1` reader - Select an event for PERFCTR1. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters."]
pub type PERFSEL1_R = crate::FieldReader<PERFSEL1_A>;
impl PERFSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PERFSEL1_A> {
        match self.bits {
            0 => Some(PERFSEL1_A::SIOB_PROC1_STALL_UPSTREAM),
            1 => Some(PERFSEL1_A::SIOB_PROC1_STALL_DOWNSTREAM),
            2 => Some(PERFSEL1_A::SIOB_PROC1_ACCESS_CONTESTED),
            3 => Some(PERFSEL1_A::SIOB_PROC1_ACCESS),
            4 => Some(PERFSEL1_A::SIOB_PROC0_STALL_UPSTREAM),
            5 => Some(PERFSEL1_A::SIOB_PROC0_STALL_DOWNSTREAM),
            6 => Some(PERFSEL1_A::SIOB_PROC0_ACCESS_CONTESTED),
            7 => Some(PERFSEL1_A::SIOB_PROC0_ACCESS),
            8 => Some(PERFSEL1_A::APB_STALL_UPSTREAM),
            9 => Some(PERFSEL1_A::APB_STALL_DOWNSTREAM),
            10 => Some(PERFSEL1_A::APB_ACCESS_CONTESTED),
            11 => Some(PERFSEL1_A::APB_ACCESS),
            12 => Some(PERFSEL1_A::FASTPERI_STALL_UPSTREAM),
            13 => Some(PERFSEL1_A::FASTPERI_STALL_DOWNSTREAM),
            14 => Some(PERFSEL1_A::FASTPERI_ACCESS_CONTESTED),
            15 => Some(PERFSEL1_A::FASTPERI_ACCESS),
            16 => Some(PERFSEL1_A::SRAM9_STALL_UPSTREAM),
            17 => Some(PERFSEL1_A::SRAM9_STALL_DOWNSTREAM),
            18 => Some(PERFSEL1_A::SRAM9_ACCESS_CONTESTED),
            19 => Some(PERFSEL1_A::SRAM9_ACCESS),
            20 => Some(PERFSEL1_A::SRAM8_STALL_UPSTREAM),
            21 => Some(PERFSEL1_A::SRAM8_STALL_DOWNSTREAM),
            22 => Some(PERFSEL1_A::SRAM8_ACCESS_CONTESTED),
            23 => Some(PERFSEL1_A::SRAM8_ACCESS),
            24 => Some(PERFSEL1_A::SRAM7_STALL_UPSTREAM),
            25 => Some(PERFSEL1_A::SRAM7_STALL_DOWNSTREAM),
            26 => Some(PERFSEL1_A::SRAM7_ACCESS_CONTESTED),
            27 => Some(PERFSEL1_A::SRAM7_ACCESS),
            28 => Some(PERFSEL1_A::SRAM6_STALL_UPSTREAM),
            29 => Some(PERFSEL1_A::SRAM6_STALL_DOWNSTREAM),
            30 => Some(PERFSEL1_A::SRAM6_ACCESS_CONTESTED),
            31 => Some(PERFSEL1_A::SRAM6_ACCESS),
            32 => Some(PERFSEL1_A::SRAM5_STALL_UPSTREAM),
            33 => Some(PERFSEL1_A::SRAM5_STALL_DOWNSTREAM),
            34 => Some(PERFSEL1_A::SRAM5_ACCESS_CONTESTED),
            35 => Some(PERFSEL1_A::SRAM5_ACCESS),
            36 => Some(PERFSEL1_A::SRAM4_STALL_UPSTREAM),
            37 => Some(PERFSEL1_A::SRAM4_STALL_DOWNSTREAM),
            38 => Some(PERFSEL1_A::SRAM4_ACCESS_CONTESTED),
            39 => Some(PERFSEL1_A::SRAM4_ACCESS),
            40 => Some(PERFSEL1_A::SRAM3_STALL_UPSTREAM),
            41 => Some(PERFSEL1_A::SRAM3_STALL_DOWNSTREAM),
            42 => Some(PERFSEL1_A::SRAM3_ACCESS_CONTESTED),
            43 => Some(PERFSEL1_A::SRAM3_ACCESS),
            44 => Some(PERFSEL1_A::SRAM2_STALL_UPSTREAM),
            45 => Some(PERFSEL1_A::SRAM2_STALL_DOWNSTREAM),
            46 => Some(PERFSEL1_A::SRAM2_ACCESS_CONTESTED),
            47 => Some(PERFSEL1_A::SRAM2_ACCESS),
            48 => Some(PERFSEL1_A::SRAM1_STALL_UPSTREAM),
            49 => Some(PERFSEL1_A::SRAM1_STALL_DOWNSTREAM),
            50 => Some(PERFSEL1_A::SRAM1_ACCESS_CONTESTED),
            51 => Some(PERFSEL1_A::SRAM1_ACCESS),
            52 => Some(PERFSEL1_A::SRAM0_STALL_UPSTREAM),
            53 => Some(PERFSEL1_A::SRAM0_STALL_DOWNSTREAM),
            54 => Some(PERFSEL1_A::SRAM0_ACCESS_CONTESTED),
            55 => Some(PERFSEL1_A::SRAM0_ACCESS),
            56 => Some(PERFSEL1_A::XIP_MAIN1_STALL_UPSTREAM),
            57 => Some(PERFSEL1_A::XIP_MAIN1_STALL_DOWNSTREAM),
            58 => Some(PERFSEL1_A::XIP_MAIN1_ACCESS_CONTESTED),
            59 => Some(PERFSEL1_A::XIP_MAIN1_ACCESS),
            60 => Some(PERFSEL1_A::XIP_MAIN0_STALL_UPSTREAM),
            61 => Some(PERFSEL1_A::XIP_MAIN0_STALL_DOWNSTREAM),
            62 => Some(PERFSEL1_A::XIP_MAIN0_ACCESS_CONTESTED),
            63 => Some(PERFSEL1_A::XIP_MAIN0_ACCESS),
            64 => Some(PERFSEL1_A::ROM_STALL_UPSTREAM),
            65 => Some(PERFSEL1_A::ROM_STALL_DOWNSTREAM),
            66 => Some(PERFSEL1_A::ROM_ACCESS_CONTESTED),
            67 => Some(PERFSEL1_A::ROM_ACCESS),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_siob_proc1_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SIOB_PROC1_STALL_UPSTREAM
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_siob_proc1_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SIOB_PROC1_STALL_DOWNSTREAM
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_siob_proc1_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SIOB_PROC1_ACCESS_CONTESTED
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_siob_proc1_access(&self) -> bool {
        *self == PERFSEL1_A::SIOB_PROC1_ACCESS
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_siob_proc0_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SIOB_PROC0_STALL_UPSTREAM
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_siob_proc0_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SIOB_PROC0_STALL_DOWNSTREAM
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_siob_proc0_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SIOB_PROC0_ACCESS_CONTESTED
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_siob_proc0_access(&self) -> bool {
        *self == PERFSEL1_A::SIOB_PROC0_ACCESS
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_apb_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::APB_STALL_UPSTREAM
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_apb_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::APB_STALL_DOWNSTREAM
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_apb_access_contested(&self) -> bool {
        *self == PERFSEL1_A::APB_ACCESS_CONTESTED
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_apb_access(&self) -> bool {
        *self == PERFSEL1_A::APB_ACCESS
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_fastperi_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::FASTPERI_STALL_UPSTREAM
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_fastperi_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::FASTPERI_STALL_DOWNSTREAM
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_fastperi_access_contested(&self) -> bool {
        *self == PERFSEL1_A::FASTPERI_ACCESS_CONTESTED
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_fastperi_access(&self) -> bool {
        *self == PERFSEL1_A::FASTPERI_ACCESS
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_sram9_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM9_STALL_UPSTREAM
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_sram9_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM9_STALL_DOWNSTREAM
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_sram9_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM9_ACCESS_CONTESTED
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_sram9_access(&self) -> bool {
        *self == PERFSEL1_A::SRAM9_ACCESS
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn is_sram8_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM8_STALL_UPSTREAM
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn is_sram8_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM8_STALL_DOWNSTREAM
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn is_sram8_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM8_ACCESS_CONTESTED
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn is_sram8_access(&self) -> bool {
        *self == PERFSEL1_A::SRAM8_ACCESS
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn is_sram7_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM7_STALL_UPSTREAM
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn is_sram7_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM7_STALL_DOWNSTREAM
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn is_sram7_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM7_ACCESS_CONTESTED
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn is_sram7_access(&self) -> bool {
        *self == PERFSEL1_A::SRAM7_ACCESS
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn is_sram6_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM6_STALL_UPSTREAM
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn is_sram6_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM6_STALL_DOWNSTREAM
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn is_sram6_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM6_ACCESS_CONTESTED
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn is_sram6_access(&self) -> bool {
        *self == PERFSEL1_A::SRAM6_ACCESS
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn is_sram5_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM5_STALL_UPSTREAM
    }
    #[doc = "`100001`"]
    #[inline(always)]
    pub fn is_sram5_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM5_STALL_DOWNSTREAM
    }
    #[doc = "`100010`"]
    #[inline(always)]
    pub fn is_sram5_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM5_ACCESS_CONTESTED
    }
    #[doc = "`100011`"]
    #[inline(always)]
    pub fn is_sram5_access(&self) -> bool {
        *self == PERFSEL1_A::SRAM5_ACCESS
    }
    #[doc = "`100100`"]
    #[inline(always)]
    pub fn is_sram4_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM4_STALL_UPSTREAM
    }
    #[doc = "`100101`"]
    #[inline(always)]
    pub fn is_sram4_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM4_STALL_DOWNSTREAM
    }
    #[doc = "`100110`"]
    #[inline(always)]
    pub fn is_sram4_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM4_ACCESS_CONTESTED
    }
    #[doc = "`100111`"]
    #[inline(always)]
    pub fn is_sram4_access(&self) -> bool {
        *self == PERFSEL1_A::SRAM4_ACCESS
    }
    #[doc = "`101000`"]
    #[inline(always)]
    pub fn is_sram3_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM3_STALL_UPSTREAM
    }
    #[doc = "`101001`"]
    #[inline(always)]
    pub fn is_sram3_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM3_STALL_DOWNSTREAM
    }
    #[doc = "`101010`"]
    #[inline(always)]
    pub fn is_sram3_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM3_ACCESS_CONTESTED
    }
    #[doc = "`101011`"]
    #[inline(always)]
    pub fn is_sram3_access(&self) -> bool {
        *self == PERFSEL1_A::SRAM3_ACCESS
    }
    #[doc = "`101100`"]
    #[inline(always)]
    pub fn is_sram2_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM2_STALL_UPSTREAM
    }
    #[doc = "`101101`"]
    #[inline(always)]
    pub fn is_sram2_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM2_STALL_DOWNSTREAM
    }
    #[doc = "`101110`"]
    #[inline(always)]
    pub fn is_sram2_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM2_ACCESS_CONTESTED
    }
    #[doc = "`101111`"]
    #[inline(always)]
    pub fn is_sram2_access(&self) -> bool {
        *self == PERFSEL1_A::SRAM2_ACCESS
    }
    #[doc = "`110000`"]
    #[inline(always)]
    pub fn is_sram1_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM1_STALL_UPSTREAM
    }
    #[doc = "`110001`"]
    #[inline(always)]
    pub fn is_sram1_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM1_STALL_DOWNSTREAM
    }
    #[doc = "`110010`"]
    #[inline(always)]
    pub fn is_sram1_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM1_ACCESS_CONTESTED
    }
    #[doc = "`110011`"]
    #[inline(always)]
    pub fn is_sram1_access(&self) -> bool {
        *self == PERFSEL1_A::SRAM1_ACCESS
    }
    #[doc = "`110100`"]
    #[inline(always)]
    pub fn is_sram0_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM0_STALL_UPSTREAM
    }
    #[doc = "`110101`"]
    #[inline(always)]
    pub fn is_sram0_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::SRAM0_STALL_DOWNSTREAM
    }
    #[doc = "`110110`"]
    #[inline(always)]
    pub fn is_sram0_access_contested(&self) -> bool {
        *self == PERFSEL1_A::SRAM0_ACCESS_CONTESTED
    }
    #[doc = "`110111`"]
    #[inline(always)]
    pub fn is_sram0_access(&self) -> bool {
        *self == PERFSEL1_A::SRAM0_ACCESS
    }
    #[doc = "`111000`"]
    #[inline(always)]
    pub fn is_xip_main1_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::XIP_MAIN1_STALL_UPSTREAM
    }
    #[doc = "`111001`"]
    #[inline(always)]
    pub fn is_xip_main1_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::XIP_MAIN1_STALL_DOWNSTREAM
    }
    #[doc = "`111010`"]
    #[inline(always)]
    pub fn is_xip_main1_access_contested(&self) -> bool {
        *self == PERFSEL1_A::XIP_MAIN1_ACCESS_CONTESTED
    }
    #[doc = "`111011`"]
    #[inline(always)]
    pub fn is_xip_main1_access(&self) -> bool {
        *self == PERFSEL1_A::XIP_MAIN1_ACCESS
    }
    #[doc = "`111100`"]
    #[inline(always)]
    pub fn is_xip_main0_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::XIP_MAIN0_STALL_UPSTREAM
    }
    #[doc = "`111101`"]
    #[inline(always)]
    pub fn is_xip_main0_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::XIP_MAIN0_STALL_DOWNSTREAM
    }
    #[doc = "`111110`"]
    #[inline(always)]
    pub fn is_xip_main0_access_contested(&self) -> bool {
        *self == PERFSEL1_A::XIP_MAIN0_ACCESS_CONTESTED
    }
    #[doc = "`111111`"]
    #[inline(always)]
    pub fn is_xip_main0_access(&self) -> bool {
        *self == PERFSEL1_A::XIP_MAIN0_ACCESS
    }
    #[doc = "`1000000`"]
    #[inline(always)]
    pub fn is_rom_stall_upstream(&self) -> bool {
        *self == PERFSEL1_A::ROM_STALL_UPSTREAM
    }
    #[doc = "`1000001`"]
    #[inline(always)]
    pub fn is_rom_stall_downstream(&self) -> bool {
        *self == PERFSEL1_A::ROM_STALL_DOWNSTREAM
    }
    #[doc = "`1000010`"]
    #[inline(always)]
    pub fn is_rom_access_contested(&self) -> bool {
        *self == PERFSEL1_A::ROM_ACCESS_CONTESTED
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn is_rom_access(&self) -> bool {
        *self == PERFSEL1_A::ROM_ACCESS
    }
}
#[doc = "Field `PERFSEL1` writer - Select an event for PERFCTR1. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters."]
pub type PERFSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PERFSEL1_A>;
impl<'a, REG> PERFSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn siob_proc1_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SIOB_PROC1_STALL_UPSTREAM)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn siob_proc1_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SIOB_PROC1_STALL_DOWNSTREAM)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn siob_proc1_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SIOB_PROC1_ACCESS_CONTESTED)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn siob_proc1_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SIOB_PROC1_ACCESS)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn siob_proc0_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SIOB_PROC0_STALL_UPSTREAM)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn siob_proc0_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SIOB_PROC0_STALL_DOWNSTREAM)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn siob_proc0_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SIOB_PROC0_ACCESS_CONTESTED)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn siob_proc0_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SIOB_PROC0_ACCESS)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn apb_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::APB_STALL_UPSTREAM)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn apb_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::APB_STALL_DOWNSTREAM)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn apb_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::APB_ACCESS_CONTESTED)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn apb_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::APB_ACCESS)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn fastperi_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::FASTPERI_STALL_UPSTREAM)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn fastperi_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::FASTPERI_STALL_DOWNSTREAM)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn fastperi_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::FASTPERI_ACCESS_CONTESTED)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn fastperi_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::FASTPERI_ACCESS)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn sram9_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM9_STALL_UPSTREAM)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn sram9_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM9_STALL_DOWNSTREAM)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn sram9_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM9_ACCESS_CONTESTED)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn sram9_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM9_ACCESS)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn sram8_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM8_STALL_UPSTREAM)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn sram8_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM8_STALL_DOWNSTREAM)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn sram8_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM8_ACCESS_CONTESTED)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn sram8_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM8_ACCESS)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn sram7_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM7_STALL_UPSTREAM)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn sram7_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM7_STALL_DOWNSTREAM)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn sram7_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM7_ACCESS_CONTESTED)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn sram7_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM7_ACCESS)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn sram6_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM6_STALL_UPSTREAM)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn sram6_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM6_STALL_DOWNSTREAM)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn sram6_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM6_ACCESS_CONTESTED)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn sram6_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM6_ACCESS)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn sram5_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM5_STALL_UPSTREAM)
    }
    #[doc = "`100001`"]
    #[inline(always)]
    pub fn sram5_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM5_STALL_DOWNSTREAM)
    }
    #[doc = "`100010`"]
    #[inline(always)]
    pub fn sram5_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM5_ACCESS_CONTESTED)
    }
    #[doc = "`100011`"]
    #[inline(always)]
    pub fn sram5_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM5_ACCESS)
    }
    #[doc = "`100100`"]
    #[inline(always)]
    pub fn sram4_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM4_STALL_UPSTREAM)
    }
    #[doc = "`100101`"]
    #[inline(always)]
    pub fn sram4_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM4_STALL_DOWNSTREAM)
    }
    #[doc = "`100110`"]
    #[inline(always)]
    pub fn sram4_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM4_ACCESS_CONTESTED)
    }
    #[doc = "`100111`"]
    #[inline(always)]
    pub fn sram4_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM4_ACCESS)
    }
    #[doc = "`101000`"]
    #[inline(always)]
    pub fn sram3_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM3_STALL_UPSTREAM)
    }
    #[doc = "`101001`"]
    #[inline(always)]
    pub fn sram3_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM3_STALL_DOWNSTREAM)
    }
    #[doc = "`101010`"]
    #[inline(always)]
    pub fn sram3_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM3_ACCESS_CONTESTED)
    }
    #[doc = "`101011`"]
    #[inline(always)]
    pub fn sram3_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM3_ACCESS)
    }
    #[doc = "`101100`"]
    #[inline(always)]
    pub fn sram2_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM2_STALL_UPSTREAM)
    }
    #[doc = "`101101`"]
    #[inline(always)]
    pub fn sram2_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM2_STALL_DOWNSTREAM)
    }
    #[doc = "`101110`"]
    #[inline(always)]
    pub fn sram2_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM2_ACCESS_CONTESTED)
    }
    #[doc = "`101111`"]
    #[inline(always)]
    pub fn sram2_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM2_ACCESS)
    }
    #[doc = "`110000`"]
    #[inline(always)]
    pub fn sram1_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM1_STALL_UPSTREAM)
    }
    #[doc = "`110001`"]
    #[inline(always)]
    pub fn sram1_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM1_STALL_DOWNSTREAM)
    }
    #[doc = "`110010`"]
    #[inline(always)]
    pub fn sram1_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM1_ACCESS_CONTESTED)
    }
    #[doc = "`110011`"]
    #[inline(always)]
    pub fn sram1_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM1_ACCESS)
    }
    #[doc = "`110100`"]
    #[inline(always)]
    pub fn sram0_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM0_STALL_UPSTREAM)
    }
    #[doc = "`110101`"]
    #[inline(always)]
    pub fn sram0_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM0_STALL_DOWNSTREAM)
    }
    #[doc = "`110110`"]
    #[inline(always)]
    pub fn sram0_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM0_ACCESS_CONTESTED)
    }
    #[doc = "`110111`"]
    #[inline(always)]
    pub fn sram0_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::SRAM0_ACCESS)
    }
    #[doc = "`111000`"]
    #[inline(always)]
    pub fn xip_main1_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::XIP_MAIN1_STALL_UPSTREAM)
    }
    #[doc = "`111001`"]
    #[inline(always)]
    pub fn xip_main1_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::XIP_MAIN1_STALL_DOWNSTREAM)
    }
    #[doc = "`111010`"]
    #[inline(always)]
    pub fn xip_main1_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::XIP_MAIN1_ACCESS_CONTESTED)
    }
    #[doc = "`111011`"]
    #[inline(always)]
    pub fn xip_main1_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::XIP_MAIN1_ACCESS)
    }
    #[doc = "`111100`"]
    #[inline(always)]
    pub fn xip_main0_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::XIP_MAIN0_STALL_UPSTREAM)
    }
    #[doc = "`111101`"]
    #[inline(always)]
    pub fn xip_main0_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::XIP_MAIN0_STALL_DOWNSTREAM)
    }
    #[doc = "`111110`"]
    #[inline(always)]
    pub fn xip_main0_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::XIP_MAIN0_ACCESS_CONTESTED)
    }
    #[doc = "`111111`"]
    #[inline(always)]
    pub fn xip_main0_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::XIP_MAIN0_ACCESS)
    }
    #[doc = "`1000000`"]
    #[inline(always)]
    pub fn rom_stall_upstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::ROM_STALL_UPSTREAM)
    }
    #[doc = "`1000001`"]
    #[inline(always)]
    pub fn rom_stall_downstream(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::ROM_STALL_DOWNSTREAM)
    }
    #[doc = "`1000010`"]
    #[inline(always)]
    pub fn rom_access_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::ROM_ACCESS_CONTESTED)
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn rom_access(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL1_A::ROM_ACCESS)
    }
}
impl R {
    #[doc = "Bits 0:6 - Select an event for PERFCTR1. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters."]
    #[inline(always)]
    pub fn perfsel1(&self) -> PERFSEL1_R {
        PERFSEL1_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Select an event for PERFCTR1. For each downstream port of the main crossbar, four events are available: ACCESS, an access took place; ACCESS_CONTESTED, an access took place that previously stalled due to contention from other masters; STALL_DOWNSTREAM, count cycles where any master stalled due to a stall on the downstream bus; STALL_UPSTREAM, count cycles where any master stalled for any reason, including contention from other masters."]
    #[inline(always)]
    #[must_use]
    pub fn perfsel1(&mut self) -> PERFSEL1_W<PERFSEL1_SPEC> {
        PERFSEL1_W::new(self, 0)
    }
}
#[doc = "Bus fabric performance event select for PERFCTR1  

You can [`read`](crate::Reg::read) this register and get [`perfsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perfsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERFSEL1_SPEC;
impl crate::RegisterSpec for PERFSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perfsel1::R`](R) reader structure"]
impl crate::Readable for PERFSEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perfsel1::W`](W) writer structure"]
impl crate::Writable for PERFSEL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERFSEL1 to value 0x1f"]
impl crate::Resettable for PERFSEL1_SPEC {
    const RESET_VALUE: u32 = 0x1f;
}
