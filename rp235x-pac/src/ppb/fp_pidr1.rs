#[doc = "Register `FP_PIDR1` reader"]
pub type R = crate::R<FP_PIDR1_SPEC>;
#[doc = "Register `FP_PIDR1` writer"]
pub type W = crate::W<FP_PIDR1_SPEC>;
#[doc = "Field `PART_1` reader - See CoreSight Architecture Specification"]
pub type PART_1_R = crate::FieldReader;
#[doc = "Field `DES_0` reader - See CoreSight Architecture Specification"]
pub type DES_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn part_1(&self) -> PART_1_R {
        PART_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_PIDR1_SPEC;
impl crate::RegisterSpec for FP_PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_pidr1::R`](R) reader structure"]
impl crate::Readable for FP_PIDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fp_pidr1::W`](W) writer structure"]
impl crate::Writable for FP_PIDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FP_PIDR1 to value 0xbd"]
impl crate::Resettable for FP_PIDR1_SPEC {
    const RESET_VALUE: u32 = 0xbd;
}
