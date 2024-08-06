#[doc = "Register `TMDS_WDATA` reader"]
pub type R = crate::R<TMDS_WDATA_SPEC>;
#[doc = "Register `TMDS_WDATA` writer"]
pub type W = crate::W<TMDS_WDATA_SPEC>;
#[doc = "Field `TMDS_WDATA` writer - "]
pub type TMDS_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn tmds_wdata(&mut self) -> TMDS_WDATA_W<TMDS_WDATA_SPEC> {
        TMDS_WDATA_W::new(self, 0)
    }
}
#[doc = "Write-only access to the TMDS colour data register.  

You can [`read`](crate::Reg::read) this register and get [`tmds_wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDS_WDATA_SPEC;
impl crate::RegisterSpec for TMDS_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmds_wdata::R`](R) reader structure"]
impl crate::Readable for TMDS_WDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmds_wdata::W`](W) writer structure"]
impl crate::Writable for TMDS_WDATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMDS_WDATA to value 0"]
impl crate::Resettable for TMDS_WDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
