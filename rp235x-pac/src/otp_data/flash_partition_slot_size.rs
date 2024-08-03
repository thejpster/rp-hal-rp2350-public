#[doc = "Register `FLASH_PARTITION_SLOT_SIZE` reader"]
pub type R = crate::R<FLASH_PARTITION_SLOT_SIZE_SPEC>;
#[doc = "Field `FLASH_PARTITION_SLOT_SIZE` reader - "]
pub type FLASH_PARTITION_SLOT_SIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn flash_partition_slot_size(&self) -> FLASH_PARTITION_SLOT_SIZE_R {
        FLASH_PARTITION_SLOT_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)  

You can [`read`](crate::Reg::read) this register and get [`flash_partition_slot_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_PARTITION_SLOT_SIZE_SPEC;
impl crate::RegisterSpec for FLASH_PARTITION_SLOT_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_partition_slot_size::R`](R) reader structure"]
impl crate::Readable for FLASH_PARTITION_SLOT_SIZE_SPEC {}
#[doc = "`reset()` method sets FLASH_PARTITION_SLOT_SIZE to value 0"]
impl crate::Resettable for FLASH_PARTITION_SLOT_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
