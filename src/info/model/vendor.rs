//! Vendor of the CPU.
//! Decoded from CPUID register 0.


// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CPUVendor {
	/// Intel Corporation.
	Intel,

	/// Advanced Micro Devices.
	AMD,

	/// Centaur Technology.
	Centaur,

	/// Hygon. AMD-Chinese joint venture.
	Hygon,

	/// Transmeta Corporation.
	Transmeta,

	/// Cyrix Corporation.
	Cyrix,

	/// Rise Technology.
	Rise,

	/// National Semiconductor.
	NSC,

	/// Silicon Integrated Systems.
	SIS,

	/// NexGen.
	NexGen,

	/// United Microelectronics Corporation.
	UMC,

	/// 
	RDC,

	/// DM&P Electronics.
	DMP,

	// Shangai Zhaoxin Semiconductor (Zhaozin).
	Zhaoxin,

	/// Elbrus.
	Elbrus,

	Unknown,
}




impl CPUVendor {
	/// Decode the CPU vendor from the CPU vendor string.
	pub fn from(string: (u32, u32, u32)) -> CPUVendor {
		match string {
			INTEL      => CPUVendor::Intel,

			AMD1       => CPUVendor::AMD,
			AMD2       => CPUVendor::AMD,
			AMD3       => CPUVendor::AMD,

			CENTAUR1   => CPUVendor::Centaur,
			CENTAUR2   => CPUVendor::Centaur,

			TRANSMETA1 => CPUVendor::Transmeta,
			TRANSMETA2 => CPUVendor::Transmeta,

			HYGON      => CPUVendor::Hygon,

			CYRIX      => CPUVendor::Cyrix,

			RISE       => CPUVendor::Rise,

			NSC        => CPUVendor::NSC,

			SIS        => CPUVendor::SIS,

			NEXGEN     => CPUVendor::NexGen,

			UMC        => CPUVendor::UMC,

			RDC        => CPUVendor::RDC,

			VORTEX     => CPUVendor::DMP,

			ZHAOXIN    => CPUVendor::Zhaoxin,

			_          => CPUVendor::Unknown,
		}
	}
}


impl core::fmt::Display for CPUVendor {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use CPUVendor::*;

		let args = match *self {
			Intel     => "Intel",
			AMD       => "AMD",
			Centaur   =>"Centaur",
			Hygon     => "Hygon",
			Transmeta => "Transmeta",
			Cyrix     => "Cyrix",
			Rise      => "",
			NSC       => "National Semiconductors",
			SIS       => "Silicon Integrated Systems",
			NexGen    => "NexGen",
			UMC       => "",
			RDC       => "",
			DMP       => "DM&P",
			Zhaoxin   => "Zhaoxin",
			Elbrus    => "Elbrus",
			Unknown   => "Unknown",
		};

		f.write_str(args)
	}
}



const INTEL      : (u32, u32, u32) = (0x756E6547, 0x49656E69, 0x6C65746E);

const AMD1       : (u32, u32, u32) = (0x68747541, 0x69746E65, 0x444D4163);
const AMD2       : (u32, u32, u32) = (0x69444D41, 0x74656273, 0x21726574);
const AMD3       : (u32, u32, u32) = (0x20444D41, 0x45425349, 0x52455454);

const CENTAUR1   : (u32, u32, u32) = (0x746E6543, 0x48727561, 0x736C7561);
const CENTAUR2   : (u32, u32, u32) = (0x20414956, 0x20414956, 0x20414956);

const HYGON      : (u32, u32, u32) = (0x6F677948, 0x6E65476E, 0x656E6975);

const TRANSMETA2 : (u32, u32, u32) = (0x756E6547, 0x54656E69, 0x3638784D);
const TRANSMETA1 : (u32, u32, u32) = (0x6E617254, 0x74656D73, 0x55504361);

const CYRIX      : (u32, u32, u32) = (0x69727943, 0x736E4978, 0x64616574);

const RISE       : (u32, u32, u32) = (0x65736952, 0x65736952, 0x65736952);

const NSC        : (u32, u32, u32) = (0x646F6547, 0x79622065, 0x43534E20);

const SIS        : (u32, u32, u32) = (0x20536953, 0x20536953, 0x20536953);

const NEXGEN     : (u32, u32, u32) = (0x4778654E, 0x72446E65, 0x6E657669);

const UMC        : (u32, u32, u32) = (0x20434D55, 0x20434D55, 0x20434D55);

const RDC        : (u32, u32, u32) = (0x756E6547, 0x20656E69, 0x43445220);

const VORTEX     : (u32, u32, u32) = (0x74726F56, 0x36387865, 0x436F5320);

const ZHAOXIN    : (u32, u32, u32) = (0x68532020, 0x68676E61, 0x20206961);
