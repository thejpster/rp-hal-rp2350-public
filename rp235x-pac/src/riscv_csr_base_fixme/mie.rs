#[doc = "Register `MIE` reader"]
pub type R = crate::R<MIE_SPEC>;
#[doc = "Register `MIE` writer"]
pub type W = crate::W<MIE_SPEC>;
#[doc = "Field `MSIE` reader - Software interrupt enable. The processor transfers to the software interrupt vector `mie.msie`, `mip.msip` and `mstatus.mie` are all 1, unless an external interrupt request is also valid at this time."]
pub type MSIE_R = crate::BitReader;
#[doc = "Field `MSIE` writer - Software interrupt enable. The processor transfers to the software interrupt vector `mie.msie`, `mip.msip` and `mstatus.mie` are all 1, unless an external interrupt request is also valid at this time."]
pub type MSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTIE` reader - Timer interrupt enable. The processor transfers to the timer interrupt vector when `mie.mtie`, `mip.mtip` and `mstatus.mie` are all 1, unless a software or external interrupt request is also valid at this time."]
pub type MTIE_R = crate::BitReader;
#[doc = "Field `MTIE` writer - Timer interrupt enable. The processor transfers to the timer interrupt vector when `mie.mtie`, `mip.mtip` and `mstatus.mie` are all 1, unless a software or external interrupt request is also valid at this time."]
pub type MTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEIE` reader - External interrupt enable. The processor transfers to the external interrupt vector when `mie.meie`, `mip.meip` and `mstatus.mie` are all 1.  

 Hazard3 has internal registers to individually filter external interrupts (see `meiea`), but this standard control can be used to mask all external interrupts at once."]
pub type MEIE_R = crate::BitReader;
#[doc = "Field `MEIE` writer - External interrupt enable. The processor transfers to the external interrupt vector when `mie.meie`, `mip.meip` and `mstatus.mie` are all 1.  

 Hazard3 has internal registers to individually filter external interrupts (see `meiea`), but this standard control can be used to mask all external interrupts at once."]
pub type MEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Software interrupt enable. The processor transfers to the software interrupt vector `mie.msie`, `mip.msip` and `mstatus.mie` are all 1, unless an external interrupt request is also valid at this time."]
    #[inline(always)]
    pub fn msie(&self) -> MSIE_R {
        MSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer interrupt enable. The processor transfers to the timer interrupt vector when `mie.mtie`, `mip.mtip` and `mstatus.mie` are all 1, unless a software or external interrupt request is also valid at this time."]
    #[inline(always)]
    pub fn mtie(&self) -> MTIE_R {
        MTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - External interrupt enable. The processor transfers to the external interrupt vector when `mie.meie`, `mip.meip` and `mstatus.mie` are all 1.  

 Hazard3 has internal registers to individually filter external interrupts (see `meiea`), but this standard control can be used to mask all external interrupts at once."]
    #[inline(always)]
    pub fn meie(&self) -> MEIE_R {
        MEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Software interrupt enable. The processor transfers to the software interrupt vector `mie.msie`, `mip.msip` and `mstatus.mie` are all 1, unless an external interrupt request is also valid at this time."]
    #[inline(always)]
    #[must_use]
    pub fn msie(&mut self) -> MSIE_W<MIE_SPEC> {
        MSIE_W::new(self, 3)
    }
    #[doc = "Bit 7 - Timer interrupt enable. The processor transfers to the timer interrupt vector when `mie.mtie`, `mip.mtip` and `mstatus.mie` are all 1, unless a software or external interrupt request is also valid at this time."]
    #[inline(always)]
    #[must_use]
    pub fn mtie(&mut self) -> MTIE_W<MIE_SPEC> {
        MTIE_W::new(self, 7)
    }
    #[doc = "Bit 11 - External interrupt enable. The processor transfers to the external interrupt vector when `mie.meie`, `mip.meip` and `mstatus.mie` are all 1.  

 Hazard3 has internal registers to individually filter external interrupts (see `meiea`), but this standard control can be used to mask all external interrupts at once."]
    #[inline(always)]
    #[must_use]
    pub fn meie(&mut self) -> MEIE_W<MIE_SPEC> {
        MEIE_W::new(self, 11)
    }
}
#[doc = "Machine interrupt enable register  

You can [`read`](crate::Reg::read) this register and get [`mie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIE_SPEC;
impl crate::RegisterSpec for MIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mie::R`](R) reader structure"]
impl crate::Readable for MIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mie::W`](W) writer structure"]
impl crate::Writable for MIE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIE to value 0"]
impl crate::Resettable for MIE_SPEC {
    const RESET_VALUE: u32 = 0;
}
