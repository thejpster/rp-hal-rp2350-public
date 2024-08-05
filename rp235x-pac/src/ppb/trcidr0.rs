#[doc = "Register `TRCIDR0` reader"]
pub type R = crate::R<TRCIDR0_SPEC>;
#[doc = "Register `TRCIDR0` writer"]
pub type W = crate::W<TRCIDR0_SPEC>;
#[doc = "Field `RES1` reader - Reserved, RES1"]
pub type RES1_R = crate::BitReader;
#[doc = "Field `INSTP0` reader - reads as `ImpDef"]
pub type INSTP0_R = crate::FieldReader;
#[doc = "Field `TRCDATA` reader - reads as `ImpDef"]
pub type TRCDATA_R = crate::FieldReader;
#[doc = "Field `TRCBB` reader - reads as `ImpDef"]
pub type TRCBB_R = crate::BitReader;
#[doc = "Field `TRCCOND` reader - reads as `ImpDef"]
pub type TRCCOND_R = crate::BitReader;
#[doc = "Field `TRCCCI` reader - reads as `ImpDef"]
pub type TRCCCI_R = crate::BitReader;
#[doc = "Field `RETSTACK` reader - reads as `ImpDef"]
pub type RETSTACK_R = crate::BitReader;
#[doc = "Field `NUMEVENT` reader - reads as `ImpDef"]
pub type NUMEVENT_R = crate::FieldReader;
#[doc = "Field `CONDTYPE` reader - reads as `ImpDef"]
pub type CONDTYPE_R = crate::FieldReader;
#[doc = "Field `QFILT` reader - reads as `ImpDef"]
pub type QFILT_R = crate::BitReader;
#[doc = "Field `QSUPP` reader - reads as `ImpDef"]
pub type QSUPP_R = crate::FieldReader;
#[doc = "Field `TRCEXDATA` reader - reads as `ImpDef"]
pub type TRCEXDATA_R = crate::BitReader;
#[doc = "Field `TSSIZE` reader - reads as `ImpDef"]
pub type TSSIZE_R = crate::FieldReader;
#[doc = "Field `COMMOPT` reader - reads as `ImpDef"]
pub type COMMOPT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reserved, RES1"]
    #[inline(always)]
    pub fn res1(&self) -> RES1_R {
        RES1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - reads as `ImpDef"]
    #[inline(always)]
    pub fn instp0(&self) -> INSTP0_R {
        INSTP0_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - reads as `ImpDef"]
    #[inline(always)]
    pub fn trcdata(&self) -> TRCDATA_R {
        TRCDATA_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - reads as `ImpDef"]
    #[inline(always)]
    pub fn trcbb(&self) -> TRCBB_R {
        TRCBB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reads as `ImpDef"]
    #[inline(always)]
    pub fn trccond(&self) -> TRCCOND_R {
        TRCCOND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reads as `ImpDef"]
    #[inline(always)]
    pub fn trccci(&self) -> TRCCCI_R {
        TRCCCI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - reads as `ImpDef"]
    #[inline(always)]
    pub fn retstack(&self) -> RETSTACK_R {
        RETSTACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numevent(&self) -> NUMEVENT_R {
        NUMEVENT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - reads as `ImpDef"]
    #[inline(always)]
    pub fn condtype(&self) -> CONDTYPE_R {
        CONDTYPE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - reads as `ImpDef"]
    #[inline(always)]
    pub fn qfilt(&self) -> QFILT_R {
        QFILT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - reads as `ImpDef"]
    #[inline(always)]
    pub fn qsupp(&self) -> QSUPP_R {
        QSUPP_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - reads as `ImpDef"]
    #[inline(always)]
    pub fn trcexdata(&self) -> TRCEXDATA_R {
        TRCEXDATA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:28 - reads as `ImpDef"]
    #[inline(always)]
    pub fn tssize(&self) -> TSSIZE_R {
        TSSIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - reads as `ImpDef"]
    #[inline(always)]
    pub fn commopt(&self) -> COMMOPT_R {
        COMMOPT_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {}
#[doc = "TRCIDR0  

You can [`read`](crate::Reg::read) this register and get [`trcidr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR0_SPEC;
impl crate::RegisterSpec for TRCIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr0::R`](R) reader structure"]
impl crate::Readable for TRCIDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcidr0::W`](W) writer structure"]
impl crate::Writable for TRCIDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIDR0 to value 0x2800_06e1"]
impl crate::Resettable for TRCIDR0_SPEC {
    const RESET_VALUE: u32 = 0x2800_06e1;
}
