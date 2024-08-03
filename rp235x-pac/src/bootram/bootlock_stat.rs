#[doc = "Register `BOOTLOCK_STAT` reader"]
pub type R = crate::R<BOOTLOCK_STAT_SPEC>;
#[doc = "Register `BOOTLOCK_STAT` writer"]
pub type W = crate::W<BOOTLOCK_STAT_SPEC>;
#[doc = "Field `BOOTLOCK_STAT` reader - "]
pub type BOOTLOCK_STAT_R = crate::FieldReader;
#[doc = "Field `BOOTLOCK_STAT` writer - "]
pub type BOOTLOCK_STAT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bootlock_stat(&self) -> BOOTLOCK_STAT_R {
        BOOTLOCK_STAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn bootlock_stat(&mut self) -> BOOTLOCK_STAT_W<BOOTLOCK_STAT_SPEC> {
        BOOTLOCK_STAT_W::new(self, 0)
    }
}
#[doc = "Bootlock status register. 1=unclaimed, 0=claimed. These locks function identically to the SIO spinlocks, but are reserved for bootrom use.  

You can [`read`](crate::Reg::read) this register and get [`bootlock_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTLOCK_STAT_SPEC;
impl crate::RegisterSpec for BOOTLOCK_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootlock_stat::R`](R) reader structure"]
impl crate::Readable for BOOTLOCK_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootlock_stat::W`](W) writer structure"]
impl crate::Writable for BOOTLOCK_STAT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTLOCK_STAT to value 0xff"]
impl crate::Resettable for BOOTLOCK_STAT_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
