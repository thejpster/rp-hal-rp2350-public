#[doc = "Register `USB_WHITE_LABEL_ADDR` reader"]
pub type R = crate::R<USB_WHITE_LABEL_ADDR_SPEC>;
#[doc = "Register `USB_WHITE_LABEL_ADDR` writer"]
pub type W = crate::W<USB_WHITE_LABEL_ADDR_SPEC>;
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum USB_WHITE_LABEL_ADDR_A {
    #[doc = "0: `0`"]
    INDEX_USB_DEVICE_VID_VALUE = 0,
    #[doc = "1: `1`"]
    INDEX_USB_DEVICE_PID_VALUE = 1,
    #[doc = "2: `10`"]
    INDEX_USB_DEVICE_BCD_DEVICE_VALUE = 2,
    #[doc = "3: `11`"]
    INDEX_USB_DEVICE_LANG_ID_VALUE = 3,
    #[doc = "4: `100`"]
    INDEX_USB_DEVICE_MANUFACTURER_STRDEF = 4,
    #[doc = "5: `101`"]
    INDEX_USB_DEVICE_PRODUCT_STRDEF = 5,
    #[doc = "6: `110`"]
    INDEX_USB_DEVICE_SERIAL_NUMBER_STRDEF = 6,
    #[doc = "7: `111`"]
    INDEX_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES = 7,
    #[doc = "8: `1000`"]
    INDEX_VOLUME_LABEL_STRDEF = 8,
    #[doc = "9: `1001`"]
    INDEX_SCSI_INQUIRY_VENDOR_STRDEF = 9,
    #[doc = "10: `1010`"]
    INDEX_SCSI_INQUIRY_PRODUCT_STRDEF = 10,
    #[doc = "11: `1011`"]
    INDEX_SCSI_INQUIRY_VERSION_STRDEF = 11,
    #[doc = "12: `1100`"]
    INDEX_INDEX_HTM_REDIRECT_URL_STRDEF = 12,
    #[doc = "13: `1101`"]
    INDEX_INDEX_HTM_REDIRECT_NAME_STRDEF = 13,
    #[doc = "14: `1110`"]
    INDEX_INFO_UF2_TXT_MODEL_STRDEF = 14,
    #[doc = "15: `1111`"]
    INDEX_INFO_UF2_TXT_BOARD_ID_STRDEF = 15,
}
impl From<USB_WHITE_LABEL_ADDR_A> for u32 {
    #[inline(always)]
    fn from(variant: USB_WHITE_LABEL_ADDR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USB_WHITE_LABEL_ADDR_A {
    type Ux = u32;
}
impl crate::IsEnum for USB_WHITE_LABEL_ADDR_A {}
#[doc = "Field `USB_WHITE_LABEL_ADDR` reader - "]
pub type USB_WHITE_LABEL_ADDR_R = crate::FieldReader<USB_WHITE_LABEL_ADDR_A>;
impl USB_WHITE_LABEL_ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USB_WHITE_LABEL_ADDR_A> {
        match self.bits {
            0 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_VID_VALUE),
            1 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_PID_VALUE),
            2 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_BCD_DEVICE_VALUE),
            3 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_LANG_ID_VALUE),
            4 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_MANUFACTURER_STRDEF),
            5 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_PRODUCT_STRDEF),
            6 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_SERIAL_NUMBER_STRDEF),
            7 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES),
            8 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_VOLUME_LABEL_STRDEF),
            9 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_SCSI_INQUIRY_VENDOR_STRDEF),
            10 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_SCSI_INQUIRY_PRODUCT_STRDEF),
            11 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_SCSI_INQUIRY_VERSION_STRDEF),
            12 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_INDEX_HTM_REDIRECT_URL_STRDEF),
            13 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_INDEX_HTM_REDIRECT_NAME_STRDEF),
            14 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_INFO_UF2_TXT_MODEL_STRDEF),
            15 => Some(USB_WHITE_LABEL_ADDR_A::INDEX_INFO_UF2_TXT_BOARD_ID_STRDEF),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_index_usb_device_vid_value(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_VID_VALUE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_index_usb_device_pid_value(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_PID_VALUE
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_index_usb_device_bcd_device_value(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_BCD_DEVICE_VALUE
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_index_usb_device_lang_id_value(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_LANG_ID_VALUE
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_index_usb_device_manufacturer_strdef(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_MANUFACTURER_STRDEF
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_index_usb_device_product_strdef(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_PRODUCT_STRDEF
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_index_usb_device_serial_number_strdef(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_USB_DEVICE_SERIAL_NUMBER_STRDEF
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_index_usb_config_attributes_max_power_values(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_index_volume_label_strdef(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_VOLUME_LABEL_STRDEF
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_index_scsi_inquiry_vendor_strdef(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_SCSI_INQUIRY_VENDOR_STRDEF
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_index_scsi_inquiry_product_strdef(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_SCSI_INQUIRY_PRODUCT_STRDEF
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_index_scsi_inquiry_version_strdef(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_SCSI_INQUIRY_VERSION_STRDEF
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_index_index_htm_redirect_url_strdef(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_INDEX_HTM_REDIRECT_URL_STRDEF
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_index_index_htm_redirect_name_strdef(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_INDEX_HTM_REDIRECT_NAME_STRDEF
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_index_info_uf2_txt_model_strdef(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_INFO_UF2_TXT_MODEL_STRDEF
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_index_info_uf2_txt_board_id_strdef(&self) -> bool {
        *self == USB_WHITE_LABEL_ADDR_A::INDEX_INFO_UF2_TXT_BOARD_ID_STRDEF
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn usb_white_label_addr(&self) -> USB_WHITE_LABEL_ADDR_R {
        USB_WHITE_LABEL_ADDR_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Row index of the USB_WHITE_LABEL structure within OTP (ECC) The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC). The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value. The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte. In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom. Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero. Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used. The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):  

You can [`read`](crate::Reg::read) this register and get [`usb_white_label_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_white_label_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_WHITE_LABEL_ADDR_SPEC;
impl crate::RegisterSpec for USB_WHITE_LABEL_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_white_label_addr::R`](R) reader structure"]
impl crate::Readable for USB_WHITE_LABEL_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_white_label_addr::W`](W) writer structure"]
impl crate::Writable for USB_WHITE_LABEL_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_WHITE_LABEL_ADDR to value 0"]
impl crate::Resettable for USB_WHITE_LABEL_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
