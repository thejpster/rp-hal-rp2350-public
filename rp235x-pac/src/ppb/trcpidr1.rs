#[doc = "Register `TRCPIDR1` reader"]
pub type R = crate::R<TRCPIDR1_SPEC>;
#[doc = "Register `TRCPIDR1` writer"]
pub type W = crate::W<TRCPIDR1_SPEC>;
#[doc = "Field `PART_0` reader - reads as `ImpDef"]
pub type PART_0_R = crate::FieldReader;
#[doc = "Field `DES_0` reader - reads as `ImpDef"]
pub type DES_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - reads as `ImpDef"]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - reads as `ImpDef"]
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "TRCPIDR1  

You can [`read`](crate::Reg::read) this register and get [`trcpidr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPIDR1_SPEC;
impl crate::RegisterSpec for TRCPIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpidr1::R`](R) reader structure"]
impl crate::Readable for TRCPIDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcpidr1::W`](W) writer structure"]
impl crate::Writable for TRCPIDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCPIDR1 to value 0xbd"]
impl crate::Resettable for TRCPIDR1_SPEC {
    const RESET_VALUE: u32 = 0xbd;
}
