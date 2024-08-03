#[doc = "Register `MPU_MAIR0` reader"]
pub type R = crate::R<MPU_MAIR0_SPEC>;
#[doc = "Register `MPU_MAIR0` writer"]
pub type W = crate::W<MPU_MAIR0_SPEC>;
#[doc = "Field `ATTR0` reader - Memory attribute encoding for MPU regions with an AttrIndex of 0"]
pub type ATTR0_R = crate::FieldReader;
#[doc = "Field `ATTR0` writer - Memory attribute encoding for MPU regions with an AttrIndex of 0"]
pub type ATTR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR1` reader - Memory attribute encoding for MPU regions with an AttrIndex of 1"]
pub type ATTR1_R = crate::FieldReader;
#[doc = "Field `ATTR1` writer - Memory attribute encoding for MPU regions with an AttrIndex of 1"]
pub type ATTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR2` reader - Memory attribute encoding for MPU regions with an AttrIndex of 2"]
pub type ATTR2_R = crate::FieldReader;
#[doc = "Field `ATTR2` writer - Memory attribute encoding for MPU regions with an AttrIndex of 2"]
pub type ATTR2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTR3` reader - Memory attribute encoding for MPU regions with an AttrIndex of 3"]
pub type ATTR3_R = crate::FieldReader;
#[doc = "Field `ATTR3` writer - Memory attribute encoding for MPU regions with an AttrIndex of 3"]
pub type ATTR3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Memory attribute encoding for MPU regions with an AttrIndex of 0"]
    #[inline(always)]
    pub fn attr0(&self) -> ATTR0_R {
        ATTR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Memory attribute encoding for MPU regions with an AttrIndex of 1"]
    #[inline(always)]
    pub fn attr1(&self) -> ATTR1_R {
        ATTR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Memory attribute encoding for MPU regions with an AttrIndex of 2"]
    #[inline(always)]
    pub fn attr2(&self) -> ATTR2_R {
        ATTR2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Memory attribute encoding for MPU regions with an AttrIndex of 3"]
    #[inline(always)]
    pub fn attr3(&self) -> ATTR3_R {
        ATTR3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Memory attribute encoding for MPU regions with an AttrIndex of 0"]
    #[inline(always)]
    #[must_use]
    pub fn attr0(&mut self) -> ATTR0_W<MPU_MAIR0_SPEC> {
        ATTR0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Memory attribute encoding for MPU regions with an AttrIndex of 1"]
    #[inline(always)]
    #[must_use]
    pub fn attr1(&mut self) -> ATTR1_W<MPU_MAIR0_SPEC> {
        ATTR1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Memory attribute encoding for MPU regions with an AttrIndex of 2"]
    #[inline(always)]
    #[must_use]
    pub fn attr2(&mut self) -> ATTR2_W<MPU_MAIR0_SPEC> {
        ATTR2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Memory attribute encoding for MPU regions with an AttrIndex of 3"]
    #[inline(always)]
    #[must_use]
    pub fn attr3(&mut self) -> ATTR3_W<MPU_MAIR0_SPEC> {
        ATTR3_W::new(self, 24)
    }
}
#[doc = "Along with MPU_MAIR1, provides the memory attribute encodings corresponding to the AttrIndex values  

You can [`read`](crate::Reg::read) this register and get [`mpu_mair0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_mair0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_MAIR0_SPEC;
impl crate::RegisterSpec for MPU_MAIR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_mair0::R`](R) reader structure"]
impl crate::Readable for MPU_MAIR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_mair0::W`](W) writer structure"]
impl crate::Writable for MPU_MAIR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_MAIR0 to value 0"]
impl crate::Resettable for MPU_MAIR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
