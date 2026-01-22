use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

use crate::errors::ParseError;
use crate::hitsounds::SampleSet;
use crate::timing::Millis;

/// Info for uninherited timing point
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UninheritedTimingInfo {
  /// Milliseconds per beat (aka beat duration)
  pub mpb: f64,

  /// The number of beats in a single measure
  pub meter: u32,
}

/// Info for inherited timing point
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InheritedTimingInfo {
  /// Slider velocity multiplier
  pub slider_velocity: f64,
}

/// An enum distinguishing between inherited and uninherited timing points.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TimingPointKind {
  /// Uninherited timing point
  Uninherited(UninheritedTimingInfo),

  /// Inherited timing point
  Inherited(InheritedTimingInfo),
}

/// A timing point, which represents configuration settings for a timing section.
///
/// This is a generic timing point struct representing both inherited and uninherited timing
/// points, distinguished by the `kind` field.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TimingPoint {
  /// The timestamp of this timing point, represented as a `TimeLocation`.
  pub time: Millis,

  /// Whether or not Kiai time should be on for this timing point.
  pub kiai: bool,

  /// The sample set associated with this timing section.
  pub sample_set: SampleSet,

  /// Index (if using a custom sample)
  pub sample_index: u32,

  /// Volume of this timing section.
  pub volume: u16,

  /// The type of this timing point. See `TimingPointKind`.
  pub kind: TimingPointKind,
}

impl Eq for TimingPoint {}

impl PartialEq for TimingPoint {
  fn eq(&self, other: &TimingPoint) -> bool {
    self.time.eq(&other.time)
  }
}

impl Ord for TimingPoint {
  fn cmp(&self, other: &TimingPoint) -> Ordering {
    self.time.cmp(&other.time)
  }
}

impl PartialOrd for TimingPoint {
  fn partial_cmp(&self, other: &TimingPoint) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl FromStr for TimingPoint {
  type Err = ParseError;

  fn from_str(input: &str) -> Result<TimingPoint, Self::Err> {
    // trim trailing commas to not have leftover empty pieces
    let input = input.trim_end_matches(',');
    let parts = input.split(',').collect::<Vec<_>>();

    if parts.len() < 2 {
      return Err(ParseError::InvalidTimingPoint(
        "timing point must have more than 2 components",
      ));
    }

    // parts.len() must be >= 2 at this point

    let timestamp = parts[0].parse::<f64>()?;
    let time = Millis(timestamp.round() as i32);

    let mpb = parts[1].parse::<f64>()?;

    if parts.len() == 2 {
      let timing_point = TimingPoint {
        kind: TimingPointKind::Uninherited(UninheritedTimingInfo {
          mpb,
          meter: 4,
        }),
        kiai: false,
        sample_set: SampleSet::Default,
        sample_index: 0,
        volume: 100,
        time,
      };

      return Ok(timing_point);
    }

    // parts.len() must be > 2 at this point

    let meter = parts[2].parse::<u32>()?;

    let kiai = if parts.len() > 7 {
      parts[7].parse::<i32>()? > 0
    } else {
      false
    };

    let sample_set = if parts.len() > 3 {
      match parts[3].parse::<u32>()? {
        0 => SampleSet::Default,
        1 => SampleSet::Normal,
        2 => SampleSet::Soft,
        3 => SampleSet::Drum,
        invalid => return Err(ParseError::InvalidSampleSet(invalid)),
      }
    } else {
      SampleSet::Default
    };

    let sample_index = if parts.len() > 4 {
      parts[4].parse::<u32>()?
    } else {
      0
    };

    let volume = if parts.len() > 5 {
      parts[5].parse::<u16>()?
    } else {
      100
    };

    let inherited = if parts.len() > 6 {
      parts[6].parse::<i32>()? == 0
    } else {
      false
    };

    // calculate bpm from mpb
    let _ = 60_000.0 / mpb;

    let timing_point = TimingPoint {
      kind: if inherited {
        TimingPointKind::Inherited(InheritedTimingInfo {
          slider_velocity: -100.0 / mpb,
        })
      } else {
        TimingPointKind::Uninherited(UninheritedTimingInfo { mpb, meter })
      },
      kiai,
      sample_set,
      sample_index,
      volume,
      time,
    };

    Ok(timing_point)
  }
}

impl fmt::Display for TimingPoint {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let inherited = match self.kind {
      TimingPointKind::Inherited { .. } => 0,
      TimingPointKind::Uninherited { .. } => 1,
    };

    let (beat_length, meter) = match self.kind {
      TimingPointKind::Inherited(InheritedTimingInfo {
        slider_velocity,
        ..
      }) => (-100.0 / slider_velocity, 0),
      TimingPointKind::Uninherited(UninheritedTimingInfo {
        mpb,
        meter,
        ..
      }) => (mpb, meter),
    };

    write!(
      f,
      "{},{},{},{},{},{},{},{}",
      self.time.0,
      beat_length,
      meter,
      self.sample_set as i32,
      self.sample_index,
      self.volume,
      inherited,
      if self.kiai { 1 } else { 0 },
    )
  }
}
