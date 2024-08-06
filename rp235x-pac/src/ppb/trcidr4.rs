#[doc = "Register `TRCIDR4` reader"]
pub type R = crate::R<TRCIDR4_SPEC>;
#[doc = "Register `TRCIDR4` writer"]
pub type W = crate::W<TRCIDR4_SPEC>;
#[doc = "Field `NUMACPAIRS` reader - reads as `ImpDef"]
pub type NUMACPAIRS_R = crate::FieldReader;
#[doc = "Field `NUMDVC` reader - reads as `ImpDef"]
pub type NUMDVC_R = crate::FieldReader;
#[doc = "Field `SUPPDAC` reader - reads as `ImpDef"]
pub type SUPPDAC_R = crate::BitReader;
#[doc = "Field `NUMPC` reader - reads as `ImpDef"]
pub type NUMPC_R = crate::FieldReader;
#[doc = "Field `NUMRSPAIR` reader - reads as `ImpDef"]
pub type NUMRSPAIR_R = crate::FieldReader;
#[doc = "Field `NUMSSCC` reader - reads as `ImpDef"]
pub type NUMSSCC_R = crate::FieldReader;
#[doc = "Field `NUMCIDC` reader - reads as `ImpDef"]
pub type NUMCIDC_R = crate::FieldReader;
#[doc = "Field `NUMVMIDC` reader - reads as `ImpDef"]
pub type NUMVMIDC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numacpairs(&self) -> NUMACPAIRS_R {
        NUMACPAIRS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numdvc(&self) -> NUMDVC_R {
        NUMDVC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - reads as `ImpDef"]
    #[inline(always)]
    pub fn suppdac(&self) -> SUPPDAC_R {
        SUPPDAC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numpc(&self) -> NUMPC_R {
        NUMPC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numrspair(&self) -> NUMRSPAIR_R {
        NUMRSPAIR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numsscc(&self) -> NUMSSCC_R {
        NUMSSCC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numcidc(&self) -> NUMCIDC_R {
        NUMCIDC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numvmidc(&self) -> NUMVMIDC_R {
        NUMVMIDC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "TRCIDR4  

You can [`read`](crate::Reg::read) this register and get [`trcidr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR4_SPEC;
impl crate::RegisterSpec for TRCIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr4::R`](R) reader structure"]
impl crate::Readable for TRCIDR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcidr4::W`](W) writer structure"]
impl crate::Writable for TRCIDR4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIDR4 to value 0x0011_4000"]
impl crate::Resettable for TRCIDR4_SPEC {
    const RESET_VALUE: u32 = 0x0011_4000;
}
