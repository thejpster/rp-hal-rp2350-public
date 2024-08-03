#[doc = "Register `FP_DEVARCH` reader"]
pub type R = crate::R<FP_DEVARCH_SPEC>;
#[doc = "Field `ARCHPART` reader - Defines the architecture of the component"]
pub type ARCHPART_R = crate::FieldReader<u16>;
#[doc = "Field `ARCHVER` reader - Defines the architecture version of the component"]
pub type ARCHVER_R = crate::FieldReader;
#[doc = "Field `REVISION` reader - Defines the architecture revision of the component"]
pub type REVISION_R = crate::FieldReader;
#[doc = "Field `PRESENT` reader - Defines that the DEVARCH register is present"]
pub type PRESENT_R = crate::BitReader;
#[doc = "Field `ARCHITECT` reader - Defines the architect of the component. Bits \\[31:28\\]
are the JEP106 continuation code (JEP106 bank ID, minus 1) and bits \\[27:21\\]
are the JEP106 ID code."]
pub type ARCHITECT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Defines the architecture of the component"]
    #[inline(always)]
    pub fn archpart(&self) -> ARCHPART_R {
        ARCHPART_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Defines the architecture version of the component"]
    #[inline(always)]
    pub fn archver(&self) -> ARCHVER_R {
        ARCHVER_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Defines the architecture revision of the component"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Defines that the DEVARCH register is present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - Defines the architect of the component. Bits \\[31:28\\]
are the JEP106 continuation code (JEP106 bank ID, minus 1) and bits \\[27:21\\]
are the JEP106 ID code."]
    #[inline(always)]
    pub fn architect(&self) -> ARCHITECT_R {
        ARCHITECT_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "Provides CoreSight discovery information for the FPB  

You can [`read`](crate::Reg::read) this register and get [`fp_devarch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_DEVARCH_SPEC;
impl crate::RegisterSpec for FP_DEVARCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_devarch::R`](R) reader structure"]
impl crate::Readable for FP_DEVARCH_SPEC {}
#[doc = "`reset()` method sets FP_DEVARCH to value 0x4770_1a03"]
impl crate::Resettable for FP_DEVARCH_SPEC {
    const RESET_VALUE: u32 = 0x4770_1a03;
}
