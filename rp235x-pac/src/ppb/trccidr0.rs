#[doc = "Register `TRCCIDR0` reader"]
pub type R = crate::R<TRCCIDR0_SPEC>;
#[doc = "Register `TRCCIDR0` writer"]
pub type W = crate::W<TRCCIDR0_SPEC>;
#[doc = "Field `PRMBL_0` reader - reads as 0b00001101"]
pub type PRMBL_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - reads as 0b00001101"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "TRCCIDR0  

You can [`read`](crate::Reg::read) this register and get [`trccidr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccidr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCCIDR0_SPEC;
impl crate::RegisterSpec for TRCCIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trccidr0::R`](R) reader structure"]
impl crate::Readable for TRCCIDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trccidr0::W`](W) writer structure"]
impl crate::Writable for TRCCIDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCCIDR0 to value 0x0d"]
impl crate::Resettable for TRCCIDR0_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
