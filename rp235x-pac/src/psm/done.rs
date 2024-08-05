#[doc = "Register `DONE` reader"]
pub type R = crate::R<DONE_SPEC>;
#[doc = "Register `DONE` writer"]
pub type W = crate::W<DONE_SPEC>;
#[doc = "Field `PROC_COLD` reader - "]
pub type PROC_COLD_R = crate::BitReader;
#[doc = "Field `OTP` reader - "]
pub type OTP_R = crate::BitReader;
#[doc = "Field `ROSC` reader - "]
pub type ROSC_R = crate::BitReader;
#[doc = "Field `XOSC` reader - "]
pub type XOSC_R = crate::BitReader;
#[doc = "Field `RESETS` reader - "]
pub type RESETS_R = crate::BitReader;
#[doc = "Field `CLOCKS` reader - "]
pub type CLOCKS_R = crate::BitReader;
#[doc = "Field `PSM_READY` reader - "]
pub type PSM_READY_R = crate::BitReader;
#[doc = "Field `BUSFABRIC` reader - "]
pub type BUSFABRIC_R = crate::BitReader;
#[doc = "Field `ROM` reader - "]
pub type ROM_R = crate::BitReader;
#[doc = "Field `BOOTRAM` reader - "]
pub type BOOTRAM_R = crate::BitReader;
#[doc = "Field `SRAM0` reader - "]
pub type SRAM0_R = crate::BitReader;
#[doc = "Field `SRAM1` reader - "]
pub type SRAM1_R = crate::BitReader;
#[doc = "Field `SRAM2` reader - "]
pub type SRAM2_R = crate::BitReader;
#[doc = "Field `SRAM3` reader - "]
pub type SRAM3_R = crate::BitReader;
#[doc = "Field `SRAM4` reader - "]
pub type SRAM4_R = crate::BitReader;
#[doc = "Field `SRAM5` reader - "]
pub type SRAM5_R = crate::BitReader;
#[doc = "Field `SRAM6` reader - "]
pub type SRAM6_R = crate::BitReader;
#[doc = "Field `SRAM7` reader - "]
pub type SRAM7_R = crate::BitReader;
#[doc = "Field `SRAM8` reader - "]
pub type SRAM8_R = crate::BitReader;
#[doc = "Field `SRAM9` reader - "]
pub type SRAM9_R = crate::BitReader;
#[doc = "Field `XIP` reader - "]
pub type XIP_R = crate::BitReader;
#[doc = "Field `SIO` reader - "]
pub type SIO_R = crate::BitReader;
#[doc = "Field `ACCESSCTRL` reader - "]
pub type ACCESSCTRL_R = crate::BitReader;
#[doc = "Field `PROC0` reader - "]
pub type PROC0_R = crate::BitReader;
#[doc = "Field `PROC1` reader - "]
pub type PROC1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn proc_cold(&self) -> PROC_COLD_R {
        PROC_COLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rosc(&self) -> ROSC_R {
        ROSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn xosc(&self) -> XOSC_R {
        XOSC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn resets(&self) -> RESETS_R {
        RESETS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clocks(&self) -> CLOCKS_R {
        CLOCKS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn psm_ready(&self) -> PSM_READY_R {
        PSM_READY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn busfabric(&self) -> BUSFABRIC_R {
        BUSFABRIC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bootram(&self) -> BOOTRAM_R {
        BOOTRAM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sram0(&self) -> SRAM0_R {
        SRAM0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sram4(&self) -> SRAM4_R {
        SRAM4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sram5(&self) -> SRAM5_R {
        SRAM5_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sram6(&self) -> SRAM6_R {
        SRAM6_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sram7(&self) -> SRAM7_R {
        SRAM7_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sram8(&self) -> SRAM8_R {
        SRAM8_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sram9(&self) -> SRAM9_R {
        SRAM9_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn xip(&self) -> XIP_R {
        XIP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sio(&self) -> SIO_R {
        SIO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn accessctrl(&self) -> ACCESSCTRL_R {
        ACCESSCTRL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn proc0(&self) -> PROC0_R {
        PROC0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn proc1(&self) -> PROC1_R {
        PROC1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {}
#[doc = "Is the subsystem ready?  

You can [`read`](crate::Reg::read) this register and get [`done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DONE_SPEC;
impl crate::RegisterSpec for DONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`done::R`](R) reader structure"]
impl crate::Readable for DONE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`done::W`](W) writer structure"]
impl crate::Writable for DONE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DONE to value 0"]
impl crate::Resettable for DONE_SPEC {
    const RESET_VALUE: u32 = 0;
}
