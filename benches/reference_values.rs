use criterion::black_box;
use floccus::quantities::{
    AtmosphericPressure, DewPointTemperature, DryBulbTemperature, MixingRatio, RelativeHumidity,
    SaturationMixingRatio, SaturationVapourPressure, SpecificHumidity, VapourPressure,
};
use uom::si::{pressure::pascal, ratio::ratio, thermodynamic_temperature::kelvin};

type Float = f64;

pub(crate) const TEMP_NORM: Float = 300.0;
pub(crate) const DWPT_NORM: Float = 290.0;
pub(crate) const PRES_NORM: Float = 100000.0;
pub(crate) const VP_NORM: Float = 1919.4253257541593;
pub(crate) const SVP_NORM: Float = 3535.4235919263083;
pub(crate) const RH_NORM: Float = 0.5429124052171476;
pub(crate) const MR_NORM: Float = 0.012172079452423202;
pub(crate) const SMR_NROM: Float = 0.022419969290542845;
pub(crate) const VPD_NORM: Float = 1615.998266172149;
pub(crate) const SH_NORM: Float = 0.012025701656390478;
pub(crate) const THETAE_NORM: Float = 331.329289539998;
pub(crate) const THETA_NORM: Float = 301.66581400702955;
pub(crate) const THETAW_NORM: Float = 292.0717306393948;
pub(crate) const WBT_NORM: Float = 293.42728654340516;
pub(crate) const VRT_NORM: Float = 302.1926517941886;

pub(crate) const TEMP_FREEZ: Float = 260.0;
pub(crate) const DWPT_FREEZ: Float = 255.0;
pub(crate) const PRES_FREEZ: Float = 100000.0;
pub(crate) const VP_FREEZ: Float = 123.17937690212507;
pub(crate) const SVP_FREEZ: Float = 195.84980045970696;
pub(crate) const RH_FREEZ: Float = 0.6289481868911442;
pub(crate) const MR_FREEZ: Float = 0.0007670962389744638;
pub(crate) const SMR_FREEZ: Float = 0.0012196493367222787;
pub(crate) const VPD_FREEZ: Float = 72.67042355758188;
pub(crate) const SH_FREEZ: Float = 0.000766508253376156;
pub(crate) const THETAE_FREEZ: Float = 261.95287841149707;
pub(crate) const THETA_FREEZ: Float = 260.0915766593588;
pub(crate) const THETAW_FREEZ: Float = 258.6611332391296;
pub(crate) const WBT_FREEZ: Float = 258.40501060754224;
pub(crate) const VRT_FREEZ: Float = 260.12112343315795;

pub struct ReferenceValues {
    pub temp: DryBulbTemperature,
    pub pres: AtmosphericPressure,
    pub dwpt: DewPointTemperature,
    pub sphu: SpecificHumidity,
    pub vapr: VapourPressure,
    pub savp: SaturationVapourPressure,
    pub rehu: RelativeHumidity,
    pub mxrt: MixingRatio,
    pub smrt: SaturationMixingRatio,
}

impl ReferenceValues {
    pub fn normal() -> Self {
        Self {
            temp: black_box(DryBulbTemperature::new::<kelvin>(TEMP_NORM)),
            pres: black_box(AtmosphericPressure::new::<pascal>(PRES_NORM)),
            dwpt: black_box(DewPointTemperature::new::<kelvin>(DWPT_NORM)),
            sphu: black_box(SpecificHumidity::new::<ratio>(SH_NORM)),
            vapr: black_box(VapourPressure::new::<pascal>(VP_NORM)),
            savp: black_box(SaturationVapourPressure::new::<pascal>(SVP_NORM)),
            rehu: black_box(RelativeHumidity::new::<ratio>(RH_NORM)),
            mxrt: black_box(MixingRatio::new::<ratio>(MR_NORM)),
            smrt: black_box(SaturationMixingRatio::new::<ratio>(SMR_NROM)),
        }
    }

    pub fn freeze() -> Self {
        Self {
            temp: black_box(DryBulbTemperature::new::<kelvin>(TEMP_FREEZ)),
            pres: black_box(AtmosphericPressure::new::<pascal>(PRES_FREEZ)),
            dwpt: black_box(DewPointTemperature::new::<kelvin>(DWPT_FREEZ)),
            sphu: black_box(SpecificHumidity::new::<ratio>(SH_FREEZ)),
            vapr: black_box(VapourPressure::new::<pascal>(VP_FREEZ)),
            savp: black_box(SaturationVapourPressure::new::<pascal>(SVP_FREEZ)),
            rehu: black_box(RelativeHumidity::new::<ratio>(RH_FREEZ)),
            mxrt: black_box(MixingRatio::new::<ratio>(MR_FREEZ)),
            smrt: black_box(SaturationMixingRatio::new::<ratio>(SMR_FREEZ)),
        }
    }
}
