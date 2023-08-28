mod tournament {
    use std::{cmp::Ordering, collections::HashMap, ops::Neg, str::FromStr};

    #[derive(Clone, Copy)]
    enum MatchResult {
        Win,
        Draw,
        Loss,
    }

    impl Neg for MatchResult {
        type Output = Self;

        fn neg(self) -> Self::Output {
            match self {
                MatchResult::Win => MatchResult::Loss,
                MatchResult::Draw => MatchResult::Draw,
                MatchResult::Loss => MatchResult::Win,
            }
        }
    }

    impl FromStr for MatchResult {
        type Err = ();

        fn from_str(input: &str) -> Result<MatchResult, Self::Err> {
            match input {
                "win" => Ok(MatchResult::Win),
                "draw" => Ok(MatchResult::Draw),
                "loss" => Ok(MatchResult::Loss),
                _ => Err(()),
            }
        }
    }

    #[derive(Debug, Default, Eq, PartialEq, Ord)]
    struct Team {
        name: String,
        total: i16,
        win: i16,
        draw: i16,
        loss: i16,
        points: i16,
    }

    impl Team {
        fn new(name: String) -> Self {
            Self {
                name: name,
                ..Self::default()
            }
        }

        fn process_result(&mut self, match_result: MatchResult) {
            match match_result {
                MatchResult::Win => {
                    self.total += 1;
                    self.win += 1;
                    self.points += 3;
                }
                MatchResult::Draw => {
                    self.total += 1;
                    self.draw += 1;
                    self.points += 1;
                }
                MatchResult::Loss => {
                    self.total += 1;
                    self.loss += 1;
                }
            }
        }
    }

    impl PartialOrd for Team {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.points == other.points {
                return Some(self.name.cmp(&other.name));
            }
            Some(other.points.cmp(&self.points))
        }
    }

    type MatchLine<'a> = (&'a str, &'a str, MatchResult);

    fn parse_line(line: &str) -> Option<MatchLine> {
        let (teams, result) = line.rsplit_once(';')?;
        let (team1, team2) = teams.split_once(';')?;
        let match_result = MatchResult::from_str(result).ok()?;
        Some((team1, team2, match_result))
    }

    pub fn tally(input: &str) -> String {
        let mut output = "Team                           | MP |  W |  D |  L |  P".to_owned();
        let mut results: HashMap<&str, Team> = HashMap::new();
        for line in input.lines() {
            println!("line: {}", line);
            let Some((team1_name, team2_name, match_result)) = parse_line(line) else {
                continue;
            };

            let team1 = results
                .entry(team1_name)
                .or_insert(Team::new(team1_name.to_string()));
            team1.process_result(match_result);

            let team2 = results
                .entry(team2_name)
                .or_insert(Team::new(team2_name.to_string()));
            team2.process_result(-match_result);
        }

        let mut table = results
            .into_values()
            .map(|(team)| team)
            .collect::<Vec<Team>>();
        table.sort();

        for team in table {
            println!("{:?}", team);
            output += &format!(
                "\n{team_name:<31}|  {MP} |  {W} |  {D} |  {L} |  {P}",
                team_name = team.name,
                MP = team.total,
                W = team.win,
                D = team.draw,
                L = team.loss,
                P = team.points
            );
        }

        output
    }
}

