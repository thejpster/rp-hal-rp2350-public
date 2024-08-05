#[doc = "Register `TRCDEVID` reader"]
pub type R = crate::R<TRCDEVID_SPEC>;
#[doc = "Register `TRCDEVID` writer"]
pub type W = crate::W<TRCDEVID_SPEC>;
#[doc = "Field `TRCDEVID` reader - "]
pub type TRCDEVID_R = crate::FieldReader<u32>;
#[doc = "Field `TRCDEVID` writer - "]
pub type TRCDEVID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn trcdevid(&self) -> TRCDEVID_R {
        TRCDEVID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn trcdevid(&mut self) -> TRCDEVID_W<TRCDEVID_SPEC> {
        TRCDEVID_W::new(self, 0)
    }
}
#[doc = "TRCDEVID  

You can [`read`](crate::Reg::read) this register and get [`trcdevid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcdevid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCDEVID_SPEC;
impl crate::RegisterSpec for TRCDEVID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcdevid::R`](R) reader structure"]
impl crate::Readable for TRCDEVID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcdevid::W`](W) writer structure"]
impl crate::Writable for TRCDEVID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCDEVID to value 0"]
impl crate::Resettable for TRCDEVID_SPEC {
    const RESET_VALUE: u32 = 0;
}
