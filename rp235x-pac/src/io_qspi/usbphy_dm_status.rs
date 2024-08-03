#[doc = "Register `USBPHY_DM_STATUS` reader"]
pub type R = crate::R<USBPHY_DM_STATUS_SPEC>;
#[doc = "Field `OUTTOPAD` reader - output signal to pad after register overide is applied"]
pub type OUTTOPAD_R = crate::BitReader;
#[doc = "Field `OETOPAD` reader - output enable to pad after register overide is applied"]
pub type OETOPAD_R = crate::BitReader;
#[doc = "Field `INFROMPAD` reader - input signal from pad, before filtering and override are applied"]
pub type INFROMPAD_R = crate::BitReader;
#[doc = "Field `IRQTOPROC` reader - interrupt to processors, after override is applied"]
pub type IRQTOPROC_R = crate::BitReader;
impl R {
    #[doc = "Bit 9 - output signal to pad after register overide is applied"]
    #[inline(always)]
    pub fn outtopad(&self) -> OUTTOPAD_R {
        OUTTOPAD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - output enable to pad after register overide is applied"]
    #[inline(always)]
    pub fn oetopad(&self) -> OETOPAD_R {
        OETOPAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - input signal from pad, before filtering and override are applied"]
    #[inline(always)]
    pub fn infrompad(&self) -> INFROMPAD_R {
        INFROMPAD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 26 - interrupt to processors, after override is applied"]
    #[inline(always)]
    pub fn irqtoproc(&self) -> IRQTOPROC_R {
        IRQTOPROC_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`usbphy_dm_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPHY_DM_STATUS_SPEC;
impl crate::RegisterSpec for USBPHY_DM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphy_dm_status::R`](R) reader structure"]
impl crate::Readable for USBPHY_DM_STATUS_SPEC {}
#[doc = "`reset()` method sets USBPHY_DM_STATUS to value 0"]
impl crate::Resettable for USBPHY_DM_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
