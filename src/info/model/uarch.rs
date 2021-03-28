//! Micro architecture of the processor.
//! Contains the information of the specific model of the CPU.


// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.



use super::CPUVendor;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CPUModel {
	// Intel
	Pentium5(u8),
	Pentium6(u8),
	Quark(u8),
	Dothan(u8),
	Yonah(u8),
	Conroe(u8),
	Penryn(u8),
	Nehalem(u8),
	SandyBridge(u8),
	IvyBridge(u8),
	Haswell(u8),
	Broadwell(u8),
	SkyLake(u8),
	PalmCove(u8),
	SunnyCove(u8),
	Bonnel(u8),
	Saltwell(u8),
	Silvermont(u8),
	Airmont(u8),
	Goldmont(u8),
	GoldmontPlus(u8),
	KnightsLanding(u8),
	KnightsMill(u8),
	Willamette(u8),
	Prescott(u8),

	// AMD 
	K5(u8),
	K6(u8),
	Geode(u8),
	K7(u8),
	K8(u8),
	K10(u8),
	Bobcat(u8),
	Bulldozer(u8),
	Piledriver(u8),
	Steamroller(u8),
	Excavator(u8),
	Puma(u8),
	Jaguar(u8),
	Zen(u8),
	Zen2(u8),
	Zen3(u8),

	// Hygon
	Dhyana(u8),

	Unknown(u8),
}


