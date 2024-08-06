#[doc = "Register `USB_BOOT_FLAGS` reader"]
pub type R = crate::R<USB_BOOT_FLAGS_SPEC>;
#[doc = "Register `USB_BOOT_FLAGS` writer"]
pub type W = crate::W<USB_BOOT_FLAGS_SPEC>;
#[doc = "Field `WL_USB_DEVICE_VID_VALUE_VALID` reader - valid flag for USB_DEVICE_VID_VALUE entry of the USB_WHITE_LABEL struct (index 0)"]
pub type WL_USB_DEVICE_VID_VALUE_VALID_R = crate::BitReader;
#[doc = "Field `WL_USB_DEVICE_PID_VALUE_VALID` reader - valid flag for USB_DEVICE_PID_VALUE entry of the USB_WHITE_LABEL struct (index 1)"]
pub type WL_USB_DEVICE_PID_VALUE_VALID_R = crate::BitReader;
#[doc = "Field `WL_USB_DEVICE_SERIAL_NUMBER_VALUE_VALID` reader - valid flag for USB_DEVICE_BCD_DEVICEVALUE entry of the USB_WHITE_LABEL struct (index 2)"]
pub type WL_USB_DEVICE_SERIAL_NUMBER_VALUE_VALID_R = crate::BitReader;
#[doc = "Field `WL_USB_DEVICE_LANG_ID_VALUE_VALID` reader - valid flag for USB_DEVICE_LANG_ID_VALUE entry of the USB_WHITE_LABEL struct (index 3)"]
pub type WL_USB_DEVICE_LANG_ID_VALUE_VALID_R = crate::BitReader;
#[doc = "Field `WL_USB_DEVICE_MANUFACTURER_STRDEF_VALID` reader - valid flag for USB_DEVICE_MANUFACTURER_STRDEF entry of the USB_WHITE_LABEL struct (index 4)"]
pub type WL_USB_DEVICE_MANUFACTURER_STRDEF_VALID_R = crate::BitReader;
#[doc = "Field `WL_USB_DEVICE_PRODUCT_STRDEF_VALID` reader - valid flag for USB_DEVICE_PRODUCT_STRDEF entry of the USB_WHITE_LABEL struct (index 5)"]
pub type WL_USB_DEVICE_PRODUCT_STRDEF_VALID_R = crate::BitReader;
#[doc = "Field `WL_USB_DEVICE_SERIAL_NUMBER_STRDEF_VALID` reader - valid flag for USB_DEVICE_SERIAL_NUMBER_STRDEF entry of the USB_WHITE_LABEL struct (index 6)"]
pub type WL_USB_DEVICE_SERIAL_NUMBER_STRDEF_VALID_R = crate::BitReader;
#[doc = "Field `WL_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES_VALID` reader - valid flag for USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES entry of the USB_WHITE_LABEL struct (index 7)"]
pub type WL_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES_VALID_R = crate::BitReader;
#[doc = "Field `WL_VOLUME_LABEL_STRDEF_VALID` reader - valid flag for VOLUME_LABEL_STRDEF entry of the USB_WHITE_LABEL struct (index 8)"]
pub type WL_VOLUME_LABEL_STRDEF_VALID_R = crate::BitReader;
#[doc = "Field `WL_SCSI_INQUIRY_VENDOR_STRDEF_VALID` reader - valid flag for SCSI_INQUIRY_VENDOR_STRDEF entry of the USB_WHITE_LABEL struct (index 9)"]
pub type WL_SCSI_INQUIRY_VENDOR_STRDEF_VALID_R = crate::BitReader;
#[doc = "Field `WL_SCSI_INQUIRY_PRODUCT_STRDEF_VALID` reader - valid flag for SCSI_INQUIRY_PRODUCT_STRDEF entry of the USB_WHITE_LABEL struct (index 10)"]
pub type WL_SCSI_INQUIRY_PRODUCT_STRDEF_VALID_R = crate::BitReader;
#[doc = "Field `WL_SCSI_INQUIRY_VERSION_STRDEF_VALID` reader - valid flag for SCSI_INQUIRY_VERSION_STRDEF entry of the USB_WHITE_LABEL struct (index 11)"]
pub type WL_SCSI_INQUIRY_VERSION_STRDEF_VALID_R = crate::BitReader;
#[doc = "Field `WL_INDEX_HTM_REDIRECT_URL_STRDEF_VALID` reader - valid flag for INDEX_HTM_REDIRECT_URL_STRDEF entry of the USB_WHITE_LABEL struct (index 12)"]
pub type WL_INDEX_HTM_REDIRECT_URL_STRDEF_VALID_R = crate::BitReader;
#[doc = "Field `WL_INDEX_HTM_REDIRECT_NAME_STRDEF_VALID` reader - valid flag for INDEX_HTM_REDIRECT_NAME_STRDEF entry of the USB_WHITE_LABEL struct (index 13)"]
pub type WL_INDEX_HTM_REDIRECT_NAME_STRDEF_VALID_R = crate::BitReader;
#[doc = "Field `WL_INFO_UF2_TXT_MODEL_STRDEF_VALID` reader - valid flag for INFO_UF2_TXT_MODEL_STRDEF entry of the USB_WHITE_LABEL struct (index 14)"]
pub type WL_INFO_UF2_TXT_MODEL_STRDEF_VALID_R = crate::BitReader;
#[doc = "Field `WL_INFO_UF2_TXT_BOARD_ID_STRDEF_VALID` reader - valid flag for the USB_WHITE_LABEL_ADDR field"]
pub type WL_INFO_UF2_TXT_BOARD_ID_STRDEF_VALID_R = crate::BitReader;
#[doc = "Field `WHITE_LABEL_ADDR_VALID` reader - valid flag for INFO_UF2_TXT_BOARD_ID_STRDEF entry of the USB_WHITE_LABEL struct (index 15)"]
pub type WHITE_LABEL_ADDR_VALID_R = crate::BitReader;
#[doc = "Field `DP_DM_SWAP` reader - Swap DM/DP during USB boot, to support board layouts with mirrored USB routing (deliberate or accidental)."]
pub type DP_DM_SWAP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - valid flag for USB_DEVICE_VID_VALUE entry of the USB_WHITE_LABEL struct (index 0)"]
    #[inline(always)]
    pub fn wl_usb_device_vid_value_valid(&self) -> WL_USB_DEVICE_VID_VALUE_VALID_R {
        WL_USB_DEVICE_VID_VALUE_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - valid flag for USB_DEVICE_PID_VALUE entry of the USB_WHITE_LABEL struct (index 1)"]
    #[inline(always)]
    pub fn wl_usb_device_pid_value_valid(&self) -> WL_USB_DEVICE_PID_VALUE_VALID_R {
        WL_USB_DEVICE_PID_VALUE_VALID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - valid flag for USB_DEVICE_BCD_DEVICEVALUE entry of the USB_WHITE_LABEL struct (index 2)"]
    #[inline(always)]
    pub fn wl_usb_device_serial_number_value_valid(
        &self,
    ) -> WL_USB_DEVICE_SERIAL_NUMBER_VALUE_VALID_R {
        WL_USB_DEVICE_SERIAL_NUMBER_VALUE_VALID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - valid flag for USB_DEVICE_LANG_ID_VALUE entry of the USB_WHITE_LABEL struct (index 3)"]
    #[inline(always)]
    pub fn wl_usb_device_lang_id_value_valid(&self) -> WL_USB_DEVICE_LANG_ID_VALUE_VALID_R {
        WL_USB_DEVICE_LANG_ID_VALUE_VALID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - valid flag for USB_DEVICE_MANUFACTURER_STRDEF entry of the USB_WHITE_LABEL struct (index 4)"]
    #[inline(always)]
    pub fn wl_usb_device_manufacturer_strdef_valid(
        &self,
    ) -> WL_USB_DEVICE_MANUFACTURER_STRDEF_VALID_R {
        WL_USB_DEVICE_MANUFACTURER_STRDEF_VALID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - valid flag for USB_DEVICE_PRODUCT_STRDEF entry of the USB_WHITE_LABEL struct (index 5)"]
    #[inline(always)]
    pub fn wl_usb_device_product_strdef_valid(&self) -> WL_USB_DEVICE_PRODUCT_STRDEF_VALID_R {
        WL_USB_DEVICE_PRODUCT_STRDEF_VALID_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - valid flag for USB_DEVICE_SERIAL_NUMBER_STRDEF entry of the USB_WHITE_LABEL struct (index 6)"]
    #[inline(always)]
    pub fn wl_usb_device_serial_number_strdef_valid(
        &self,
    ) -> WL_USB_DEVICE_SERIAL_NUMBER_STRDEF_VALID_R {
        WL_USB_DEVICE_SERIAL_NUMBER_STRDEF_VALID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - valid flag for USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES entry of the USB_WHITE_LABEL struct (index 7)"]
    #[inline(always)]
    pub fn wl_usb_config_attributes_max_power_values_valid(
        &self,
    ) -> WL_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES_VALID_R {
        WL_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES_VALID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - valid flag for VOLUME_LABEL_STRDEF entry of the USB_WHITE_LABEL struct (index 8)"]
    #[inline(always)]
    pub fn wl_volume_label_strdef_valid(&self) -> WL_VOLUME_LABEL_STRDEF_VALID_R {
        WL_VOLUME_LABEL_STRDEF_VALID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - valid flag for SCSI_INQUIRY_VENDOR_STRDEF entry of the USB_WHITE_LABEL struct (index 9)"]
    #[inline(always)]
    pub fn wl_scsi_inquiry_vendor_strdef_valid(&self) -> WL_SCSI_INQUIRY_VENDOR_STRDEF_VALID_R {
        WL_SCSI_INQUIRY_VENDOR_STRDEF_VALID_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - valid flag for SCSI_INQUIRY_PRODUCT_STRDEF entry of the USB_WHITE_LABEL struct (index 10)"]
    #[inline(always)]
    pub fn wl_scsi_inquiry_product_strdef_valid(&self) -> WL_SCSI_INQUIRY_PRODUCT_STRDEF_VALID_R {
        WL_SCSI_INQUIRY_PRODUCT_STRDEF_VALID_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - valid flag for SCSI_INQUIRY_VERSION_STRDEF entry of the USB_WHITE_LABEL struct (index 11)"]
    #[inline(always)]
    pub fn wl_scsi_inquiry_version_strdef_valid(&self) -> WL_SCSI_INQUIRY_VERSION_STRDEF_VALID_R {
        WL_SCSI_INQUIRY_VERSION_STRDEF_VALID_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - valid flag for INDEX_HTM_REDIRECT_URL_STRDEF entry of the USB_WHITE_LABEL struct (index 12)"]
    #[inline(always)]
    pub fn wl_index_htm_redirect_url_strdef_valid(
        &self,
    ) -> WL_INDEX_HTM_REDIRECT_URL_STRDEF_VALID_R {
        WL_INDEX_HTM_REDIRECT_URL_STRDEF_VALID_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - valid flag for INDEX_HTM_REDIRECT_NAME_STRDEF entry of the USB_WHITE_LABEL struct (index 13)"]
    #[inline(always)]
    pub fn wl_index_htm_redirect_name_strdef_valid(
        &self,
    ) -> WL_INDEX_HTM_REDIRECT_NAME_STRDEF_VALID_R {
        WL_INDEX_HTM_REDIRECT_NAME_STRDEF_VALID_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - valid flag for INFO_UF2_TXT_MODEL_STRDEF entry of the USB_WHITE_LABEL struct (index 14)"]
    #[inline(always)]
    pub fn wl_info_uf2_txt_model_strdef_valid(&self) -> WL_INFO_UF2_TXT_MODEL_STRDEF_VALID_R {
        WL_INFO_UF2_TXT_MODEL_STRDEF_VALID_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - valid flag for the USB_WHITE_LABEL_ADDR field"]
    #[inline(always)]
    pub fn wl_info_uf2_txt_board_id_strdef_valid(&self) -> WL_INFO_UF2_TXT_BOARD_ID_STRDEF_VALID_R {
        WL_INFO_UF2_TXT_BOARD_ID_STRDEF_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 22 - valid flag for INFO_UF2_TXT_BOARD_ID_STRDEF entry of the USB_WHITE_LABEL struct (index 15)"]
    #[inline(always)]
    pub fn white_label_addr_valid(&self) -> WHITE_LABEL_ADDR_VALID_R {
        WHITE_LABEL_ADDR_VALID_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Swap DM/DP during USB boot, to support board layouts with mirrored USB routing (deliberate or accidental)."]
    #[inline(always)]
    pub fn dp_dm_swap(&self) -> DP_DM_SWAP_R {
        DP_DM_SWAP_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {}
#[doc = "USB boot specific feature flags (RBIT-3)  

You can [`read`](crate::Reg::read) this register and get [`usb_boot_flags::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_boot_flags::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_BOOT_FLAGS_SPEC;
impl crate::RegisterSpec for USB_BOOT_FLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_boot_flags::R`](R) reader structure"]
impl crate::Readable for USB_BOOT_FLAGS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_boot_flags::W`](W) writer structure"]
impl crate::Writable for USB_BOOT_FLAGS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_BOOT_FLAGS to value 0"]
impl crate::Resettable for USB_BOOT_FLAGS_SPEC {
    const RESET_VALUE: u32 = 0;
}
