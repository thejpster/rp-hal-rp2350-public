#[doc = "Register `TRCCIDR2` reader"]
pub type R = crate::R<TRCCIDR2_SPEC>;
#[doc = "Register `TRCCIDR2` writer"]
pub type W = crate::W<TRCCIDR2_SPEC>;
#[doc = "Field `PRMBL_2` reader - reads as 0b00000101"]
pub type PRMBL_2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - reads as 0b00000101"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "TRCCIDR2  

You can [`read`](crate::Reg::read) this register and get [`trccidr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccidr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCCIDR2_SPEC;
impl crate::RegisterSpec for TRCCIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trccidr2::R`](R) reader structure"]
impl crate::Readable for TRCCIDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trccidr2::W`](W) writer structure"]
impl crate::Writable for TRCCIDR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCCIDR2 to value 0x05"]
impl crate::Resettable for TRCCIDR2_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
