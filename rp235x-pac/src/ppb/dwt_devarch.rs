#[doc = "Register `DWT_DEVARCH` reader"]
pub type R = crate::R<DWT_DEVARCH_SPEC>;
#[doc = "Register `DWT_DEVARCH` writer"]
pub type W = crate::W<DWT_DEVARCH_SPEC>;
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
impl W {}
#[doc = "Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_devarch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_devarch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_DEVARCH_SPEC;
impl crate::RegisterSpec for DWT_DEVARCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_devarch::R`](R) reader structure"]
impl crate::Readable for DWT_DEVARCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_devarch::W`](W) writer structure"]
impl crate::Writable for DWT_DEVARCH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_DEVARCH to value 0x4770_1a02"]
impl crate::Resettable for DWT_DEVARCH_SPEC {
    const RESET_VALUE: u32 = 0x4770_1a02;
}
