#[doc = "Register `PERI_NONSEC` reader"]
pub type R = crate::R<PERI_NONSEC_SPEC>;
#[doc = "Register `PERI_NONSEC` writer"]
pub type W = crate::W<PERI_NONSEC_SPEC>;
#[doc = "Field `INTERP0` reader - If 1, detach interpolator 0 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
pub type INTERP0_R = crate::BitReader;
#[doc = "Field `INTERP0` writer - If 1, detach interpolator 0 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
pub type INTERP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERP1` reader - If 1, detach interpolator 1 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
pub type INTERP1_R = crate::BitReader;
#[doc = "Field `INTERP1` writer - If 1, detach interpolator 1 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
pub type INTERP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMDS` reader - IF 1, detach TMDS encoder (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
pub type TMDS_R = crate::BitReader;
#[doc = "Field `TMDS` writer - IF 1, detach TMDS encoder (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
pub type TMDS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If 1, detach interpolator 0 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    pub fn interp0(&self) -> INTERP0_R {
        INTERP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If 1, detach interpolator 1 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    pub fn interp1(&self) -> INTERP1_R {
        INTERP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - IF 1, detach TMDS encoder (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    pub fn tmds(&self) -> TMDS_R {
        TMDS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If 1, detach interpolator 0 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    #[must_use]
    pub fn interp0(&mut self) -> INTERP0_W<PERI_NONSEC_SPEC> {
        INTERP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - If 1, detach interpolator 1 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    #[must_use]
    pub fn interp1(&mut self) -> INTERP1_W<PERI_NONSEC_SPEC> {
        INTERP1_W::new(self, 1)
    }
    #[doc = "Bit 5 - IF 1, detach TMDS encoder (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    #[must_use]
    pub fn tmds(&mut self) -> TMDS_W<PERI_NONSEC_SPEC> {
        TMDS_W::new(self, 5)
    }
}
#[doc = "Detach certain core-local peripherals from Secure SIO, and attach them to Non-secure SIO, so that Non-secure software can use them. Attempting to access one of these peripherals from the Secure SIO when it is attached to the Non-secure SIO, or vice versa, will generate a bus error. This register is per-core, and is only present on the Secure SIO. Most SIO hardware is duplicated across the Secure and Non-secure SIO, so is not listed in this register.  

You can [`read`](crate::Reg::read) this register and get [`peri_nonsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_nonsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_NONSEC_SPEC;
impl crate::RegisterSpec for PERI_NONSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_nonsec::R`](R) reader structure"]
impl crate::Readable for PERI_NONSEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_nonsec::W`](W) writer structure"]
impl crate::Writable for PERI_NONSEC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_NONSEC to value 0"]
impl crate::Resettable for PERI_NONSEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
