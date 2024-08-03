#[doc = "Register `CLK_SYS_SELECTED` reader"]
pub type R = crate::R<CLK_SYS_SELECTED_SPEC>;
#[doc = "Field `CLK_SYS_SELECTED` reader - The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
pub type CLK_SYS_SELECTED_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
    #[inline(always)]
    pub fn clk_sys_selected(&self) -> CLK_SYS_SELECTED_R {
        CLK_SYS_SELECTED_R::new((self.bits & 3) as u8)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)  

You can [`read`](crate::Reg::read) this register and get [`clk_sys_selected::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SYS_SELECTED_SPEC;
impl crate::RegisterSpec for CLK_SYS_SELECTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_sys_selected::R`](R) reader structure"]
impl crate::Readable for CLK_SYS_SELECTED_SPEC {}
#[doc = "`reset()` method sets CLK_SYS_SELECTED to value 0x01"]
impl crate::Resettable for CLK_SYS_SELECTED_SPEC {
    const RESET_VALUE: u32 = 0x01;
}