#[test]
fn just_the_header_if_no_input() {
    let input = "";
    let expected = "Team                           | MP |  W |  D |  L |  P";
    assert_eq!(tournament::tally(input), expected);
}
#[test]
#[ignore]
fn a_win_is_three_points_a_loss_is_zero_points() {
    let input = "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3\n"
        + "Blithering Badgers             |  1 |  0 |  0 |  1 |  0";
    assert_eq!(tournament::tally(input), expected);
}
#[test]
#[ignore]
fn a_win_can_also_be_expressed_as_a_loss() {
    let input = "Blithering Badgers;Allegoric Alaskans;loss";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3\n"
        + "Blithering Badgers             |  1 |  0 |  0 |  1 |  0";
    assert_eq!(tournament::tally(input), expected);
}
#[test]
#[ignore]
fn a_different_team_can_win() {
    let input = "Blithering Badgers;Allegoric Alaskans;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Blithering Badgers             |  1 |  1 |  0 |  0 |  3\n"
        + "Allegoric Alaskans             |  1 |  0 |  0 |  1 |  0";
    assert_eq!(tournament::tally(input), expected);
}
#[test]
#[ignore]
fn there_can_be_more_than_one_match() {
    let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6\n"
        + "Blithering Badgers             |  2 |  0 |  0 |  2 |  0";
    assert_eq!(tournament::tally(&input), expected);
}
#[test]
#[ignore]
fn a_draw_is_one_point_each() {
    let input = "Allegoric Alaskans;Blithering Badgers;draw\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  1 |  1 |  0 |  4\n"
        + "Blithering Badgers             |  2 |  0 |  1 |  1 |  1";
    assert_eq!(tournament::tally(&input), expected);
}
#[test]
#[ignore]
fn there_can_be_more_than_one_winner() {
    let input = "Allegoric Alaskans;Blithering Badgers;loss\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  1 |  0 |  1 |  3\n"
        + "Blithering Badgers             |  2 |  1 |  0 |  1 |  3";
    assert_eq!(tournament::tally(&input), expected);
}
#[test]
#[ignore]
fn there_can_be_more_than_two_teams() {
    let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
        + "Blithering Badgers;Courageous Californians;win\n"
        + "Courageous Californians;Allegoric Alaskans;loss";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6\n"
        + "Blithering Badgers             |  2 |  1 |  0 |  1 |  3\n"
        + "Courageous Californians        |  2 |  0 |  0 |  2 |  0";
    assert_eq!(tournament::tally(&input), expected);
}
#[test]
#[ignore]
fn typical_input() {
    let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
        + "Devastating Donkeys;Courageous Californians;draw\n"
        + "Devastating Donkeys;Allegoric Alaskans;win\n"
        + "Courageous Californians;Blithering Badgers;loss\n"
        + "Blithering Badgers;Devastating Donkeys;loss\n"
        + "Allegoric Alaskans;Courageous Californians;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7\n"
        + "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6\n"
        + "Blithering Badgers             |  3 |  1 |  0 |  2 |  3\n"
        + "Courageous Californians        |  3 |  0 |  1 |  2 |  1";
    assert_eq!(tournament::tally(&input), expected);
}
#[test]
#[ignore]
fn incomplete_competition_not_all_pairs_have_played() {
    let input = "Allegoric Alaskans;Blithering Badgers;loss\n".to_string()
        + "Devastating Donkeys;Allegoric Alaskans;loss\n"
        + "Courageous Californians;Blithering Badgers;draw\n"
        + "Allegoric Alaskans;Courageous Californians;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6\n"
        + "Blithering Badgers             |  2 |  1 |  1 |  0 |  4\n"
        + "Courageous Californians        |  2 |  0 |  1 |  1 |  1\n"
        + "Devastating Donkeys            |  1 |  0 |  0 |  1 |  0";
    assert_eq!(tournament::tally(&input), expected);
}
#[test]
#[ignore]
fn ties_broken_alphabetically() {
    let input = "Courageous Californians;Devastating Donkeys;win\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win\n"
        + "Devastating Donkeys;Allegoric Alaskans;loss\n"
        + "Courageous Californians;Blithering Badgers;win\n"
        + "Blithering Badgers;Devastating Donkeys;draw\n"
        + "Allegoric Alaskans;Courageous Californians;draw";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  3 |  2 |  1 |  0 |  7\n"
        + "Courageous Californians        |  3 |  2 |  1 |  0 |  7\n"
        + "Blithering Badgers             |  3 |  0 |  1 |  2 |  1\n"
        + "Devastating Donkeys            |  3 |  0 |  1 |  2 |  1";
    assert_eq!(tournament::tally(&input), expected);
}
