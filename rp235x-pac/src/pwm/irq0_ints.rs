#[doc = "Register `IRQ0_INTS` reader"]
pub type R = crate::R<IRQ0_INTS_SPEC>;
#[doc = "Field `CH0` reader - "]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH1` reader - "]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH2` reader - "]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH3` reader - "]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH4` reader - "]
pub type CH4_R = crate::BitReader;
#[doc = "Field `CH5` reader - "]
pub type CH5_R = crate::BitReader;
#[doc = "Field `CH6` reader - "]
pub type CH6_R = crate::BitReader;
#[doc = "Field `CH7` reader - "]
pub type CH7_R = crate::BitReader;
#[doc = "Field `CH8` reader - "]
pub type CH8_R = crate::BitReader;
#[doc = "Field `CH9` reader - "]
pub type CH9_R = crate::BitReader;
#[doc = "Field `CH10` reader - "]
pub type CH10_R = crate::BitReader;
#[doc = "Field `CH11` reader - "]
pub type CH11_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Interrupt status after masking &amp; forcing for irq0  

You can [`read`](crate::Reg::read) this register and get [`irq0_ints::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ0_INTS_SPEC;
impl crate::RegisterSpec for IRQ0_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq0_ints::R`](R) reader structure"]
impl crate::Readable for IRQ0_INTS_SPEC {}
#[doc = "`reset()` method sets IRQ0_INTS to value 0"]
impl crate::Resettable for IRQ0_INTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
