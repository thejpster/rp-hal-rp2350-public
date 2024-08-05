#[doc = "Register `BOOTDIS` reader"]
pub type R = crate::R<BOOTDIS_SPEC>;
#[doc = "Register `BOOTDIS` writer"]
pub type W = crate::W<BOOTDIS_SPEC>;
#[doc = "Field `NOW` reader - When powman resets the RSM, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared. The bootrom checks this flag before reading the BOOT0..3 registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data."]
pub type NOW_R = crate::BitReader;
#[doc = "Field `NOW` writer - When powman resets the RSM, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared. The bootrom checks this flag before reading the BOOT0..3 registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data."]
pub type NOW_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NEXT` reader - This flag always ORs writes into its current contents. It can be set but not cleared by software. The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the BOOT0..3 registers will be ignored following the next reset of the RSM by powman. This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it by power cycling."]
pub type NEXT_R = crate::BitReader;
#[doc = "Field `NEXT` writer - This flag always ORs writes into its current contents. It can be set but not cleared by software. The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the BOOT0..3 registers will be ignored following the next reset of the RSM by powman. This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it by power cycling."]
pub type NEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When powman resets the RSM, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared. The bootrom checks this flag before reading the BOOT0..3 registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data."]
    #[inline(always)]
    pub fn now(&self) -> NOW_R {
        NOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This flag always ORs writes into its current contents. It can be set but not cleared by software. The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the BOOT0..3 registers will be ignored following the next reset of the RSM by powman. This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it by power cycling."]
    #[inline(always)]
    pub fn next(&self) -> NEXT_R {
        NEXT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When powman resets the RSM, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared. The bootrom checks this flag before reading the BOOT0..3 registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data."]
    #[inline(always)]
    #[must_use]
    pub fn now(&mut self) -> NOW_W<BOOTDIS_SPEC> {
        NOW_W::new(self, 0)
    }
    #[doc = "Bit 1 - This flag always ORs writes into its current contents. It can be set but not cleared by software. The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the BOOT0..3 registers will be ignored following the next reset of the RSM by powman. This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it by power cycling."]
    #[inline(always)]
    #[must_use]
    pub fn next(&mut self) -> NEXT_W<BOOTDIS_SPEC> {
        NEXT_W::new(self, 1)
    }
}
#[doc = "Tell the bootrom to ignore the BOOT0..3 registers following the next RSM reset (e.g. the next core power down/up). If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by powering the core up and down. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the OTP BOOTDIS register.  

You can [`read`](crate::Reg::read) this register and get [`bootdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTDIS_SPEC;
impl crate::RegisterSpec for BOOTDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootdis::R`](R) reader structure"]
impl crate::Readable for BOOTDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootdis::W`](W) writer structure"]
impl crate::Writable for BOOTDIS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets BOOTDIS to value 0"]
impl crate::Resettable for BOOTDIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
