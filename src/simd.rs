//! SIMD Flags of the host architecture.


// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.



#[cfg(target_arch = "x86")]
use core::arch::x86::{
	__cpuid       as cpuid,
	__cpuid_count as cpuidex,
};

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
	__cpuid       as cpuid,
	__cpuid_count as cpuidex,
	_xgetbv       as xgetbv,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SIMDFlags {
	/// AVX-512 extensions flags.
	avx512: u32,

	/// Standard SIMD flags.
	simd: u32,
}



impl SIMDFlags {
	// SIMD Flags.
	// ****************************************************

	/// FMA (FLoating point Multiply Accumulate).
	pub const FMA  : u32 = 1 << 0;
	/// FMA2 (FLoating point Multiply Accumulate).
	pub const FMA2 : u32 = 1 << 1;
	/// FMA3 (FLoating point Multiply Accumulate).
	pub const FMA3 : u32 = 1 << 2;
	/// FMA4 (FLoating point Multiply Accumulate).
	pub const FMA4 : u32 = 1 << 3;


	/// MMX vector extension.
	pub const MMX : u32 = 1 << 4;


	/// SSE vector extension.
	pub const SSE1  : u32 = 1 << 5;

	/// SSE2 vector extension.
	pub const SSE2  : u32 = 1 << 6;

	/// SSE3 vector extension.
	pub const SSE3  : u32 = 1 << 7;

	/// SSSE3 vector extension.
	pub const SSSE3 : u32 = 1 << 8;

	/// SSE4.1 vector extension.
	pub const SSE41 : u32 = 1 << 9;

	/// SSE4.2 vector extension.
	pub const SSE42 : u32 = 1 << 10;

	/// SSE4.A vector extension.
	pub const SSE4A : u32 = 1 << 11;

	/// SSE5 vector extension.
	pub const SSE5  : u32 = 1 << 12;


	/// XOP vector extension.
	pub const XOP   : u32 = 1 << 14;


	/// AVX vector extension.
	pub const AVX    : u32 = 1 << 16;

	/// AVX-2 vector extension.
	pub const AVX2   : u32 = 1 << 17;

	/// AVX-512 vector extension.
	pub const AVX512 : u32 = 1 << 18;


	/// XSAVE instruction.
	pub const XSAVE   : u32 = 1 << 24;

	/// OS enabled XSAVE.
	pub const OSXSAVE : u32 = 1 << 25;

	// ****************************************************



	// AVX512 extension flags.
	// ****************************************************

	pub const AVX512F    : u32 = 1 <<  1;
	pub const AVX512DQ   : u32 = 1 <<  2;
	pub const AVX512IFMA : u32 = 1 <<  3;
	pub const AVX512PF   : u32 = 1 <<  4;
	pub const AVX512ER   : u32 = 1 <<  5;
	pub const AVX512CD   : u32 = 1 <<  6;
	pub const AVX512BW   : u32 = 1 <<  7;
	pub const AVX512VL   : u32 = 1 <<  8;

	pub const AVX512GFNI      : u32 = 1 <<  9;
	pub const AVX512VBMI      : u32 = 1 << 10;
	pub const AVX512VBMI2     : u32 = 1 << 11;
	pub const AVX512VNNI      : u32 = 1 << 12;
	pub const AVX512BITALG    : u32 = 1 << 13;
	pub const AVX512VPOPCNTDQ : u32 = 1 << 14;

	pub const AVX5124VNNIW       : u32 = 1 << 16;
	pub const AVX5124FMAPS       : u32 = 1 << 17;
	pub const AVX512VP2INTERSECT : u32 = 1 << 18;

	pub const AVX512BF16 : u32 = 1 << 24;

	pub const AVX512VPCLMUL : u32 = 1 << 28;
	// ****************************************************




	/// Returns `true` if the SIMD flag given is present.
	#[inline(always)]
	pub fn simd(&self, f: u32) -> bool {
		(self.simd & f) != 0
	}



	/// Reads the SIMD flags from CPUID.
	pub fn read() -> SIMDFlags {
		let (simd, avx512) = Self::inner();

		SIMDFlags { simd, avx512 }
	}


