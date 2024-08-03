#[doc = "Register `SFSR` reader"]
pub type R = crate::R<SFSR_SPEC>;
#[doc = "Register `SFSR` writer"]
pub type W = crate::W<SFSR_SPEC>;
#[doc = "Field `INVEP` reader - This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
pub type INVEP_R = crate::BitReader;
#[doc = "Field `INVEP` writer - This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
pub type INVEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVIS` reader - This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
pub type INVIS_R = crate::BitReader;
#[doc = "Field `INVIS` writer - This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
pub type INVIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVER` reader - This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
pub type INVER_R = crate::BitReader;
#[doc = "Field `INVER` writer - This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
pub type INVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUVIOL` reader - Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
pub type AUVIOL_R = crate::BitReader;
#[doc = "Field `AUVIOL` writer - Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
pub type AUVIOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVTRAN` reader - Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
pub type INVTRAN_R = crate::BitReader;
#[doc = "Field `INVTRAN` writer - Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
pub type INVTRAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPERR` reader - Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
pub type LSPERR_R = crate::BitReader;
#[doc = "Field `LSPERR` writer - Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
pub type LSPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFARVALID` reader - This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
pub type SFARVALID_R = crate::BitReader;
#[doc = "Field `SFARVALID` writer - This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
pub type SFARVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERR` reader - Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
pub type LSERR_R = crate::BitReader;
#[doc = "Field `LSERR` writer - Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
pub type LSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
    #[inline(always)]
    pub fn invep(&self) -> INVEP_R {
        INVEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
    #[inline(always)]
    pub fn invis(&self) -> INVIS_R {
        INVIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
    #[inline(always)]
    pub fn inver(&self) -> INVER_R {
        INVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
    #[inline(always)]
    pub fn auviol(&self) -> AUVIOL_R {
        AUVIOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
    #[inline(always)]
    pub fn invtran(&self) -> INVTRAN_R {
        INVTRAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
    #[inline(always)]
    pub fn lsperr(&self) -> LSPERR_R {
        LSPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
    #[inline(always)]
    pub fn sfarvalid(&self) -> SFARVALID_R {
        SFARVALID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
    #[inline(always)]
    pub fn lserr(&self) -> LSERR_R {
        LSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set if a function call from the Non-secure state or exception targets a non-SG instruction in the Secure state. This bit is also set if the target address is a SG instruction, but there is no matching SAU/IDAU region with the NSC flag set"]
    #[inline(always)]
    #[must_use]
    pub fn invep(&mut self) -> INVEP_W<SFSR_SPEC> {
        INVEP_W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is set if the integrity signature in an exception stack frame is found to be invalid during the unstacking operation"]
    #[inline(always)]
    #[must_use]
    pub fn invis(&mut self) -> INVIS_W<SFSR_SPEC> {
        INVIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - This can be caused by EXC_RETURN.DCRS being set to 0 when returning from an exception in the Non-secure state, or by EXC_RETURN.ES being set to 1 when returning from an exception in the Non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn inver(&mut self) -> INVER_W<SFSR_SPEC> {
        INVER_W::new(self, 2)
    }
    #[doc = "Bit 3 - Sticky flag indicating that an attempt was made to access parts of the address space that are marked as Secure with NS-Req for the transaction set to Non-secure. This bit is not set if the violation occurred during lazy state preservation. See LSPERR"]
    #[inline(always)]
    #[must_use]
    pub fn auviol(&mut self) -> AUVIOL_W<SFSR_SPEC> {
        AUVIOL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Sticky flag indicating that an exception was raised due to a branch that was not flagged as being domain crossing causing a transition from Secure to Non-secure memory"]
    #[inline(always)]
    #[must_use]
    pub fn invtran(&mut self) -> INVTRAN_W<SFSR_SPEC> {
        INVTRAN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stick flag indicating that an SAU or IDAU violation occurred during the lazy preservation of floating-point state"]
    #[inline(always)]
    #[must_use]
    pub fn lsperr(&mut self) -> LSPERR_W<SFSR_SPEC> {
        LSPERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is set when the SFAR register contains a valid value. As with similar fields, such as BFSR.BFARVALID and MMFSR.MMARVALID, this bit can be cleared by other exceptions, such as BusFault"]
    #[inline(always)]
    #[must_use]
    pub fn sfarvalid(&mut self) -> SFARVALID_W<SFSR_SPEC> {
        SFARVALID_W::new(self, 6)
    }
    #[doc = "Bit 7 - Sticky flag indicating that an error occurred during lazy state activation or deactivation"]
    #[inline(always)]
    #[must_use]
    pub fn lserr(&mut self) -> LSERR_W<SFSR_SPEC> {
        LSERR_W::new(self, 7)
    }
}
#[doc = "Provides information about any security related faults  

You can [`read`](crate::Reg::read) this register and get [`sfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFSR_SPEC;
impl crate::RegisterSpec for SFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfsr::R`](R) reader structure"]
impl crate::Readable for SFSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sfsr::W`](W) writer structure"]
impl crate::Writable for SFSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SFSR to value 0"]
impl crate::Resettable for SFSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
