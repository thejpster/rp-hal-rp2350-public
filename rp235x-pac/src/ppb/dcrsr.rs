#[doc = "Register `DCRSR` reader"]
pub type R = crate::R<DCRSR_SPEC>;
#[doc = "Register `DCRSR` writer"]
pub type W = crate::W<DCRSR_SPEC>;
#[doc = "Field `REGSEL` reader - Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
pub type REGSEL_R = crate::FieldReader;
#[doc = "Field `REGSEL` writer - Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
pub type REGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `REGWNR` reader - Specifies the access type for the transfer"]
pub type REGWNR_R = crate::BitReader;
#[doc = "Field `REGWNR` writer - Specifies the access type for the transfer"]
pub type REGWNR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
    #[inline(always)]
    pub fn regsel(&self) -> REGSEL_R {
        REGSEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Specifies the access type for the transfer"]
    #[inline(always)]
    pub fn regwnr(&self) -> REGWNR_R {
        REGWNR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Specifies the general-purpose register, special-purpose register, or FP register to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn regsel(&mut self) -> REGSEL_W<DCRSR_SPEC> {
        REGSEL_W::new(self, 0)
    }
    #[doc = "Bit 16 - Specifies the access type for the transfer"]
    #[inline(always)]
    #[must_use]
    pub fn regwnr(&mut self) -> REGWNR_W<DCRSR_SPEC> {
        REGWNR_W::new(self, 16)
    }
}
#[doc = "With the DCRDR, provides debug access to the general-purpose registers, special-purpose registers, and the FP extension registers. A write to the DCRSR specifies the register to transfer, whether the transfer is a read or write, and starts the transfer  

You can [`read`](crate::Reg::read) this register and get [`dcrsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCRSR_SPEC;
impl crate::RegisterSpec for DCRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrsr::R`](R) reader structure"]
impl crate::Readable for DCRSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcrsr::W`](W) writer structure"]
impl crate::Writable for DCRSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCRSR to value 0"]
impl crate::Resettable for DCRSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
