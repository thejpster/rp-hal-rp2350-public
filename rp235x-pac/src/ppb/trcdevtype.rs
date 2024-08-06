#[doc = "Register `TRCDEVTYPE` reader"]
pub type R = crate::R<TRCDEVTYPE_SPEC>;
#[doc = "Register `TRCDEVTYPE` writer"]
pub type W = crate::W<TRCDEVTYPE_SPEC>;
#[doc = "Field `MAJOR` reader - reads as 0b0011"]
pub type MAJOR_R = crate::FieldReader;
#[doc = "Field `SUB` reader - reads as 0b0001"]
pub type SUB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - reads as 0b0011"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - reads as 0b0001"]
    #[inline(always)]
    pub fn sub(&self) -> SUB_R {
        SUB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "TRCDEVTYPE  

You can [`read`](crate::Reg::read) this register and get [`trcdevtype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcdevtype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCDEVTYPE_SPEC;
impl crate::RegisterSpec for TRCDEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcdevtype::R`](R) reader structure"]
impl crate::Readable for TRCDEVTYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcdevtype::W`](W) writer structure"]
impl crate::Writable for TRCDEVTYPE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCDEVTYPE to value 0x13"]
impl crate::Resettable for TRCDEVTYPE_SPEC {
    const RESET_VALUE: u32 = 0x13;
}
