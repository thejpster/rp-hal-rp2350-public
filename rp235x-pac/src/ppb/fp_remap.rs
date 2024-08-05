#[doc = "Register `FP_REMAP` reader"]
pub type R = crate::R<FP_REMAP_SPEC>;
#[doc = "Register `FP_REMAP` writer"]
pub type W = crate::W<FP_REMAP_SPEC>;
#[doc = "Field `REMAP` reader - Holds the bits\\[28:5\\]
of the Flash Patch remap address"]
pub type REMAP_R = crate::FieldReader<u32>;
#[doc = "Field `RMPSPT` reader - Indicates whether the FPB unit supports the Flash Patch remap function"]
pub type RMPSPT_R = crate::BitReader;
impl R {
    #[doc = "Bits 5:28 - Holds the bits\\[28:5\\]
of the Flash Patch remap address"]
    #[inline(always)]
    pub fn remap(&self) -> REMAP_R {
        REMAP_R::new((self.bits >> 5) & 0x00ff_ffff)
    }
    #[doc = "Bit 29 - Indicates whether the FPB unit supports the Flash Patch remap function"]
    #[inline(always)]
    pub fn rmpspt(&self) -> RMPSPT_R {
        RMPSPT_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {}
#[doc = "Indicates whether the implementation supports Flash Patch remap and, if it does, holds the target address for remap  

You can [`read`](crate::Reg::read) this register and get [`fp_remap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_remap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_REMAP_SPEC;
impl crate::RegisterSpec for FP_REMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_remap::R`](R) reader structure"]
impl crate::Readable for FP_REMAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fp_remap::W`](W) writer structure"]
impl crate::Writable for FP_REMAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FP_REMAP to value 0"]
impl crate::Resettable for FP_REMAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
