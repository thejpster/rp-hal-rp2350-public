#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sw_lock0: SW_LOCK0,
    sw_lock1: SW_LOCK1,
    sw_lock2: SW_LOCK2,
    sw_lock3: SW_LOCK3,
    sw_lock4: SW_LOCK4,
    sw_lock5: SW_LOCK5,
    sw_lock6: SW_LOCK6,
    sw_lock7: SW_LOCK7,
    sw_lock8: SW_LOCK8,
    sw_lock9: SW_LOCK9,
    sw_lock10: SW_LOCK10,
    sw_lock11: SW_LOCK11,
    sw_lock12: SW_LOCK12,
    sw_lock13: SW_LOCK13,
    sw_lock14: SW_LOCK14,
    sw_lock15: SW_LOCK15,
    sw_lock16: SW_LOCK16,
    sw_lock17: SW_LOCK17,
    sw_lock18: SW_LOCK18,
    sw_lock19: SW_LOCK19,
    sw_lock20: SW_LOCK20,
    sw_lock21: SW_LOCK21,
    sw_lock22: SW_LOCK22,
    sw_lock23: SW_LOCK23,
    sw_lock24: SW_LOCK24,
    sw_lock25: SW_LOCK25,
    sw_lock26: SW_LOCK26,
    sw_lock27: SW_LOCK27,
    sw_lock28: SW_LOCK28,
    sw_lock29: SW_LOCK29,
    sw_lock30: SW_LOCK30,
    sw_lock31: SW_LOCK31,
    sw_lock32: SW_LOCK32,
    sw_lock33: SW_LOCK33,
    sw_lock34: SW_LOCK34,
    sw_lock35: SW_LOCK35,
    sw_lock36: SW_LOCK36,
    sw_lock37: SW_LOCK37,
    sw_lock38: SW_LOCK38,
    sw_lock39: SW_LOCK39,
    sw_lock40: SW_LOCK40,
    sw_lock41: SW_LOCK41,
    sw_lock42: SW_LOCK42,
    sw_lock43: SW_LOCK43,
    sw_lock44: SW_LOCK44,
    sw_lock45: SW_LOCK45,
    sw_lock46: SW_LOCK46,
    sw_lock47: SW_LOCK47,
    sw_lock48: SW_LOCK48,
    sw_lock49: SW_LOCK49,
    sw_lock50: SW_LOCK50,
    sw_lock51: SW_LOCK51,
    sw_lock52: SW_LOCK52,
    sw_lock53: SW_LOCK53,
    sw_lock54: SW_LOCK54,
    sw_lock55: SW_LOCK55,
    sw_lock56: SW_LOCK56,
    sw_lock57: SW_LOCK57,
    sw_lock58: SW_LOCK58,
    sw_lock59: SW_LOCK59,
    sw_lock60: SW_LOCK60,
    sw_lock61: SW_LOCK61,
    sw_lock62: SW_LOCK62,
    sw_lock63: SW_LOCK63,
    sbpi_instr: SBPI_INSTR,
    sbpi_wdata_0: SBPI_WDATA_0,
    sbpi_wdata_1: SBPI_WDATA_1,
    sbpi_wdata_2: SBPI_WDATA_2,
    sbpi_wdata_3: SBPI_WDATA_3,
    sbpi_rdata_0: SBPI_RDATA_0,
    sbpi_rdata_1: SBPI_RDATA_1,
    sbpi_rdata_2: SBPI_RDATA_2,
    sbpi_rdata_3: SBPI_RDATA_3,
    sbpi_status: SBPI_STATUS,
    usr: USR,
    dbg: DBG,
    _reserved76: [u8; 0x04],
    bist: BIST,
    crt_key_w0: CRT_KEY_W0,
    crt_key_w1: CRT_KEY_W1,
    crt_key_w2: CRT_KEY_W2,
    crt_key_w3: CRT_KEY_W3,
    critical: CRITICAL,
    key_valid: KEY_VALID,
    debugen: DEBUGEN,
    debugen_lock: DEBUGEN_LOCK,
    archsel: ARCHSEL,
    archsel_status: ARCHSEL_STATUS,
    bootdis: BOOTDIS,
    intr: INTR,
    inte: INTE,
    intf: INTF,
    ints: INTS,
}
impl RegisterBlock {
    #[doc = "0x00 - Software lock register for page 0. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock0(&self) -> &SW_LOCK0 {
        &self.sw_lock0
    }
    #[doc = "0x04 - Software lock register for page 1. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock1(&self) -> &SW_LOCK1 {
        &self.sw_lock1
    }
    #[doc = "0x08 - Software lock register for page 2. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock2(&self) -> &SW_LOCK2 {
        &self.sw_lock2
    }
    #[doc = "0x0c - Software lock register for page 3. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock3(&self) -> &SW_LOCK3 {
        &self.sw_lock3
    }
    #[doc = "0x10 - Software lock register for page 4. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock4(&self) -> &SW_LOCK4 {
        &self.sw_lock4
    }
    #[doc = "0x14 - Software lock register for page 5. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock5(&self) -> &SW_LOCK5 {
        &self.sw_lock5
    }
    #[doc = "0x18 - Software lock register for page 6. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock6(&self) -> &SW_LOCK6 {
        &self.sw_lock6
    }
    #[doc = "0x1c - Software lock register for page 7. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock7(&self) -> &SW_LOCK7 {
        &self.sw_lock7
    }
    #[doc = "0x20 - Software lock register for page 8. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock8(&self) -> &SW_LOCK8 {
        &self.sw_lock8
    }
    #[doc = "0x24 - Software lock register for page 9. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock9(&self) -> &SW_LOCK9 {
        &self.sw_lock9
    }
    #[doc = "0x28 - Software lock register for page 10. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock10(&self) -> &SW_LOCK10 {
        &self.sw_lock10
    }
    #[doc = "0x2c - Software lock register for page 11. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock11(&self) -> &SW_LOCK11 {
        &self.sw_lock11
    }
    #[doc = "0x30 - Software lock register for page 12. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock12(&self) -> &SW_LOCK12 {
        &self.sw_lock12
    }
    #[doc = "0x34 - Software lock register for page 13. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock13(&self) -> &SW_LOCK13 {
        &self.sw_lock13
    }
    #[doc = "0x38 - Software lock register for page 14. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock14(&self) -> &SW_LOCK14 {
        &self.sw_lock14
    }
    #[doc = "0x3c - Software lock register for page 15. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock15(&self) -> &SW_LOCK15 {
        &self.sw_lock15
    }
    #[doc = "0x40 - Software lock register for page 16. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock16(&self) -> &SW_LOCK16 {
        &self.sw_lock16
    }
    #[doc = "0x44 - Software lock register for page 17. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock17(&self) -> &SW_LOCK17 {
        &self.sw_lock17
    }
    #[doc = "0x48 - Software lock register for page 18. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock18(&self) -> &SW_LOCK18 {
        &self.sw_lock18
    }
    #[doc = "0x4c - Software lock register for page 19. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock19(&self) -> &SW_LOCK19 {
        &self.sw_lock19
    }
    #[doc = "0x50 - Software lock register for page 20. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock20(&self) -> &SW_LOCK20 {
        &self.sw_lock20
    }
    #[doc = "0x54 - Software lock register for page 21. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock21(&self) -> &SW_LOCK21 {
        &self.sw_lock21
    }
    #[doc = "0x58 - Software lock register for page 22. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock22(&self) -> &SW_LOCK22 {
        &self.sw_lock22
    }
    #[doc = "0x5c - Software lock register for page 23. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock23(&self) -> &SW_LOCK23 {
        &self.sw_lock23
    }
    #[doc = "0x60 - Software lock register for page 24. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock24(&self) -> &SW_LOCK24 {
        &self.sw_lock24
    }
    #[doc = "0x64 - Software lock register for page 25. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock25(&self) -> &SW_LOCK25 {
        &self.sw_lock25
    }
    #[doc = "0x68 - Software lock register for page 26. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock26(&self) -> &SW_LOCK26 {
        &self.sw_lock26
    }
    #[doc = "0x6c - Software lock register for page 27. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock27(&self) -> &SW_LOCK27 {
        &self.sw_lock27
    }
    #[doc = "0x70 - Software lock register for page 28. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock28(&self) -> &SW_LOCK28 {
        &self.sw_lock28
    }
    #[doc = "0x74 - Software lock register for page 29. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock29(&self) -> &SW_LOCK29 {
        &self.sw_lock29
    }
    #[doc = "0x78 - Software lock register for page 30. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock30(&self) -> &SW_LOCK30 {
        &self.sw_lock30
    }
    #[doc = "0x7c - Software lock register for page 31. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock31(&self) -> &SW_LOCK31 {
        &self.sw_lock31
    }
    #[doc = "0x80 - Software lock register for page 32. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock32(&self) -> &SW_LOCK32 {
        &self.sw_lock32
    }
    #[doc = "0x84 - Software lock register for page 33. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock33(&self) -> &SW_LOCK33 {
        &self.sw_lock33
    }
    #[doc = "0x88 - Software lock register for page 34. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock34(&self) -> &SW_LOCK34 {
        &self.sw_lock34
    }
    #[doc = "0x8c - Software lock register for page 35. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock35(&self) -> &SW_LOCK35 {
        &self.sw_lock35
    }
    #[doc = "0x90 - Software lock register for page 36. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock36(&self) -> &SW_LOCK36 {
        &self.sw_lock36
    }
    #[doc = "0x94 - Software lock register for page 37. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock37(&self) -> &SW_LOCK37 {
        &self.sw_lock37
    }
    #[doc = "0x98 - Software lock register for page 38. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock38(&self) -> &SW_LOCK38 {
        &self.sw_lock38
    }
    #[doc = "0x9c - Software lock register for page 39. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock39(&self) -> &SW_LOCK39 {
        &self.sw_lock39
    }
    #[doc = "0xa0 - Software lock register for page 40. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock40(&self) -> &SW_LOCK40 {
        &self.sw_lock40
    }
    #[doc = "0xa4 - Software lock register for page 41. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock41(&self) -> &SW_LOCK41 {
        &self.sw_lock41
    }
    #[doc = "0xa8 - Software lock register for page 42. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock42(&self) -> &SW_LOCK42 {
        &self.sw_lock42
    }
    #[doc = "0xac - Software lock register for page 43. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock43(&self) -> &SW_LOCK43 {
        &self.sw_lock43
    }
    #[doc = "0xb0 - Software lock register for page 44. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock44(&self) -> &SW_LOCK44 {
        &self.sw_lock44
    }
    #[doc = "0xb4 - Software lock register for page 45. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock45(&self) -> &SW_LOCK45 {
        &self.sw_lock45
    }
    #[doc = "0xb8 - Software lock register for page 46. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock46(&self) -> &SW_LOCK46 {
        &self.sw_lock46
    }
    #[doc = "0xbc - Software lock register for page 47. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock47(&self) -> &SW_LOCK47 {
        &self.sw_lock47
    }
    #[doc = "0xc0 - Software lock register for page 48. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock48(&self) -> &SW_LOCK48 {
        &self.sw_lock48
    }
    #[doc = "0xc4 - Software lock register for page 49. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock49(&self) -> &SW_LOCK49 {
        &self.sw_lock49
    }
    #[doc = "0xc8 - Software lock register for page 50. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock50(&self) -> &SW_LOCK50 {
        &self.sw_lock50
    }
    #[doc = "0xcc - Software lock register for page 51. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock51(&self) -> &SW_LOCK51 {
        &self.sw_lock51
    }
    #[doc = "0xd0 - Software lock register for page 52. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock52(&self) -> &SW_LOCK52 {
        &self.sw_lock52
    }
    #[doc = "0xd4 - Software lock register for page 53. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock53(&self) -> &SW_LOCK53 {
        &self.sw_lock53
    }
    #[doc = "0xd8 - Software lock register for page 54. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock54(&self) -> &SW_LOCK54 {
        &self.sw_lock54
    }
    #[doc = "0xdc - Software lock register for page 55. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock55(&self) -> &SW_LOCK55 {
        &self.sw_lock55
    }
    #[doc = "0xe0 - Software lock register for page 56. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock56(&self) -> &SW_LOCK56 {
        &self.sw_lock56
    }
    #[doc = "0xe4 - Software lock register for page 57. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock57(&self) -> &SW_LOCK57 {
        &self.sw_lock57
    }
    #[doc = "0xe8 - Software lock register for page 58. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock58(&self) -> &SW_LOCK58 {
        &self.sw_lock58
    }
    #[doc = "0xec - Software lock register for page 59. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock59(&self) -> &SW_LOCK59 {
        &self.sw_lock59
    }
    #[doc = "0xf0 - Software lock register for page 60. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock60(&self) -> &SW_LOCK60 {
        &self.sw_lock60
    }
    #[doc = "0xf4 - Software lock register for page 61. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock61(&self) -> &SW_LOCK61 {
        &self.sw_lock61
    }
    #[doc = "0xf8 - Software lock register for page 62. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock62(&self) -> &SW_LOCK62 {
        &self.sw_lock62
    }
    #[doc = "0xfc - Software lock register for page 63. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock63(&self) -> &SW_LOCK63 {
        &self.sw_lock63
    }
    #[doc = "0x100 - Dispatch instructions to the SBPI interface, used for programming the OTP fuses."]
    #[inline(always)]
    pub const fn sbpi_instr(&self) -> &SBPI_INSTR {
        &self.sbpi_instr
    }
    #[doc = "0x104 - SBPI write payload bytes 3..0"]
    #[inline(always)]
    pub const fn sbpi_wdata_0(&self) -> &SBPI_WDATA_0 {
        &self.sbpi_wdata_0
    }
    #[doc = "0x108 - SBPI write payload bytes 7..4"]
    #[inline(always)]
    pub const fn sbpi_wdata_1(&self) -> &SBPI_WDATA_1 {
        &self.sbpi_wdata_1
    }
    #[doc = "0x10c - SBPI write payload bytes 11..8"]
    #[inline(always)]
    pub const fn sbpi_wdata_2(&self) -> &SBPI_WDATA_2 {
        &self.sbpi_wdata_2
    }
    #[doc = "0x110 - SBPI write payload bytes 15..12"]
    #[inline(always)]
    pub const fn sbpi_wdata_3(&self) -> &SBPI_WDATA_3 {
        &self.sbpi_wdata_3
    }
    #[doc = "0x114 - Read payload bytes 3..0. Once read, the data in the register will automatically clear to 0."]
    #[inline(always)]
    pub const fn sbpi_rdata_0(&self) -> &SBPI_RDATA_0 {
        &self.sbpi_rdata_0
    }
    #[doc = "0x118 - Read payload bytes 7..4. Once read, the data in the register will automatically clear to 0."]
    #[inline(always)]
    pub const fn sbpi_rdata_1(&self) -> &SBPI_RDATA_1 {
        &self.sbpi_rdata_1
    }
    #[doc = "0x11c - Read payload bytes 11..8. Once read, the data in the register will automatically clear to 0."]
    #[inline(always)]
    pub const fn sbpi_rdata_2(&self) -> &SBPI_RDATA_2 {
        &self.sbpi_rdata_2
    }
    #[doc = "0x120 - Read payload bytes 15..12. Once read, the data in the register will automatically clear to 0."]
    #[inline(always)]
    pub const fn sbpi_rdata_3(&self) -> &SBPI_RDATA_3 {
        &self.sbpi_rdata_3
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn sbpi_status(&self) -> &SBPI_STATUS {
        &self.sbpi_status
    }
    #[doc = "0x128 - Controls for APB data read interface (USER interface)"]
    #[inline(always)]
    pub const fn usr(&self) -> &USR {
        &self.usr
    }
    #[doc = "0x12c - Debug for OTP power-on state machine"]
    #[inline(always)]
    pub const fn dbg(&self) -> &DBG {
        &self.dbg
    }
    #[doc = "0x134 - During BIST, count address locations that have at least one leaky bit"]
    #[inline(always)]
    pub const fn bist(&self) -> &BIST {
        &self.bist
    }
    #[doc = "0x138 - Word 0 (bits 31..0) of the key. Write only, read returns 0x0"]
    #[inline(always)]
    pub const fn crt_key_w0(&self) -> &CRT_KEY_W0 {
        &self.crt_key_w0
    }
    #[doc = "0x13c - Word 1 (bits 63..32) of the key. Write only, read returns 0x0"]
    #[inline(always)]
    pub const fn crt_key_w1(&self) -> &CRT_KEY_W1 {
        &self.crt_key_w1
    }
    #[doc = "0x140 - Word 2 (bits 95..64) of the key. Write only, read returns 0x0"]
    #[inline(always)]
    pub const fn crt_key_w2(&self) -> &CRT_KEY_W2 {
        &self.crt_key_w2
    }
    #[doc = "0x144 - Word 3 (bits 127..96) of the key. Write only, read returns 0x0"]
    #[inline(always)]
    pub const fn crt_key_w3(&self) -> &CRT_KEY_W3 {
        &self.crt_key_w3
    }
    #[doc = "0x148 - Quickly check values of critical flags read during boot up"]
    #[inline(always)]
    pub const fn critical(&self) -> &CRITICAL {
        &self.critical
    }
    #[doc = "0x14c - Which keys were valid (enrolled) at boot time"]
    #[inline(always)]
    pub const fn key_valid(&self) -> &KEY_VALID {
        &self.key_valid
    }
    #[doc = "0x150 - Enable a debug feature that has been disabled. Debug features are disabled if one of the relevant critical boot flags is set in OTP (DEBUG_DISABLE or SECURE_DEBUG_DISABLE), OR if a debug key is marked valid in OTP, and the matching key value has not been supplied over SWD. Specifically: - The DEBUG_DISABLE flag disables all debug features. This can be fully overridden by setting all bits of this register. - The SECURE_DEBUG_DISABLE flag disables secure processor debug. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If a single debug key has been registered, and no matching key value has been supplied over SWD, then all debug features are disabled. This can be fully overridden by setting all bits of this register. - If both debug keys have been registered, and the Non-secure key's value (key 6) has been supplied over SWD, secure processor debug is disabled. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If both debug keys have been registered, and the Secure key's value (key 5) has been supplied over SWD, then no debug features are disabled by the key mechanism. However, note that in this case debug features may still be disabled by the critical boot flags."]
    #[inline(always)]
    pub const fn debugen(&self) -> &DEBUGEN {
        &self.debugen
    }
    #[doc = "0x154 - Write 1s to lock corresponding bits in DEBUGEN. This register is reset by the processor cold reset."]
    #[inline(always)]
    pub const fn debugen_lock(&self) -> &DEBUGEN_LOCK {
        &self.debugen_lock
    }
    #[doc = "0x158 - Architecture select (Arm/RISC-V). The default and allowable values of this register are constrained by the critical boot flags. This register is reset by the earliest reset in the switched core power domain (before a processor cold reset). Cores sample their architecture select signal on a warm reset. The source of the warm reset could be the system power-up state machine, the watchdog timer, Arm SYSRESETREQ or from RISC-V hartresetreq. Note that when an Arm core is deselected, its cold reset domain is also held in reset, since in particular the SYSRESETREQ bit becomes inaccessible once the core is deselected. Note also the RISC-V cores do not have a cold reset domain, since their corresponding controls are located in the Debug Module."]
    #[inline(always)]
    pub const fn archsel(&self) -> &ARCHSEL {
        &self.archsel
    }
    #[doc = "0x15c - Get the current architecture select state of each core. Cores sample the current value of the ARCHSEL register when their warm reset is released, at which point the corresponding bit in this register will also update."]
    #[inline(always)]
    pub const fn archsel_status(&self) -> &ARCHSEL_STATUS {
        &self.archsel_status
    }
    #[doc = "0x160 - Tell the bootrom to ignore scratch register boot vectors (both power manager and watchdog) on the next power up. If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by performing a watchdog reset that resets the OTP. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the power manager BOOTDIS register."]
    #[inline(always)]
    pub const fn bootdis(&self) -> &BOOTDIS {
        &self.bootdis
    }
    #[doc = "0x164 - Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(&self) -> &INTR {
        &self.intr
    }
    #[doc = "0x168 - Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(&self) -> &INTE {
        &self.inte
    }
    #[doc = "0x16c - Interrupt Force"]
    #[inline(always)]
    pub const fn intf(&self) -> &INTF {
        &self.intf
    }
    #[doc = "0x170 - Interrupt status after masking &amp; forcing"]
    #[inline(always)]
    pub const fn ints(&self) -> &INTS {
        &self.ints
    }
}
#[doc = "SW_LOCK0 (rw) register accessor: Software lock register for page 0. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock0`]
module"]
pub type SW_LOCK0 = crate::Reg<sw_lock0::SW_LOCK0_SPEC>;
#[doc = "Software lock register for page 0. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock0;
#[doc = "SW_LOCK1 (rw) register accessor: Software lock register for page 1. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock1`]
module"]
pub type SW_LOCK1 = crate::Reg<sw_lock1::SW_LOCK1_SPEC>;
#[doc = "Software lock register for page 1. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock1;
#[doc = "SW_LOCK2 (rw) register accessor: Software lock register for page 2. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock2`]
module"]
pub type SW_LOCK2 = crate::Reg<sw_lock2::SW_LOCK2_SPEC>;
#[doc = "Software lock register for page 2. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock2;
#[doc = "SW_LOCK3 (rw) register accessor: Software lock register for page 3. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock3`]
module"]
pub type SW_LOCK3 = crate::Reg<sw_lock3::SW_LOCK3_SPEC>;
#[doc = "Software lock register for page 3. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock3;
#[doc = "SW_LOCK4 (rw) register accessor: Software lock register for page 4. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock4`]
module"]
pub type SW_LOCK4 = crate::Reg<sw_lock4::SW_LOCK4_SPEC>;
#[doc = "Software lock register for page 4. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock4;
#[doc = "SW_LOCK5 (rw) register accessor: Software lock register for page 5. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock5`]
module"]
pub type SW_LOCK5 = crate::Reg<sw_lock5::SW_LOCK5_SPEC>;
#[doc = "Software lock register for page 5. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock5;
#[doc = "SW_LOCK6 (rw) register accessor: Software lock register for page 6. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock6`]
module"]
pub type SW_LOCK6 = crate::Reg<sw_lock6::SW_LOCK6_SPEC>;
#[doc = "Software lock register for page 6. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock6;
#[doc = "SW_LOCK7 (rw) register accessor: Software lock register for page 7. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock7`]
module"]
pub type SW_LOCK7 = crate::Reg<sw_lock7::SW_LOCK7_SPEC>;
#[doc = "Software lock register for page 7. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock7;
#[doc = "SW_LOCK8 (rw) register accessor: Software lock register for page 8. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock8`]
module"]
pub type SW_LOCK8 = crate::Reg<sw_lock8::SW_LOCK8_SPEC>;
#[doc = "Software lock register for page 8. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock8;
#[doc = "SW_LOCK9 (rw) register accessor: Software lock register for page 9. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock9`]
module"]
pub type SW_LOCK9 = crate::Reg<sw_lock9::SW_LOCK9_SPEC>;
#[doc = "Software lock register for page 9. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock9;
#[doc = "SW_LOCK10 (rw) register accessor: Software lock register for page 10. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock10`]
module"]
pub type SW_LOCK10 = crate::Reg<sw_lock10::SW_LOCK10_SPEC>;
#[doc = "Software lock register for page 10. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock10;
#[doc = "SW_LOCK11 (rw) register accessor: Software lock register for page 11. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock11`]
module"]
pub type SW_LOCK11 = crate::Reg<sw_lock11::SW_LOCK11_SPEC>;
#[doc = "Software lock register for page 11. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock11;
#[doc = "SW_LOCK12 (rw) register accessor: Software lock register for page 12. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock12`]
module"]
pub type SW_LOCK12 = crate::Reg<sw_lock12::SW_LOCK12_SPEC>;
#[doc = "Software lock register for page 12. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock12;
#[doc = "SW_LOCK13 (rw) register accessor: Software lock register for page 13. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock13`]
module"]
pub type SW_LOCK13 = crate::Reg<sw_lock13::SW_LOCK13_SPEC>;
#[doc = "Software lock register for page 13. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock13;
#[doc = "SW_LOCK14 (rw) register accessor: Software lock register for page 14. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock14`]
module"]
pub type SW_LOCK14 = crate::Reg<sw_lock14::SW_LOCK14_SPEC>;
#[doc = "Software lock register for page 14. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock14;
#[doc = "SW_LOCK15 (rw) register accessor: Software lock register for page 15. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock15`]
module"]
pub type SW_LOCK15 = crate::Reg<sw_lock15::SW_LOCK15_SPEC>;
#[doc = "Software lock register for page 15. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock15;
#[doc = "SW_LOCK16 (rw) register accessor: Software lock register for page 16. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock16`]
module"]
pub type SW_LOCK16 = crate::Reg<sw_lock16::SW_LOCK16_SPEC>;
#[doc = "Software lock register for page 16. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock16;
#[doc = "SW_LOCK17 (rw) register accessor: Software lock register for page 17. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock17`]
module"]
pub type SW_LOCK17 = crate::Reg<sw_lock17::SW_LOCK17_SPEC>;
#[doc = "Software lock register for page 17. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock17;
#[doc = "SW_LOCK18 (rw) register accessor: Software lock register for page 18. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock18`]
module"]
pub type SW_LOCK18 = crate::Reg<sw_lock18::SW_LOCK18_SPEC>;
#[doc = "Software lock register for page 18. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock18;
#[doc = "SW_LOCK19 (rw) register accessor: Software lock register for page 19. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock19`]
module"]
pub type SW_LOCK19 = crate::Reg<sw_lock19::SW_LOCK19_SPEC>;
#[doc = "Software lock register for page 19. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock19;
#[doc = "SW_LOCK20 (rw) register accessor: Software lock register for page 20. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock20`]
module"]
pub type SW_LOCK20 = crate::Reg<sw_lock20::SW_LOCK20_SPEC>;
#[doc = "Software lock register for page 20. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock20;
#[doc = "SW_LOCK21 (rw) register accessor: Software lock register for page 21. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock21`]
module"]
pub type SW_LOCK21 = crate::Reg<sw_lock21::SW_LOCK21_SPEC>;
#[doc = "Software lock register for page 21. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock21;
#[doc = "SW_LOCK22 (rw) register accessor: Software lock register for page 22. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock22`]
module"]
pub type SW_LOCK22 = crate::Reg<sw_lock22::SW_LOCK22_SPEC>;
#[doc = "Software lock register for page 22. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock22;
#[doc = "SW_LOCK23 (rw) register accessor: Software lock register for page 23. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock23`]
module"]
pub type SW_LOCK23 = crate::Reg<sw_lock23::SW_LOCK23_SPEC>;
#[doc = "Software lock register for page 23. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock23;
#[doc = "SW_LOCK24 (rw) register accessor: Software lock register for page 24. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock24`]
module"]
pub type SW_LOCK24 = crate::Reg<sw_lock24::SW_LOCK24_SPEC>;
#[doc = "Software lock register for page 24. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock24;
#[doc = "SW_LOCK25 (rw) register accessor: Software lock register for page 25. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock25`]
module"]
pub type SW_LOCK25 = crate::Reg<sw_lock25::SW_LOCK25_SPEC>;
#[doc = "Software lock register for page 25. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock25;
#[doc = "SW_LOCK26 (rw) register accessor: Software lock register for page 26. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock26`]
module"]
pub type SW_LOCK26 = crate::Reg<sw_lock26::SW_LOCK26_SPEC>;
#[doc = "Software lock register for page 26. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock26;
#[doc = "SW_LOCK27 (rw) register accessor: Software lock register for page 27. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock27`]
module"]
pub type SW_LOCK27 = crate::Reg<sw_lock27::SW_LOCK27_SPEC>;
#[doc = "Software lock register for page 27. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock27;
#[doc = "SW_LOCK28 (rw) register accessor: Software lock register for page 28. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock28`]
module"]
pub type SW_LOCK28 = crate::Reg<sw_lock28::SW_LOCK28_SPEC>;
#[doc = "Software lock register for page 28. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock28;
#[doc = "SW_LOCK29 (rw) register accessor: Software lock register for page 29. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock29`]
module"]
pub type SW_LOCK29 = crate::Reg<sw_lock29::SW_LOCK29_SPEC>;
#[doc = "Software lock register for page 29. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock29;
#[doc = "SW_LOCK30 (rw) register accessor: Software lock register for page 30. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock30`]
module"]
pub type SW_LOCK30 = crate::Reg<sw_lock30::SW_LOCK30_SPEC>;
#[doc = "Software lock register for page 30. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock30;
#[doc = "SW_LOCK31 (rw) register accessor: Software lock register for page 31. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock31`]
module"]
pub type SW_LOCK31 = crate::Reg<sw_lock31::SW_LOCK31_SPEC>;
#[doc = "Software lock register for page 31. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock31;
#[doc = "SW_LOCK32 (rw) register accessor: Software lock register for page 32. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock32`]
module"]
pub type SW_LOCK32 = crate::Reg<sw_lock32::SW_LOCK32_SPEC>;
#[doc = "Software lock register for page 32. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock32;
#[doc = "SW_LOCK33 (rw) register accessor: Software lock register for page 33. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock33`]
module"]
pub type SW_LOCK33 = crate::Reg<sw_lock33::SW_LOCK33_SPEC>;
#[doc = "Software lock register for page 33. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock33;
#[doc = "SW_LOCK34 (rw) register accessor: Software lock register for page 34. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock34`]
module"]
pub type SW_LOCK34 = crate::Reg<sw_lock34::SW_LOCK34_SPEC>;
#[doc = "Software lock register for page 34. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock34;
#[doc = "SW_LOCK35 (rw) register accessor: Software lock register for page 35. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock35`]
module"]
pub type SW_LOCK35 = crate::Reg<sw_lock35::SW_LOCK35_SPEC>;
#[doc = "Software lock register for page 35. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock35;
#[doc = "SW_LOCK36 (rw) register accessor: Software lock register for page 36. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock36`]
module"]
pub type SW_LOCK36 = crate::Reg<sw_lock36::SW_LOCK36_SPEC>;
#[doc = "Software lock register for page 36. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock36;
#[doc = "SW_LOCK37 (rw) register accessor: Software lock register for page 37. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock37`]
module"]
pub type SW_LOCK37 = crate::Reg<sw_lock37::SW_LOCK37_SPEC>;
#[doc = "Software lock register for page 37. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock37;
#[doc = "SW_LOCK38 (rw) register accessor: Software lock register for page 38. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock38`]
module"]
pub type SW_LOCK38 = crate::Reg<sw_lock38::SW_LOCK38_SPEC>;
#[doc = "Software lock register for page 38. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock38;
#[doc = "SW_LOCK39 (rw) register accessor: Software lock register for page 39. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock39`]
module"]
pub type SW_LOCK39 = crate::Reg<sw_lock39::SW_LOCK39_SPEC>;
#[doc = "Software lock register for page 39. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock39;
#[doc = "SW_LOCK40 (rw) register accessor: Software lock register for page 40. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock40`]
module"]
pub type SW_LOCK40 = crate::Reg<sw_lock40::SW_LOCK40_SPEC>;
#[doc = "Software lock register for page 40. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock40;
#[doc = "SW_LOCK41 (rw) register accessor: Software lock register for page 41. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock41`]
module"]
pub type SW_LOCK41 = crate::Reg<sw_lock41::SW_LOCK41_SPEC>;
#[doc = "Software lock register for page 41. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock41;
#[doc = "SW_LOCK42 (rw) register accessor: Software lock register for page 42. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock42`]
module"]
pub type SW_LOCK42 = crate::Reg<sw_lock42::SW_LOCK42_SPEC>;
#[doc = "Software lock register for page 42. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock42;
#[doc = "SW_LOCK43 (rw) register accessor: Software lock register for page 43. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock43`]
module"]
pub type SW_LOCK43 = crate::Reg<sw_lock43::SW_LOCK43_SPEC>;
#[doc = "Software lock register for page 43. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock43;
#[doc = "SW_LOCK44 (rw) register accessor: Software lock register for page 44. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock44`]
module"]
pub type SW_LOCK44 = crate::Reg<sw_lock44::SW_LOCK44_SPEC>;
#[doc = "Software lock register for page 44. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock44;
#[doc = "SW_LOCK45 (rw) register accessor: Software lock register for page 45. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock45`]
module"]
pub type SW_LOCK45 = crate::Reg<sw_lock45::SW_LOCK45_SPEC>;
#[doc = "Software lock register for page 45. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock45;
#[doc = "SW_LOCK46 (rw) register accessor: Software lock register for page 46. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock46`]
module"]
pub type SW_LOCK46 = crate::Reg<sw_lock46::SW_LOCK46_SPEC>;
#[doc = "Software lock register for page 46. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock46;
#[doc = "SW_LOCK47 (rw) register accessor: Software lock register for page 47. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock47`]
module"]
pub type SW_LOCK47 = crate::Reg<sw_lock47::SW_LOCK47_SPEC>;
#[doc = "Software lock register for page 47. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock47;
#[doc = "SW_LOCK48 (rw) register accessor: Software lock register for page 48. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock48`]
module"]
pub type SW_LOCK48 = crate::Reg<sw_lock48::SW_LOCK48_SPEC>;
#[doc = "Software lock register for page 48. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock48;
#[doc = "SW_LOCK49 (rw) register accessor: Software lock register for page 49. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock49`]
module"]
pub type SW_LOCK49 = crate::Reg<sw_lock49::SW_LOCK49_SPEC>;
#[doc = "Software lock register for page 49. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock49;
#[doc = "SW_LOCK50 (rw) register accessor: Software lock register for page 50. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock50`]
module"]
pub type SW_LOCK50 = crate::Reg<sw_lock50::SW_LOCK50_SPEC>;
#[doc = "Software lock register for page 50. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock50;
#[doc = "SW_LOCK51 (rw) register accessor: Software lock register for page 51. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock51`]
module"]
pub type SW_LOCK51 = crate::Reg<sw_lock51::SW_LOCK51_SPEC>;
#[doc = "Software lock register for page 51. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock51;
#[doc = "SW_LOCK52 (rw) register accessor: Software lock register for page 52. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock52`]
module"]
pub type SW_LOCK52 = crate::Reg<sw_lock52::SW_LOCK52_SPEC>;
#[doc = "Software lock register for page 52. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock52;
#[doc = "SW_LOCK53 (rw) register accessor: Software lock register for page 53. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock53`]
module"]
pub type SW_LOCK53 = crate::Reg<sw_lock53::SW_LOCK53_SPEC>;
#[doc = "Software lock register for page 53. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock53;
#[doc = "SW_LOCK54 (rw) register accessor: Software lock register for page 54. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock54`]
module"]
pub type SW_LOCK54 = crate::Reg<sw_lock54::SW_LOCK54_SPEC>;
#[doc = "Software lock register for page 54. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock54;
#[doc = "SW_LOCK55 (rw) register accessor: Software lock register for page 55. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock55`]
module"]
pub type SW_LOCK55 = crate::Reg<sw_lock55::SW_LOCK55_SPEC>;
#[doc = "Software lock register for page 55. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock55;
#[doc = "SW_LOCK56 (rw) register accessor: Software lock register for page 56. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock56`]
module"]
pub type SW_LOCK56 = crate::Reg<sw_lock56::SW_LOCK56_SPEC>;
#[doc = "Software lock register for page 56. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock56;
#[doc = "SW_LOCK57 (rw) register accessor: Software lock register for page 57. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock57`]
module"]
pub type SW_LOCK57 = crate::Reg<sw_lock57::SW_LOCK57_SPEC>;
#[doc = "Software lock register for page 57. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock57;
#[doc = "SW_LOCK58 (rw) register accessor: Software lock register for page 58. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock58`]
module"]
pub type SW_LOCK58 = crate::Reg<sw_lock58::SW_LOCK58_SPEC>;
#[doc = "Software lock register for page 58. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock58;
#[doc = "SW_LOCK59 (rw) register accessor: Software lock register for page 59. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock59`]
module"]
pub type SW_LOCK59 = crate::Reg<sw_lock59::SW_LOCK59_SPEC>;
#[doc = "Software lock register for page 59. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock59;
#[doc = "SW_LOCK60 (rw) register accessor: Software lock register for page 60. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock60`]
module"]
pub type SW_LOCK60 = crate::Reg<sw_lock60::SW_LOCK60_SPEC>;
#[doc = "Software lock register for page 60. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock60;
#[doc = "SW_LOCK61 (rw) register accessor: Software lock register for page 61. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock61`]
module"]
pub type SW_LOCK61 = crate::Reg<sw_lock61::SW_LOCK61_SPEC>;
#[doc = "Software lock register for page 61. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock61;
#[doc = "SW_LOCK62 (rw) register accessor: Software lock register for page 62. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock62`]
module"]
pub type SW_LOCK62 = crate::Reg<sw_lock62::SW_LOCK62_SPEC>;
#[doc = "Software lock register for page 62. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock62;
#[doc = "SW_LOCK63 (rw) register accessor: Software lock register for page 63. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page.  

