#[doc = "Register `GPIO_OE_XOR` writer"]
pub type W = crate::W<GPIO_OE_XOR_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<GPIO_OE_XOR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "GPIO0...31 output enable XOR  
 Perform an atomic bitwise XOR on GPIO_OE, i.e. `GPIO_OE ^= wdata`  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_oe_xor::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OE_XOR_SPEC;
impl crate::RegisterSpec for GPIO_OE_XOR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_oe_xor::W`](W) writer structure"]
impl crate::Writable for GPIO_OE_XOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_OE_XOR to value 0"]
impl crate::Resettable for GPIO_OE_XOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
