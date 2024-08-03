#[doc = "Register `ITM_TER0` reader"]
pub type R = crate::R<ITM_TER0_SPEC>;
#[doc = "Register `ITM_TER0` writer"]
pub type W = crate::W<ITM_TER0_SPEC>;
#[doc = "Field `STIMENA` reader - For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
pub type STIMENA_R = crate::FieldReader<u32>;
#[doc = "Field `STIMENA` writer - For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
pub type STIMENA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
    #[inline(always)]
    pub fn stimena(&self) -> STIMENA_R {
        STIMENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn stimena(&mut self) -> STIMENA_W<ITM_TER0_SPEC> {
        STIMENA_W::new(self, 0)
    }
}
#[doc = "Provide an individual enable bit for each ITM_STIM register  

You can [`read`](crate::Reg::read) this register and get [`itm_ter0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_ter0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_TER0_SPEC;
impl crate::RegisterSpec for ITM_TER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_ter0::R`](R) reader structure"]
impl crate::Readable for ITM_TER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itm_ter0::W`](W) writer structure"]
impl crate::Writable for ITM_TER0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITM_TER0 to value 0"]
impl crate::Resettable for ITM_TER0_SPEC {
    const RESET_VALUE: u32 = 0;
}
