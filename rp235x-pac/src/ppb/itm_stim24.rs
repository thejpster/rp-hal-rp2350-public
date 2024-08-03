#[doc = "Register `ITM_STIM24` reader"]
pub type R = crate::R<ITM_STIM24_SPEC>;
#[doc = "Register `ITM_STIM24` writer"]
pub type W = crate::W<ITM_STIM24_SPEC>;
#[doc = "Field `STIMULUS` reader - Data to write to the Stimulus Port FIFO, for forwarding as an Instrumentation packet. The size of write access determines the type of Instrumentation packet generated."]
pub type STIMULUS_R = crate::FieldReader<u32>;
#[doc = "Field `STIMULUS` writer - Data to write to the Stimulus Port FIFO, for forwarding as an Instrumentation packet. The size of write access determines the type of Instrumentation packet generated."]
pub type STIMULUS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data to write to the Stimulus Port FIFO, for forwarding as an Instrumentation packet. The size of write access determines the type of Instrumentation packet generated."]
    #[inline(always)]
    pub fn stimulus(&self) -> STIMULUS_R {
        STIMULUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to write to the Stimulus Port FIFO, for forwarding as an Instrumentation packet. The size of write access determines the type of Instrumentation packet generated."]
    #[inline(always)]
    #[must_use]
    pub fn stimulus(&mut self) -> STIMULUS_W<ITM_STIM24_SPEC> {
        STIMULUS_W::new(self, 0)
    }
}
#[doc = "Provides the interface for generating Instrumentation packets  

You can [`read`](crate::Reg::read) this register and get [`itm_stim24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_stim24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_STIM24_SPEC;
impl crate::RegisterSpec for ITM_STIM24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_stim24::R`](R) reader structure"]
impl crate::Readable for ITM_STIM24_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itm_stim24::W`](W) writer structure"]
impl crate::Writable for ITM_STIM24_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITM_STIM24 to value 0"]
impl crate::Resettable for ITM_STIM24_SPEC {
    const RESET_VALUE: u32 = 0;
}
