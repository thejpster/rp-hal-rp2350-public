#[doc = "Register `TRCCNTRLDVR0` reader"]
pub type R = crate::R<TRCCNTRLDVR0_SPEC>;
#[doc = "Register `TRCCNTRLDVR0` writer"]
pub type W = crate::W<TRCCNTRLDVR0_SPEC>;
#[doc = "Field `VALUE` reader - Defines the reload value for the counter. This value is loaded into the counter each time the reload event occurs"]
pub type VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - Defines the reload value for the counter. This value is loaded into the counter each time the reload event occurs"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the reload value for the counter. This value is loaded into the counter each time the reload event occurs"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the reload value for the counter. This value is loaded into the counter each time the reload event occurs"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<TRCCNTRLDVR0_SPEC> {
        VALUE_W::new(self, 0)
    }
}
#[doc = "The TRCCNTRLDVR defines the reload value for the reduced function counter  

You can [`read`](crate::Reg::read) this register and get [`trccntrldvr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccntrldvr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCCNTRLDVR0_SPEC;
impl crate::RegisterSpec for TRCCNTRLDVR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trccntrldvr0::R`](R) reader structure"]
impl crate::Readable for TRCCNTRLDVR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trccntrldvr0::W`](W) writer structure"]
impl crate::Writable for TRCCNTRLDVR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCCNTRLDVR0 to value 0"]
impl crate::Resettable for TRCCNTRLDVR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
