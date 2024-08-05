#[doc = "Register `BOOT_FLAGS0_R1` reader"]
pub type R = crate::R<BOOT_FLAGS0_R1_SPEC>;
#[doc = "Register `BOOT_FLAGS0_R1` writer"]
pub type W = crate::W<BOOT_FLAGS0_R1_SPEC>;
#[doc = "Field `BOOT_FLAGS0_R1` reader - "]
pub type BOOT_FLAGS0_R1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn boot_flags0_r1(&self) -> BOOT_FLAGS0_R1_R {
        BOOT_FLAGS0_R1_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Redundant copy of BOOT_FLAGS0  

You can [`read`](crate::Reg::read) this register and get [`boot_flags0_r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_flags0_r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_FLAGS0_R1_SPEC;
impl crate::RegisterSpec for BOOT_FLAGS0_R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_flags0_r1::R`](R) reader structure"]
impl crate::Readable for BOOT_FLAGS0_R1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot_flags0_r1::W`](W) writer structure"]
impl crate::Writable for BOOT_FLAGS0_R1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT_FLAGS0_R1 to value 0"]
impl crate::Resettable for BOOT_FLAGS0_R1_SPEC {
    const RESET_VALUE: u32 = 0;
}
