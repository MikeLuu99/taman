use chrono::{DateTime, NaiveDate, Utc, Duration};
use crate::plant::Plant;

const FLOWER_EMOJIS: &[&str] = &["🌸", "🌺", "🌻", "🌹", "🌷", "🪷", "🌼", "💐"];

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompletedPlant {
    pub plant: Plant,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum GardenEntryKind {
    Plant,
    Flower,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GardenEntry {
    pub kind: GardenEntryKind,
    pub label: String,
    pub emoji: String,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Garden {
    pub completed_plants: Vec<CompletedPlant>,
    pub entries: Vec<GardenEntry>,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub current_streak_start_date: Option<DateTime<Utc>>,
    pub longest_streak_end_date: Option<DateTime<Utc>>,
    pub current_streak_dates: Vec<NaiveDate>,
    pub longest_streak_dates: Vec<NaiveDate>,
}

impl Garden {
    pub fn new() -> Self {
        Garden {
            completed_plants: vec![],
            entries: vec![],
            current_streak: 0,
            longest_streak: 0,
            current_streak_start_date: None,
            longest_streak_end_date: None,
            current_streak_dates: vec![],
            longest_streak_dates: vec![],
        }
    }

    pub fn add_completed_plant(&mut self, plant: Plant) {
        let completed = CompletedPlant {
            plant,
            completed_at: Utc::now(),
        };
        self.completed_plants.push(completed);
    }

    pub fn add_flower_entry(&mut self, title: &str) {
        let emoji = FLOWER_EMOJIS[self.entries.len() % FLOWER_EMOJIS.len()].to_string();
        self.entries.push(GardenEntry {
            kind: GardenEntryKind::Flower,
            label: title.to_string(),
            emoji,
            completed_at: Utc::now(),
        });
    }

    pub fn add_plant_entry(&mut self) {
        self.entries.push(GardenEntry {
            kind: GardenEntryKind::Plant,
            label: "Fully Grown Plant".to_string(),
            emoji: "🪴".to_string(),
            completed_at: Utc::now(),
        });
    }



    pub fn total_completed(&self) -> usize {
        self.completed_plants.len()
    }

    pub fn update_streaks(&mut self, recent_sessions: &[(DateTime<chrono::Local>, u32)]) {
        use chrono::Local;
        if recent_sessions.is_empty() {
            self.current_streak = 0;
            self.longest_streak = 0;
            self.current_streak_start_date = None;
            self.longest_streak_end_date = None;
            self.current_streak_dates = vec![];
            self.longest_streak_dates = vec![];
            return;
        }
        let mut dates: Vec<NaiveDate> = recent_sessions.iter().map(|(d, _)| d.date_naive()).collect();
        dates.sort();
        dates.dedup();
        let today = Local::now().date_naive();
        let yesterday = today - Duration::days(1);
        let last_date = *dates.last().unwrap();
        let (current_streak, current_dates) = if last_date == today || last_date == yesterday {
            // find the group ending with last_date
            let mut i = dates.len() - 1;
            while i > 0 && dates[i] == dates[i - 1] + Duration::days(1) {
                i -= 1;
            }
            let current_dates = dates[i..].to_vec();
            (current_dates.len() as u32, current_dates)
        } else {
            (0, vec![])
        };
        self.current_streak = current_streak;
        self.current_streak_dates = current_dates;
        if current_streak > 0 {
            self.current_streak_start_date = Some(DateTime::<Utc>::from_naive_utc_and_offset(
                self.current_streak_dates[0].and_hms_opt(0, 0, 0).unwrap(),
                Utc,
            ));
        } else {
            self.current_streak_start_date = None;
        }
        // now for longest
        let mut longest = 0;
        let mut longest_dates = vec![];
        let mut i = 0;
        while i < dates.len() {
            let mut j = i;
            while j + 1 < dates.len() && dates[j + 1] == dates[j] + Duration::days(1) {
                j += 1;
            }
            let len = (j - i + 1) as u32;
            if len > longest {
                longest = len;
                longest_dates = dates[i..=j].to_vec();
            }
            i = j + 1;
        }
        self.longest_streak = longest;
        self.longest_streak_dates = longest_dates;
        if longest > 0 {
            self.longest_streak_end_date = Some(DateTime::<Utc>::from_naive_utc_and_offset(
                self.longest_streak_dates.last().unwrap().and_hms_opt(0, 0, 0).unwrap(),
                Utc,
            ));
        } else {
            self.longest_streak_end_date = None;
        }
    }
}