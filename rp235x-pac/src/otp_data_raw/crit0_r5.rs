#[doc = "Register `CRIT0_R5` reader"]
pub type R = crate::R<CRIT0_R5_SPEC>;
#[doc = "Register `CRIT0_R5` writer"]
pub type W = crate::W<CRIT0_R5_SPEC>;
#[doc = "Field `CRIT0_R5` reader - "]
pub type CRIT0_R5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn crit0_r5(&self) -> CRIT0_R5_R {
        CRIT0_R5_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Redundant copy of CRIT0  

You can [`read`](crate::Reg::read) this register and get [`crit0_r5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crit0_r5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRIT0_R5_SPEC;
impl crate::RegisterSpec for CRIT0_R5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crit0_r5::R`](R) reader structure"]
impl crate::Readable for CRIT0_R5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crit0_r5::W`](W) writer structure"]
impl crate::Writable for CRIT0_R5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRIT0_R5 to value 0"]
impl crate::Resettable for CRIT0_R5_SPEC {
    const RESET_VALUE: u32 = 0;
}
