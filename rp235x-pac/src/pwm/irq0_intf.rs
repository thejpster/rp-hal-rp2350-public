#[doc = "Register `IRQ0_INTF` reader"]
pub type R = crate::R<IRQ0_INTF_SPEC>;
#[doc = "Register `IRQ0_INTF` writer"]
pub type W = crate::W<IRQ0_INTF_SPEC>;
#[doc = "Field `CH0` reader - "]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH0` writer - "]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - "]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH1` writer - "]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - "]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH2` writer - "]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - "]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH3` writer - "]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` reader - "]
pub type CH4_R = crate::BitReader;
#[doc = "Field `CH4` writer - "]
pub type CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` reader - "]
pub type CH5_R = crate::BitReader;
#[doc = "Field `CH5` writer - "]
pub type CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` reader - "]
pub type CH6_R = crate::BitReader;
#[doc = "Field `CH6` writer - "]
pub type CH6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` reader - "]
pub type CH7_R = crate::BitReader;
#[doc = "Field `CH7` writer - "]
pub type CH7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` reader - "]
pub type CH8_R = crate::BitReader;
#[doc = "Field `CH8` writer - "]
pub type CH8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` reader - "]
pub type CH9_R = crate::BitReader;
#[doc = "Field `CH9` writer - "]
pub type CH9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` reader - "]
pub type CH10_R = crate::BitReader;
#[doc = "Field `CH10` writer - "]
pub type CH10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` reader - "]
pub type CH11_R = crate::BitReader;
#[doc = "Field `CH11` writer - "]
pub type CH11_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<IRQ0_INTF_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<IRQ0_INTF_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<IRQ0_INTF_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<IRQ0_INTF_SPEC> {
        CH3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<IRQ0_INTF_SPEC> {
        CH4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<IRQ0_INTF_SPEC> {
        CH5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<IRQ0_INTF_SPEC> {
        CH6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<IRQ0_INTF_SPEC> {
        CH7_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<IRQ0_INTF_SPEC> {
        CH8_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<IRQ0_INTF_SPEC> {
        CH9_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<IRQ0_INTF_SPEC> {
        CH10_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<IRQ0_INTF_SPEC> {
        CH11_W::new(self, 11)
    }
}
#[doc = "Interrupt Force for irq0  

You can [`read`](crate::Reg::read) this register and get [`irq0_intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq0_intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ0_INTF_SPEC;
impl crate::RegisterSpec for IRQ0_INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq0_intf::R`](R) reader structure"]
impl crate::Readable for IRQ0_INTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq0_intf::W`](W) writer structure"]
impl crate::Writable for IRQ0_INTF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ0_INTF to value 0"]
impl crate::Resettable for IRQ0_INTF_SPEC {
    const RESET_VALUE: u32 = 0;
}
