#[doc = "Register `DPIDR5` reader"]
pub type R = crate::R<DPIDR5_SPEC>;
#[doc = "Register `DPIDR5` writer"]
pub type W = crate::W<DPIDR5_SPEC>;
#[doc = "Field `DPIDR5` reader - "]
pub type DPIDR5_R = crate::FieldReader<u32>;
#[doc = "Field `DPIDR5` writer - "]
pub type DPIDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dpidr5(&self) -> DPIDR5_R {
        DPIDR5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dpidr5(&mut self) -> DPIDR5_W<DPIDR5_SPEC> {
        DPIDR5_W::new(self, 0)
    }
}
#[doc = "Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPIDR5_SPEC;
impl crate::RegisterSpec for DPIDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpidr5::R`](R) reader structure"]
impl crate::Readable for DPIDR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpidr5::W`](W) writer structure"]
impl crate::Writable for DPIDR5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPIDR5 to value 0"]
impl crate::Resettable for DPIDR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
