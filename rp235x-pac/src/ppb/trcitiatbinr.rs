#[doc = "Register `TRCITIATBINR` reader"]
pub type R = crate::R<TRCITIATBINR_SPEC>;
#[doc = "Register `TRCITIATBINR` writer"]
pub type W = crate::W<TRCITIATBINR_SPEC>;
#[doc = "Field `ATREADYM` reader - Integration Mode instruction ATREADYM in"]
pub type ATREADYM_R = crate::BitReader;
#[doc = "Field `ATREADYM` writer - Integration Mode instruction ATREADYM in"]
pub type ATREADYM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFVALIDM` reader - Integration Mode instruction AFVALIDM in"]
pub type AFVALIDM_R = crate::BitReader;
#[doc = "Field `AFVALIDM` writer - Integration Mode instruction AFVALIDM in"]
pub type AFVALIDM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Integration Mode instruction ATREADYM in"]
    #[inline(always)]
    pub fn atreadym(&self) -> ATREADYM_R {
        ATREADYM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Integration Mode instruction AFVALIDM in"]
    #[inline(always)]
    pub fn afvalidm(&self) -> AFVALIDM_R {
        AFVALIDM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration Mode instruction ATREADYM in"]
    #[inline(always)]
    #[must_use]
    pub fn atreadym(&mut self) -> ATREADYM_W<TRCITIATBINR_SPEC> {
        ATREADYM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Integration Mode instruction AFVALIDM in"]
    #[inline(always)]
    #[must_use]
    pub fn afvalidm(&mut self) -> AFVALIDM_W<TRCITIATBINR_SPEC> {
        AFVALIDM_W::new(self, 1)
    }
}
#[doc = "Trace Integration Instruction ATB In Register  

You can [`read`](crate::Reg::read) this register and get [`trcitiatbinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitiatbinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCITIATBINR_SPEC;
impl crate::RegisterSpec for TRCITIATBINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcitiatbinr::R`](R) reader structure"]
impl crate::Readable for TRCITIATBINR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcitiatbinr::W`](W) writer structure"]
impl crate::Writable for TRCITIATBINR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCITIATBINR to value 0"]
impl crate::Resettable for TRCITIATBINR_SPEC {
    const RESET_VALUE: u32 = 0;
}
