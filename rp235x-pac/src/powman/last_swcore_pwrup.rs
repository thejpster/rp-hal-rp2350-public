#[doc = "Register `LAST_SWCORE_PWRUP` reader"]
pub type R = crate::R<LAST_SWCORE_PWRUP_SPEC>;
#[doc = "Register `LAST_SWCORE_PWRUP` writer"]
pub type W = crate::W<LAST_SWCORE_PWRUP_SPEC>;
#[doc = "Field `LAST_SWCORE_PWRUP` reader - "]
pub type LAST_SWCORE_PWRUP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn last_swcore_pwrup(&self) -> LAST_SWCORE_PWRUP_R {
        LAST_SWCORE_PWRUP_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {}
#[doc = "Indicates which pwrup source triggered the last switched-core power up 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup  

You can [`read`](crate::Reg::read) this register and get [`last_swcore_pwrup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`last_swcore_pwrup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LAST_SWCORE_PWRUP_SPEC;
impl crate::RegisterSpec for LAST_SWCORE_PWRUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`last_swcore_pwrup::R`](R) reader structure"]
impl crate::Readable for LAST_SWCORE_PWRUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`last_swcore_pwrup::W`](W) writer structure"]
impl crate::Writable for LAST_SWCORE_PWRUP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAST_SWCORE_PWRUP to value 0"]
impl crate::Resettable for LAST_SWCORE_PWRUP_SPEC {
    const RESET_VALUE: u32 = 0;
}
