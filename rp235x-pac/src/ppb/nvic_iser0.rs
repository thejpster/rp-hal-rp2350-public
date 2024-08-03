#[doc = "Register `NVIC_ISER0` reader"]
pub type R = crate::R<NVIC_ISER0_SPEC>;
#[doc = "Register `NVIC_ISER0` writer"]
pub type W = crate::W<NVIC_ISER0_SPEC>;
#[doc = "Field `SETENA` reader - For SETENA\\[m\\]
in NVIC_ISER*n, indicates whether interrupt 32*n + m is enabled"]
pub type SETENA_R = crate::FieldReader<u32>;
#[doc = "Field `SETENA` writer - For SETENA\\[m\\]
in NVIC_ISER*n, indicates whether interrupt 32*n + m is enabled"]
pub type SETENA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For SETENA\\[m\\]
in NVIC_ISER*n, indicates whether interrupt 32*n + m is enabled"]
    #[inline(always)]
    pub fn setena(&self) -> SETENA_R {
        SETENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - For SETENA\\[m\\]
in NVIC_ISER*n, indicates whether interrupt 32*n + m is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn setena(&mut self) -> SETENA_W<NVIC_ISER0_SPEC> {
        SETENA_W::new(self, 0)
    }
}
#[doc = "Enables or reads the enabled state of each group of 32 interrupts  

You can [`read`](crate::Reg::read) this register and get [`nvic_iser0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_iser0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ISER0_SPEC;
impl crate::RegisterSpec for NVIC_ISER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_iser0::R`](R) reader structure"]
impl crate::Readable for NVIC_ISER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_iser0::W`](W) writer structure"]
impl crate::Writable for NVIC_ISER0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ISER0 to value 0"]
impl crate::Resettable for NVIC_ISER0_SPEC {
    const RESET_VALUE: u32 = 0;
}
