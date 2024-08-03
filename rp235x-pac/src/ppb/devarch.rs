#[doc = "Register `DEVARCH` reader"]
pub type R = crate::R<DEVARCH_SPEC>;
#[doc = "Field `ARCHID` reader - Indicates the component"]
pub type ARCHID_R = crate::FieldReader<u16>;
#[doc = "Field `REVISION` reader - Indicates the architecture revision"]
pub type REVISION_R = crate::FieldReader;
#[doc = "Field `PRESENT` reader - Indicates whether the DEVARCH register is present"]
pub type PRESENT_R = crate::BitReader;
#[doc = "Field `ARCHITECT` reader - Indicates the component architect"]
pub type ARCHITECT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Indicates the component"]
    #[inline(always)]
    pub fn archid(&self) -> ARCHID_R {
        ARCHID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Indicates the architecture revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Indicates whether the DEVARCH register is present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - Indicates the component architect"]
    #[inline(always)]
    pub fn architect(&self) -> ARCHITECT_R {
        ARCHITECT_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "Device Architecture register  

You can [`read`](crate::Reg::read) this register and get [`devarch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVARCH_SPEC;
impl crate::RegisterSpec for DEVARCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devarch::R`](R) reader structure"]
impl crate::Readable for DEVARCH_SPEC {}
#[doc = "`reset()` method sets DEVARCH to value 0x4770_1a14"]
impl crate::Resettable for DEVARCH_SPEC {
    const RESET_VALUE: u32 = 0x4770_1a14;
}
