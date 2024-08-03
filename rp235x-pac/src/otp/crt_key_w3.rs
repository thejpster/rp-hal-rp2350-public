#[doc = "Register `CRT_KEY_W3` writer"]
pub type W = crate::W<CRT_KEY_W3_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<CRT_KEY_W3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Word 3 (bits 127..96) of the key. Write only, read returns 0x0  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crt_key_w3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRT_KEY_W3_SPEC;
impl crate::RegisterSpec for CRT_KEY_W3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crt_key_w3::W`](W) writer structure"]
impl crate::Writable for CRT_KEY_W3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRT_KEY_W3 to value 0"]
impl crate::Resettable for CRT_KEY_W3_SPEC {
    const RESET_VALUE: u32 = 0;
}
