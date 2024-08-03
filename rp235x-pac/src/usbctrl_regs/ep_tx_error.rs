#[doc = "Register `EP_TX_ERROR` reader"]
pub type R = crate::R<EP_TX_ERROR_SPEC>;
#[doc = "Register `EP_TX_ERROR` writer"]
pub type W = crate::W<EP_TX_ERROR_SPEC>;
#[doc = "Field `EP0` reader - "]
pub type EP0_R = crate::FieldReader;
#[doc = "Field `EP0` writer - "]
pub type EP0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP1` reader - "]
pub type EP1_R = crate::FieldReader;
#[doc = "Field `EP1` writer - "]
pub type EP1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP2` reader - "]
pub type EP2_R = crate::FieldReader;
#[doc = "Field `EP2` writer - "]
pub type EP2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP3` reader - "]
pub type EP3_R = crate::FieldReader;
#[doc = "Field `EP3` writer - "]
pub type EP3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP4` reader - "]
pub type EP4_R = crate::FieldReader;
#[doc = "Field `EP4` writer - "]
pub type EP4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP5` reader - "]
pub type EP5_R = crate::FieldReader;
#[doc = "Field `EP5` writer - "]
pub type EP5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP6` reader - "]
pub type EP6_R = crate::FieldReader;
#[doc = "Field `EP6` writer - "]
pub type EP6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP7` reader - "]
pub type EP7_R = crate::FieldReader;
#[doc = "Field `EP7` writer - "]
pub type EP7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP8` reader - "]
pub type EP8_R = crate::FieldReader;
#[doc = "Field `EP8` writer - "]
pub type EP8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP9` reader - "]
pub type EP9_R = crate::FieldReader;
#[doc = "Field `EP9` writer - "]
pub type EP9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP10` reader - "]
pub type EP10_R = crate::FieldReader;
#[doc = "Field `EP10` writer - "]
pub type EP10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP11` reader - "]
pub type EP11_R = crate::FieldReader;
#[doc = "Field `EP11` writer - "]
pub type EP11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP12` reader - "]
pub type EP12_R = crate::FieldReader;
#[doc = "Field `EP12` writer - "]
pub type EP12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP13` reader - "]
pub type EP13_R = crate::FieldReader;
#[doc = "Field `EP13` writer - "]
pub type EP13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP14` reader - "]
pub type EP14_R = crate::FieldReader;
#[doc = "Field `EP14` writer - "]
pub type EP14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP15` reader - "]
pub type EP15_R = crate::FieldReader;
#[doc = "Field `EP15` writer - "]
pub type EP15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ep0(&self) -> EP0_R {
        EP0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ep1(&self) -> EP1_R {
        EP1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ep2(&self) -> EP2_R {
        EP2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn ep3(&self) -> EP3_R {
        EP3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ep4(&self) -> EP4_R {
        EP4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn ep5(&self) -> EP5_R {
        EP5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ep6(&self) -> EP6_R {
        EP6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ep7(&self) -> EP7_R {
        EP7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ep8(&self) -> EP8_R {
        EP8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn ep9(&self) -> EP9_R {
        EP9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn ep10(&self) -> EP10_R {
        EP10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn ep11(&self) -> EP11_R {
        EP11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ep12(&self) -> EP12_R {
        EP12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ep13(&self) -> EP13_R {
        EP13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn ep14(&self) -> EP14_R {
        EP14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn ep15(&self) -> EP15_R {
        EP15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn ep0(&mut self) -> EP0_W<EP_TX_ERROR_SPEC> {
        EP0_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn ep1(&mut self) -> EP1_W<EP_TX_ERROR_SPEC> {
        EP1_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn ep2(&mut self) -> EP2_W<EP_TX_ERROR_SPEC> {
        EP2_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn ep3(&mut self) -> EP3_W<EP_TX_ERROR_SPEC> {
        EP3_W::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn ep4(&mut self) -> EP4_W<EP_TX_ERROR_SPEC> {
        EP4_W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn ep5(&mut self) -> EP5_W<EP_TX_ERROR_SPEC> {
        EP5_W::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn ep6(&mut self) -> EP6_W<EP_TX_ERROR_SPEC> {
        EP6_W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn ep7(&mut self) -> EP7_W<EP_TX_ERROR_SPEC> {
        EP7_W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn ep8(&mut self) -> EP8_W<EP_TX_ERROR_SPEC> {
        EP8_W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn ep9(&mut self) -> EP9_W<EP_TX_ERROR_SPEC> {
        EP9_W::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn ep10(&mut self) -> EP10_W<EP_TX_ERROR_SPEC> {
        EP10_W::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn ep11(&mut self) -> EP11_W<EP_TX_ERROR_SPEC> {
        EP11_W::new(self, 22)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn ep12(&mut self) -> EP12_W<EP_TX_ERROR_SPEC> {
        EP12_W::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn ep13(&mut self) -> EP13_W<EP_TX_ERROR_SPEC> {
        EP13_W::new(self, 26)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn ep14(&mut self) -> EP14_W<EP_TX_ERROR_SPEC> {
        EP14_W::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn ep15(&mut self) -> EP15_W<EP_TX_ERROR_SPEC> {
        EP15_W::new(self, 30)
    }
}
#[doc = "TX error count for each endpoint. Write to each field to reset the counter to 0.  

You can [`read`](crate::Reg::read) this register and get [`ep_tx_error::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_tx_error::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP_TX_ERROR_SPEC;
impl crate::RegisterSpec for EP_TX_ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_tx_error::R`](R) reader structure"]
impl crate::Readable for EP_TX_ERROR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep_tx_error::W`](W) writer structure"]
impl crate::Writable for EP_TX_ERROR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets EP_TX_ERROR to value 0"]
impl crate::Resettable for EP_TX_ERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
