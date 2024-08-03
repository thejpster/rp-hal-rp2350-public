#[doc = "Register `SHPR3` reader"]
pub type R = crate::R<SHPR3_SPEC>;
#[doc = "Register `SHPR3` writer"]
pub type W = crate::W<SHPR3_SPEC>;
#[doc = "Field `PRI_12_3` reader - Priority of system handler 12, SecureFault"]
pub type PRI_12_3_R = crate::FieldReader;
#[doc = "Field `PRI_12_3` writer - Priority of system handler 12, SecureFault"]
pub type PRI_12_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRI_13` reader - Reserved, RES0"]
pub type PRI_13_R = crate::FieldReader;
#[doc = "Field `PRI_14_3` reader - Priority of system handler 14, SecureFault"]
pub type PRI_14_3_R = crate::FieldReader;
#[doc = "Field `PRI_14_3` writer - Priority of system handler 14, SecureFault"]
pub type PRI_14_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRI_15_3` reader - Priority of system handler 15, SecureFault"]
pub type PRI_15_3_R = crate::FieldReader;
#[doc = "Field `PRI_15_3` writer - Priority of system handler 15, SecureFault"]
pub type PRI_15_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 5:7 - Priority of system handler 12, SecureFault"]
    #[inline(always)]
    pub fn pri_12_3(&self) -> PRI_12_3_R {
        PRI_12_3_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Reserved, RES0"]
    #[inline(always)]
    pub fn pri_13(&self) -> PRI_13_R {
        PRI_13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 21:23 - Priority of system handler 14, SecureFault"]
    #[inline(always)]
    pub fn pri_14_3(&self) -> PRI_14_3_R {
        PRI_14_3_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Priority of system handler 15, SecureFault"]
    #[inline(always)]
    pub fn pri_15_3(&self) -> PRI_15_3_R {
        PRI_15_3_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 5:7 - Priority of system handler 12, SecureFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_12_3(&mut self) -> PRI_12_3_W<SHPR3_SPEC> {
        PRI_12_3_W::new(self, 5)
    }
    #[doc = "Bits 21:23 - Priority of system handler 14, SecureFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_14_3(&mut self) -> PRI_14_3_W<SHPR3_SPEC> {
        PRI_14_3_W::new(self, 21)
    }
    #[doc = "Bits 29:31 - Priority of system handler 15, SecureFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_15_3(&mut self) -> PRI_15_3_W<SHPR3_SPEC> {
        PRI_15_3_W::new(self, 29)
    }
}
#[doc = "Sets or returns priority for system handlers 12 - 15  

You can [`read`](crate::Reg::read) this register and get [`shpr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHPR3_SPEC;
impl crate::RegisterSpec for SHPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr3::R`](R) reader structure"]
impl crate::Readable for SHPR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shpr3::W`](W) writer structure"]
impl crate::Writable for SHPR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHPR3 to value 0"]
impl crate::Resettable for SHPR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
