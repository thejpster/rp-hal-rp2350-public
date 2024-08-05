#[doc = "Register `TRCIDR1` reader"]
pub type R = crate::R<TRCIDR1_SPEC>;
#[doc = "Register `TRCIDR1` writer"]
pub type W = crate::W<TRCIDR1_SPEC>;
#[doc = "Field `REVISION` reader - reads as `ImpDef"]
pub type REVISION_R = crate::FieldReader;
#[doc = "Field `TRCARCHMIN` reader - reads as 0b0000"]
pub type TRCARCHMIN_R = crate::FieldReader;
#[doc = "Field `TRCARCHMAJ` reader - reads as 0b0100"]
pub type TRCARCHMAJ_R = crate::FieldReader;
#[doc = "Field `RES1` reader - Reserved, RES1"]
pub type RES1_R = crate::FieldReader;
#[doc = "Field `DESIGNER` reader - reads as `ImpDef"]
pub type DESIGNER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - reads as `ImpDef"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - reads as 0b0000"]
    #[inline(always)]
    pub fn trcarchmin(&self) -> TRCARCHMIN_R {
        TRCARCHMIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - reads as 0b0100"]
    #[inline(always)]
    pub fn trcarchmaj(&self) -> TRCARCHMAJ_R {
        TRCARCHMAJ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Reserved, RES1"]
    #[inline(always)]
    pub fn res1(&self) -> RES1_R {
        RES1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn designer(&self) -> DESIGNER_R {
        DESIGNER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "TRCIDR1  

You can [`read`](crate::Reg::read) this register and get [`trcidr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR1_SPEC;
impl crate::RegisterSpec for TRCIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr1::R`](R) reader structure"]
impl crate::Readable for TRCIDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcidr1::W`](W) writer structure"]
impl crate::Writable for TRCIDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIDR1 to value 0x4100_f421"]
impl crate::Resettable for TRCIDR1_SPEC {
    const RESET_VALUE: u32 = 0x4100_f421;
}
