#[doc = "Register `TRCITIATBOUTR` reader"]
pub type R = crate::R<TRCITIATBOUTR_SPEC>;
#[doc = "Register `TRCITIATBOUTR` writer"]
pub type W = crate::W<TRCITIATBOUTR_SPEC>;
#[doc = "Field `ATVALID` reader - Integration Mode instruction ATVALID out"]
pub type ATVALID_R = crate::BitReader;
#[doc = "Field `ATVALID` writer - Integration Mode instruction ATVALID out"]
pub type ATVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFREADY` reader - Integration Mode instruction AFREADY out"]
pub type AFREADY_R = crate::BitReader;
#[doc = "Field `AFREADY` writer - Integration Mode instruction AFREADY out"]
pub type AFREADY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Integration Mode instruction ATVALID out"]
    #[inline(always)]
    pub fn atvalid(&self) -> ATVALID_R {
        ATVALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Integration Mode instruction AFREADY out"]
    #[inline(always)]
    pub fn afready(&self) -> AFREADY_R {
        AFREADY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration Mode instruction ATVALID out"]
    #[inline(always)]
    #[must_use]
    pub fn atvalid(&mut self) -> ATVALID_W<TRCITIATBOUTR_SPEC> {
        ATVALID_W::new(self, 0)
    }
    #[doc = "Bit 1 - Integration Mode instruction AFREADY out"]
    #[inline(always)]
    #[must_use]
    pub fn afready(&mut self) -> AFREADY_W<TRCITIATBOUTR_SPEC> {
        AFREADY_W::new(self, 1)
    }
}
#[doc = "Trace Integration Instruction ATB Out Register  

You can [`read`](crate::Reg::read) this register and get [`trcitiatboutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitiatboutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCITIATBOUTR_SPEC;
impl crate::RegisterSpec for TRCITIATBOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcitiatboutr::R`](R) reader structure"]
impl crate::Readable for TRCITIATBOUTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcitiatboutr::W`](W) writer structure"]
impl crate::Writable for TRCITIATBOUTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCITIATBOUTR to value 0"]
impl crate::Resettable for TRCITIATBOUTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
