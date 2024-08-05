#[doc = "Register `CFSR` reader"]
pub type R = crate::R<CFSR_SPEC>;
#[doc = "Register `CFSR` writer"]
pub type W = crate::W<CFSR_SPEC>;
#[doc = "Field `MMFSR` reader - Provides information on MemManage exceptions"]
pub type MMFSR_R = crate::FieldReader;
#[doc = "Field `MMFSR` writer - Provides information on MemManage exceptions"]
pub type MMFSR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BFSR_IBUSERR` reader - Records whether a BusFault on an instruction prefetch has occurred"]
pub type BFSR_IBUSERR_R = crate::BitReader;
#[doc = "Field `BFSR_IBUSERR` writer - Records whether a BusFault on an instruction prefetch has occurred"]
pub type BFSR_IBUSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFSR_PRECISERR` reader - Records whether a precise data access error has occurred"]
pub type BFSR_PRECISERR_R = crate::BitReader;
#[doc = "Field `BFSR_PRECISERR` writer - Records whether a precise data access error has occurred"]
pub type BFSR_PRECISERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFSR_IMPRECISERR` reader - Records whether an imprecise data access error has occurred"]
pub type BFSR_IMPRECISERR_R = crate::BitReader;
#[doc = "Field `BFSR_IMPRECISERR` writer - Records whether an imprecise data access error has occurred"]
pub type BFSR_IMPRECISERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFSR_UNSTKERR` reader - Records whether a derived BusFault occurred during exception return unstacking"]
pub type BFSR_UNSTKERR_R = crate::BitReader;
#[doc = "Field `BFSR_UNSTKERR` writer - Records whether a derived BusFault occurred during exception return unstacking"]
pub type BFSR_UNSTKERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFSR_STKERR` reader - Records whether a derived BusFault occurred during exception entry stacking"]
pub type BFSR_STKERR_R = crate::BitReader;
#[doc = "Field `BFSR_STKERR` writer - Records whether a derived BusFault occurred during exception entry stacking"]
pub type BFSR_STKERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFSR_LSPERR` reader - Records whether a BusFault occurred during FP lazy state preservation"]
pub type BFSR_LSPERR_R = crate::BitReader;
#[doc = "Field `BFSR_LSPERR` writer - Records whether a BusFault occurred during FP lazy state preservation"]
pub type BFSR_LSPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFSR_BFARVALID` reader - Indicates validity of the contents of the BFAR register"]
pub type BFSR_BFARVALID_R = crate::BitReader;
#[doc = "Field `BFSR_BFARVALID` writer - Indicates validity of the contents of the BFAR register"]
pub type BFSR_BFARVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFSR_UNDEFINSTR` reader - Sticky flag indicating whether an undefined instruction error has occurred"]
pub type UFSR_UNDEFINSTR_R = crate::BitReader;
#[doc = "Field `UFSR_UNDEFINSTR` writer - Sticky flag indicating whether an undefined instruction error has occurred"]
pub type UFSR_UNDEFINSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFSR_INVSTATE` reader - Sticky flag indicating whether an EPSR.T or EPSR.IT validity error has occurred"]
pub type UFSR_INVSTATE_R = crate::BitReader;
#[doc = "Field `UFSR_INVSTATE` writer - Sticky flag indicating whether an EPSR.T or EPSR.IT validity error has occurred"]
pub type UFSR_INVSTATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFSR_INVPC` reader - Sticky flag indicating whether an integrity check error has occurred"]
pub type UFSR_INVPC_R = crate::BitReader;
#[doc = "Field `UFSR_INVPC` writer - Sticky flag indicating whether an integrity check error has occurred"]
pub type UFSR_INVPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFSR_NOCP` reader - Sticky flag indicating whether a coprocessor disabled or not present error has occurred"]
pub type UFSR_NOCP_R = crate::BitReader;
#[doc = "Field `UFSR_NOCP` writer - Sticky flag indicating whether a coprocessor disabled or not present error has occurred"]
pub type UFSR_NOCP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFSR_STKOF` reader - Sticky flag indicating whether a stack overflow error has occurred"]
pub type UFSR_STKOF_R = crate::BitReader;
#[doc = "Field `UFSR_STKOF` writer - Sticky flag indicating whether a stack overflow error has occurred"]
pub type UFSR_STKOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFSR_UNALIGNED` reader - Sticky flag indicating whether an unaligned access error has occurred"]
pub type UFSR_UNALIGNED_R = crate::BitReader;
#[doc = "Field `UFSR_UNALIGNED` writer - Sticky flag indicating whether an unaligned access error has occurred"]
pub type UFSR_UNALIGNED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFSR_DIVBYZERO` reader - Sticky flag indicating whether an integer division by zero error has occurred"]
pub type UFSR_DIVBYZERO_R = crate::BitReader;
#[doc = "Field `UFSR_DIVBYZERO` writer - Sticky flag indicating whether an integer division by zero error has occurred"]
pub type UFSR_DIVBYZERO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Provides information on MemManage exceptions"]
    #[inline(always)]
    pub fn mmfsr(&self) -> MMFSR_R {
        MMFSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Records whether a BusFault on an instruction prefetch has occurred"]
    #[inline(always)]
    pub fn bfsr_ibuserr(&self) -> BFSR_IBUSERR_R {
        BFSR_IBUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Records whether a precise data access error has occurred"]
    #[inline(always)]
    pub fn bfsr_preciserr(&self) -> BFSR_PRECISERR_R {
        BFSR_PRECISERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Records whether an imprecise data access error has occurred"]
    #[inline(always)]
    pub fn bfsr_impreciserr(&self) -> BFSR_IMPRECISERR_R {
        BFSR_IMPRECISERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Records whether a derived BusFault occurred during exception return unstacking"]
    #[inline(always)]
    pub fn bfsr_unstkerr(&self) -> BFSR_UNSTKERR_R {
        BFSR_UNSTKERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Records whether a derived BusFault occurred during exception entry stacking"]
    #[inline(always)]
    pub fn bfsr_stkerr(&self) -> BFSR_STKERR_R {
        BFSR_STKERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Records whether a BusFault occurred during FP lazy state preservation"]
    #[inline(always)]
    pub fn bfsr_lsperr(&self) -> BFSR_LSPERR_R {
        BFSR_LSPERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates validity of the contents of the BFAR register"]
    #[inline(always)]
    pub fn bfsr_bfarvalid(&self) -> BFSR_BFARVALID_R {
        BFSR_BFARVALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Sticky flag indicating whether an undefined instruction error has occurred"]
    #[inline(always)]
    pub fn ufsr_undefinstr(&self) -> UFSR_UNDEFINSTR_R {
        UFSR_UNDEFINSTR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sticky flag indicating whether an EPSR.T or EPSR.IT validity error has occurred"]
    #[inline(always)]
    pub fn ufsr_invstate(&self) -> UFSR_INVSTATE_R {
        UFSR_INVSTATE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Sticky flag indicating whether an integrity check error has occurred"]
    #[inline(always)]
    pub fn ufsr_invpc(&self) -> UFSR_INVPC_R {
        UFSR_INVPC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Sticky flag indicating whether a coprocessor disabled or not present error has occurred"]
    #[inline(always)]
    pub fn ufsr_nocp(&self) -> UFSR_NOCP_R {
        UFSR_NOCP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Sticky flag indicating whether a stack overflow error has occurred"]
    #[inline(always)]
    pub fn ufsr_stkof(&self) -> UFSR_STKOF_R {
        UFSR_STKOF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Sticky flag indicating whether an unaligned access error has occurred"]
    #[inline(always)]
    pub fn ufsr_unaligned(&self) -> UFSR_UNALIGNED_R {
        UFSR_UNALIGNED_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Sticky flag indicating whether an integer division by zero error has occurred"]
    #[inline(always)]
    pub fn ufsr_divbyzero(&self) -> UFSR_DIVBYZERO_R {
        UFSR_DIVBYZERO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Provides information on MemManage exceptions"]
    #[inline(always)]
    #[must_use]
    pub fn mmfsr(&mut self) -> MMFSR_W<CFSR_SPEC> {
        MMFSR_W::new(self, 0)
    }
    #[doc = "Bit 8 - Records whether a BusFault on an instruction prefetch has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn bfsr_ibuserr(&mut self) -> BFSR_IBUSERR_W<CFSR_SPEC> {
        BFSR_IBUSERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Records whether a precise data access error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn bfsr_preciserr(&mut self) -> BFSR_PRECISERR_W<CFSR_SPEC> {
        BFSR_PRECISERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Records whether an imprecise data access error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn bfsr_impreciserr(&mut self) -> BFSR_IMPRECISERR_W<CFSR_SPEC> {
        BFSR_IMPRECISERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Records whether a derived BusFault occurred during exception return unstacking"]
    #[inline(always)]
    #[must_use]
    pub fn bfsr_unstkerr(&mut self) -> BFSR_UNSTKERR_W<CFSR_SPEC> {
        BFSR_UNSTKERR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Records whether a derived BusFault occurred during exception entry stacking"]
    #[inline(always)]
    #[must_use]
    pub fn bfsr_stkerr(&mut self) -> BFSR_STKERR_W<CFSR_SPEC> {
        BFSR_STKERR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Records whether a BusFault occurred during FP lazy state preservation"]
    #[inline(always)]
    #[must_use]
    pub fn bfsr_lsperr(&mut self) -> BFSR_LSPERR_W<CFSR_SPEC> {
        BFSR_LSPERR_W::new(self, 13)
    }
    #[doc = "Bit 15 - Indicates validity of the contents of the BFAR register"]
    #[inline(always)]
    #[must_use]
    pub fn bfsr_bfarvalid(&mut self) -> BFSR_BFARVALID_W<CFSR_SPEC> {
        BFSR_BFARVALID_W::new(self, 15)
    }
    #[doc = "Bit 16 - Sticky flag indicating whether an undefined instruction error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn ufsr_undefinstr(&mut self) -> UFSR_UNDEFINSTR_W<CFSR_SPEC> {
        UFSR_UNDEFINSTR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Sticky flag indicating whether an EPSR.T or EPSR.IT validity error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn ufsr_invstate(&mut self) -> UFSR_INVSTATE_W<CFSR_SPEC> {
        UFSR_INVSTATE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Sticky flag indicating whether an integrity check error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn ufsr_invpc(&mut self) -> UFSR_INVPC_W<CFSR_SPEC> {
        UFSR_INVPC_W::new(self, 18)
    }
    #[doc = "Bit 19 - Sticky flag indicating whether a coprocessor disabled or not present error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn ufsr_nocp(&mut self) -> UFSR_NOCP_W<CFSR_SPEC> {
        UFSR_NOCP_W::new(self, 19)
    }
    #[doc = "Bit 20 - Sticky flag indicating whether a stack overflow error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn ufsr_stkof(&mut self) -> UFSR_STKOF_W<CFSR_SPEC> {
        UFSR_STKOF_W::new(self, 20)
    }
    #[doc = "Bit 24 - Sticky flag indicating whether an unaligned access error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn ufsr_unaligned(&mut self) -> UFSR_UNALIGNED_W<CFSR_SPEC> {
        UFSR_UNALIGNED_W::new(self, 24)
    }
    #[doc = "Bit 25 - Sticky flag indicating whether an integer division by zero error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn ufsr_divbyzero(&mut self) -> UFSR_DIVBYZERO_W<CFSR_SPEC> {
        UFSR_DIVBYZERO_W::new(self, 25)
    }
}
#[doc = "Contains the three Configurable Fault Status Registers. 31:16 UFSR: Provides information on UsageFault exceptions 15:8 BFSR: Provides information on BusFault exceptions 7:0 MMFSR: Provides information on MemManage exceptions  

You can [`read`](crate::Reg::read) this register and get [`cfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFSR_SPEC;
impl crate::RegisterSpec for CFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfsr::R`](R) reader structure"]
impl crate::Readable for CFSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfsr::W`](W) writer structure"]
impl crate::Writable for CFSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFSR to value 0"]
impl crate::Resettable for CFSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
