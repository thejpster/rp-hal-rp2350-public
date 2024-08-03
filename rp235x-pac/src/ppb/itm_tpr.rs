#[doc = "Register `ITM_TPR` reader"]
pub type R = crate::R<ITM_TPR_SPEC>;
#[doc = "Register `ITM_TPR` writer"]
pub type W = crate::W<ITM_TPR_SPEC>;
#[doc = "Field `PRIVMASK` reader - Bit mask to enable tracing on ITM stimulus ports"]
pub type PRIVMASK_R = crate::FieldReader;
#[doc = "Field `PRIVMASK` writer - Bit mask to enable tracing on ITM stimulus ports"]
pub type PRIVMASK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Bit mask to enable tracing on ITM stimulus ports"]
    #[inline(always)]
    pub fn privmask(&self) -> PRIVMASK_R {
        PRIVMASK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bit mask to enable tracing on ITM stimulus ports"]
    #[inline(always)]
    #[must_use]
    pub fn privmask(&mut self) -> PRIVMASK_W<ITM_TPR_SPEC> {
        PRIVMASK_W::new(self, 0)
    }
}
#[doc = "Controls which stimulus ports can be accessed by unprivileged code  

You can [`read`](crate::Reg::read) this register and get [`itm_tpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_tpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_TPR_SPEC;
impl crate::RegisterSpec for ITM_TPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_tpr::R`](R) reader structure"]
impl crate::Readable for ITM_TPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itm_tpr::W`](W) writer structure"]
impl crate::Writable for ITM_TPR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITM_TPR to value 0"]
impl crate::Resettable for ITM_TPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
