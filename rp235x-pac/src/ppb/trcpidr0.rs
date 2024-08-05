#[doc = "Register `TRCPIDR0` reader"]
pub type R = crate::R<TRCPIDR0_SPEC>;
#[doc = "Register `TRCPIDR0` writer"]
pub type W = crate::W<TRCPIDR0_SPEC>;
#[doc = "Field `PART_0` reader - reads as `ImpDef"]
pub type PART_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - reads as `ImpDef"]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "TRCPIDR0  

You can [`read`](crate::Reg::read) this register and get [`trcpidr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPIDR0_SPEC;
impl crate::RegisterSpec for TRCPIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpidr0::R`](R) reader structure"]
impl crate::Readable for TRCPIDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcpidr0::W`](W) writer structure"]
impl crate::Writable for TRCPIDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCPIDR0 to value 0x21"]
impl crate::Resettable for TRCPIDR0_SPEC {
    const RESET_VALUE: u32 = 0x21;
}