impl CPUModel {
	pub fn from(vendor: CPUVendor, family: u32, model: u32, xmodel: u32) -> CPUModel {
		use CPUModel::*;

		match vendor {
			CPUVendor::Intel => match family {
				0x05 => match model {
					0x01 =>  Pentium5(0x01),
					0x02 =>  Pentium5(0x02),
					0x03 =>  Pentium5(0x03),
					0x04 =>  Pentium5(0x04),

					0x09 =>  Quark(0x09),

					_ => Unknown(0x00),
				},

				0x06 => match model {
					0x01 =>  Pentium6(0x01),
					0x03 =>  Pentium6(0x03),
					0x05 =>  Pentium6(0x05),
					0x06 =>  Pentium6(0x06),
					0x07 =>  Pentium6(0x07),
					0x08 =>  Pentium6(0x08),
					0x0A =>  Pentium6(0x0A),
					0x0B =>  Pentium6(0x0B),

					0x09 =>  Dothan(0x09),
					0x0D =>  Dothan(0x0D),
					0x15 =>  Dothan(0x15),

					0x0E =>  Yonah(0x0E),

					0x0F =>  Conroe(0x0F),
					0x16 =>  Conroe(0x16),

					0x17 =>  Penryn(0x17),
					0x1D =>  Penryn(0x1D),

					0x1A =>  Nehalem(0x1A),
					0x1E =>  Nehalem(0x1E),
					0x1F =>  Nehalem(0x1F),
					0x2E =>  Nehalem(0x2E),
					0x25 =>  Nehalem(0x25),
					0x2C =>  Nehalem(0x2C),
					0x2F =>  Nehalem(0x2F),

					0x2A =>  SandyBridge(0x2A),
					0x2D =>  SandyBridge(0x2D),

					0x3A =>  IvyBridge(0x3A),
					0x3E =>  IvyBridge(0x3E),

					0x3C =>  Haswell(0x3C),
					0x3F =>  Haswell(0x3F),
					0x45 =>  Haswell(0x45),
					0x46 =>  Haswell(0x46),

					0x3D =>  Broadwell(0x3D),
					0x47 =>  Broadwell(0x47),
					0x4F =>  Broadwell(0x4F),
					0x56 =>  Broadwell(0x56),

					0x4E =>  SkyLake(0x4E),
					0x55 =>  SkyLake(0x55),
					0x5E =>  SkyLake(0x5E),
					0x8E =>  SkyLake(0x8E),
					0x9E =>  SkyLake(0x9E),
					0xA5 =>  SkyLake(0xA5),
					0xA6 =>  SkyLake(0xA6),

					0x66 =>  PalmCove(0x66),

					0x6A =>  SunnyCove(0x6A),
					0x6C =>  SunnyCove(0x6C),
					0x7D =>  SunnyCove(0x7D),
					0x7E =>  SunnyCove(0x7E),

					0x1C =>  Bonnel(0x1C),
					0x26 =>  Bonnel(0x26),

					0x27 =>  Saltwell(0x27),
					0x35 =>  Saltwell(0x35),
					0x36 =>  Saltwell(0x36),

					0x37 =>  Silvermont(0x37),
					0x4A =>  Silvermont(0x4A),
					0x4D =>  Silvermont(0x4D),
					0x5A =>  Silvermont(0x5A),
					0x5D =>  Silvermont(0x5D),

					0x4C =>  Airmont(0x4C),
					0x75 =>  Airmont(0x75),

					0x5C =>  Goldmont(0x5C),
					0x5F =>  Goldmont(0x5F),

					0x7A =>  GoldmontPlus(0x7A),

					0x57 =>  KnightsLanding(0x57),
					0x85 =>  KnightsMill(0x85),

					_ => Unknown(0x00),
				},

				0x0F => match model {
					0x00 =>  Willamette(0x00),
					0x01 =>  Willamette(0x01),
					0x02 =>  Willamette(0x02),

					0x03 =>  Prescott(0x03),
					0x04 =>  Prescott(0x04),
					0x06 =>  Prescott(0x06),

					_ => Unknown(0x00),
				},

				_ => Unknown(0x00),
			},

			CPUVendor::AMD => match family {
				0x05 => match model {
					0x00 =>  K5(0x00),
					0x01 =>  K5(0x01),
					0x02 =>  K5(0x02),

					0x06 =>  K6(0x06),
					0x07 =>  K6(0x07),
					0x08 =>  K6(0x08),
					0x0D =>  K6(0x0D),

					0x0A =>  Geode(0x0A),

					_ => Unknown(0x00),
				},

				0x06 =>  K7(0x06),

				0x0F =>  K8(0x0F),

				0x11 =>  K8(0x11),

				0x10 =>  K10(0x10),
				0x12 =>  K10(0x12),

				0x14 =>  Bobcat(0x14),

				0x15 => match model {
					0x00 => Bulldozer(0x00),
					0x01 => Bulldozer(0x01),

					0x02 => Piledriver(0x02),
					0x10 => Piledriver(0x10),
					0x13 => Piledriver(0x13),

					0x38 => Steamroller(0x38),
					0x30 => Steamroller(0x30),

					0x60 => Excavator(0x60),
					0x65 => Excavator(0x65),
					0x70 => Excavator(0x70),

					_ => match xmodel {
						0x00 =>  Bulldozer(0x00),

						0x01 =>  Piledriver(0x01),
						0x02 =>  Piledriver(0x02),

						0x03 =>  Steamroller(0x03),
						0x04 =>  Steamroller(0x04),

						_ =>  Unknown(0x00),
					},
				},

				0x16 => match model {
					n if n >= 3 =>  Puma(n as u8),
					n =>  Jaguar(n as u8),
				},

				0x17 => match model {
					0x01 =>  Zen(0x01),
					0x08 =>  Zen(0x08),
					0x11 =>  Zen(0x11),
					0x18 =>  Zen(0x18),

					0x31 =>  Zen2(0x31),
					0x60 =>  Zen2(0x60),
					0x68 =>  Zen2(0x68),
					0x71 =>  Zen2(0x71),
					0x90 =>  Zen2(0x90),
					0x98 =>  Zen2(0x98),

					_ => Unknown(0x00),
				},

				0x19 => match model {
					0x01 =>  Zen3(0x01),
					0x21 =>  Zen3(0x21),
					0x30 =>  Zen3(0x30),
					0x40 =>  Zen3(0x40),
					0x50 =>  Zen3(0x50),

					_ => Unknown(0x00),
				},

				_ => Unknown(0x00),
			},

			CPUVendor::Hygon => match family {
				0x00 =>  Dhyana(0x00),

				_ => Unknown(0x00),
			},

			_ => Unknown(0x00),
		}
	}
}



impl core::fmt::Display for CPUModel {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use CPUModel::*;

