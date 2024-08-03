#[doc = "Register `MIP` reader"]
pub type R = crate::R<MIP_SPEC>;
#[doc = "Register `MIP` writer"]
pub type W = crate::W<MIP_SPEC>;
#[doc = "Field `MSIE` reader - Software interrupt pending. The processor transfers to the software interrupt vector `mie.msie`, `mip.msip` and `mstatus.mie` are all 1, unless an external interrupt request is also valid at this time."]
pub type MSIE_R = crate::BitReader;
#[doc = "Field `MSIE` writer - Software interrupt pending. The processor transfers to the software interrupt vector `mie.msie`, `mip.msip` and `mstatus.mie` are all 1, unless an external interrupt request is also valid at this time."]
pub type MSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTIP` reader - Timer interrupt pending. The processor transfers to the timer interrupt vector when `mie.mtie`, `mip.mtip` and `mstatus.mie` are all 1, unless a software or external interrupt request is also valid at this time."]
pub type MTIP_R = crate::BitReader;
#[doc = "Field `MTIP` writer - Timer interrupt pending. The processor transfers to the timer interrupt vector when `mie.mtie`, `mip.mtip` and `mstatus.mie` are all 1, unless a software or external interrupt request is also valid at this time."]
pub type MTIP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEIP` reader - External interrupt pending. The processor transfers to the external interrupt vector when `mie.meie`, `mip.meip` and `mstatus.mie` are all 1.  

 Hazard3 has internal registers to individually filter which external IRQs appear in `meip`. When `meip` is 1, this indicates there is at least one external interrupt which is asserted (hence pending in `mieipa`), enabled in `meiea`, and of priority greater than or equal to the current preemption level in `meicontext.preempt`."]
pub type MEIP_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Software interrupt pending. The processor transfers to the software interrupt vector `mie.msie`, `mip.msip` and `mstatus.mie` are all 1, unless an external interrupt request is also valid at this time."]
    #[inline(always)]
    pub fn msie(&self) -> MSIE_R {
        MSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer interrupt pending. The processor transfers to the timer interrupt vector when `mie.mtie`, `mip.mtip` and `mstatus.mie` are all 1, unless a software or external interrupt request is also valid at this time."]
    #[inline(always)]
    pub fn mtip(&self) -> MTIP_R {
        MTIP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - External interrupt pending. The processor transfers to the external interrupt vector when `mie.meie`, `mip.meip` and `mstatus.mie` are all 1.  

 Hazard3 has internal registers to individually filter which external IRQs appear in `meip`. When `meip` is 1, this indicates there is at least one external interrupt which is asserted (hence pending in `mieipa`), enabled in `meiea`, and of priority greater than or equal to the current preemption level in `meicontext.preempt`."]
    #[inline(always)]
    pub fn meip(&self) -> MEIP_R {
        MEIP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Software interrupt pending. The processor transfers to the software interrupt vector `mie.msie`, `mip.msip` and `mstatus.mie` are all 1, unless an external interrupt request is also valid at this time."]
    #[inline(always)]
    #[must_use]
    pub fn msie(&mut self) -> MSIE_W<MIP_SPEC> {
        MSIE_W::new(self, 3)
    }
    #[doc = "Bit 7 - Timer interrupt pending. The processor transfers to the timer interrupt vector when `mie.mtie`, `mip.mtip` and `mstatus.mie` are all 1, unless a software or external interrupt request is also valid at this time."]
    #[inline(always)]
    #[must_use]
    pub fn mtip(&mut self) -> MTIP_W<MIP_SPEC> {
        MTIP_W::new(self, 7)
    }
}
#[doc = "Machine interrupt pending  

You can [`read`](crate::Reg::read) this register and get [`mip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIP_SPEC;
impl crate::RegisterSpec for MIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mip::R`](R) reader structure"]
impl crate::Readable for MIP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mip::W`](W) writer structure"]
impl crate::Writable for MIP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIP to value 0"]
impl crate::Resettable for MIP_SPEC {
    const RESET_VALUE: u32 = 0;
}
