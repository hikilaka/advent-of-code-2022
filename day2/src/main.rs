#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum GameResult {
    PlayerAWon,
    PlayerBWon,
    Draw,
}

impl GameResult {
    fn move_for_desired_outcome(self, player_a_move: PlayerMove) -> PlayerMove {
        if self == GameResult::Draw {
            return player_a_move;
        }

        match self {
            GameResult::Draw => player_a_move,
            GameResult::PlayerAWon => player_a_move.opposing_move().opposing_move(),
            GameResult::PlayerBWon => player_a_move.opposing_move(),
        }
    }
}

impl From<char> for GameResult {
    fn from(ch: char) -> Self {
        match ch {
            'X' => GameResult::PlayerAWon,
            'Y' => GameResult::Draw,
            'Z' => GameResult::PlayerBWon,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum PlayerMove {
    Rock,
    Paper,
    Scissors,
}

impl PlayerMove {
    fn score_for_move(self) -> u32 {
        match self {
            PlayerMove::Rock => 1,
            PlayerMove::Paper => 2,
            PlayerMove::Scissors => 3,
        }
    }

    fn result_of(self, other: PlayerMove) -> GameResult {
        if self == other {
            return GameResult::Draw;
        }

        if self.opposing_move() == other {
            GameResult::PlayerBWon
        } else {
            GameResult::PlayerAWon
        }
    }

    fn opposing_move(self) -> PlayerMove {
        match self {
            PlayerMove::Rock => PlayerMove::Paper,
            PlayerMove::Paper => PlayerMove::Scissors,
            PlayerMove::Scissors => PlayerMove::Rock,
        }
    }
}

impl From<char> for PlayerMove {
    fn from(ch: char) -> Self {
        match ch {
            'A' | 'X' => PlayerMove::Rock,
            'B' | 'Y' => PlayerMove::Paper,
            'C' | 'Z' => PlayerMove::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct GameRound {
    their_move: PlayerMove,
    desired_outcome: GameResult,
}

impl GameRound {
    fn score_for_round(self, our_move: PlayerMove) -> u32 {
        let mut score = our_move.score_for_move();

        score += match self.their_move.result_of(our_move) {
            GameResult::Draw => 3,
            GameResult::PlayerAWon => 0,
            GameResult::PlayerBWon => 6,
        };

        score
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let moves = data
        .lines()
        .map(|line| GameRound {
            their_move: PlayerMove::from(line.chars().nth(0).unwrap()),
            desired_outcome: GameResult::from(line.chars().nth(2).unwrap()),
        })
        .collect::<Vec<GameRound>>();

    let total_score = moves
        .into_iter()
        .map(|round| {
            round.score_for_round(
                round
                    .desired_outcome
                    .move_for_desired_outcome(round.their_move),
            )
        })
        .sum::<u32>();

    println!("{total_score}");
}
