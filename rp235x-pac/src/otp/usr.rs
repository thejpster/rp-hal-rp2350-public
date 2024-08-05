#[doc = "Register `USR` reader"]
pub type R = crate::R<USR_SPEC>;
#[doc = "Register `USR` writer"]
pub type W = crate::W<USR_SPEC>;
#[doc = "Field `DCTRL` reader - 1 enables USER interface; 0 disables USER interface (enables SBPI). This bit must be cleared before performing any SBPI access, such as when programming the OTP. The APB data read interface (USER interface) will be inaccessible during this time, and will return a bus error if any read is attempted."]
pub type DCTRL_R = crate::BitReader;
#[doc = "Field `DCTRL` writer - 1 enables USER interface; 0 disables USER interface (enables SBPI). This bit must be cleared before performing any SBPI access, such as when programming the OTP. The APB data read interface (USER interface) will be inaccessible during this time, and will return a bus error if any read is attempted."]
pub type DCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD` reader - Power-down; 1 disables current reference. Must be 0 to read data from the OTP."]
pub type PD_R = crate::BitReader;
#[doc = "Field `PD` writer - Power-down; 1 disables current reference. Must be 0 to read data from the OTP."]
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1 enables USER interface; 0 disables USER interface (enables SBPI). This bit must be cleared before performing any SBPI access, such as when programming the OTP. The APB data read interface (USER interface) will be inaccessible during this time, and will return a bus error if any read is attempted."]
    #[inline(always)]
    pub fn dctrl(&self) -> DCTRL_R {
        DCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Power-down; 1 disables current reference. Must be 0 to read data from the OTP."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 enables USER interface; 0 disables USER interface (enables SBPI). This bit must be cleared before performing any SBPI access, such as when programming the OTP. The APB data read interface (USER interface) will be inaccessible during this time, and will return a bus error if any read is attempted."]
    #[inline(always)]
    #[must_use]
    pub fn dctrl(&mut self) -> DCTRL_W<USR_SPEC> {
        DCTRL_W::new(self, 0)
    }
    #[doc = "Bit 4 - Power-down; 1 disables current reference. Must be 0 to read data from the OTP."]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<USR_SPEC> {
        PD_W::new(self, 4)
    }
}
#[doc = "Controls for APB data read interface (USER interface)  

You can [`read`](crate::Reg::read) this register and get [`usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USR_SPEC;
impl crate::RegisterSpec for USR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usr::R`](R) reader structure"]
impl crate::Readable for USR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usr::W`](W) writer structure"]
impl crate::Writable for USR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USR to value 0x01"]
impl crate::Resettable for USR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
