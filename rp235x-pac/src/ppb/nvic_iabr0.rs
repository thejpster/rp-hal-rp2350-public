#[doc = "Register `NVIC_IABR0` reader"]
pub type R = crate::R<NVIC_IABR0_SPEC>;
#[doc = "Register `NVIC_IABR0` writer"]
pub type W = crate::W<NVIC_IABR0_SPEC>;
#[doc = "Field `ACTIVE` reader - For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
pub type ACTIVE_R = crate::FieldReader<u32>;
#[doc = "Field `ACTIVE` writer - For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
pub type ACTIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<NVIC_IABR0_SPEC> {
        ACTIVE_W::new(self, 0)
    }
}
#[doc = "For each group of 32 interrupts, shows the active state of each interrupt  

You can [`read`](crate::Reg::read) this register and get [`nvic_iabr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_iabr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IABR0_SPEC;
impl crate::RegisterSpec for NVIC_IABR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_iabr0::R`](R) reader structure"]
impl crate::Readable for NVIC_IABR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_iabr0::W`](W) writer structure"]
impl crate::Writable for NVIC_IABR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IABR0 to value 0"]
impl crate::Resettable for NVIC_IABR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