You can [`read`](crate::Reg::read) this register and get [`sw_lock63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_lock63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sw_lock63`]
module"]
pub type SW_LOCK63 = crate::Reg<sw_lock63::SW_LOCK63_SPEC>;
#[doc = "Software lock register for page 63. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
pub mod sw_lock63;
#[doc = "SBPI_INSTR (rw) register accessor: Dispatch instructions to the SBPI interface, used for programming the OTP fuses.  

You can [`read`](crate::Reg::read) this register and get [`sbpi_instr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_instr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbpi_instr`]
module"]
pub type SBPI_INSTR = crate::Reg<sbpi_instr::SBPI_INSTR_SPEC>;
#[doc = "Dispatch instructions to the SBPI interface, used for programming the OTP fuses."]
pub mod sbpi_instr;
#[doc = "SBPI_WDATA_0 (rw) register accessor: SBPI write payload bytes 3..0  

You can [`read`](crate::Reg::read) this register and get [`sbpi_wdata_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_wdata_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbpi_wdata_0`]
module"]
pub type SBPI_WDATA_0 = crate::Reg<sbpi_wdata_0::SBPI_WDATA_0_SPEC>;
#[doc = "SBPI write payload bytes 3..0"]
pub mod sbpi_wdata_0;
#[doc = "SBPI_WDATA_1 (rw) register accessor: SBPI write payload bytes 7..4  

