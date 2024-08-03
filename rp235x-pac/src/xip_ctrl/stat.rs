#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Field `FIFO_EMPTY` reader - When 1, indicates the XIP streaming FIFO is completely empty."]
pub type FIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `FIFO_FULL` reader - When 1, indicates the XIP streaming FIFO is completely full.  
 The streaming FIFO is 2 entries deep, so the full and empty  
 flag allow its level to be ascertained."]
pub type FIFO_FULL_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - When 1, indicates the XIP streaming FIFO is completely empty."]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, indicates the XIP streaming FIFO is completely full.  
 The streaming FIFO is 2 entries deep, so the full and empty  
 flag allow its level to be ascertained."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0x02"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
