#[doc = "Register `TRCSSPCICR` reader"]
pub type R = crate::R<TRCSSPCICR_SPEC>;
#[doc = "Register `TRCSSPCICR` writer"]
pub type W = crate::W<TRCSSPCICR_SPEC>;
#[doc = "Field `PC` reader - Selects one or more PE comparator inputs for Single-shot control. TRCIDR4.NUMPC defines the size of the PC field. 1 bit is provided for each implemented PE comparator input. For example, if bit\\[1\\]
== 1 this selects PE comparator input 1 for Single-shot control"]
pub type PC_R = crate::FieldReader;
#[doc = "Field `PC` writer - Selects one or more PE comparator inputs for Single-shot control. TRCIDR4.NUMPC defines the size of the PC field. 1 bit is provided for each implemented PE comparator input. For example, if bit\\[1\\]
== 1 this selects PE comparator input 1 for Single-shot control"]
pub type PC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Selects one or more PE comparator inputs for Single-shot control. TRCIDR4.NUMPC defines the size of the PC field. 1 bit is provided for each implemented PE comparator input. For example, if bit\\[1\\]
== 1 this selects PE comparator input 1 for Single-shot control"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects one or more PE comparator inputs for Single-shot control. TRCIDR4.NUMPC defines the size of the PC field. 1 bit is provided for each implemented PE comparator input. For example, if bit\\[1\\]
== 1 this selects PE comparator input 1 for Single-shot control"]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PC_W<TRCSSPCICR_SPEC> {
        PC_W::new(self, 0)
    }
}
#[doc = "Selects the PE comparator inputs for Single-shot control  

You can [`read`](crate::Reg::read) this register and get [`trcsspcicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcsspcicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCSSPCICR_SPEC;
impl crate::RegisterSpec for TRCSSPCICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcsspcicr::R`](R) reader structure"]
impl crate::Readable for TRCSSPCICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcsspcicr::W`](W) writer structure"]
impl crate::Writable for TRCSSPCICR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCSSPCICR to value 0"]
impl crate::Resettable for TRCSSPCICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