You can [`read`](crate::Reg::read) this register and get [`sbpi_wdata_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_wdata_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbpi_wdata_1`]
module"]
pub type SBPI_WDATA_1 = crate::Reg<sbpi_wdata_1::SBPI_WDATA_1_SPEC>;
#[doc = "SBPI write payload bytes 7..4"]
pub mod sbpi_wdata_1;
#[doc = "SBPI_WDATA_2 (rw) register accessor: SBPI write payload bytes 11..8  

You can [`read`](crate::Reg::read) this register and get [`sbpi_wdata_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_wdata_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbpi_wdata_2`]
module"]
pub type SBPI_WDATA_2 = crate::Reg<sbpi_wdata_2::SBPI_WDATA_2_SPEC>;
#[doc = "SBPI write payload bytes 11..8"]
pub mod sbpi_wdata_2;
#[doc = "SBPI_WDATA_3 (rw) register accessor: SBPI write payload bytes 15..12  

You can [`read`](crate::Reg::read) this register and get [`sbpi_wdata_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_wdata_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbpi_wdata_3`]
module"]
pub type SBPI_WDATA_3 = crate::Reg<sbpi_wdata_3::SBPI_WDATA_3_SPEC>;
#[doc = "SBPI write payload bytes 15..12"]
pub mod sbpi_wdata_3;
#[doc = "SBPI_RDATA_0 (rw) register accessor: Read payload bytes 3..0. Once read, the data in the register will automatically clear to 0.  

