//! Information on microarchitecture and model.


// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.



mod uarch;
mod vendor;


pub use self::{ uarch::CPUModel, vendor::CPUVendor };

#[derive(Debug, Clone, Copy)]
pub struct Model {
	/// Manufacturer and vendor of the CPU.
	vendor: CPUVendor,

	/// Specific model of the CPU (family and model).
	model: CPUModel,
}


impl Model {
	// Creates an empty struct.
	pub const fn empty() -> Self {
		Model {
			vendor: CPUVendor::Unknown,
			model: CPUModel::Unknown(0x00),
		}
	}

	/// Reads the model information from the CPUID registers.
	pub fn read() -> Self {
		match unsafe { core::arch::x86_64::__cpuid(0).eax } {
			0 => Model::empty(),
			_ => {
				let mut model = Model::empty();
				model.decode();
				model
			},
		}
	}

	/// Returns the vendor.
	pub fn vendor(&self) -> CPUVendor {
		self.vendor
	}

	/// Returns the model.
	pub fn model(&self) -> CPUModel {
		self.model
	}

	/// Decodes the model information from the raw CPUID registers.
	fn decode(&mut self) {
		// First decode the vendor.
		let leaf0 = unsafe { core::arch::x86_64::__cpuid(0x00) };

		self.vendor = CPUVendor::from( (leaf0.ebx, leaf0.edx, leaf0.ecx) );


		// Then decode the model and family.
		if leaf0 .eax >= 0x01 {
			let eax = unsafe { core::arch::x86_64::__cpuid(0x01).eax };

			let _stepping = (eax      ) & 0x0F;
			let bmodel   = (eax >>  4) & 0x0F;
			let bfamily  = (eax >>  8) & 0x0F;

			let _ptype    = (eax >> 12) & 0x03;

			let xmodel   = (eax >> 16) & 0x0F;
			let xfamily  = (eax >> 20) & 0xFF;

			let family = bfamily + (xfamily    );
			let model  = bmodel  + (xmodel << 4);

			self.model = CPUModel::from(self.vendor, family, model, xmodel);
		}
	}
}




impl core::fmt::Display for Model {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{}\n{}\n", self.vendor, self.model)
	}
}