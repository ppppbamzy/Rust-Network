use anyhow::format_err;
use serde::Deserialize;
// use std::collections::VecDeque;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct OneData {
  pub data: Vec<u8>,
}

// #[derive(Debug, Clone)]
// struct DataQueue {
//   pub data: VecDeque<OneData>,
// }

/// The communication statistics
#[derive(Default, Debug, Clone)]
pub struct NetStat {
  /// Number of calls to `io.send`.
  pub sent_count: usize,
  /// The real data size.
  pub sent_bytes: usize,
  /// Total size. Protobuf serialized data (message id, real data, etc.) size.
  pub sent_bytes_all: usize,
}

/// Participant info
#[derive(Debug, Clone, Deserialize)]
pub struct Participant {
  /// The party id, from 0 to n-1.
  pub partyid: u32,
  /// The node id. Not used now.
  pub nodeid: String,
  /// Now only supports valid IPv4.
  pub addr: String,
}

impl Participant {
  pub fn from_default(parties: u32) -> Vec<Participant> {
    let mut participants = Vec::new();
    for i in 0..parties {
      let participant = Participant {
        partyid: i,
        nodeid: "node".to_string() + &i.to_string(),
        addr: "127.0.0.1:".to_string() + &(i + 13500).to_string(),
      };
      participants.push(participant);
    }
    return participants;
  }
  pub fn from_config(config_path: &Path) -> Result<Vec<Participant>, anyhow::Error> {
    let configstr = fs::read_to_string(config_path)
      .map_err(|e| format_err!("Couldn't open {}: {}", config_path.display(), e))?;

    let participants: Vec<Participant> = serde_json::from_str(&configstr)
      .map_err(|e| format_err!("Couldn't deserialize config: {}", e))?;

    Ok(participants)
  }
}

/// For debug or simple usage.
pub fn get_default_participants(parties: u32) -> Vec<Participant> {
  Participant::from_default(parties)
}
