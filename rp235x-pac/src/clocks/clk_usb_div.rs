#[doc = "Register `CLK_USB_DIV` reader"]
pub type R = crate::R<CLK_USB_DIV_SPEC>;
#[doc = "Register `CLK_USB_DIV` writer"]
pub type W = crate::W<CLK_USB_DIV_SPEC>;
#[doc = "Field `INT` reader - Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
pub type INT_R = crate::FieldReader;
#[doc = "Field `INT` writer - Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
pub type INT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 16:19 - Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<CLK_USB_DIV_SPEC> {
        INT_W::new(self, 16)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`clk_usb_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_usb_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_USB_DIV_SPEC;
impl crate::RegisterSpec for CLK_USB_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_usb_div::R`](R) reader structure"]
impl crate::Readable for CLK_USB_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_usb_div::W`](W) writer structure"]
impl crate::Writable for CLK_USB_DIV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_USB_DIV to value 0x0001_0000"]
impl crate::Resettable for CLK_USB_DIV_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
