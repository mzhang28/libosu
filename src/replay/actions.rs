#[cfg(feature = "replay-data")]
#[cfg_attr(docsrs, doc(cfg(feature = "replay-data")))]
use std::io::BufRead;

use crate::timing::Millis;

#[cfg(feature = "replay-data")]
#[cfg_attr(docsrs, doc(cfg(feature = "replay-data")))]
use super::ReplayResult;

/// An action by the player while playing the map
#[derive(Debug, Clone)]
pub struct ReplayAction {
  /// The time since the last replay action in milliseconds.
  ///
  /// After osu! version `20130319` if this is the last action in the stream
  /// it may be set `-12345` indicating the `buttons` field holds the RNG seed for this score.
  pub time: Millis,

  /// Cursor position X, in the range 0-512
  pub x: f32,

  /// Cursor position Y, in the range 0-384
  pub y: f32,

  /// bitwise combination of keys and mousebuttons pressed.
  pub buttons: Buttons,
}

bitflags! {
    /// The buttons being pressed during a frame of a replay
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Buttons: u32 {
        /// First mouse button
        const M1 = 1;

        /// Second mouse button
        const M2 = 2;

        /// First keyboard button
        const K1 = 4;

        /// Second keyboard button
        const K2 = 8;

        /// Smoke button
        const SMOKE = 16;
    }
}

/// A parser for decompressed replay actions
/// to read compressed replay actions see `create_decompressing_replay_action_parser`
#[derive(Debug, Clone)]
pub struct ReplayActionData {
  /// The frames of the replay
  pub frames: Vec<ReplayAction>,

  /// Only for replays from version 20130319 of later, this is the RNG seed used for the score
  pub rng_seed: Option<u32>,
}

impl ReplayActionData {
  #[cfg(feature = "replay-data")]
  #[cfg_attr(docsrs, doc(cfg(feature = "replay-data")))]
  /// create a new ReplayActionParser from a BufRead
  pub fn parse(data: impl BufRead) -> ReplayResult<Self> {
    use super::ReplayError;

    let data = super::lzma::decode(data)?;
    let string = String::from_utf8(data)?;
    let mut frames = string
      .split(',')
      .filter(|action_str| !action_str.trim().is_empty())
      .map(|action_str| {
        let mut parts = action_str.split('|');
        let time = Millis(parts.next().unwrap().parse::<i32>()?);
        let x = parts.next().unwrap().parse::<f32>()?;
        let y = parts.next().unwrap().parse::<f32>()?;
        let bits = parts.next().unwrap().parse::<u32>()?;

        let buttons = if time.0 == -12345 {
          // allow this
          Buttons::from_bits_retain(bits)
          // unsafe { Buttons::from_bits_unchecked(bits) }
        } else {
          Buttons::from_bits(bits).ok_or(ReplayError::InvalidButtons(bits))?
        };
        Ok(ReplayAction {
          time,
          x,
          y,
          buttons,
        })
      })
      .collect::<ReplayResult<Vec<_>>>()?;

    let has_seed = matches!(
      frames.last(),
      Some(ReplayAction {
        time: Millis(-12345),
        ..
      })
    );
    let rng_seed = if has_seed {
      let last_element = frames.pop().expect("has_seed checked");
      Some(last_element.buttons.bits())
    } else {
      None
    };

    Ok(ReplayActionData { frames, rng_seed })
  }
}
