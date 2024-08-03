#[doc = "Register `DATA0` reader"]
pub type R = crate::R<DATA0_SPEC>;
#[doc = "Register `DATA0` writer"]
pub type W = crate::W<DATA0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "data0 through data11 are basic read/write registers that may be read or changed by abstract commands. abstractcs.datacount indicates how many of them are implemented, starting at data0, counting up.  

 Accessing these registers while an abstract command is executing causes abstractcs.cmderr to be set to 1 (busy) if it is 0.  

 Attempts to write them while abstractcs.busy is set does not change their value.  

 The values in these registers may not be preserved after an abstract command is executed. The only guarantees on their contents are the ones offered by the command in question. If the command fails, no assumptions can be made about the contents of these registers.  

 (Note: Hazard3 implements data0 only.)  

You can [`read`](crate::Reg::read) this register and get [`data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA0_SPEC;
impl crate::RegisterSpec for DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0::R`](R) reader structure"]
impl crate::Readable for DATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data0::W`](W) writer structure"]
impl crate::Writable for DATA0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for DATA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
