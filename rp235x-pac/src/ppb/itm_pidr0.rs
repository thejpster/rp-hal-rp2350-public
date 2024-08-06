#[doc = "Register `ITM_PIDR0` reader"]
pub type R = crate::R<ITM_PIDR0_SPEC>;
#[doc = "Register `ITM_PIDR0` writer"]
pub type W = crate::W<ITM_PIDR0_SPEC>;
#[doc = "Field `PART_0` reader - See CoreSight Architecture Specification"]
pub type PART_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_pidr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_PIDR0_SPEC;
impl crate::RegisterSpec for ITM_PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_pidr0::R`](R) reader structure"]
impl crate::Readable for ITM_PIDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itm_pidr0::W`](W) writer structure"]
impl crate::Writable for ITM_PIDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITM_PIDR0 to value 0x21"]
impl crate::Resettable for ITM_PIDR0_SPEC {
    const RESET_VALUE: u32 = 0x21;
}
