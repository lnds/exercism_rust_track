use std::collections::BTreeMap;

#[derive(Default)]
struct Score {
    wins: u16,
    losses: u16,
    draws: u16,
}

impl Score {
    fn points(&self) -> u16 {
        self.wins * 3 + self.draws
    }

    fn played(&self) -> u16 {
        self.wins + self.losses + self.draws
    }
}

struct ScoreTable {
    scores: BTreeMap<String, Score>,
}

impl ScoreTable {
    fn new() -> Self {
        ScoreTable {
            scores: BTreeMap::new(),
        }
    }

    fn add_win(&mut self, team: &str) -> &mut Self {
        self.scores
            .entry(team.to_string())
            .or_insert_with(Score::default)
            .wins += 1;
        self
    }

    fn add_draw(&mut self, team: &str) -> &mut Self {
        self.scores
            .entry(team.to_string())
            .or_insert_with(Score::default)
            .draws += 1;
        self
    }

    fn add_loss(&mut self, team: &str) -> &mut Self {
        self.scores
            .entry(team.to_string())
            .or_insert_with(Score::default)
            .losses += 1;
        self
    }

    fn to_string(&self) -> String {
        let header = format!("{:<31}| MP |  W |  D |  L |  P", "Team");
        let mut scores: Vec<_> = self
            .scores
            .iter()
            .map(|(name, score)| (name, score))
            .collect();
        scores.sort_by(|a, b| a.1.points().cmp(&b.1.points()).reverse());
        scores
            .into_iter()
            .fold(vec![header], |mut result, x| {
                let (name, score) = x;
                result.push(format!(
                    "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
                    name,
                    score.played(),
                    score.wins,
                    score.draws,
                    score.losses,
                    score.points()
                ));
                result
            })
            .join("\n")
    }
}

pub fn tally(match_results: &str) -> String {
    let mut score_table = ScoreTable::new();
    match_results
        .lines()
        .map(|l| l.split(';').collect::<Vec<_>>())
        .for_each(|line| match line[2] {
            "win" => {
                score_table.add_win(line[0]).add_loss(line[1]);
            }
            "draw" => {
                score_table.add_draw(line[0]).add_draw(line[1]);
            }
            "loss" => {
                score_table.add_loss(line[0]).add_win(line[1]);
            }
            _ => panic!("unexpected result for match"),
        });
    score_table.to_string()
}
