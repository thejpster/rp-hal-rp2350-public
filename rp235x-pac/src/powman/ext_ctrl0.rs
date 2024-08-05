#[doc = "Register `EXT_CTRL0` reader"]
pub type R = crate::R<EXT_CTRL0_SPEC>;
#[doc = "Register `EXT_CTRL0` writer"]
pub type W = crate::W<EXT_CTRL0_SPEC>;
#[doc = "Field `GPIO_SELECT` reader - selects from gpio 0->30 set to 31 to disable this feature"]
pub type GPIO_SELECT_R = crate::FieldReader;
#[doc = "Field `GPIO_SELECT` writer - selects from gpio 0->30 set to 31 to disable this feature"]
pub type GPIO_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `INIT` reader - "]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - "]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INIT_STATE` reader - "]
pub type INIT_STATE_R = crate::BitReader;
#[doc = "Field `INIT_STATE` writer - "]
pub type INIT_STATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ENTRY_STATE` reader - output level when entering the low power state"]
pub type LP_ENTRY_STATE_R = crate::BitReader;
#[doc = "Field `LP_ENTRY_STATE` writer - output level when entering the low power state"]
pub type LP_ENTRY_STATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_EXIT_STATE` reader - output level when exiting the low power state"]
pub type LP_EXIT_STATE_R = crate::BitReader;
#[doc = "Field `LP_EXIT_STATE` writer - output level when exiting the low power state"]
pub type LP_EXIT_STATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - selects from gpio 0->30 set to 31 to disable this feature"]
    #[inline(always)]
    pub fn gpio_select(&self) -> GPIO_SELECT_R {
        GPIO_SELECT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn init_state(&self) -> INIT_STATE_R {
        INIT_STATE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - output level when entering the low power state"]
    #[inline(always)]
    pub fn lp_entry_state(&self) -> LP_ENTRY_STATE_R {
        LP_ENTRY_STATE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output level when exiting the low power state"]
    #[inline(always)]
    pub fn lp_exit_state(&self) -> LP_EXIT_STATE_R {
        LP_EXIT_STATE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - selects from gpio 0->30 set to 31 to disable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_select(&mut self) -> GPIO_SELECT_W<EXT_CTRL0_SPEC> {
        GPIO_SELECT_W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<EXT_CTRL0_SPEC> {
        INIT_W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn init_state(&mut self) -> INIT_STATE_W<EXT_CTRL0_SPEC> {
        INIT_STATE_W::new(self, 12)
    }
    #[doc = "Bit 13 - output level when entering the low power state"]
    #[inline(always)]
    #[must_use]
    pub fn lp_entry_state(&mut self) -> LP_ENTRY_STATE_W<EXT_CTRL0_SPEC> {
        LP_ENTRY_STATE_W::new(self, 13)
    }
    #[doc = "Bit 14 - output level when exiting the low power state"]
    #[inline(always)]
    #[must_use]
    pub fn lp_exit_state(&mut self) -> LP_EXIT_STATE_W<EXT_CTRL0_SPEC> {
        LP_EXIT_STATE_W::new(self, 14)
    }
}
#[doc = "Configures a gpio as a power mode aware control output  

You can [`read`](crate::Reg::read) this register and get [`ext_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_CTRL0_SPEC;
impl crate::RegisterSpec for EXT_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_ctrl0::R`](R) reader structure"]
impl crate::Readable for EXT_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_ctrl0::W`](W) writer structure"]
impl crate::Writable for EXT_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_CTRL0 to value 0x3f"]
impl crate::Resettable for EXT_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x3f;
}