	/// Performs CPUID checks for SIMD features.
	fn inner() -> (u32, u32) {

		// Get number of low and high leafs.
		let lo = unsafe { cpuid(0x00000000).eax };
		let hi = unsafe { cpuid(0x80000000).eax };


		let mut simd   = 0;
		let mut avx512 = 0;

		if lo < 0x1 { return (simd, avx512); }

		// CPUID 0x00000001
		{
			let info = unsafe { cpuid(0x00000001) };

			if ( info.ecx        & 1) == 1 { simd |= Self::SSE3  }
			if ((info.ecx >>  9) & 1) == 1 { simd |= Self::SSSE3 }
			if ((info.ecx >> 19) & 1) == 1 { simd |= Self::SSE41 }
			if ((info.ecx >> 20) & 1) == 1 { simd |= Self::SSE42 }

			if ((info.edx >> 23) & 1) == 1 { simd |= Self::MMX   }
			if ((info.edx >> 25) & 1) == 1 { simd |= Self::SSE1  }
			if ((info.edx >> 26) & 1) == 1 { simd |= Self::SSE2  }

			match (info.ecx >> 26) & 0b111 {
				0b111 => {
					simd |= Self::OSXSAVE | Self::XSAVE;

					let xcr = unsafe { xgetbv(0) };

					match xcr & 0xE6 {
						0xE6 => simd |= Self::AVX | Self::AVX512,
						0x6  => simd |= Self::AVX,
						_ => (),

					}
				},

				0b011 => simd |= Self::OSXSAVE | Self::XSAVE,

				0b001 => simd |= Self::XSAVE,

				_ => (),
			}
		}

		if lo < 0x7 { return (simd, avx512); }

		// CPUID 0x00000007
		{
			let info = unsafe { cpuid(0x00000007) };

			if ((info.ebx >> 3)  & 1) == 1 { simd |= Self::AVX2 }


			if ((info.ebx >> 16) & 1) == 1 { avx512 |= Self::AVX512F    }
			if ((info.ebx >> 17) & 1) == 1 { avx512 |= Self::AVX512DQ   }
			if ((info.ebx >> 21) & 1) == 1 { avx512 |= Self::AVX512IFMA }
			if ((info.ebx >> 26) & 1) == 1 { avx512 |= Self::AVX512PF   }
			if ((info.ebx >> 27) & 1) == 1 { avx512 |= Self::AVX512ER   }
			if ((info.ebx >> 28) & 1) == 1 { avx512 |= Self::AVX512CD   }
			if ((info.ebx >> 30) & 1) == 1 { avx512 |= Self::AVX512BW   }
			if ((info.ebx >> 31) & 1) == 1 { avx512 |= Self::AVX512VL   }

			if ((info.ecx >>  1) & 1) == 1 { avx512 |= Self::AVX512VBMI      }
			if ((info.ecx >>  6) & 1) == 1 { avx512 |= Self::AVX512VBMI2     }
			if ((info.ecx >> 11) & 1) == 1 { avx512 |= Self::AVX512VNNI      }
			if ((info.ecx >> 12) & 1) == 1 { avx512 |= Self::AVX512BITALG    }
			if ((info.ecx >> 14) & 1) == 1 { avx512 |= Self::AVX512VPOPCNTDQ }


			if ((info.edx >>  2) & 1) == 1 { avx512 |= Self::AVX5124VNNIW       }
			if ((info.edx >>  3) & 1) == 1 { avx512 |= Self::AVX5124FMAPS       }
			if ((info.edx >>  8) & 1) == 1 { avx512 |= Self::AVX512VP2INTERSECT }

			let info = unsafe { cpuidex(0x7, 1) };

			if ((info.eax >> 5) & 1) == 1 { avx512 |= Self::AVX512BF16 }
		}

		if hi < 0x80000001 { return (simd, avx512); }

		// CPUID 0x80000001
		{
			let info = unsafe { cpuid(0x80000001) };

			if ((info.ecx >>  6) & 1) == 1 { simd |= Self::SSE4A }
			if ((info.ecx >> 11) & 1) == 1 { simd |= Self::XOP   }
			if ((info.ecx >> 16) & 1) == 1 { simd |= Self::FMA4  }

			if ((info.edx >> 23) & 1) == 1 { simd |= Self::MMX   }
		}

		(simd, avx512)
	}
}

impl std::fmt::Display for SIMDFlags {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut args = String::new();

		args += "CPU SIMD features\n";

		if self.simd(Self::MMX) { args += "  MMX\n\n"; }

		args += "SIMD 128-bit:\n";

		if self.simd(Self::SSE1)  { args += "  SSE    (Streaming SIMD Extensions)\n"; }
		if self.simd(Self::SSE2)  { args += "  SSE2   (Streaming SIMD Extensions) (Instruction Set   2)\n"; }
		if self.simd(Self::SSE3)  { args += "  SSE3   (Streaming SIMD Extensions) (Instruction Set   3)\n"; }
		if self.simd(Self::SSE41) { args += "  SSE4.1 (Streaming SIMD Extensions) (Instruction Set 4.1)\n"; }
		if self.simd(Self::SSE42) { args += "  SSE4.2 (Streaming SIMD Extensions) (Instruction Set 4.2)\n"; }
		if self.simd(Self::SSE4A) { args += "  SSE4.A (Streaming SIMD Extensions) (Instruction Set 4.A)\n"; }
		if self.simd(Self::SSE5)  { args += "  SSE5   (Streaming SIMD Extensions) (Instruction Set   5)\n"; }
		if self.simd(Self::SSSE3) { args += "  SSSE3  (Supplemental Streaming SIMD Extensions) (Instruction Set 3)\n\n"; }

		if self.simd(Self::XOP) { args += "  XOP (eXtended Operations) (AMD®)\n\n"; }

		args += "SIMD 256-bit:\n";


		if self.simd(Self::AVX)    { args += "  AVX    (Advanced Vector Extensions)\n"; }
		if self.simd(Self::AVX2)   { args += "  AVX2   (Advanced Vector Extensions) (Instruction Set 2)\n\n"; }

		args += "SIMD 512-bit:\n";

		if self.simd(Self::AVX512) { args += "  AVX512 (Advanced Vector Extensions)\n\n"; }


		f.write_str(&args)
	}
}