		let args = match *self {
			Pentium5(n) => match n {
				0x01 => "Pentium (800 nm)",
				0x02 => "Pentium (600 nm | 350 nm)",
				0x03 => "Pentium Overdrive (600 nm)",
				0x04 => "Pentium MMX (800 nm)",

				_ => "Inconsistent data",
			},

			Quark(n) => match n {
				0x09 => "Quark",

				_ => "Inconsistent data",
			},

			Pentium6(n) => match n {
				0x01 => "Pentium Pro (350 nm)",
				0x03 => "Pentium II (350 nm)\nPentium II Overdrive (250 nm)",
				0x05 => "Pentium II (250 nm)\nPentium II Celeron (250 nm)\nPentium II Xeon (250 nm)",
				0x06 => "Pentium II (250 nm)\nPentium II Celeron (250 nm)",

				0x07 => "Pentium III (250 nm)\nPentium III Xeon (250 nm)",
				0x08 => "Pentium III (250 nm)\nPentium II Celeron (250 nm)\nPentium III Xeon (180 nm)",
				0x0A => "Pentium III Xeon (180 nm)",
				0x0B => "Pentium III (130 nm)\nPentium III Celeron (130 nm)",

				_ => "Inconsistent data",
			},

			Dothan(n) => match n {
				0x09 => "Pentium M (130 nm)",
				0x0D => "Pentium M (90 nm)",
				0x15 => "Intel 80579 (90 nm)",

				_ => "Inconsistent data",
			},

			Yonah(n) => match n {
				0x0E => "Core Solo/Duo | Pentium Dual-Core T2xxx | Celeron M | Dual-Core Xeon (65 nm)",

				_ => "Inconsistent data",
			},

			Conroe(n) => match n {
				0x0F => "Core 2 Duo  (65 nm)\nCore 2 Quad (65 nm)\nXeon (65 nm)",
				0x16 => "Celeron (65 nm)\nCore 2 Duo (65 nm)",

				_ => "Inconsistent data",
			},

			Penryn(n) => match n {
				0x17 => "Core 2 Duo (45 nm)\nCore 2 Quad (45 nm)\nCore 2 Extreme (45 nm)\nXeon (45 nm)\nPentium Dual-Core (45 nm)",
				0x1D => "Xeon (45 nm)",

				_ => "Inconsistent data",
			},

			Nehalem(n) => match n {
				0x1A => "Core iX (45 nm)\nXeon (45 nm)",
				0x1E => "Core iX (45 nm)",
				0x1F => "Core iX (45 nm)",
				0x2E => "Xeon (45 nm)",
				0x25 => "Core iX (45 nm)",
				0x2C => "Core iX (45 nm)\nXeon (45 nm)",
				0x2F => "Xeon (45 nm)",

				_ => "Inconsistent data",
			},

			SandyBridge(n) => match n {
				0x2A => "Core iX (32 nm)",
				0x2D => "Core iX (32 nm)\nXeon (32 nm)",

				_ => "Inconsistent data",
			},

			IvyBridge(n) => match n {
				0x3A => "Core iX (22 nm)",
				0x3E => "Ivi Bridge-E (22 nm)",

				_ => "Inconsistent data",
			},

			Haswell(n) => match n {
				0x3C => "Haswell (22 nm)",
				0x3F => "Haswell-E (22 nm)",
				0x45 => "Haswell-ULT (22 nm)",
				0x46 => "Haswell [eDRAM] (22 nm)",

				_ => "Inconsistent data",
			},

			Broadwell(n) => match n {
				0x3D => "Broadwell-U (14 nm)",
				0x47 => "Broadwell-H (14 nm)",
				0x4F => "Broadwell-E (14 nm)",
				0x56 => "Broadwell-DE (14 nm)",

				_ => "Inconsistent data",
			},

			SkyLake(n) => match n {
				0x4E => "Sky Lake Client Y/U (14 nm)",
				0x55 => "Sky/Cascade/Cooper Lake Server (14 nm)",
				0x5E => "Sky Lake Client DT/H/S (14 nm)",
				0x8E => "Kaby/Whiskey/Amber/Comet Lake Y/U (14 nm)",
				0x9E => "Kaby/Coffee Lake DT/H/S (14 nm)",
				0xA5 => "Comet Lake H/S (14 nm)",
				0xA6 => "Comet Lake U/Y (14 nm)",

				_ => "Inconsistent data"
			},

			PalmCove(n) => match n {
				0x66 => "Cannon Lake (10 nm)",

				_ => "Inconsistent data",
			},

			SunnyCove(n) => match n {
				0x6A => "Ice Lake-DE (10+ nm)",
				0x6C => "Ice Lake-SP (10+ nm)",
				0x7D => "Ice Lake-Y (10+ nm)",
				0x7E => "Ice Lake-U (10+ nm)",

				_ => "Inconsistent data",
			},

			Bonnel(n) => match n {
				0x1C => "Diamondville (45 nm)\nSilverthrone (45 nm)\nPineview (45 nm)",
				0x26 => "Tunnel Creek (45 nm)",

				_ => "Inconsistent data",
			},

			Saltwell(n) => match n {
				0x27 => "Medfield (32 nm)",
				0x35 => "Cloverview (32 nm)",
				0x36 => "Cedarview (32 nm)\nCenterton (32 nm)",

				_ => "Inconsistent data",
			},

			Silvermont(n) => match n {
				0x37 => "Bay Trail (22 nm)",
				0x4A => "Merrifield (22 nm)",
				0x4D => "Avoton (22 nm)\nRangeley (22 nm)",
				0x5A => "Moorefield (22 nm)",
				0x5D => "SoFIA (22 nm)",

				_ => "Inconsistent data",
			},

			Airmont(n) => match n {
				0x4C => "Braswell (14 nm)\nCheery Trail (14 nm)",
				0x75 => "Spreadtrum SC9853I-IA (14 nm)",

				_ => "Inconsistent data",
			},

			Goldmont(n) => match n {
				0x5C => "Apollo Lake (14 nm)",
				0x5F => "Denverton (14 nm)",

				_ => "Inconsistent data",
			},

			GoldmontPlus(n) => match n {
				0x7A => "Gemini Lake (14 nm)",

				_ => "Inconsistent data",
			},

			Willamette(n) => match n {
				0x00 => "Pentium 4 Xeon (180 nm)",
				0x01 => "Pentium 4 Celeron (180 nm)\nPentium 4 Xeon (180 nm)",
				0x02 => "Pentium 4 (130 nm)\nPentium 4 EE (130 nm)\nPentium 4 Celeron (130 nm)\nPentium 4 Xeon (130 nm)",

				_ => "Inconsistent data",
			},

			Prescott(n) => match n {
				0x03 => "Pentium 4 (90 nm)\nPentium 4 Xeon (90 nm)",
				0x04 => "Pentium 4 (90 nm)\nPentium 4 EE (90 nm)\nPentium D (90 nm)\nCeleron D (90 nm)\nPentium 4 Xeon (90 nm)",
				0x06 => "Pentium 4 (65 nm)\nPentium D EE (65 nm)\nCeleron D (65 nm)\nPentium 4 Xeon (65 nm)",

				_ => "Inconsistent data",
			},


			KnightsLanding(n) => match n {
				0x57 => "Knights Landing (14 nm)",

				_ => "Inconsistent data",
			},

			KnightsMill(n) => match n {
				0x85 => "Knights Mill (14 nm)",

				_ => "Inconsistent data",
			},



			// AMD
			K5(n) => match n {
				0x00 | 0x01 | 0x02 => "K5 (500 nm - 350 nm)",

				_ => "Inconsistent data",
			},

			K6(n) => match n {
				0x06 | 0x07 | 0x08 | 0x0D => "K6 (350 nm - 250 nm)",

				_ => "Inconsistent data",
			},

			Geode(n) => match n {
				0x0A => "Geode",

				_ => "Inconsistent data",
			},

			K7(n) => match n {
				0x06 => "K7 (250 nm - 14 nm)",

				_ => "Inconsistent data",
			},

			K8(n) => match n {
				0x0F => "K8 (130 nm - 65 nm)",
				0x11 => "K8 (90 nm - 65 nm)",

				_ => "Inconsistent data",
			},

			K10(n) => match n {
				0x10 => "K10 Opteron\nK10 Phenom\nK10 Athlon\nK10 Sempron",
				0x12 => "K10 Llano APU",

				_ => "Inconsistent data",
			},

			Bobcat(n) => match n {
				0x14 => "Bobcat",

				_ => "Inconsistent data",
			},

			Bulldozer(n) => match n {
				0x00 => "Bulldozer (Engineer sample)",
				0x01 => "Bulldozer Zambezi\nBulldozer Interlagos",

				_ => "Inconsistent data",
			},

			Piledriver(n) => match n {
				0x02 => "Piledriver Vishera",
				0x10 => "Piledriver Trinity",
				0x13 => "Piledriver Richland",

				_ => "Inconsistent data",
			},

			Steamroller(n) => match n {
				0x38 => "Steamroller Godavari",
				0x30 => "Steamroller Kaveri",

				_ => "Inconsistent data",
			},

			Excavator(n) => match n {
				0x60 => "Excavator Carrizo",
				0x65 => "Excavator Bristol Ridge",
				0x70 => "Excavator Stone Ridge",

				_ => "Inconsistent data",
			},

			Puma(n) => match n {
				0..=2 => "Inconsistent data",
				_ => "Puma",
			},

			Jaguar(n) => match n {
				0..=2 => "Jaguar",
				_ => "Inconsistent data",
			},

			Zen(n) => match n {
				0x01 => "Zen  - Ryzen 1000 (14 nm)\n[EPYC Server] 'Naples'\n[Threadripper CPU] 'Whitehaven'\n[Desktop CPU] 'Summit ridge'\n[Embedded Server] 'Snowy Owl'",
				0x08 => "Zen+ - Ryzen 2000 (12 nm)\n[Desktop CPU] 'Pinnacle Ridge'\n[Threadripper CPU] 'Colfax'",
				0x11 => "Zen  - Ryzen 1000 (14 nm)\n[Desktop APU] 'Raven Ridge'\n[Mobile APU] 'Raven Ridge'\n[Embedded APU] 'Great Horned Owl'",
				0x18 => "Zen+ - Ryzen 2000 (12 nm)\n[Desktop APU] 'Picasso'\n[Mobile APU] 'Picasso'",

				_ => "Inconsistent data",
			},

			Zen2(n) => match n {
				0x31 => "Zen 2 - Ryzen 3000 (7 nm)\n[EPYC Server] 'Rome'",
				0x60 => "Zen 2 - Ryzen 3000 (7 nm)\n[Desktop APU] 'Renoir'\n[Mobile APU] 'Renoir'\n[Threadripper CPU] 'Castle Peak'",
				0x68 => "Zen 2 - Ryzen 5000 (7 nm)\n[Mobile APU] 'Lucienne'",
				0x71 => "Zen 2 - Ryzen 3000 (7 nm)\n[Desktop CPU] 'Matisse'",
				0x90 => "Zen 2 - Ryzen X000 (7 nm)\n[Desktop APU] 'Van Gogh'",
				0x98 => "Zen 2 - Ryzen X000 (7 nm)\n[Mobile APU] 'Mero'",

				_ => "Inconsistent data",
			},

			Zen3(n) => match n {
				0x01 => "Zen 3 - Ryzen X000 (7 nm)\n[Threadripper CPU] 'Genesis Peak'",
				0x21 => "Zen 3 - Ryzen 5000 (7 nm)\n[Desktop CPU] 'Vermeer'",
				0x30 => "Zen 3 - Ryzen X000 (7 nm)\n[] 'Badami''Trento'",
				0x40 => "Zen 3 - Ryzen 5000 (7 nm)\n[Desktop APU] 'Rembrandt'",
				0x50 => "Zen 3 - Ryzen 5000 (7 nm)\n[Mobile APU] 'Cezanne'",

				_ => "Inconsistent data",
			},

			Dhyana(n) => match n {
				0x00 => "Dhyana",

				_ => "Inconsistent data",
			},

			Unknown(n) => match n {
				0x00 => "Unknown architecture",

				_ => "Inconsistent data",
			},
		};

		f.write_str(args)
	}
}
