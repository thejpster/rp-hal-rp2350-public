#[doc = "Register `FORCE_CORE_NS` reader"]
pub type R = crate::R<FORCE_CORE_NS_SPEC>;
#[doc = "Register `FORCE_CORE_NS` writer"]
pub type W = crate::W<FORCE_CORE_NS_SPEC>;
#[doc = "Field `CORE1` reader - "]
pub type CORE1_R = crate::BitReader;
#[doc = "Field `CORE1` writer - "]
pub type CORE1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn core1(&self) -> CORE1_R {
        CORE1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn core1(&mut self) -> CORE1_W<FORCE_CORE_NS_SPEC> {
        CORE1_W::new(self, 1)
    }
}
#[doc = "Force core 1's bus accesses to always be Non-secure, no matter the core's internal state. Useful for schemes where one core is designated as the Non-secure core, since some peripherals may filter individual registers internally based on security state but not on master ID.  

You can [`read`](crate::Reg::read) this register and get [`force_core_ns::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_core_ns::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FORCE_CORE_NS_SPEC;
impl crate::RegisterSpec for FORCE_CORE_NS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`force_core_ns::R`](R) reader structure"]
impl crate::Readable for FORCE_CORE_NS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`force_core_ns::W`](W) writer structure"]
impl crate::Writable for FORCE_CORE_NS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FORCE_CORE_NS to value 0"]
impl crate::Resettable for FORCE_CORE_NS_SPEC {
    const RESET_VALUE: u32 = 0;
}
