#![warn(clippy::pedantic)]
#![allow(clippy::redundant_locals)]

use std::fmt::{self, Display, Formatter};

use interoptopus::{
	extra_type, ffi_service, ffi_service_ctor, ffi_service_method, ffi_type, pattern,
	patterns::{option::FFIOption, string::AsciiPointer},
	Inventory, InventoryBuilder,
};
use rosu_pp::{
	any::{HitResultPriority, Performance as AnyPP, PerformanceAttributes},
	catch::CatchPerformanceAttributes,
	mania::ManiaPerformanceAttributes,
	model::{beatmap::BeatmapAttributes, mode::GameMode},
	osu::OsuPerformanceAttributes,
	taiko::TaikoPerformanceAttributes,
	Beatmap,
};

mod result;
use result::{Error, FFIError};

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq)]
pub enum Mode {
	#[default]
	Osu = 0,
	Taiko = 1,
	Catch = 2,
	Mania = 3,
}

impl std::fmt::Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self:?}"))
	}
}

#[ffi_type(opaque)]
#[repr(C)]
#[derive(Clone, Default, PartialEq, Copy)]
#[allow(non_snake_case)]
pub struct ScoreParams {
	pub mode: FFIOption<Mode>,
	pub mods: u32,
	pub acc: FFIOption<f64>,
	pub n300: FFIOption<u32>,
	pub n100: FFIOption<u32>,
	pub n50: FFIOption<u32>,
	pub nMisses: FFIOption<u32>,
	pub nKatu: FFIOption<u32>,
	pub combo: FFIOption<u32>,
	pub score: FFIOption<u32>,
	pub passedObjects: FFIOption<u32>,
	pub clockRate: FFIOption<f64>,
}

#[ffi_service(error = "FFIError", prefix = "score_params_")]
impl ScoreParams {
	#[ffi_service_ctor]
	pub fn new() -> Result<Self, Error> {
		Ok(Self::default())
	}

	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn mode(&mut self, mode: Mode) {
		self.mode = Some(mode).into();
	}

	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn mods(&mut self, mods: u32) {
		self.mods = mods;
	}
	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn acc(&mut self, acc: f64) {
		self.acc = Some(acc).into();
	}
	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn n300(&mut self, n300: u32) {
		self.n300 = Some(n300).into();
	}
	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn n100(&mut self, n100: u32) {
		self.n100 = Some(n100).into();
	}
	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn n50(&mut self, n50: u32) {
		self.n50 = Some(n50).into();
	}

	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn combo(&mut self, combo: u32) {
		self.combo = Some(combo).into();
	}
	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn score(&mut self, score: u32) {
		self.score = Some(score).into();
	}

	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn n_misses(&mut self, n_misses: u32) {
		self.nMisses = Some(n_misses).into();
	}

	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn n_katu(&mut self, n_katu: u32) {
		self.nKatu = Some(n_katu).into();
	}

	#[ffi_service_method(on_panic = "return_default")]
	pub fn passed_objects(&mut self, passed_objects: u32) {
		self.passedObjects = Some(passed_objects).into();
	}

	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn clock_rate(&mut self, clock_rate: f64) {
		self.clockRate = Some(clock_rate).into();
	}
}