You can [`read`](crate::Reg::read) this register and get [`sbpi_rdata_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_rdata_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbpi_rdata_0`]
module"]
pub type SBPI_RDATA_0 = crate::Reg<sbpi_rdata_0::SBPI_RDATA_0_SPEC>;
#[doc = "Read payload bytes 3..0. Once read, the data in the register will automatically clear to 0."]
pub mod sbpi_rdata_0;
#[doc = "SBPI_RDATA_1 (rw) register accessor: Read payload bytes 7..4. Once read, the data in the register will automatically clear to 0.  

You can [`read`](crate::Reg::read) this register and get [`sbpi_rdata_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_rdata_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbpi_rdata_1`]
module"]
pub type SBPI_RDATA_1 = crate::Reg<sbpi_rdata_1::SBPI_RDATA_1_SPEC>;
#[doc = "Read payload bytes 7..4. Once read, the data in the register will automatically clear to 0."]
pub mod sbpi_rdata_1;
#[doc = "SBPI_RDATA_2 (rw) register accessor: Read payload bytes 11..8. Once read, the data in the register will automatically clear to 0.  

You can [`read`](crate::Reg::read) this register and get [`sbpi_rdata_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_rdata_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbpi_rdata_2`]
module"]
pub type SBPI_RDATA_2 = crate::Reg<sbpi_rdata_2::SBPI_RDATA_2_SPEC>;
#[doc = "Read payload bytes 11..8. Once read, the data in the register will automatically clear to 0."]
pub mod sbpi_rdata_2;
#[doc = "SBPI_RDATA_3 (rw) register accessor: Read payload bytes 15..12. Once read, the data in the register will automatically clear to 0.  

