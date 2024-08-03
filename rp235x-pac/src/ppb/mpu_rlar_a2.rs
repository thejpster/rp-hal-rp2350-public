#[doc = "Register `MPU_RLAR_A2` reader"]
pub type R = crate::R<MPU_RLAR_A2_SPEC>;
#[doc = "Register `MPU_RLAR_A2` writer"]
pub type W = crate::W<MPU_RLAR_A2_SPEC>;
#[doc = "Field `EN` reader - Region enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Region enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTRINDX` reader - Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
pub type ATTRINDX_R = crate::FieldReader;
#[doc = "Field `ATTRINDX` writer - Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
pub type ATTRINDX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LIMIT` reader - Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
pub type LIMIT_R = crate::FieldReader<u32>;
#[doc = "Field `LIMIT` writer - Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
pub type LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - Region enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
    #[inline(always)]
    pub fn attrindx(&self) -> ATTRINDX_R {
        ATTRINDX_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 5:31 - Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<MPU_RLAR_A2_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
    #[inline(always)]
    #[must_use]
    pub fn attrindx(&mut self) -> ATTRINDX_W<MPU_RLAR_A2_SPEC> {
        ATTRINDX_W::new(self, 1)
    }
    #[doc = "Bits 5:31 - Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LIMIT_W<MPU_RLAR_A2_SPEC> {
        LIMIT_W::new(self, 5)
    }
}
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\]) `FTSSS  

You can [`read`](crate::Reg::read) this register and get [`mpu_rlar_a2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_rlar_a2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_RLAR_A2_SPEC;
impl crate::RegisterSpec for MPU_RLAR_A2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rlar_a2::R`](R) reader structure"]
impl crate::Readable for MPU_RLAR_A2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_rlar_a2::W`](W) writer structure"]
impl crate::Writable for MPU_RLAR_A2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RLAR_A2 to value 0"]
impl crate::Resettable for MPU_RLAR_A2_SPEC {
    const RESET_VALUE: u32 = 0;
}
