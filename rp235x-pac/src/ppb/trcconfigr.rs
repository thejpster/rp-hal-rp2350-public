#[doc = "Register `TRCCONFIGR` reader"]
pub type R = crate::R<TRCCONFIGR_SPEC>;
#[doc = "Register `TRCCONFIGR` writer"]
pub type W = crate::W<TRCCONFIGR_SPEC>;
#[doc = "Field `BB` reader - Branch broadcast mode"]
pub type BB_R = crate::BitReader;
#[doc = "Field `BB` writer - Branch broadcast mode"]
pub type BB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCI` reader - Cycle counting in instruction trace"]
pub type CCI_R = crate::BitReader;
#[doc = "Field `CCI` writer - Cycle counting in instruction trace"]
pub type CCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COND` reader - Conditional instruction tracing"]
pub type COND_R = crate::FieldReader;
#[doc = "Field `COND` writer - Conditional instruction tracing"]
pub type COND_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TS` reader - Global timestamp tracing"]
pub type TS_R = crate::BitReader;
#[doc = "Field `TS` writer - Global timestamp tracing"]
pub type TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS` reader - Return stack enable"]
pub type RS_R = crate::BitReader;
#[doc = "Field `RS` writer - Return stack enable"]
pub type RS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Branch broadcast mode"]
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cycle counting in instruction trace"]
    #[inline(always)]
    pub fn cci(&self) -> CCI_R {
        CCI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:10 - Conditional instruction tracing"]
    #[inline(always)]
    pub fn cond(&self) -> COND_R {
        COND_R::new(((self.bits >> 5) & 0x3f) as u8)
    }
    #[doc = "Bit 11 - Global timestamp tracing"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Return stack enable"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Branch broadcast mode"]
    #[inline(always)]
    #[must_use]
    pub fn bb(&mut self) -> BB_W<TRCCONFIGR_SPEC> {
        BB_W::new(self, 3)
    }
    #[doc = "Bit 4 - Cycle counting in instruction trace"]
    #[inline(always)]
    #[must_use]
    pub fn cci(&mut self) -> CCI_W<TRCCONFIGR_SPEC> {
        CCI_W::new(self, 4)
    }
    #[doc = "Bits 5:10 - Conditional instruction tracing"]
    #[inline(always)]
    #[must_use]
    pub fn cond(&mut self) -> COND_W<TRCCONFIGR_SPEC> {
        COND_W::new(self, 5)
    }
    #[doc = "Bit 11 - Global timestamp tracing"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<TRCCONFIGR_SPEC> {
        TS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Return stack enable"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<TRCCONFIGR_SPEC> {
        RS_W::new(self, 12)
    }
}
#[doc = "The TRCCONFIGR sets the basic tracing options for the trace unit  

You can [`read`](crate::Reg::read) this register and get [`trcconfigr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcconfigr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCCONFIGR_SPEC;
impl crate::RegisterSpec for TRCCONFIGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcconfigr::R`](R) reader structure"]
impl crate::Readable for TRCCONFIGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcconfigr::W`](W) writer structure"]
impl crate::Writable for TRCCONFIGR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCCONFIGR to value 0"]
impl crate::Resettable for TRCCONFIGR_SPEC {
    const RESET_VALUE: u32 = 0;
}