impl ScoreParams {
	fn apply(self, mut calculator: AnyPP) -> AnyPP {
		let ScoreParams {
			mode,
			mods,
			n300,
			n100,
			n50,
			nMisses,
			nKatu,
			acc,
			combo,
			score: _,
			passedObjects,
			clockRate,
		} = self;

		if let Some(mode) = mode.into_option() {
			let mode = match mode {
				Mode::Osu => GameMode::Osu,
				Mode::Taiko => GameMode::Taiko,
				Mode::Catch => GameMode::Catch,
				Mode::Mania => GameMode::Mania,
			};

			calculator = calculator.mode_or_ignore(mode);
		}

		if let Some(n300) = n300.into_option() {
			calculator = calculator.n300(n300);
		}

		if let Some(n100) = n100.into_option() {
			calculator = calculator.n100(n100);
		}

		if let Some(n50) = n50.into_option() {
			calculator = calculator.n50(n50);
		}

		if let Some(n_misses) = nMisses.into_option() {
			calculator = calculator.misses(n_misses);
		}

		if let Some(n_katu) = nKatu.into_option() {
			calculator = calculator.n_katu(n_katu);
		}

		if let Some(combo) = combo.into_option() {
			calculator = calculator.combo(combo);
		}

		if let Some(passed_objects) = passedObjects.into_option() {
			calculator = calculator.passed_objects(passed_objects);
		}

		if let Some(clock_rate) = clockRate.into_option() {
			calculator = calculator.clock_rate(clock_rate);
		}

		calculator = calculator.mods(mods);

		// Because mania hitresult generation is fucked
		calculator = calculator.hitresult_priority(HitResultPriority::Fastest);

		if let Some(acc) = acc.into_option() {
			calculator = calculator.accuracy(acc);
		}

		calculator
	}
}

#[ffi_type]
#[repr(C)]
#[derive(Clone, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct CalculateResult {
	pub mode: u8,
	pub stars: f64,
	pub pp: f64,
	pub ppAcc: FFIOption<f64>,
	pub ppAim: FFIOption<f64>,
	pub ppFlashlight: FFIOption<f64>,
	pub ppSpeed: FFIOption<f64>,
	pub ppStrain: FFIOption<f64>,
	pub nFruits: FFIOption<u32>,
	pub nDroplets: FFIOption<u32>,
	pub nTinyDroplets: FFIOption<u32>,
	pub aimStrain: FFIOption<f64>,
	pub speedStrain: FFIOption<f64>,
	pub flashlightRating: FFIOption<f64>,
	pub sliderFactor: FFIOption<f64>,
	pub ppDifficulty: FFIOption<f64>,
	pub effectiveMissCount: FFIOption<f64>,

	pub ar: f64,
	pub cs: f64,
	pub hp: f64,
	pub od: f64,
	pub bpm: f64,
	pub clockRate: f64,
	pub timePreempt: FFIOption<f64>,
	pub greatHitWindow: FFIOption<f64>,
	pub nObjects: FFIOption<u32>,
	pub nSliders: FFIOption<u32>,
	pub nSpinners: FFIOption<u32>,
	pub maxCombo: FFIOption<u32>,
}

