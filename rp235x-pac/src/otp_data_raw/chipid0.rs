#[doc = "Register `CHIPID0` reader"]
pub type R = crate::R<CHIPID0_SPEC>;
#[doc = "Register `CHIPID0` writer"]
pub type W = crate::W<CHIPID0_SPEC>;
#[doc = "Field `CHIPID0` reader - "]
pub type CHIPID0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn chipid0(&self) -> CHIPID0_R {
        CHIPID0_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 15:0 of public device ID. (ECC) The CHIPID0..3 rows contain a 64-bit random identifier for this chip, which can be read from the USB bootloader PICOBOOT interface or from the get_sys_info ROM API. The number of random bits makes the occurrence of twins exceedingly unlikely: for example, a fleet of a hundred million devices has a 99.97% probability of no twinned IDs. This is estimated to be lower than the occurrence of process errors in the assignment of sequential random IDs, and for practical purposes CHIPID may be treated as unique.  

You can [`read`](crate::Reg::read) this register and get [`chipid0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIPID0_SPEC;
impl crate::RegisterSpec for CHIPID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid0::R`](R) reader structure"]
impl crate::Readable for CHIPID0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chipid0::W`](W) writer structure"]
impl crate::Writable for CHIPID0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHIPID0 to value 0"]
impl crate::Resettable for CHIPID0_SPEC {
    const RESET_VALUE: u32 = 0;
}
