#[doc = "Register `SAU_RNR` reader"]
pub type R = crate::R<SAU_RNR_SPEC>;
#[doc = "Register `SAU_RNR` writer"]
pub type W = crate::W<SAU_RNR_SPEC>;
#[doc = "Field `REGION` reader - Indicates the SAU region accessed by SAU_RBAR and SAU_RLAR"]
pub type REGION_R = crate::FieldReader;
#[doc = "Field `REGION` writer - Indicates the SAU region accessed by SAU_RBAR and SAU_RLAR"]
pub type REGION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates the SAU region accessed by SAU_RBAR and SAU_RLAR"]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates the SAU region accessed by SAU_RBAR and SAU_RLAR"]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> REGION_W<SAU_RNR_SPEC> {
        REGION_W::new(self, 0)
    }
}
#[doc = "Selects the region currently accessed by SAU_RBAR and SAU_RLAR  

You can [`read`](crate::Reg::read) this register and get [`sau_rnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sau_rnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAU_RNR_SPEC;
impl crate::RegisterSpec for SAU_RNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sau_rnr::R`](R) reader structure"]
impl crate::Readable for SAU_RNR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sau_rnr::W`](W) writer structure"]
impl crate::Writable for SAU_RNR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAU_RNR to value 0"]
impl crate::Resettable for SAU_RNR_SPEC {
    const RESET_VALUE: u32 = 0;
}
