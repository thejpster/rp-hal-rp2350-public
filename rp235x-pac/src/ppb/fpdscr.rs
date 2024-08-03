#[doc = "Register `FPDSCR` reader"]
pub type R = crate::R<FPDSCR_SPEC>;
#[doc = "Register `FPDSCR` writer"]
pub type W = crate::W<FPDSCR_SPEC>;
#[doc = "Field `RMODE` reader - Default value for FPSCR.RMode"]
pub type RMODE_R = crate::FieldReader;
#[doc = "Field `RMODE` writer - Default value for FPSCR.RMode"]
pub type RMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FZ` reader - Default value for FPSCR.FZ"]
pub type FZ_R = crate::BitReader;
#[doc = "Field `FZ` writer - Default value for FPSCR.FZ"]
pub type FZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DN` reader - Default value for FPSCR.DN"]
pub type DN_R = crate::BitReader;
#[doc = "Field `DN` writer - Default value for FPSCR.DN"]
pub type DN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHP` reader - Default value for FPSCR.AHP"]
pub type AHP_R = crate::BitReader;
#[doc = "Field `AHP` writer - Default value for FPSCR.AHP"]
pub type AHP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode"]
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ"]
    #[inline(always)]
    pub fn fz(&self) -> FZ_R {
        FZ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN"]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP"]
    #[inline(always)]
    pub fn ahp(&self) -> AHP_R {
        AHP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode"]
    #[inline(always)]
    #[must_use]
    pub fn rmode(&mut self) -> RMODE_W<FPDSCR_SPEC> {
        RMODE_W::new(self, 22)
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ"]
    #[inline(always)]
    #[must_use]
    pub fn fz(&mut self) -> FZ_W<FPDSCR_SPEC> {
        FZ_W::new(self, 24)
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN"]
    #[inline(always)]
    #[must_use]
    pub fn dn(&mut self) -> DN_W<FPDSCR_SPEC> {
        DN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP"]
    #[inline(always)]
    #[must_use]
    pub fn ahp(&mut self) -> AHP_W<FPDSCR_SPEC> {
        AHP_W::new(self, 26)
    }
}
#[doc = "Holds the default values for the floating-point status control data that the PE assigns to the FPSCR when it creates a new floating-point context  

You can [`read`](crate::Reg::read) this register and get [`fpdscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpdscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPDSCR_SPEC;
impl crate::RegisterSpec for FPDSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpdscr::R`](R) reader structure"]
impl crate::Readable for FPDSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpdscr::W`](W) writer structure"]
impl crate::Writable for FPDSCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPDSCR to value 0"]
impl crate::Resettable for FPDSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
