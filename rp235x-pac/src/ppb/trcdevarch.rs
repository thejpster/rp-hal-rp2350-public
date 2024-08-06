#[doc = "Register `TRCDEVARCH` reader"]
pub type R = crate::R<TRCDEVARCH_SPEC>;
#[doc = "Register `TRCDEVARCH` writer"]
pub type W = crate::W<TRCDEVARCH_SPEC>;
#[doc = "Field `ARCHID` reader - reads as 0b0100101000010011"]
pub type ARCHID_R = crate::FieldReader<u16>;
#[doc = "Field `REVISION` reader - reads as 0b0000"]
pub type REVISION_R = crate::FieldReader;
#[doc = "Field `PRESENT` reader - reads as 0b1"]
pub type PRESENT_R = crate::BitReader;
#[doc = "Field `ARCHITECT` reader - reads as 0b01000111011"]
pub type ARCHITECT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - reads as 0b0100101000010011"]
    #[inline(always)]
    pub fn archid(&self) -> ARCHID_R {
        ARCHID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - reads as 0b0000"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - reads as 0b1"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - reads as 0b01000111011"]
    #[inline(always)]
    pub fn architect(&self) -> ARCHITECT_R {
        ARCHITECT_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {}
#[doc = "TRCDEVARCH  

You can [`read`](crate::Reg::read) this register and get [`trcdevarch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcdevarch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCDEVARCH_SPEC;
impl crate::RegisterSpec for TRCDEVARCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcdevarch::R`](R) reader structure"]
impl crate::Readable for TRCDEVARCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcdevarch::W`](W) writer structure"]
impl crate::Writable for TRCDEVARCH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCDEVARCH to value 0x4772_4a13"]
impl crate::Resettable for TRCDEVARCH_SPEC {
    const RESET_VALUE: u32 = 0x4772_4a13;
}
