#[doc = "Register `BOOT_FLAGS1` reader"]
pub type R = crate::R<BOOT_FLAGS1_SPEC>;
#[doc = "Register `BOOT_FLAGS1` writer"]
pub type W = crate::W<BOOT_FLAGS1_SPEC>;
#[doc = "Field `KEY_VALID` reader - Mark each of the possible boot keys as valid. The bootrom will check signatures against all valid boot keys, and ignore invalid boot keys. Each bit in this field corresponds to one of the four 256-bit boot key hashes that may be stored in page 2 of the OTP. A KEY_VALID bit is ignored if the corresponding KEY_INVALID bit is set. Boot keys are considered valid only when KEY_VALID is set and KEY_INVALID is clear. Do not mark a boot key as KEY_VALID if it does not contain a valid SHA-256 hash of your secp256k1 public key. Verify keys after programming, before setting the KEY_VALID bits -- a boot key with uncorrectable ECC faults will render your device unbootable if secure boot is enabled. Do not enable secure boot without first installing a valid key. This will render your device unbootable."]
pub type KEY_VALID_R = crate::FieldReader;
#[doc = "Field `KEY_INVALID` reader - Mark a boot key as invalid, or prevent it from ever becoming valid. The bootrom will ignore any boot key marked as invalid during secure boot signature checks. Each bit in this field corresponds to one of the four 256-bit boot key hashes that may be stored in page 2 of the OTP. When provisioning boot keys, it's recommended to mark any boot key slots you don't intend to use as KEY_INVALID, so that spurious keys can not be installed at a later time."]
pub type KEY_INVALID_R = crate::FieldReader;
#[doc = "Field `DOUBLE_TAP_DELAY` reader - Adjust how long to wait for a second reset when double tap BOOTSEL mode is enabled via DOUBLE_TAP. The minimum is 50 milliseconds, and each unit of this field adds an additional 50 milliseconds. For example, settings this field to its maximum value of 7 will cause the chip to wait for 400 milliseconds at boot to check for a second reset which requests entry to BOOTSEL mode. 200 milliseconds (DOUBLE_TAP_DELAY=3) is a good intermediate value."]
pub type DOUBLE_TAP_DELAY_R = crate::FieldReader;
#[doc = "Field `DOUBLE_TAP` reader - Enable entering BOOTSEL mode via double-tap of the RUN/RSTn pin. Adds a significant delay to boot time, as configured by DOUBLE_TAP_DELAY. This functions by waiting at startup (i.e. following a reset) to see if a second reset is applied soon afterward. The second reset is detected by the bootrom with help of the POWMAN_CHIP_RESET_DOUBLE_TAP flag, which is not reset by the external reset pin, and the bootrom enters BOOTSEL mode (NSBOOT) to await further instruction over USB or UART."]
pub type DOUBLE_TAP_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Mark each of the possible boot keys as valid. The bootrom will check signatures against all valid boot keys, and ignore invalid boot keys. Each bit in this field corresponds to one of the four 256-bit boot key hashes that may be stored in page 2 of the OTP. A KEY_VALID bit is ignored if the corresponding KEY_INVALID bit is set. Boot keys are considered valid only when KEY_VALID is set and KEY_INVALID is clear. Do not mark a boot key as KEY_VALID if it does not contain a valid SHA-256 hash of your secp256k1 public key. Verify keys after programming, before setting the KEY_VALID bits -- a boot key with uncorrectable ECC faults will render your device unbootable if secure boot is enabled. Do not enable secure boot without first installing a valid key. This will render your device unbootable."]
    #[inline(always)]
    pub fn key_valid(&self) -> KEY_VALID_R {
        KEY_VALID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Mark a boot key as invalid, or prevent it from ever becoming valid. The bootrom will ignore any boot key marked as invalid during secure boot signature checks. Each bit in this field corresponds to one of the four 256-bit boot key hashes that may be stored in page 2 of the OTP. When provisioning boot keys, it's recommended to mark any boot key slots you don't intend to use as KEY_INVALID, so that spurious keys can not be installed at a later time."]
    #[inline(always)]
    pub fn key_invalid(&self) -> KEY_INVALID_R {
        KEY_INVALID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Adjust how long to wait for a second reset when double tap BOOTSEL mode is enabled via DOUBLE_TAP. The minimum is 50 milliseconds, and each unit of this field adds an additional 50 milliseconds. For example, settings this field to its maximum value of 7 will cause the chip to wait for 400 milliseconds at boot to check for a second reset which requests entry to BOOTSEL mode. 200 milliseconds (DOUBLE_TAP_DELAY=3) is a good intermediate value."]
    #[inline(always)]
    pub fn double_tap_delay(&self) -> DOUBLE_TAP_DELAY_R {
        DOUBLE_TAP_DELAY_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Enable entering BOOTSEL mode via double-tap of the RUN/RSTn pin. Adds a significant delay to boot time, as configured by DOUBLE_TAP_DELAY. This functions by waiting at startup (i.e. following a reset) to see if a second reset is applied soon afterward. The second reset is detected by the bootrom with help of the POWMAN_CHIP_RESET_DOUBLE_TAP flag, which is not reset by the external reset pin, and the bootrom enters BOOTSEL mode (NSBOOT) to await further instruction over USB or UART."]
    #[inline(always)]
    pub fn double_tap(&self) -> DOUBLE_TAP_R {
        DOUBLE_TAP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {}
#[doc = "Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)  

You can [`read`](crate::Reg::read) this register and get [`boot_flags1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_flags1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_FLAGS1_SPEC;
impl crate::RegisterSpec for BOOT_FLAGS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_flags1::R`](R) reader structure"]
impl crate::Readable for BOOT_FLAGS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot_flags1::W`](W) writer structure"]
impl crate::Writable for BOOT_FLAGS1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT_FLAGS1 to value 0"]
impl crate::Resettable for BOOT_FLAGS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
