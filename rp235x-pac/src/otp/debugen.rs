#[doc = "Register `DEBUGEN` reader"]
pub type R = crate::R<DEBUGEN_SPEC>;
#[doc = "Register `DEBUGEN` writer"]
pub type W = crate::W<DEBUGEN_SPEC>;
#[doc = "Field `PROC0` reader - Enable core 0's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
pub type PROC0_R = crate::BitReader;
#[doc = "Field `PROC0` writer - Enable core 0's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
pub type PROC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC0_SECURE` reader - Permit core 0's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 0 (SPIDEN and SPNIDEN). Secure debug of core 0 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
pub type PROC0_SECURE_R = crate::BitReader;
#[doc = "Field `PROC0_SECURE` writer - Permit core 0's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 0 (SPIDEN and SPNIDEN). Secure debug of core 0 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
pub type PROC0_SECURE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC1` reader - Enable core 1's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
pub type PROC1_R = crate::BitReader;
#[doc = "Field `PROC1` writer - Enable core 1's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
pub type PROC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC1_SECURE` reader - Permit core 1's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 1 (SPIDEN and SPNIDEN). Secure debug of core 1 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD."]
pub type PROC1_SECURE_R = crate::BitReader;
#[doc = "Field `PROC1_SECURE` writer - Permit core 1's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 1 (SPIDEN and SPNIDEN). Secure debug of core 1 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD."]
pub type PROC1_SECURE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISC` reader - Enable other debug components. Specifically, the CTI, and the APB-AP used to access the RISC-V Debug Module. These components are disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
pub type MISC_R = crate::BitReader;
#[doc = "Field `MISC` writer - Enable other debug components. Specifically, the CTI, and the APB-AP used to access the RISC-V Debug Module. These components are disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
pub type MISC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable core 0's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
    #[inline(always)]
    pub fn proc0(&self) -> PROC0_R {
        PROC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Permit core 0's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 0 (SPIDEN and SPNIDEN). Secure debug of core 0 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
    #[inline(always)]
    pub fn proc0_secure(&self) -> PROC0_SECURE_R {
        PROC0_SECURE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable core 1's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
    #[inline(always)]
    pub fn proc1(&self) -> PROC1_R {
        PROC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Permit core 1's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 1 (SPIDEN and SPNIDEN). Secure debug of core 1 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD."]
    #[inline(always)]
    pub fn proc1_secure(&self) -> PROC1_SECURE_R {
        PROC1_SECURE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable other debug components. Specifically, the CTI, and the APB-AP used to access the RISC-V Debug Module. These components are disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
    #[inline(always)]
    pub fn misc(&self) -> MISC_R {
        MISC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable core 0's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
    #[inline(always)]
    #[must_use]
    pub fn proc0(&mut self) -> PROC0_W<DEBUGEN_SPEC> {
        PROC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Permit core 0's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 0 (SPIDEN and SPNIDEN). Secure debug of core 0 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
    #[inline(always)]
    #[must_use]
    pub fn proc0_secure(&mut self) -> PROC0_SECURE_W<DEBUGEN_SPEC> {
        PROC0_SECURE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable core 1's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
    #[inline(always)]
    #[must_use]
    pub fn proc1(&mut self) -> PROC1_W<DEBUGEN_SPEC> {
        PROC1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Permit core 1's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 1 (SPIDEN and SPNIDEN). Secure debug of core 1 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD."]
    #[inline(always)]
    #[must_use]
    pub fn proc1_secure(&mut self) -> PROC1_SECURE_W<DEBUGEN_SPEC> {
        PROC1_SECURE_W::new(self, 3)
    }
    #[doc = "Bit 8 - Enable other debug components. Specifically, the CTI, and the APB-AP used to access the RISC-V Debug Module. These components are disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
    #[inline(always)]
    #[must_use]
    pub fn misc(&mut self) -> MISC_W<DEBUGEN_SPEC> {
        MISC_W::new(self, 8)
    }
}
#[doc = "Enable a debug feature that has been disabled. Debug features are disabled if one of the relevant critical boot flags is set in OTP (DEBUG_DISABLE or SECURE_DEBUG_DISABLE), OR if a debug key is marked valid in OTP, and the matching key value has not been supplied over SWD. Specifically: - The DEBUG_DISABLE flag disables all debug features. This can be fully overridden by setting all bits of this register. - The SECURE_DEBUG_DISABLE flag disables secure processor debug. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If a single debug key has been registered, and no matching key value has been supplied over SWD, then all debug features are disabled. This can be fully overridden by setting all bits of this register. - If both debug keys have been registered, and the Non-secure key's value (key 6) has been supplied over SWD, secure processor debug is disabled. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If both debug keys have been registered, and the Secure key's value (key 5) has been supplied over SWD, then no debug features are disabled by the key mechanism. However, note that in this case debug features may still be disabled by the critical boot flags.  

You can [`read`](crate::Reg::read) this register and get [`debugen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUGEN_SPEC;
impl crate::RegisterSpec for DEBUGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugen::R`](R) reader structure"]
impl crate::Readable for DEBUGEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debugen::W`](W) writer structure"]
impl crate::Writable for DEBUGEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUGEN to value 0"]
impl crate::Resettable for DEBUGEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
