#[doc = "Register `AUTOCORR_STATISTIC` reader"]
pub type R = crate::R<AUTOCORR_STATISTIC_SPEC>;
#[doc = "Register `AUTOCORR_STATISTIC` writer"]
pub type W = crate::W<AUTOCORR_STATISTIC_SPEC>;
#[doc = "Field `AUTOCORR_TRYS` reader - Count each time an autocorrelation test starts. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
pub type AUTOCORR_TRYS_R = crate::FieldReader<u16>;
#[doc = "Field `AUTOCORR_TRYS` writer - Count each time an autocorrelation test starts. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
pub type AUTOCORR_TRYS_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `AUTOCORR_FAILS` reader - Count each time an autocorrelation test fails. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
pub type AUTOCORR_FAILS_R = crate::FieldReader;
#[doc = "Field `AUTOCORR_FAILS` writer - Count each time an autocorrelation test fails. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
pub type AUTOCORR_FAILS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:13 - Count each time an autocorrelation test starts. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
    #[inline(always)]
    pub fn autocorr_trys(&self) -> AUTOCORR_TRYS_R {
        AUTOCORR_TRYS_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:21 - Count each time an autocorrelation test fails. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
    #[inline(always)]
    pub fn autocorr_fails(&self) -> AUTOCORR_FAILS_R {
        AUTOCORR_FAILS_R::new(((self.bits >> 14) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Count each time an autocorrelation test starts. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
    #[inline(always)]
    #[must_use]
    pub fn autocorr_trys(&mut self) -> AUTOCORR_TRYS_W<AUTOCORR_STATISTIC_SPEC> {
        AUTOCORR_TRYS_W::new(self, 0)
    }
    #[doc = "Bits 14:21 - Count each time an autocorrelation test fails. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
    #[inline(always)]
    #[must_use]
    pub fn autocorr_fails(&mut self) -> AUTOCORR_FAILS_W<AUTOCORR_STATISTIC_SPEC> {
        AUTOCORR_FAILS_W::new(self, 14)
    }
}
#[doc = "Statistic about Autocorrelation test activations.  

You can [`read`](crate::Reg::read) this register and get [`autocorr_statistic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocorr_statistic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUTOCORR_STATISTIC_SPEC;
impl crate::RegisterSpec for AUTOCORR_STATISTIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocorr_statistic::R`](R) reader structure"]
impl crate::Readable for AUTOCORR_STATISTIC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`autocorr_statistic::W`](W) writer structure"]
impl crate::Writable for AUTOCORR_STATISTIC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOCORR_STATISTIC to value 0"]
impl crate::Resettable for AUTOCORR_STATISTIC_SPEC {
    const RESET_VALUE: u32 = 0;
}