You can [`read`](crate::Reg::read) this register and get [`sbpi_rdata_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_rdata_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbpi_rdata_3`]
module"]
pub type SBPI_RDATA_3 = crate::Reg<sbpi_rdata_3::SBPI_RDATA_3_SPEC>;
#[doc = "Read payload bytes 15..12. Once read, the data in the register will automatically clear to 0."]
pub mod sbpi_rdata_3;
#[doc = "SBPI_STATUS (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`sbpi_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbpi_status`]
module"]
pub type SBPI_STATUS = crate::Reg<sbpi_status::SBPI_STATUS_SPEC>;
#[doc = ""]
pub mod sbpi_status;
#[doc = "USR (rw) register accessor: Controls for APB data read interface (USER interface)  

You can [`read`](crate::Reg::read) this register and get [`usr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usr`]
module"]
pub type USR = crate::Reg<usr::USR_SPEC>;
#[doc = "Controls for APB data read interface (USER interface)"]
pub mod usr;
#[doc = "DBG (rw) register accessor: Debug for OTP power-on state machine  

You can [`read`](crate::Reg::read) this register and get [`dbg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbg`]
module"]
pub type DBG = crate::Reg<dbg::DBG_SPEC>;
#[doc = "Debug for OTP power-on state machine"]
pub mod dbg;
#[doc = "BIST (rw) register accessor: During BIST, count address locations that have at least one leaky bit  

