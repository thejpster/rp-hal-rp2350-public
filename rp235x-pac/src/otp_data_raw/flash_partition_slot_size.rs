#[doc = "Register `FLASH_PARTITION_SLOT_SIZE` reader"]
pub type R = crate::R<FLASH_PARTITION_SLOT_SIZE_SPEC>;
#[doc = "Register `FLASH_PARTITION_SLOT_SIZE` writer"]
pub type W = crate::W<FLASH_PARTITION_SLOT_SIZE_SPEC>;
#[doc = "Field `FLASH_PARTITION_SLOT_SIZE` reader - "]
pub type FLASH_PARTITION_SLOT_SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn flash_partition_slot_size(&self) -> FLASH_PARTITION_SLOT_SIZE_R {
        FLASH_PARTITION_SLOT_SIZE_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)  

You can [`read`](crate::Reg::read) this register and get [`flash_partition_slot_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_partition_slot_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_PARTITION_SLOT_SIZE_SPEC;
impl crate::RegisterSpec for FLASH_PARTITION_SLOT_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_partition_slot_size::R`](R) reader structure"]
impl crate::Readable for FLASH_PARTITION_SLOT_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_partition_slot_size::W`](W) writer structure"]
impl crate::Writable for FLASH_PARTITION_SLOT_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_PARTITION_SLOT_SIZE to value 0"]
impl crate::Resettable for FLASH_PARTITION_SLOT_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
