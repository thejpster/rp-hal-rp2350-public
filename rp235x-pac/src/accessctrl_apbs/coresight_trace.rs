#[doc = "Register `CORESIGHT_TRACE` reader"]
pub type R = crate::R<CORESIGHT_TRACE_SPEC>;
#[doc = "Register `CORESIGHT_TRACE` writer"]
pub type W = crate::W<CORESIGHT_TRACE_SPEC>;
#[doc = "Field `NSU` reader - If 1, and NSP is also set, CORESIGHT_TRACE can be accessed from a NonSecure, Unprivileged context.  

 This bit is writable from a NonSecure, Privileged context, if and only if the NSP bit is set."]
pub type NSU_R = crate::BitReader;
#[doc = "Field `NSU` writer - If 1, and NSP is also set, CORESIGHT_TRACE can be accessed from a NonSecure, Unprivileged context.  

 This bit is writable from a NonSecure, Privileged context, if and only if the NSP bit is set."]
pub type NSU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSP` reader - If 1, CORESIGHT_TRACE can be accessed from a NonSecure, Privileged context."]
pub type NSP_R = crate::BitReader;
#[doc = "Field `NSP` writer - If 1, CORESIGHT_TRACE can be accessed from a NonSecure, Privileged context."]
pub type NSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SU` reader - If 1, and SP is also set, CORESIGHT_TRACE can be accessed from a Secure, Unprivileged context."]
pub type SU_R = crate::BitReader;
#[doc = "Field `SU` writer - If 1, and SP is also set, CORESIGHT_TRACE can be accessed from a Secure, Unprivileged context."]
pub type SU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP` reader - If 1, CORESIGHT_TRACE can be accessed from a Secure, Privileged context."]
pub type SP_R = crate::BitReader;
#[doc = "Field `SP` writer - If 1, CORESIGHT_TRACE can be accessed from a Secure, Privileged context."]
pub type SP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0` reader - If 1, CORESIGHT_TRACE can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type CORE0_R = crate::BitReader;
#[doc = "Field `CORE0` writer - If 1, CORESIGHT_TRACE can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type CORE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1` reader - If 1, CORESIGHT_TRACE can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type CORE1_R = crate::BitReader;
#[doc = "Field `CORE1` writer - If 1, CORESIGHT_TRACE can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type CORE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - If 1, CORESIGHT_TRACE can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type DMA_R = crate::BitReader;
#[doc = "Field `DMA` writer - If 1, CORESIGHT_TRACE can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG` reader - If 1, CORESIGHT_TRACE can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type DBG_R = crate::BitReader;
#[doc = "Field `DBG` writer - If 1, CORESIGHT_TRACE can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type DBG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If 1, and NSP is also set, CORESIGHT_TRACE can be accessed from a NonSecure, Unprivileged context.  

 This bit is writable from a NonSecure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn nsu(&self) -> NSU_R {
        NSU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If 1, CORESIGHT_TRACE can be accessed from a NonSecure, Privileged context."]
    #[inline(always)]
    pub fn nsp(&self) -> NSP_R {
        NSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If 1, and SP is also set, CORESIGHT_TRACE can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If 1, CORESIGHT_TRACE can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If 1, CORESIGHT_TRACE can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn core0(&self) -> CORE0_R {
        CORE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If 1, CORESIGHT_TRACE can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn core1(&self) -> CORE1_R {
        CORE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If 1, CORESIGHT_TRACE can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If 1, CORESIGHT_TRACE can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If 1, and NSP is also set, CORESIGHT_TRACE can be accessed from a NonSecure, Unprivileged context.  

 This bit is writable from a NonSecure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn nsu(&mut self) -> NSU_W<CORESIGHT_TRACE_SPEC> {
        NSU_W::new(self, 0)
    }
    #[doc = "Bit 1 - If 1, CORESIGHT_TRACE can be accessed from a NonSecure, Privileged context."]
    #[inline(always)]
    #[must_use]
    pub fn nsp(&mut self) -> NSP_W<CORESIGHT_TRACE_SPEC> {
        NSP_W::new(self, 1)
    }
    #[doc = "Bit 2 - If 1, and SP is also set, CORESIGHT_TRACE can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<CORESIGHT_TRACE_SPEC> {
        SU_W::new(self, 2)
    }
    #[doc = "Bit 3 - If 1, CORESIGHT_TRACE can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<CORESIGHT_TRACE_SPEC> {
        SP_W::new(self, 3)
    }
    #[doc = "Bit 4 - If 1, CORESIGHT_TRACE can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    #[must_use]
    pub fn core0(&mut self) -> CORE0_W<CORESIGHT_TRACE_SPEC> {
        CORE0_W::new(self, 4)
    }
    #[doc = "Bit 5 - If 1, CORESIGHT_TRACE can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    #[must_use]
    pub fn core1(&mut self) -> CORE1_W<CORESIGHT_TRACE_SPEC> {
        CORE1_W::new(self, 5)
    }
    #[doc = "Bit 6 - If 1, CORESIGHT_TRACE can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<CORESIGHT_TRACE_SPEC> {
        DMA_W::new(self, 6)
    }
    #[doc = "Bit 7 - If 1, CORESIGHT_TRACE can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    #[must_use]
    pub fn dbg(&mut self) -> DBG_W<CORESIGHT_TRACE_SPEC> {
        DBG_W::new(self, 7)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_TRACE, and at what security/privilege levels they can do so.  

 Defaults to Secure, Privileged processor or debug access only.  

 This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes NonSecure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`coresight_trace::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coresight_trace::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORESIGHT_TRACE_SPEC;
impl crate::RegisterSpec for CORESIGHT_TRACE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coresight_trace::R`](R) reader structure"]
impl crate::Readable for CORESIGHT_TRACE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`coresight_trace::W`](W) writer structure"]
impl crate::Writable for CORESIGHT_TRACE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORESIGHT_TRACE to value 0xb8"]
impl crate::Resettable for CORESIGHT_TRACE_SPEC {
    const RESET_VALUE: u32 = 0xb8;
}