You can [`read`](crate::Reg::read) this register and get [`bist::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bist::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bist`]
module"]
pub type BIST = crate::Reg<bist::BIST_SPEC>;
#[doc = "During BIST, count address locations that have at least one leaky bit"]
pub mod bist;
#[doc = "CRT_KEY_W0 (rw) register accessor: Word 0 (bits 31..0) of the key. Write only, read returns 0x0  

You can [`read`](crate::Reg::read) this register and get [`crt_key_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crt_key_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crt_key_w0`]
module"]
pub type CRT_KEY_W0 = crate::Reg<crt_key_w0::CRT_KEY_W0_SPEC>;
#[doc = "Word 0 (bits 31..0) of the key. Write only, read returns 0x0"]
pub mod crt_key_w0;
#[doc = "CRT_KEY_W1 (rw) register accessor: Word 1 (bits 63..32) of the key. Write only, read returns 0x0  

You can [`read`](crate::Reg::read) this register and get [`crt_key_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crt_key_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crt_key_w1`]
module"]
pub type CRT_KEY_W1 = crate::Reg<crt_key_w1::CRT_KEY_W1_SPEC>;
#[doc = "Word 1 (bits 63..32) of the key. Write only, read returns 0x0"]
pub mod crt_key_w1;
#[doc = "CRT_KEY_W2 (rw) register accessor: Word 2 (bits 95..64) of the key. Write only, read returns 0x0  

