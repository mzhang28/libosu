pub mod binary;

use std::io;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{
  data::{Grade, Mode, Mods, RankedStatus},
  timing::Millis,
};

pub use self::binary::{Error, ReadBytesOsu, WriteBytesOsu};

/// Result type for .db file processing
pub type DbResult<T, E = DbError> = std::result::Result<T, E>;

/// Errors that could occur while processing .db files
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum DbError {
  /// Error reading binary data
  #[error("error during binary read: {0}")]
  Read(#[from] self::binary::Error),

  /// IO Error
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),

  /// Invalid mod combination
  #[error("unexpected mods: {0}")]
  InvalidMods(u32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// Timing point for beatmap in osu!.db
pub struct DbBeatmapTimingPoint {
  /// Bpm of the timing point
  pub bpm: f64,

  /// Offset of the timing point
  pub offset: f64,

  /// Whether or not it's inherited or uninherited
  pub is_uninherited: bool,
}

#[derive(Debug, PartialEq, Clone)]
/// Beatmap located in osu!.db, they are different from normal beatmaps
pub struct DbBeatmap {
  /// Size in bytes of the beatmap entry. Only present if version is less than 20191106.
  pub size: Option<u32>,

  /// Artist's name. (ASCII)
  pub artist_name: String,
  /// Artist's name. (UTF-8)
  pub artist_name_unicode: String,

  /// Song title. (ASCII)
  pub song_title: String,
  /// Song title. (UTF-8)
  pub song_title_unicode: String,

  /// Creator name.
  pub creator_name: String,
  /// Difficulty name.
  pub difficulty: String,

  /// Name of the audio file.
  pub audio_file_name: String,
  /// MD5 hash of the beatmap.
  pub hash: String,
  /// Name of the .osu file.
  pub beatmap_file_name: String,
  /// Ranked status.
  pub ranked_status: RankedStatus,

  /// How many hitcircles in the beatmap.
  pub hitcircle_count: u16,
  /// How many sliders in the beatmap.
  pub slider_count: u16,
  /// How many spinners in the beatmap.
  pub spinner_count: u16,

  /// When the beatmap was last modified.
  ///
  /// This is value is measured in [windows ticks][1].  It counts the number of ticks from
  /// 12:00:00 midnight, January 1, 0001 to the time this replay was created
  ///
  /// [1]: https://docs.microsoft.com/en-us/dotnet/api/system.datetime.ticks?redirectedfrom=MSDN&view=net-5.0#System_DateTime_Ticks
  pub modification_date: u64,

  /// AR rating.
  pub approach_rate: f32,
  /// CS rating.
  pub circle_size: f32,
  /// HP rating.
  pub hp_drain: f32,
  /// OD rating.
  pub overall_difficulty: f32,

  /// Slider velocity setting of the beatmap.
  pub slider_velocity: f64,
  /// A list of calculated star ratings for different mods for standard. Empty if version less than 20140609.
  pub std_star_rating: Vec<(Mods, f64)>,
  /// A list of calculated star ratings for different mods for taiko. Empty if version less than 20140609.
  pub std_taiko_rating: Vec<(Mods, f64)>,
  /// A list of calculated star ratings for different mods for ctb. Empty if version less than 20140609.
  pub std_ctb_rating: Vec<(Mods, f64)>,
  /// A list of calculated star ratings for different mods for mania. Empty if version less than 20140609.
  pub std_mania_rating: Vec<(Mods, f64)>,

  /// The drain time in milliseconds.
  pub drain_time: Millis,
  /// The total time in milliseconds.
  pub total_time: Millis,
  /// The preview time point in milliseconds.
  pub preview_time: Millis,

  /// Timing points for the beatmap.
  pub timing_points: Vec<DbBeatmapTimingPoint>,

  /// Id of the beatmap.
  pub beatmap_id: u32,
  /// Id of the beatmap set.
  pub beatmap_set_id: u32,
  /// Id of the beatmap thread.
  pub thread_id: u32,

  /// The grade achieved in standard.
  pub std_grade: Grade,
  /// The grade achieved in taiko.
  pub taiko_grade: Grade,
  /// The grade achieved in ctb.
  pub ctb_grade: Grade,
  /// The grade achieved in mania.
  pub mania_grade: Grade,

  /// Local offset.
  pub beatmap_offset: u16,

  /// Stack leniency.
  pub stack_leniency: f32,

  /// Mode of the beatmap.
  pub mode: Mode,

  /// Source.
  pub source: String,
  /// Tags.
  pub tags: String,

  /// Online offset.
  pub online_offset: u16,

  /// Title font.
  pub title_font: String,

  /// Is the beatmap not played.
  pub is_unplayed: bool,

  /// Last time the map was played.
  ///
  /// This is value is measured in [windows ticks][1].  It counts the number of ticks from
  /// 12:00:00 midnight, January 1, 0001 to the time this replay was created
  ///
  /// [1]: https://docs.microsoft.com/en-us/dotnet/api/system.datetime.ticks?redirectedfrom=MSDN&view=net-5.0#System_DateTime_Ticks
  pub last_played: u64,

  /// If the format is osz2.
  pub is_osz2: bool,

  /// Folder name relative to the Songs folder.
  pub folder_name: String,

  /// Last time beatmap was checked to the online repository.
  ///
  /// This is value is measured in [windows ticks][1].  It counts the number of ticks from
  /// 12:00:00 midnight, January 1, 0001 to the time this replay was created
  ///
  /// [1]: https://docs.microsoft.com/en-us/dotnet/api/system.datetime.ticks?redirectedfrom=MSDN&view=net-5.0#System_DateTime_Ticks
  pub last_checked: u64,

  /// Ignore beatmap sounds.
  pub ignore_beatmap_sounds: bool,
  /// Ignore beatmap skin.
  pub ignore_beatmap_skin: bool,
  /// Disable storyboard.
  pub disable_storyboard: bool,
  /// Disable video.
  pub disable_video: bool,
  /// Visual override.
  pub visual_override: bool,

  /// Unknown. Only present if version is less than 20140609.
  pub unknown: Option<u16>,

  /// Last modification time (?).
  pub unknown_modification_date: u32,

  /// Scroll speed for mania.
  pub mania_scrollspeed: u8,
}

#[derive(Debug, Clone)]
/// osu!.db object  
pub struct Db {
  /// osu! game mode that this replay was recorded for
  pub version: u32,
  /// The amount of folders?
  pub folder_count: u32,
  /// If the account is unlocked, aka not banned or locked.
  pub account_unlocked: bool,

  /// When the account will be unlocked
  ///
  /// This is value is measured in [windows ticks][1].  It counts the number of ticks from
  /// 12:00:00 midnight, January 1, 0001 to the time this replay was created
  ///
  /// [1]: https://docs.microsoft.com/en-us/dotnet/api/system.datetime.ticks?redirectedfrom=MSDN&view=net-5.0#System_DateTime_Ticks
  pub unlocked_date: u64,
  /// The player's username
  pub player_name: String,
  /// The amount of beatmaps cached
  pub beatmap_count: u32,
  /// The cached beatmaps     
  pub beatmaps: Vec<DbBeatmap>,
  /// The permissions of the user
  pub permissions: u8,
}

impl DbBeatmap {
  fn read_star_rating(
    mut reader: impl io::BufRead,
    version: u32,
  ) -> DbResult<Vec<(Mods, f64)>> {
    let count = reader.read_u32::<LittleEndian>()?;
    let ratings = (0..count)
      .map(|_| -> DbResult<(Mods, f64)> {
        Ok((
          {
            assert_eq!(reader.read_u8()?, 0x08);
            let value = reader.read_u32::<LittleEndian>()?;
            Mods::from_bits(value).ok_or(DbError::InvalidMods(value))?
          },
          {
            // "Until database version 20250107, this was a collection of Int-Double pairs"
            if version > 20250107 {
              assert_eq!(reader.read_u8()?, 0x0C);
              reader.read_f32::<LittleEndian>()? as f64
            } else {
              assert_eq!(reader.read_u8()?, 0x0D);
              reader.read_f64::<LittleEndian>()?
            }
          },
        ))
      })
      .collect::<DbResult<Vec<_>>>()?;
    Ok(ratings)
  }

  fn read_timing_points(
    mut reader: impl io::BufRead,
  ) -> DbResult<Vec<DbBeatmapTimingPoint>> {
    let count = reader.read_u32::<LittleEndian>()?;
    let points = (0..count)
      .map(|_| {
        Ok(DbBeatmapTimingPoint {
          bpm: reader.read_f64::<LittleEndian>()?,
          offset: reader.read_f64::<LittleEndian>()?,
          is_uninherited: reader.read_u8()? > 0,
        })
      })
      .collect::<DbResult<Vec<_>>>()?;
    Ok(points)
  }

  fn parse(mut reader: impl io::BufRead, version: u32) -> DbResult<DbBeatmap> {
    Ok(DbBeatmap {
      size: if version < 20191106 {
        Some(reader.read_u32::<LittleEndian>()?)
      } else {
        None
      },
      artist_name: reader.read_uleb128_string()?,
      artist_name_unicode: reader.read_uleb128_string()?,
      song_title: reader.read_uleb128_string()?,
      song_title_unicode: reader.read_uleb128_string()?,
      creator_name: reader.read_uleb128_string()?,
      difficulty: reader.read_uleb128_string()?,
      audio_file_name: reader.read_uleb128_string()?,
      hash: reader.read_uleb128_string()?,
      beatmap_file_name: reader.read_uleb128_string()?,
      ranked_status: num::FromPrimitive::from_u8(reader.read_u8()?).unwrap(),
      hitcircle_count: reader.read_u16::<LittleEndian>()?,
      slider_count: reader.read_u16::<LittleEndian>()?,
      spinner_count: reader.read_u16::<LittleEndian>()?,
      modification_date: reader.read_u64::<LittleEndian>()?,
      approach_rate: reader.read_f32::<LittleEndian>()?,
      circle_size: reader.read_f32::<LittleEndian>()?,
      hp_drain: reader.read_f32::<LittleEndian>()?,
      overall_difficulty: reader.read_f32::<LittleEndian>()?,
      slider_velocity: reader.read_f64::<LittleEndian>()?,
      std_star_rating: Self::read_star_rating(&mut reader, version)?,
      std_taiko_rating: Self::read_star_rating(&mut reader, version)?,
      std_ctb_rating: Self::read_star_rating(&mut reader, version)?,
      std_mania_rating: Self::read_star_rating(&mut reader, version)?,
      drain_time: Millis(reader.read_i32::<LittleEndian>()? * 1000), // the file contains seconds, not milliseconds
      total_time: Millis(reader.read_i32::<LittleEndian>()?),
      preview_time: Millis(reader.read_i32::<LittleEndian>()?),
      timing_points: Self::read_timing_points(&mut reader)?,
      beatmap_id: reader.read_u32::<LittleEndian>()?,
      beatmap_set_id: reader.read_u32::<LittleEndian>()?,
      thread_id: reader.read_u32::<LittleEndian>()?,
      std_grade: num::FromPrimitive::from_u8(reader.read_u8()?).unwrap(),
      taiko_grade: num::FromPrimitive::from_u8(reader.read_u8()?).unwrap(),
      ctb_grade: num::FromPrimitive::from_u8(reader.read_u8()?).unwrap(),
      mania_grade: num::FromPrimitive::from_u8(reader.read_u8()?).unwrap(),
      beatmap_offset: reader.read_u16::<LittleEndian>()?,
      stack_leniency: reader.read_f32::<LittleEndian>()?,
      mode: num::FromPrimitive::from_u8(reader.read_u8()?).unwrap(),
      source: reader.read_uleb128_string()?,
      tags: reader.read_uleb128_string()?,
      online_offset: reader.read_u16::<LittleEndian>()?,
      title_font: reader.read_uleb128_string()?,
      is_unplayed: reader.read_u8()? > 0,
      last_played: reader.read_u64::<LittleEndian>()?,
      is_osz2: reader.read_u8()? > 0,
      folder_name: reader.read_uleb128_string()?,
      last_checked: reader.read_u64::<LittleEndian>()?,
      ignore_beatmap_sounds: reader.read_u8()? > 0,
      ignore_beatmap_skin: reader.read_u8()? > 0,
      disable_storyboard: reader.read_u8()? > 0,
      disable_video: reader.read_u8()? > 0,
      visual_override: reader.read_u8()? > 0,
      unknown: if version < 20140609 {
        Some(reader.read_u16::<LittleEndian>()?)
      } else {
        None
      },
      unknown_modification_date: reader.read_u32::<LittleEndian>()?,
      mania_scrollspeed: reader.read_u8()?,
    })
  }
}

impl Db {
  /// Parse the osu!.db data from a reader.
  pub fn parse(mut reader: impl io::BufRead) -> DbResult<Db> {
    let version = reader.read_u32::<LittleEndian>()?;
    let folder_count = reader.read_u32::<LittleEndian>()?;
    let account_unlocked = reader.read_u8()? > 0;
    let unlocked_date = reader.read_u64::<LittleEndian>()?;
    let player_name = reader.read_uleb128_string()?;
    let beatmap_count = reader.read_u32::<LittleEndian>()?;
    let beatmaps = (0..beatmap_count)
      .map(|_| DbBeatmap::parse(&mut reader, version))
      .collect::<DbResult<Vec<_>>>()?;
    let permissions = reader.read_u8()?;

    Ok(Db {
      version,
      folder_count,
      account_unlocked,
      unlocked_date,
      player_name,
      beatmap_count,
      beatmaps,
      permissions,
    })
  }
}

#[cfg(test)]
mod tests {
  use std::io::BufReader;

  use crate::{
    data::{Grade, Mode, Mods, RankedStatus, UserPermission},
    timing::Millis,
  };

  use super::{Db, DbBeatmap, DbBeatmapTimingPoint};

  #[test]
  fn test_modern_osudb_parse() {
    use std::fs::File;

    let osr = File::open("tests/files/modern_osu.db").unwrap();
    let db = Db::parse(BufReader::new(osr)).unwrap();
    assert_eq!(db.version, 20251022);
  }

  // Thanks vernonlim for the osu.db file
  #[test]
  fn test_osudb_parse() {
    use std::fs::File;

    let osr = File::open("tests/files/osu.db").unwrap();
    let db = Db::parse(BufReader::new(osr)).unwrap();

    assert_eq!(db.version, 20201210);
    assert_eq!(db.player_name, "vernonlim");
    assert_eq!(db.folder_count, 62);
    assert_eq!(db.account_unlocked, true);
    assert_eq!(db.unlocked_date, 0);
    assert_eq!(db.beatmap_count, 245);
    assert_eq!(db.beatmaps.len(), 245);
    assert_eq!(db.beatmaps.first(), Some(&DbBeatmap {
        size: None,
        artist_name: "Drop".to_owned(),
        artist_name_unicode: "Drop".to_owned(),
        song_title: "TRICK or TREAT".to_owned(),
        song_title_unicode: "TRICK or TREAT".to_owned(),
        creator_name: "SUKIJames".to_owned(),
        difficulty: "FUTSUU".to_owned(),
        audio_file_name: "audio.mp3".to_owned(),
        hash: "7956380054f6a8023fa7614e18ffe1b6".to_owned(),
        beatmap_file_name: "Drop - TRICK or TREAT (SUKIJames) [FUTSUU].osu".to_owned(),
        ranked_status: RankedStatus::Ranked,
        hitcircle_count: 93,
        slider_count: 0,
        spinner_count: 1,
        modification_date: 637441288088683788,
        approach_rate: 10.0,
        circle_size: 3.0,
        hp_drain: 8.5,
        overall_difficulty: 4.0,
        slider_velocity: 1.4,
        std_star_rating: vec![],
        std_taiko_rating: vec![
            (
                Mods::None,
                2.2660608625099203,
            ),
            (
                Mods::DoubleTime,
                2.8621242901351933,
            ),
            (
                Mods::HalfTime,
                1.9135948603059216,
            ),
            (
                Mods::Easy,
                2.2660608625099203,
            ),
            (
                Mods::Easy | Mods::DoubleTime,
                2.8621242901351933,
            ),
            (
                Mods::HalfTime | Mods::Easy,
                1.9135948603059216,
            ),
            (
                Mods::HardRock,
                2.2660608625099203,
            ),
            (
                Mods::HardRock | Mods::DoubleTime,
                2.8621242901351933,
            ),
            (
                Mods::HardRock | Mods::HalfTime,
                1.9135948603059216,
            ),
        ],
        std_ctb_rating: vec![],
        std_mania_rating: vec![],
        drain_time: Millis(31000),
        total_time: Millis(34109),
        preview_time: Millis(5),
        timing_points: vec![
            DbBeatmapTimingPoint {
                bpm: 566.037735849057,
                offset: 147.0,
                is_uninherited: true,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 147.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 9203.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 10335.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 10618.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 10901.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 11184.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 12599.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 12882.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 13165.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 13731.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 15996.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 16562.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 17128.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 18260.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 27316.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 29580.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 29863.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 30713.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 31279.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 31845.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 32411.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 32977.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 33543.0,
                is_uninherited: false,
            },
            DbBeatmapTimingPoint {
                bpm: -71.4285714285714,
                offset: 34109.0,
                is_uninherited: false,
            },
        ],
        beatmap_id: 1989137,
        beatmap_set_id: 952626,
        thread_id: 0,
        std_grade: Grade::None,
        taiko_grade: Grade::None,
        ctb_grade: Grade::None,
        mania_grade: Grade::None,
        beatmap_offset: 0,
        stack_leniency: 0.2,
        mode: Mode::Taiko,
        source: "".to_owned(),
        tags: "stingy hatsuki yura 葉月ゆら HATU-019 wicked m3-34 instrument instrumental halloween".to_owned(),
        online_offset: 0,
        title_font: "".to_owned(),
        is_unplayed: true,
        last_played: 0,
        is_osz2: false,
        folder_name: "952626 Drop - TRICK or TREAT".to_owned(),
        last_checked: 637441576105762815,
        ignore_beatmap_sounds: false,
        ignore_beatmap_skin: false,
        disable_storyboard: false,
        disable_video: false,
        visual_override: false,
        unknown: None,
        unknown_modification_date: 0,
        mania_scrollspeed: 0,
    }));
    assert_eq!(
      db.permissions,
      UserPermission::Normal | UserPermission::Supporter
    );
  }
}
