#[doc = "Register `TRNG_DEBUG_CONTROL` reader"]
pub type R = crate::R<TRNG_DEBUG_CONTROL_SPEC>;
#[doc = "Register `TRNG_DEBUG_CONTROL` writer"]
pub type W = crate::W<TRNG_DEBUG_CONTROL_SPEC>;
#[doc = "Field `VNC_BYPASS` reader - When set, the Von-Neuman balancer is bypassed (including the 32 consecutive bits test)."]
pub type VNC_BYPASS_R = crate::BitReader;
#[doc = "Field `VNC_BYPASS` writer - When set, the Von-Neuman balancer is bypassed (including the 32 consecutive bits test)."]
pub type VNC_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_CRNGT_BYPASS` reader - When set, the CRNGT test in the RNG is bypassed."]
pub type TRNG_CRNGT_BYPASS_R = crate::BitReader;
#[doc = "Field `TRNG_CRNGT_BYPASS` writer - When set, the CRNGT test in the RNG is bypassed."]
pub type TRNG_CRNGT_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CORRELATE_BYPASS` reader - When set, the autocorrelation test in the TRNG module is bypassed."]
pub type AUTO_CORRELATE_BYPASS_R = crate::BitReader;
#[doc = "Field `AUTO_CORRELATE_BYPASS` writer - When set, the autocorrelation test in the TRNG module is bypassed."]
pub type AUTO_CORRELATE_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - When set, the Von-Neuman balancer is bypassed (including the 32 consecutive bits test)."]
    #[inline(always)]
    pub fn vnc_bypass(&self) -> VNC_BYPASS_R {
        VNC_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set, the CRNGT test in the RNG is bypassed."]
    #[inline(always)]
    pub fn trng_crngt_bypass(&self) -> TRNG_CRNGT_BYPASS_R {
        TRNG_CRNGT_BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set, the autocorrelation test in the TRNG module is bypassed."]
    #[inline(always)]
    pub fn auto_correlate_bypass(&self) -> AUTO_CORRELATE_BYPASS_R {
        AUTO_CORRELATE_BYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - When set, the Von-Neuman balancer is bypassed (including the 32 consecutive bits test)."]
    #[inline(always)]
    #[must_use]
    pub fn vnc_bypass(&mut self) -> VNC_BYPASS_W<TRNG_DEBUG_CONTROL_SPEC> {
        VNC_BYPASS_W::new(self, 1)
    }
    #[doc = "Bit 2 - When set, the CRNGT test in the RNG is bypassed."]
    #[inline(always)]
    #[must_use]
    pub fn trng_crngt_bypass(&mut self) -> TRNG_CRNGT_BYPASS_W<TRNG_DEBUG_CONTROL_SPEC> {
        TRNG_CRNGT_BYPASS_W::new(self, 2)
    }
    #[doc = "Bit 3 - When set, the autocorrelation test in the TRNG module is bypassed."]
    #[inline(always)]
    #[must_use]
    pub fn auto_correlate_bypass(&mut self) -> AUTO_CORRELATE_BYPASS_W<TRNG_DEBUG_CONTROL_SPEC> {
        AUTO_CORRELATE_BYPASS_W::new(self, 3)
    }
}
#[doc = "Debug register.  

You can [`read`](crate::Reg::read) this register and get [`trng_debug_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_debug_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRNG_DEBUG_CONTROL_SPEC;
impl crate::RegisterSpec for TRNG_DEBUG_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_debug_control::R`](R) reader structure"]
impl crate::Readable for TRNG_DEBUG_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trng_debug_control::W`](W) writer structure"]
impl crate::Writable for TRNG_DEBUG_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRNG_DEBUG_CONTROL to value 0"]
impl crate::Resettable for TRNG_DEBUG_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
