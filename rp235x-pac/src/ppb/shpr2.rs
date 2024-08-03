#[doc = "Register `SHPR2` reader"]
pub type R = crate::R<SHPR2_SPEC>;
#[doc = "Register `SHPR2` writer"]
pub type W = crate::W<SHPR2_SPEC>;
#[doc = "Field `PRI_8` reader - Reserved, RES0"]
pub type PRI_8_R = crate::FieldReader;
#[doc = "Field `PRI_9` reader - Reserved, RES0"]
pub type PRI_9_R = crate::FieldReader;
#[doc = "Field `PRI_10` reader - Reserved, RES0"]
pub type PRI_10_R = crate::FieldReader;
#[doc = "Field `PRI_11_3` reader - Priority of system handler 11, SecureFault"]
pub type PRI_11_3_R = crate::FieldReader;
#[doc = "Field `PRI_11_3` writer - Priority of system handler 11, SecureFault"]
pub type PRI_11_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Reserved, RES0"]
    #[inline(always)]
    pub fn pri_8(&self) -> PRI_8_R {
        PRI_8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved, RES0"]
    #[inline(always)]
    pub fn pri_9(&self) -> PRI_9_R {
        PRI_9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved, RES0"]
    #[inline(always)]
    pub fn pri_10(&self) -> PRI_10_R {
        PRI_10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 29:31 - Priority of system handler 11, SecureFault"]
    #[inline(always)]
    pub fn pri_11_3(&self) -> PRI_11_3_R {
        PRI_11_3_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - Priority of system handler 11, SecureFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_11_3(&mut self) -> PRI_11_3_W<SHPR2_SPEC> {
        PRI_11_3_W::new(self, 29)
    }
}
#[doc = "Sets or returns priority for system handlers 8 - 11  

You can [`read`](crate::Reg::read) this register and get [`shpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHPR2_SPEC;
impl crate::RegisterSpec for SHPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr2::R`](R) reader structure"]
impl crate::Readable for SHPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shpr2::W`](W) writer structure"]
impl crate::Writable for SHPR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHPR2 to value 0"]
impl crate::Resettable for SHPR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