impl CalculateResult {
	fn new(
		attrs: PerformanceAttributes,
		map: &Beatmap,
		mods: u32,
		clock_rate: Option<f64>,
	) -> Self {
		let attr_builder = map.attributes();

		if let Some(clock_rate) = clock_rate {
			let _ = attr_builder.clone().clock_rate(clock_rate);
		}

		let mode = match &attrs {
			PerformanceAttributes::Catch(_) => GameMode::Catch,
			PerformanceAttributes::Mania(_) => GameMode::Mania,
			PerformanceAttributes::Osu(_) => GameMode::Osu,
			PerformanceAttributes::Taiko(_) => GameMode::Taiko,
		};

		let is_convert = map.mode == GameMode::Osu && mode != GameMode::Osu;

		let cloned_builder = attr_builder.clone();

		let BeatmapAttributes {
			ar,
			cs,
			hp,
			od,
			clock_rate,
			hit_windows,
		} = cloned_builder.mods(mods).mode(mode, is_convert).build();

		let bpm = map.bpm() * clock_rate;

		match attrs {
			PerformanceAttributes::Catch(CatchPerformanceAttributes { pp, difficulty }) => Self {
				mode: 2,
				pp,
				stars: difficulty.stars,
				maxCombo: Some(difficulty.n_fruits + difficulty.n_droplets).into(),
				nFruits: Some(difficulty.n_fruits).into(),
				nDroplets: Some(difficulty.n_droplets).into(),
				nTinyDroplets: Some(difficulty.n_tiny_droplets).into(),
				// nSpinners: Some().into(),
				ar,
				cs,
				hp,
				od,
				bpm,
				clockRate: clock_rate,
				..Default::default()
			},
			PerformanceAttributes::Mania(ManiaPerformanceAttributes {
				difficulty,
				pp_difficulty,
				pp,
			}) => Self {
				mode: 3,
				pp,
				ppDifficulty: Some(pp_difficulty).into(),
				stars: difficulty.stars,
				nSliders: Some(difficulty.n_hold_notes).into(),
				nObjects: Some(difficulty.n_objects).into(),
				ar,
				cs,
				hp,
				od,
				bpm,
				clockRate: clock_rate,
				greatHitWindow: Some(hit_windows.od_great).into(),
				..Default::default()
			},
			PerformanceAttributes::Osu(OsuPerformanceAttributes {
				difficulty,
				effective_miss_count,
				pp,
				pp_acc,
				pp_aim,
				pp_flashlight,
				pp_speed,
				..
			}) => Self {
				mode: 0,
				pp,
				ppAcc: Some(pp_acc).into(),
				ppAim: Some(pp_aim).into(),
				ppFlashlight: Some(pp_flashlight).into(),
				effectiveMissCount: Some(effective_miss_count).into(),
				ppSpeed: Some(pp_speed).into(),
				stars: difficulty.stars,
				maxCombo: Some(difficulty.max_combo).into(),
				aimStrain: Some(pp_aim).into(),
				speedStrain: Some(pp_speed).into(),
				flashlightRating: Some(pp_flashlight).into(),
				sliderFactor: Some(difficulty.slider_factor).into(),
				nObjects: Some(difficulty.n_circles).into(),
				nSliders: Some(difficulty.n_sliders).into(),
				nSpinners: Some(difficulty.n_spinners).into(),
				ar,
				cs,
				hp,
				od,
				bpm,
				clockRate: clock_rate,
				timePreempt: Some(hit_windows.ar).into(),
				greatHitWindow: Some(hit_windows.od_great).into(),
				..Default::default()
			},
			PerformanceAttributes::Taiko(TaikoPerformanceAttributes {
				difficulty,
				effective_miss_count,
				pp,
				pp_acc,
				pp_difficulty,
				..
			}) => Self {
				mode: 1,
				pp,
				ppAcc: Some(pp_acc).into(),
				ppDifficulty: Some(pp_difficulty).into(),
				effectiveMissCount: Some(effective_miss_count).into(),
				stars: difficulty.stars,
				maxCombo: Some(difficulty.max_combo).into(),
				nObjects: Some(map.hit_objects.len() as u32).into(),
				ar,
				cs,
				hp,
				od,
				bpm,
				clockRate: clock_rate,
				greatHitWindow: Some(hit_windows.od_great).into(),
				..Default::default()
			},
		}
	}
}

#[ffi_type(opaque)]
#[derive(Default)]
pub struct Calculator {
	pub inner: Beatmap,
}

#[ffi_service(error = "FFIError", prefix = "calculator_")]
impl Calculator {
	#[ffi_service_ctor(on_panic = "ffi_error")]
	pub fn new(beatmap_path: AsciiPointer) -> Result<Self, Error> {
		let str = beatmap_path.as_c_str().unwrap().to_str().unwrap();
		Ok(Self {
			inner: Beatmap::from_path(str).unwrap(),
		})
	}

	#[ffi_service_ctor(on_panic = "ffi_error")]
	pub fn from_data(beatmap_data: AsciiPointer) -> Result<Self, Error> {
		let str = beatmap_data.as_c_str().unwrap().to_str().unwrap();
		Ok(Self {
			inner: Beatmap::from_bytes(str.as_bytes()).unwrap(),
		})
	}

