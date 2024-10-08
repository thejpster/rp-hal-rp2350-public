#[doc = "Register `TIMELW` reader"]
pub type R = crate::R<TIMELW_SPEC>;
#[doc = "Register `TIMELW` writer"]
pub type W = crate::W<TIMELW_SPEC>;
#[doc = "Field `TIMELW` writer - "]
pub type TIMELW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn timelw(&mut self) -> TIMELW_W<TIMELW_SPEC> {
        TIMELW_W::new(self, 0)
    }
}
#[doc = "Write to bits 31:0 of time writes do not get copied to time until timehw is written  

You can [`read`](crate::Reg::read) this register and get [`timelw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timelw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMELW_SPEC;
impl crate::RegisterSpec for TIMELW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timelw::R`](R) reader structure"]
impl crate::Readable for TIMELW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timelw::W`](W) writer structure"]
impl crate::Writable for TIMELW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMELW to value 0"]
impl crate::Resettable for TIMELW_SPEC {
    const RESET_VALUE: u32 = 0;
}
