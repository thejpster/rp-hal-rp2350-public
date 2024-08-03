#[doc = "Register `MPU_MAIR1` reader"]
pub type R = crate::R<MPU_MAIR1_SPEC>;
#[doc = "Register `MPU_MAIR1` writer"]
pub type W = crate::W<MPU_MAIR1_SPEC>;
#[doc = "Field `ATTR4` reader - Memory attribute encoding for MPU regions with an AttrIndex of 4"]
pub type ATTR4_R = crate::FieldReader;
#[doc = "Field `ATTR4` writer - Memory attribute encoding for MPU regions with an AttrIndex of 4"]
pub type ATTR4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR5` reader - Memory attribute encoding for MPU regions with an AttrIndex of 5"]
pub type ATTR5_R = crate::FieldReader;
#[doc = "Field `ATTR5` writer - Memory attribute encoding for MPU regions with an AttrIndex of 5"]
pub type ATTR5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR6` reader - Memory attribute encoding for MPU regions with an AttrIndex of 6"]
pub type ATTR6_R = crate::FieldReader;
#[doc = "Field `ATTR6` writer - Memory attribute encoding for MPU regions with an AttrIndex of 6"]
pub type ATTR6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR7` reader - Memory attribute encoding for MPU regions with an AttrIndex of 7"]
pub type ATTR7_R = crate::FieldReader;
#[doc = "Field `ATTR7` writer - Memory attribute encoding for MPU regions with an AttrIndex of 7"]
pub type ATTR7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Memory attribute encoding for MPU regions with an AttrIndex of 4"]
    #[inline(always)]
    pub fn attr4(&self) -> ATTR4_R {
        ATTR4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Memory attribute encoding for MPU regions with an AttrIndex of 5"]
    #[inline(always)]
    pub fn attr5(&self) -> ATTR5_R {
        ATTR5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Memory attribute encoding for MPU regions with an AttrIndex of 6"]
    #[inline(always)]
    pub fn attr6(&self) -> ATTR6_R {
        ATTR6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Memory attribute encoding for MPU regions with an AttrIndex of 7"]
    #[inline(always)]
    pub fn attr7(&self) -> ATTR7_R {
        ATTR7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Memory attribute encoding for MPU regions with an AttrIndex of 4"]
    #[inline(always)]
    #[must_use]
    pub fn attr4(&mut self) -> ATTR4_W<MPU_MAIR1_SPEC> {
        ATTR4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Memory attribute encoding for MPU regions with an AttrIndex of 5"]
    #[inline(always)]
    #[must_use]
    pub fn attr5(&mut self) -> ATTR5_W<MPU_MAIR1_SPEC> {
        ATTR5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Memory attribute encoding for MPU regions with an AttrIndex of 6"]
    #[inline(always)]
    #[must_use]
    pub fn attr6(&mut self) -> ATTR6_W<MPU_MAIR1_SPEC> {
        ATTR6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Memory attribute encoding for MPU regions with an AttrIndex of 7"]
    #[inline(always)]
    #[must_use]
    pub fn attr7(&mut self) -> ATTR7_W<MPU_MAIR1_SPEC> {
        ATTR7_W::new(self, 24)
    }
}
#[doc = "Along with MPU_MAIR0, provides the memory attribute encodings corresponding to the AttrIndex values  

You can [`read`](crate::Reg::read) this register and get [`mpu_mair1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_mair1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_MAIR1_SPEC;
impl crate::RegisterSpec for MPU_MAIR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_mair1::R`](R) reader structure"]
impl crate::Readable for MPU_MAIR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_mair1::W`](W) writer structure"]
impl crate::Writable for MPU_MAIR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_MAIR1 to value 0"]
impl crate::Resettable for MPU_MAIR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
