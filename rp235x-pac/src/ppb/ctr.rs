#[doc = "Register `CTR` reader"]
pub type R = crate::R<CTR_SPEC>;
#[doc = "Field `IMINLINE` reader - Log2 of the number of words in the smallest cache line of all the instruction caches that are controlled by the PE"]
pub type IMINLINE_R = crate::FieldReader;
#[doc = "Field `RES1_1` reader - Reserved, RES1"]
pub type RES1_1_R = crate::FieldReader;
#[doc = "Field `DMINLINE` reader - Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the PE"]
pub type DMINLINE_R = crate::FieldReader;
#[doc = "Field `ERG` reader - Log2 of the number of words of the maximum size of the reservation granule that has been implemented for the Load-Exclusive and Store-Exclusive instructions"]
pub type ERG_R = crate::FieldReader;
#[doc = "Field `CWG` reader - Log2 of the number of words of the maximum size of memory that can be overwritten as a result of the eviction of a cache entry that has had a memory location in it modified"]
pub type CWG_R = crate::FieldReader;
#[doc = "Field `RES1` reader - Reserved, RES1"]
pub type RES1_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Log2 of the number of words in the smallest cache line of all the instruction caches that are controlled by the PE"]
    #[inline(always)]
    pub fn iminline(&self) -> IMINLINE_R {
        IMINLINE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - Reserved, RES1"]
    #[inline(always)]
    pub fn res1_1(&self) -> RES1_1_R {
        RES1_1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the PE"]
    #[inline(always)]
    pub fn dminline(&self) -> DMINLINE_R {
        DMINLINE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Log2 of the number of words of the maximum size of the reservation granule that has been implemented for the Load-Exclusive and Store-Exclusive instructions"]
    #[inline(always)]
    pub fn erg(&self) -> ERG_R {
        ERG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Log2 of the number of words of the maximum size of memory that can be overwritten as a result of the eviction of a cache entry that has had a memory location in it modified"]
    #[inline(always)]
    pub fn cwg(&self) -> CWG_R {
        CWG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Reserved, RES1"]
    #[inline(always)]
    pub fn res1(&self) -> RES1_R {
        RES1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Provides information about the architecture of the caches. CTR is RES0 if CLIDR is zero.  

You can [`read`](crate::Reg::read) this register and get [`ctr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CTR_SPEC {}
#[doc = "`reset()` method sets CTR to value 0x8000_c000"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: u32 = 0x8000_c000;
}