You can [`read`](crate::Reg::read) this register and get [`crt_key_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crt_key_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crt_key_w2`]
module"]
pub type CRT_KEY_W2 = crate::Reg<crt_key_w2::CRT_KEY_W2_SPEC>;
#[doc = "Word 2 (bits 95..64) of the key. Write only, read returns 0x0"]
pub mod crt_key_w2;
#[doc = "CRT_KEY_W3 (rw) register accessor: Word 3 (bits 127..96) of the key. Write only, read returns 0x0  

You can [`read`](crate::Reg::read) this register and get [`crt_key_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crt_key_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crt_key_w3`]
module"]
pub type CRT_KEY_W3 = crate::Reg<crt_key_w3::CRT_KEY_W3_SPEC>;
#[doc = "Word 3 (bits 127..96) of the key. Write only, read returns 0x0"]
pub mod crt_key_w3;
#[doc = "CRITICAL (rw) register accessor: Quickly check values of critical flags read during boot up  

You can [`read`](crate::Reg::read) this register and get [`critical::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`critical::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@critical`]
module"]
pub type CRITICAL = crate::Reg<critical::CRITICAL_SPEC>;
#[doc = "Quickly check values of critical flags read during boot up"]
pub mod critical;
#[doc = "KEY_VALID (rw) register accessor: Which keys were valid (enrolled) at boot time  

You can [`read`](crate::Reg::read) this register and get [`key_valid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_valid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key_valid`]
module"]
pub type KEY_VALID = crate::Reg<key_valid::KEY_VALID_SPEC>;
#[doc = "Which keys were valid (enrolled) at boot time"]
pub mod key_valid;
#[doc = "DEBUGEN (rw) register accessor: Enable a debug feature that has been disabled. Debug features are disabled if one of the relevant critical boot flags is set in OTP (DEBUG_DISABLE or SECURE_DEBUG_DISABLE), OR if a debug key is marked valid in OTP, and the matching key value has not been supplied over SWD. Specifically: - The DEBUG_DISABLE flag disables all debug features. This can be fully overridden by setting all bits of this register. - The SECURE_DEBUG_DISABLE flag disables secure processor debug. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If a single debug key has been registered, and no matching key value has been supplied over SWD, then all debug features are disabled. This can be fully overridden by setting all bits of this register. - If both debug keys have been registered, and the Non-secure key's value (key 6) has been supplied over SWD, secure processor debug is disabled. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If both debug keys have been registered, and the Secure key's value (key 5) has been supplied over SWD, then no debug features are disabled by the key mechanism. However, note that in this case debug features may still be disabled by the critical boot flags.  

You can [`read`](crate::Reg::read) this register and get [`debugen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@debugen`]
module"]
pub type DEBUGEN = crate::Reg<debugen::DEBUGEN_SPEC>;
#[doc = "Enable a debug feature that has been disabled. Debug features are disabled if one of the relevant critical boot flags is set in OTP (DEBUG_DISABLE or SECURE_DEBUG_DISABLE), OR if a debug key is marked valid in OTP, and the matching key value has not been supplied over SWD. Specifically: - The DEBUG_DISABLE flag disables all debug features. This can be fully overridden by setting all bits of this register. - The SECURE_DEBUG_DISABLE flag disables secure processor debug. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If a single debug key has been registered, and no matching key value has been supplied over SWD, then all debug features are disabled. This can be fully overridden by setting all bits of this register. - If both debug keys have been registered, and the Non-secure key's value (key 6) has been supplied over SWD, secure processor debug is disabled. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If both debug keys have been registered, and the Secure key's value (key 5) has been supplied over SWD, then no debug features are disabled by the key mechanism. However, note that in this case debug features may still be disabled by the critical boot flags."]
pub mod debugen;
#[doc = "DEBUGEN_LOCK (rw) register accessor: Write 1s to lock corresponding bits in DEBUGEN. This register is reset by the processor cold reset.  

You can [`read`](crate::Reg::read) this register and get [`debugen_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugen_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@debugen_lock`]
module"]
pub type DEBUGEN_LOCK = crate::Reg<debugen_lock::DEBUGEN_LOCK_SPEC>;
#[doc = "Write 1s to lock corresponding bits in DEBUGEN. This register is reset by the processor cold reset."]
pub mod debugen_lock;
#[doc = "ARCHSEL (rw) register accessor: Architecture select (Arm/RISC-V). The default and allowable values of this register are constrained by the critical boot flags. This register is reset by the earliest reset in the switched core power domain (before a processor cold reset). Cores sample their architecture select signal on a warm reset. The source of the warm reset could be the system power-up state machine, the watchdog timer, Arm SYSRESETREQ or from RISC-V hartresetreq. Note that when an Arm core is deselected, its cold reset domain is also held in reset, since in particular the SYSRESETREQ bit becomes inaccessible once the core is deselected. Note also the RISC-V cores do not have a cold reset domain, since their corresponding controls are located in the Debug Module.  

You can [`read`](crate::Reg::read) this register and get [`archsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`archsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@archsel`]
module"]
pub type ARCHSEL = crate::Reg<archsel::ARCHSEL_SPEC>;
#[doc = "Architecture select (Arm/RISC-V). The default and allowable values of this register are constrained by the critical boot flags. This register is reset by the earliest reset in the switched core power domain (before a processor cold reset). Cores sample their architecture select signal on a warm reset. The source of the warm reset could be the system power-up state machine, the watchdog timer, Arm SYSRESETREQ or from RISC-V hartresetreq. Note that when an Arm core is deselected, its cold reset domain is also held in reset, since in particular the SYSRESETREQ bit becomes inaccessible once the core is deselected. Note also the RISC-V cores do not have a cold reset domain, since their corresponding controls are located in the Debug Module."]
pub mod archsel;
#[doc = "ARCHSEL_STATUS (rw) register accessor: Get the current architecture select state of each core. Cores sample the current value of the ARCHSEL register when their warm reset is released, at which point the corresponding bit in this register will also update.  

