#[doc = "Register `ROM` reader"]
pub type R = crate::R<ROM_SPEC>;
#[doc = "Register `ROM` writer"]
pub type W = crate::W<ROM_SPEC>;
#[doc = "Field `NSU` reader - If 1, and NSP is also set, ROM can be accessed from a NonSecure, Unprivileged context.  

 This bit is writable from a NonSecure, Privileged context, if and only if the NSP bit is set."]
pub type NSU_R = crate::BitReader;
#[doc = "Field `NSU` writer - If 1, and NSP is also set, ROM can be accessed from a NonSecure, Unprivileged context.  

 This bit is writable from a NonSecure, Privileged context, if and only if the NSP bit is set."]
pub type NSU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSP` reader - If 1, ROM can be accessed from a NonSecure, Privileged context."]
pub type NSP_R = crate::BitReader;
#[doc = "Field `NSP` writer - If 1, ROM can be accessed from a NonSecure, Privileged context."]
pub type NSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SU` reader - If 1, and SP is also set, ROM can be accessed from a Secure, Unprivileged context."]
pub type SU_R = crate::BitReader;
#[doc = "Field `SU` writer - If 1, and SP is also set, ROM can be accessed from a Secure, Unprivileged context."]
pub type SU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP` reader - If 1, ROM can be accessed from a Secure, Privileged context."]
pub type SP_R = crate::BitReader;
#[doc = "Field `SP` writer - If 1, ROM can be accessed from a Secure, Privileged context."]
pub type SP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0` reader - If 1, ROM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type CORE0_R = crate::BitReader;
#[doc = "Field `CORE0` writer - If 1, ROM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type CORE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1` reader - If 1, ROM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type CORE1_R = crate::BitReader;
#[doc = "Field `CORE1` writer - If 1, ROM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type CORE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - If 1, ROM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type DMA_R = crate::BitReader;
#[doc = "Field `DMA` writer - If 1, ROM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG` reader - If 1, ROM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type DBG_R = crate::BitReader;
#[doc = "Field `DBG` writer - If 1, ROM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
pub type DBG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If 1, and NSP is also set, ROM can be accessed from a NonSecure, Unprivileged context.  

 This bit is writable from a NonSecure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn nsu(&self) -> NSU_R {
        NSU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If 1, ROM can be accessed from a NonSecure, Privileged context."]
    #[inline(always)]
    pub fn nsp(&self) -> NSP_R {
        NSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If 1, and SP is also set, ROM can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If 1, ROM can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If 1, ROM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn core0(&self) -> CORE0_R {
        CORE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If 1, ROM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn core1(&self) -> CORE1_R {
        CORE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If 1, ROM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If 1, ROM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If 1, and NSP is also set, ROM can be accessed from a NonSecure, Unprivileged context.  

 This bit is writable from a NonSecure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn nsu(&mut self) -> NSU_W<ROM_SPEC> {
        NSU_W::new(self, 0)
    }
    #[doc = "Bit 1 - If 1, ROM can be accessed from a NonSecure, Privileged context."]
    #[inline(always)]
    #[must_use]
    pub fn nsp(&mut self) -> NSP_W<ROM_SPEC> {
        NSP_W::new(self, 1)
    }
    #[doc = "Bit 2 - If 1, and SP is also set, ROM can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<ROM_SPEC> {
        SU_W::new(self, 2)
    }
    #[doc = "Bit 3 - If 1, ROM can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<ROM_SPEC> {
        SP_W::new(self, 3)
    }
    #[doc = "Bit 4 - If 1, ROM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    #[must_use]
    pub fn core0(&mut self) -> CORE0_W<ROM_SPEC> {
        CORE0_W::new(self, 4)
    }
    #[doc = "Bit 5 - If 1, ROM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    #[must_use]
    pub fn core1(&mut self) -> CORE1_W<ROM_SPEC> {
        CORE1_W::new(self, 5)
    }
    #[doc = "Bit 6 - If 1, ROM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<ROM_SPEC> {
        DMA_W::new(self, 6)
    }
    #[doc = "Bit 7 - If 1, ROM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    #[must_use]
    pub fn dbg(&mut self) -> DBG_W<ROM_SPEC> {
        DBG_W::new(self, 7)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access ROM, and at what security/privilege levels they can do so.  

 Defaults to fully open access.  

 This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes NonSecure-Privileged-writable when the NSP bit is set.  

You can [`read`](crate::Reg::read) this register and get [`rom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_SPEC;
impl crate::RegisterSpec for ROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom::R`](R) reader structure"]
impl crate::Readable for ROM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom::W`](W) writer structure"]
impl crate::Writable for ROM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROM to value 0xff"]
impl crate::Resettable for ROM_SPEC {
    const RESET_VALUE: u32 = 0xff;
}