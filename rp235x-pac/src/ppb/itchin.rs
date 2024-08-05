#[doc = "Register `ITCHIN` reader"]
pub type R = crate::R<ITCHIN_SPEC>;
#[doc = "Register `ITCHIN` writer"]
pub type W = crate::W<ITCHIN_SPEC>;
#[doc = "Field `CTCHIN` reader - Reads the value of the ctichin inputs."]
pub type CTCHIN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Reads the value of the ctichin inputs."]
    #[inline(always)]
    pub fn ctchin(&self) -> CTCHIN_R {
        CTCHIN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Integration Test Channel Input register  

You can [`read`](crate::Reg::read) this register and get [`itchin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itchin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITCHIN_SPEC;
impl crate::RegisterSpec for ITCHIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itchin::R`](R) reader structure"]
impl crate::Readable for ITCHIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itchin::W`](W) writer structure"]
impl crate::Writable for ITCHIN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITCHIN to value 0"]
impl crate::Resettable for ITCHIN_SPEC {
    const RESET_VALUE: u32 = 0;
}