You can [`read`](crate::Reg::read) this register and get [`archsel_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`archsel_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@archsel_status`]
module"]
pub type ARCHSEL_STATUS = crate::Reg<archsel_status::ARCHSEL_STATUS_SPEC>;
#[doc = "Get the current architecture select state of each core. Cores sample the current value of the ARCHSEL register when their warm reset is released, at which point the corresponding bit in this register will also update."]
pub mod archsel_status;
#[doc = "BOOTDIS (rw) register accessor: Tell the bootrom to ignore scratch register boot vectors (both power manager and watchdog) on the next power up. If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by performing a watchdog reset that resets the OTP. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the power manager BOOTDIS register.  

You can [`read`](crate::Reg::read) this register and get [`bootdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootdis`]
module"]
pub type BOOTDIS = crate::Reg<bootdis::BOOTDIS_SPEC>;
#[doc = "Tell the bootrom to ignore scratch register boot vectors (both power manager and watchdog) on the next power up. If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by performing a watchdog reset that resets the OTP. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the power manager BOOTDIS register."]
pub mod bootdis;
#[doc = "INTR (rw) register accessor: Raw Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Raw Interrupts"]
pub mod intr;
#[doc = "INTE (rw) register accessor: Interrupt Enable  

You can [`read`](crate::Reg::read) this register and get [`inte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@inte`]
module"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable"]
pub mod inte;
#[doc = "INTF (rw) register accessor: Interrupt Force  

You can [`read`](crate::Reg::read) this register and get [`intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt Force"]
pub mod intf;
#[doc = "INTS (rw) register accessor: Interrupt status after masking &amp; forcing  

You can [`read`](crate::Reg::read) this register and get [`ints::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ints::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ints`]
module"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status after masking &amp; forcing"]
pub mod ints;
