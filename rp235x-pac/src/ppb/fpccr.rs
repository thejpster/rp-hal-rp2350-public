#[doc = "Register `FPCCR` reader"]
pub type R = crate::R<FPCCR_SPEC>;
#[doc = "Register `FPCCR` writer"]
pub type W = crate::W<FPCCR_SPEC>;
#[doc = "Field `LSPACT` reader - Indicates whether lazy preservation of the floating-point state is active"]
pub type LSPACT_R = crate::BitReader;
#[doc = "Field `LSPACT` writer - Indicates whether lazy preservation of the floating-point state is active"]
pub type LSPACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USER` reader - Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
pub type USER_R = crate::BitReader;
#[doc = "Field `USER` writer - Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
pub type USER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S` reader - Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
pub type S_R = crate::BitReader;
#[doc = "Field `S` writer - Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
pub type S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THREAD` reader - Indicates the PE mode when it allocated the floating-point stack frame"]
pub type THREAD_R = crate::BitReader;
#[doc = "Field `THREAD` writer - Indicates the PE mode when it allocated the floating-point stack frame"]
pub type THREAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRDY` reader - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
pub type HFRDY_R = crate::BitReader;
#[doc = "Field `HFRDY` writer - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
pub type HFRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRDY` reader - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
pub type MMRDY_R = crate::BitReader;
#[doc = "Field `MMRDY` writer - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
pub type MMRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFRDY` reader - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
pub type BFRDY_R = crate::BitReader;
#[doc = "Field `BFRDY` writer - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
pub type BFRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFRDY` reader - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
pub type SFRDY_R = crate::BitReader;
#[doc = "Field `SFRDY` writer - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
pub type SFRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONRDY` reader - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
pub type MONRDY_R = crate::BitReader;
#[doc = "Field `MONRDY` writer - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
pub type MONRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLIMVIOL` reader - This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
pub type SPLIMVIOL_R = crate::BitReader;
#[doc = "Field `SPLIMVIOL` writer - This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
pub type SPLIMVIOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFRDY` reader - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
pub type UFRDY_R = crate::BitReader;
#[doc = "Field `UFRDY` writer - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
pub type UFRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS` reader - Treat floating-point registers as Secure enable"]
pub type TS_R = crate::BitReader;
#[doc = "Field `TS` writer - Treat floating-point registers as Secure enable"]
pub type TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRONRETS` reader - This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
pub type CLRONRETS_R = crate::BitReader;
#[doc = "Field `CLRONRETS` writer - This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
pub type CLRONRETS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRONRET` reader - Clear floating-point caller saved registers on exception return"]
pub type CLRONRET_R = crate::BitReader;
#[doc = "Field `CLRONRET` writer - Clear floating-point caller saved registers on exception return"]
pub type CLRONRET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPENS` reader - This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
pub type LSPENS_R = crate::BitReader;
#[doc = "Field `LSPENS` writer - This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
pub type LSPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPEN` reader - Enables lazy context save of floating-point state"]
pub type LSPEN_R = crate::BitReader;
#[doc = "Field `LSPEN` writer - Enables lazy context save of floating-point state"]
pub type LSPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASPEN` reader - When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
pub type ASPEN_R = crate::BitReader;
#[doc = "Field `ASPEN` writer - When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
pub type ASPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates whether lazy preservation of the floating-point state is active"]
    #[inline(always)]
    pub fn lspact(&self) -> LSPACT_R {
        LSPACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates the PE mode when it allocated the floating-point stack frame"]
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
    #[inline(always)]
    pub fn hfrdy(&self) -> HFRDY_R {
        HFRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
    #[inline(always)]
    pub fn mmrdy(&self) -> MMRDY_R {
        MMRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
    #[inline(always)]
    pub fn bfrdy(&self) -> BFRDY_R {
        BFRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
    #[inline(always)]
    pub fn sfrdy(&self) -> SFRDY_R {
        SFRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
    #[inline(always)]
    pub fn splimviol(&self) -> SPLIMVIOL_R {
        SPLIMVIOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
    #[inline(always)]
    pub fn ufrdy(&self) -> UFRDY_R {
        UFRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 26 - Treat floating-point registers as Secure enable"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
    #[inline(always)]
    pub fn clronrets(&self) -> CLRONRETS_R {
        CLRONRETS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Clear floating-point caller saved registers on exception return"]
    #[inline(always)]
    pub fn clronret(&self) -> CLRONRET_R {
        CLRONRET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
    #[inline(always)]
    pub fn lspens(&self) -> LSPENS_R {
        LSPENS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enables lazy context save of floating-point state"]
    #[inline(always)]
    pub fn lspen(&self) -> LSPEN_R {
        LSPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
    #[inline(always)]
    pub fn aspen(&self) -> ASPEN_R {
        ASPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether lazy preservation of the floating-point state is active"]
    #[inline(always)]
    #[must_use]
    pub fn lspact(&mut self) -> LSPACT_W<FPCCR_SPEC> {
        LSPACT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates the privilege level of the software executing when the PE allocated the floating-point stack frame"]
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> USER_W<FPCCR_SPEC> {
        USER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Security status of the floating-point context. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state. This bit is updated whenever lazy state preservation is activated, or when a floating-point instruction is executed"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<FPCCR_SPEC> {
        S_W::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates the PE mode when it allocated the floating-point stack frame"]
    #[inline(always)]
    #[must_use]
    pub fn thread(&mut self) -> THREAD_W<FPCCR_SPEC> {
        THREAD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the HardFault exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn hfrdy(&mut self) -> HFRDY_W<FPCCR_SPEC> {
        HFRDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the MemManage exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn mmrdy(&mut self) -> MMRDY_W<FPCCR_SPEC> {
        MMRDY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the BusFault exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn bfrdy(&mut self) -> BFRDY_W<FPCCR_SPEC> {
        BFRDY_W::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the SecureFault exception to pending. This bit is only present in the Secure version of the register, and behaves as RAZ/WI when accessed from the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn sfrdy(&mut self) -> SFRDY_W<FPCCR_SPEC> {
        SFRDY_W::new(self, 7)
    }
    #[doc = "Bit 8 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the DebugMonitor exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn monrdy(&mut self) -> MONRDY_W<FPCCR_SPEC> {
        MONRDY_W::new(self, 8)
    }
    #[doc = "Bit 9 - This bit is banked between the Security states and indicates whether the floating-point context violates the stack pointer limit that was active when lazy state preservation was activated. SPLIMVIOL modifies the lazy floating-point state preservation behavior"]
    #[inline(always)]
    #[must_use]
    pub fn splimviol(&mut self) -> SPLIMVIOL_W<FPCCR_SPEC> {
        SPLIMVIOL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Indicates whether the software executing when the PE allocated the floating-point stack frame was able to set the UsageFault exception to pending"]
    #[inline(always)]
    #[must_use]
    pub fn ufrdy(&mut self) -> UFRDY_W<FPCCR_SPEC> {
        UFRDY_W::new(self, 10)
    }
    #[doc = "Bit 26 - Treat floating-point registers as Secure enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<FPCCR_SPEC> {
        TS_W::new(self, 26)
    }
    #[doc = "Bit 27 - This bit controls whether the CLRONRET bit is writeable from the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn clronrets(&mut self) -> CLRONRETS_W<FPCCR_SPEC> {
        CLRONRETS_W::new(self, 27)
    }
    #[doc = "Bit 28 - Clear floating-point caller saved registers on exception return"]
    #[inline(always)]
    #[must_use]
    pub fn clronret(&mut self) -> CLRONRET_W<FPCCR_SPEC> {
        CLRONRET_W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit controls whether the LSPEN bit is writeable from the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn lspens(&mut self) -> LSPENS_W<FPCCR_SPEC> {
        LSPENS_W::new(self, 29)
    }
    #[doc = "Bit 30 - Enables lazy context save of floating-point state"]
    #[inline(always)]
    #[must_use]
    pub fn lspen(&mut self) -> LSPEN_W<FPCCR_SPEC> {
        LSPEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
    #[inline(always)]
    #[must_use]
    pub fn aspen(&mut self) -> ASPEN_W<FPCCR_SPEC> {
        ASPEN_W::new(self, 31)
    }
}
#[doc = "Holds control data for the Floating-point extension  

You can [`read`](crate::Reg::read) this register and get [`fpccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPCCR_SPEC;
impl crate::RegisterSpec for FPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpccr::R`](R) reader structure"]
impl crate::Readable for FPCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpccr::W`](W) writer structure"]
impl crate::Writable for FPCCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPCCR to value 0x2000_0472"]
impl crate::Resettable for FPCCR_SPEC {
    const RESET_VALUE: u32 = 0x2000_0472;
}
