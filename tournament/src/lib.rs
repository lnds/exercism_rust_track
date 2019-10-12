use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::HashMap;

#[derive(Eq, Ord)]
struct Score {
    name: String,
    played: u16,
    winnings: u16,
    losses: u16,
    draws: u16,
}

impl Score {
    fn new(name: &str) -> Self {
        Score {
            name: name.to_string(),
            played: 0,
            winnings: 0,
            losses: 0,
            draws: 0,
        }
    }

    fn points(&self) -> u16 {
        self.winnings * 3 + self.draws
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Score) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        if self.points() < other.points() {
            Some(Ordering::Greater)
        } else if self.points() > other.points() {
            Some(Ordering::Less)
        } else if self.name == other.name {
            Some(Ordering::Equal)
        } else if self.name < other.name {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

struct ScoreTable {
    scores: HashMap<String, Score>,
}

impl ScoreTable {
    fn new() -> Self {
        ScoreTable {
            scores: HashMap::new(),
        }
    }

    fn add_win(&mut self, team: &str) -> &mut Self {
        let score = self
            .scores
            .entry(team.to_string())
            .or_insert_with(|| Score::new(team));
        score.played += 1;
        score.winnings += 1;
        self
    }

    fn add_draw(&mut self, team: &str) -> &mut Self {
        let score = self
            .scores
            .entry(team.to_string())
            .or_insert_with(|| Score::new(team));
        score.played += 1;
        score.draws += 1;
        self
    }

    fn add_loss(&mut self, team: &str) -> &mut Self {
        let score = self
            .scores
            .entry(team.to_string())
            .or_insert_with(|| Score::new(team));
        score.played += 1;
        score.losses += 1;
        self
    }

    fn to_string(&self) -> String {
        let mut result = String::from("Team                           | MP |  W |  D |  L |  P");
        let mut table: Vec<&Score> = self.scores.values().collect();
        if !table.is_empty() {
            table.sort();
            let data: String = table
                .iter()
                .map(|score| {
                    format!(
                        "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
                        score.name,
                        score.played,
                        score.winnings,
                        score.draws,
                        score.losses,
                        score.points()
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");
            result += "\n";
            result += data.as_str();
        }
        result
    }
}

pub fn tally(match_results: &str) -> String {
    let mut score_table = ScoreTable::new();
    for line in match_results.lines() {
        let elements: Vec<&str> = line.split(';').collect();
        if elements.len() == 3 {
            let team1 = elements[0];
            let team2 = elements[1];
            let result = elements[2];
            match result {
                "win" => {
                    score_table.add_win(team1).add_loss(team2);
                }
                "draw" => {
                    score_table.add_draw(team1).add_draw(team2);
                }
                "loss" => {
                    score_table.add_loss(team1).add_win(team2);
                }
                _ => (),
            }
        }
    }
    score_table.to_string()
}
