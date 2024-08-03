#[doc = "Register `SHPR1` reader"]
pub type R = crate::R<SHPR1_SPEC>;
#[doc = "Register `SHPR1` writer"]
pub type W = crate::W<SHPR1_SPEC>;
#[doc = "Field `PRI_4_3` reader - Priority of system handler 4, SecureFault"]
pub type PRI_4_3_R = crate::FieldReader;
#[doc = "Field `PRI_4_3` writer - Priority of system handler 4, SecureFault"]
pub type PRI_4_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRI_5_3` reader - Priority of system handler 5, SecureFault"]
pub type PRI_5_3_R = crate::FieldReader;
#[doc = "Field `PRI_5_3` writer - Priority of system handler 5, SecureFault"]
pub type PRI_5_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRI_6_3` reader - Priority of system handler 6, SecureFault"]
pub type PRI_6_3_R = crate::FieldReader;
#[doc = "Field `PRI_6_3` writer - Priority of system handler 6, SecureFault"]
pub type PRI_6_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRI_7_3` reader - Priority of system handler 7, SecureFault"]
pub type PRI_7_3_R = crate::FieldReader;
#[doc = "Field `PRI_7_3` writer - Priority of system handler 7, SecureFault"]
pub type PRI_7_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 5:7 - Priority of system handler 4, SecureFault"]
    #[inline(always)]
    pub fn pri_4_3(&self) -> PRI_4_3_R {
        PRI_4_3_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Priority of system handler 5, SecureFault"]
    #[inline(always)]
    pub fn pri_5_3(&self) -> PRI_5_3_R {
        PRI_5_3_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Priority of system handler 6, SecureFault"]
    #[inline(always)]
    pub fn pri_6_3(&self) -> PRI_6_3_R {
        PRI_6_3_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Priority of system handler 7, SecureFault"]
    #[inline(always)]
    pub fn pri_7_3(&self) -> PRI_7_3_R {
        PRI_7_3_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 5:7 - Priority of system handler 4, SecureFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_4_3(&mut self) -> PRI_4_3_W<SHPR1_SPEC> {
        PRI_4_3_W::new(self, 5)
    }
    #[doc = "Bits 13:15 - Priority of system handler 5, SecureFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_5_3(&mut self) -> PRI_5_3_W<SHPR1_SPEC> {
        PRI_5_3_W::new(self, 13)
    }
    #[doc = "Bits 21:23 - Priority of system handler 6, SecureFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_6_3(&mut self) -> PRI_6_3_W<SHPR1_SPEC> {
        PRI_6_3_W::new(self, 21)
    }
    #[doc = "Bits 29:31 - Priority of system handler 7, SecureFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_7_3(&mut self) -> PRI_7_3_W<SHPR1_SPEC> {
        PRI_7_3_W::new(self, 29)
    }
}
#[doc = "Sets or returns priority for system handlers 4 - 7  

You can [`read`](crate::Reg::read) this register and get [`shpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHPR1_SPEC;
impl crate::RegisterSpec for SHPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr1::R`](R) reader structure"]
impl crate::Readable for SHPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shpr1::W`](W) writer structure"]
impl crate::Writable for SHPR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHPR1 to value 0"]
impl crate::Resettable for SHPR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
