#[doc = "Register `DWT_DEVTYPE` reader"]
pub type R = crate::R<DWT_DEVTYPE_SPEC>;
#[doc = "Register `DWT_DEVTYPE` writer"]
pub type W = crate::W<DWT_DEVTYPE_SPEC>;
#[doc = "Field `MAJOR` reader - Component major type"]
pub type MAJOR_R = crate::FieldReader;
#[doc = "Field `SUB` reader - Component sub-type"]
pub type SUB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Component major type"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component sub-type"]
    #[inline(always)]
    pub fn sub(&self) -> SUB_R {
        SUB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_devtype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_devtype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_DEVTYPE_SPEC;
impl crate::RegisterSpec for DWT_DEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_devtype::R`](R) reader structure"]
impl crate::Readable for DWT_DEVTYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_devtype::W`](W) writer structure"]
impl crate::Writable for DWT_DEVTYPE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_DEVTYPE to value 0"]
impl crate::Resettable for DWT_DEVTYPE_SPEC {
    const RESET_VALUE: u32 = 0;
}