	#[ffi_service_method(on_panic = "undefined_behavior")]
	pub fn calculate(&mut self, score_params: &ScoreParams) -> CalculateResult {
		let mods = score_params.mods;
		let clock_rate = score_params.clockRate;
		let calculator = score_params.apply(AnyPP::new(&self.inner));

		CalculateResult::new(
			calculator.calculate(),
			&self.inner,
			mods,
			clock_rate.into_option(),
		)
	}
}

pub fn my_inventory() -> Inventory {
	InventoryBuilder::new()
		.register(pattern!(Calculator))
		.register(extra_type!(CalculateResult))
		.register(extra_type!(Mode))
		.register(pattern!(ScoreParams))
		.inventory()
}

impl Display for CalculateResult {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let mut s = f.debug_struct("CalculateResult");

		s.field("mode", &self.mode)
			.field("stars", &self.stars)
			.field("pp", &self.pp);

		if let Some(ref pp_acc) = self.ppAcc.into_option() {
			s.field("ppAcc", pp_acc);
		}

		if let Some(ref pp_aim) = self.ppAim.into_option() {
			s.field("ppAim", pp_aim);
		}

		if let Some(ref pp_flashlight) = self.ppFlashlight.into_option() {
			s.field("ppFlashlight", pp_flashlight);
		}

		if let Some(ref pp_speed) = self.ppSpeed.into_option() {
			s.field("ppSpeed", pp_speed);
		}

		if let Some(ref pp_strain) = self.ppStrain.into_option() {
			s.field("ppStrain", pp_strain);
		}

		if let Some(ref n_fruits) = self.nFruits.into_option() {
			s.field("nFruits", n_fruits);
		}

		if let Some(ref n_droplets) = self.nDroplets.into_option() {
			s.field("nDroplets", n_droplets);
		}

		if let Some(ref n_tiny_droplets) = self.nTinyDroplets.into_option() {
			s.field("nTinyDroplets", n_tiny_droplets);
		}

		if let Some(ref aim_strain) = self.aimStrain.into_option() {
			s.field("aimStrain", aim_strain);
		}

		if let Some(ref speed_strain) = self.speedStrain.into_option() {
			s.field("speedStrain", speed_strain);
		}

		if let Some(ref flashlight_rating) = self.flashlightRating.into_option() {
			s.field("flashlightRating", flashlight_rating);
		}

		if let Some(ref slider_factor) = self.sliderFactor.into_option() {
			s.field("sliderFactor", slider_factor);
		}

		s.field("ar", &self.ar)
			.field("cs", &self.cs)
			.field("hp", &self.hp)
			.field("od", &self.od)
			.field("bpm", &self.bpm)
			.field("clockRate", &self.clockRate);

		if let Some(ref time_preempt) = self.timePreempt.into_option() {
			s.field("timePreempt", time_preempt);
		}

		if let Some(ref great_hit_window) = self.greatHitWindow.into_option() {
			s.field("greatHitWindow", great_hit_window);
		}

		if let Some(ref n_circles) = self.nObjects.into_option() {
			s.field("nCircles", n_circles);
		}

		if let Some(ref n_sliders) = self.nSliders.into_option() {
			s.field("nSliders", n_sliders);
		}

		if let Some(ref n_spinners) = self.nSpinners.into_option() {
			s.field("nSpinners", n_spinners);
		}

		if let Some(ref combo) = self.maxCombo.into_option() {
			s.field("maxCombo", combo);
		}

		s.finish()
	}
}

impl Display for ScoreParams {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let mut s = f.debug_struct("ScoreParams");

		s.field("mode", &self.mode.into_option())
			.field("mods", &self.mods)
			.field("n300", &self.n300.into_option())
			.field("n100", &self.n100.into_option())
			.field("n50", &self.n50.into_option())
			.field("nMisses", &self.nMisses.into_option())
			.field("nKatu", &self.nKatu.into_option())
			.field("acc", &self.acc.into_option())
			.field("combo", &self.combo.into_option())
			.field("score", &self.score.into_option())
			.field("passedObjects", &self.passedObjects.into_option())
			.field("clockRate", &self.clockRate.into_option());

		s.finish()
	}
}
