#[doc = "Register `IC_CLR_STOP_DET` reader"]
pub type R = crate::R<IC_CLR_STOP_DET_SPEC>;
#[doc = "Register `IC_CLR_STOP_DET` writer"]
pub type W = crate::W<IC_CLR_STOP_DET_SPEC>;
#[doc = "Field `CLR_STOP_DET` reader - Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
pub type CLR_STOP_DET_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    #[inline(always)]
    pub fn clr_stop_det(&self) -> CLR_STOP_DET_R {
        CLR_STOP_DET_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Clear STOP_DET Interrupt Register  

You can [`read`](crate::Reg::read) this register and get [`ic_clr_stop_det::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_clr_stop_det::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CLR_STOP_DET_SPEC;
impl crate::RegisterSpec for IC_CLR_STOP_DET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_stop_det::R`](R) reader structure"]
impl crate::Readable for IC_CLR_STOP_DET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_clr_stop_det::W`](W) writer structure"]
impl crate::Writable for IC_CLR_STOP_DET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_CLR_STOP_DET to value 0"]
impl crate::Resettable for IC_CLR_STOP_DET_SPEC {
    const RESET_VALUE: u32 = 0;
}
