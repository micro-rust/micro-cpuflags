//! CPU Information accessible through CPUID.


// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.




mod model;


pub use self::model::Model;


#[cfg(target_arch = "x86")]
use core::arch::x86::{
	__cpuid       as cpuid,
	__cpuid_count as cpuidex,
};

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
	__cpuid       as cpuid,
	__cpuid_count as cpuidex,
};



#[derive(Debug, Clone)]
pub struct CPUInfo {
	/// x86 or x64 architecture.
	x64: bool,

	/// Base and max CPU frequency.
	freq: (u32, u32),

	/// Vendor & UArch.
	model: Model,

	/// Vendor brand encoded in UTF-8.
	vbrand: String,

	/// Processor brand encoded in UTF-8.
	pbrand: String,

	/// Vendor ID.
	vid: u32,

	/// Product ID.
	pid: u32,
}


impl CPUInfo {
	/// Returns the frequencies of the CPU.
	pub fn freq(&self) -> (u32, u32) {
		self.freq
	}

	/// Returns the Vendor Brand of the CPU.
	pub fn vbrand(&self) -> String {
		self.vbrand.clone()
	}

	/// Returns the Product Brand of the CPU.
	pub fn pbrand(&self) -> String {
		self.pbrand.clone()
	}

	/// Return the model of the CPU.
	pub fn model(&self) -> Model {
		self.model
	}

	/// Returns the VID and PID of the CPU.
	pub fn ids(&self) -> (u32, u32) {
		(self.vid, self.pid)
	}

	/// Reads the CPU Info from the CPUID results.
	pub fn read() -> CPUInfo {
		// Get number of low and high leafs.
		let lo = unsafe { cpuid(0x00000000).eax };
		let hi = unsafe { cpuid(0x80000000).eax };

		let mut x64 = false;

		let (mut base, mut max) = (0, 0);

		let (mut vid, mut pid) = (0, 0);

		let (mut vbrand, mut pbrand) = (String::with_capacity(48), String::with_capacity(48));

		// Get CPU frequencies.
		if lo >= 0x00000016 {
			let info = unsafe { cpuid(0x00000016) };

			base = info.eax;
			max  = info.ebx;
		}


		// Get CPU VID and PID. Read if possible Vendor Brand.
		if lo >= 0x00000017 {
			let info = unsafe { cpuid(0x00000017) };

			vid = info.ebx;
			pid = info.ecx;

			let mut rawstr = [0u32; 12];

			for i in 0..core::cmp::min(info.eax, 3) {
				let raw = unsafe { cpuidex(0x00000017, i + 1) };

				rawstr[(i * 4    ) as usize] = raw.eax;
				rawstr[(i * 4 + 1) as usize] = raw.ebx;
				rawstr[(i * 4 + 2) as usize] = raw.ecx;
				rawstr[(i * 4 + 3) as usize] = raw.edx;
			}

			let mut last = 0;

			let rawstr = unsafe { core::slice::from_raw_parts( &rawstr as *const u32 as *const u8, 12 * 4) };

			for c in rawstr {
				if *c == 0 { break }
				last += 1;
			}

			vbrand = unsafe { String::from_utf8_unchecked(rawstr[0..last].to_vec()) };
		}

		// Check if x64.
		if hi >= 0x80000001 {
			let info = unsafe { cpuid(0x80000001) };

			x64 = ((info.edx >> 29) & 1) == 1;
		}

		// Get CPU Product Brand.
		if hi >= 0x80000004 {
			let mut rawstr = [0u32; 12];

			for i in 0..3 {
				let raw = unsafe { cpuid(0x80000002 + i) };

				rawstr[(i * 4    ) as usize] = raw.eax;
				rawstr[(i * 4 + 1) as usize] = raw.ebx;
				rawstr[(i * 4 + 2) as usize] = raw.ecx;
				rawstr[(i * 4 + 3) as usize] = raw.edx;
			}

			let mut last = 0;

			let rawstr = unsafe { core::slice::from_raw_parts( &rawstr as *const u32 as *const u8, 12 * 4) };

			for c in rawstr {
				if *c == 0 { break }
				last += 1;
			}

			pbrand = unsafe { String::from_utf8_unchecked(rawstr[0..last].to_vec()) };
		}


		CPUInfo {
			x64,

			model: Model::read(),

			freq: (base, max),

			vbrand,
			pbrand,

			vid,
			pid,
		}
	}
}
