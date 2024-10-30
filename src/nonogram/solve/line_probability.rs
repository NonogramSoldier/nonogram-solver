use super::*;

#[derive(Debug)]
pub struct LineProbability {
    color_cases: Vec<Vec<u128>>,
    painting_count: u128,
    description_notes: Vec<DescriptionNote>,
}

impl LineProbability {
    pub fn new(
        resources: &SolveResources,
        line_id: LineId,
        parent: Option<&LineProbability>,
    ) -> Self {
        Self {
            color_cases: vec![vec![0; resources.color_num]; resources.get_length(line_id)],
            painting_count: 0,
            description_notes: match parent {
                Some(parent) => parent.description_notes.clone(),
                None => {
                    let free = resources.get_free(line_id);
                    let line_clue = resources.get_line_clue(line_id);
                    let mut description_notes: Vec<DescriptionNote> = Default::default();

                    for clue_index in 0..line_clue.len() {
                        if clue_index == 0 {
                            description_notes.push(DescriptionNote::new(free, 0));
                        } else {
                            description_notes.push(DescriptionNote::new(free, {
                                let pre_index = clue_index - 1;
                                description_notes[pre_index].min_index
                                    + line_clue[pre_index].number
                                    + if line_clue[pre_index].color_index
                                        == line_clue[clue_index].color_index
                                    {
                                        1
                                    } else {
                                        0
                                    }
                            }));
                        }
                    }
                    description_notes
                }
            },
        }
    }

    pub fn solve(&mut self, line_memo: &Vec<usize>, line_clue: &LineClue) -> bool {
        if line_clue.len() == 0 {
            for (index, memo) in line_memo.iter().enumerate() {
                if memo & 1 == 0 {
                    return false;
                }
                self.color_cases[index][0] = 1;
            }
            self.painting_count = 1;
            true
        } else {
            for (clue_index, description) in line_clue.iter().enumerate() {
                let min_index = self.description_notes[clue_index].min_index;
                let is_first_clue = clue_index == 0;
                for place_index in 0..self.description_notes[clue_index].segments.len() {
                    let is_first_place = place_index == 0;

                    if is_first_place {
                        for index in (0..description.number).rev() {
                            if line_memo[min_index + index] & (1 << description.color_index) == 0 {
                                self.description_notes[clue_index].segments[place_index]
                                    .block_states = BlockStates::Blocked(index);
                                break;
                            }
                        }
                    } else {
                        if line_memo[min_index + place_index + description.number - 1]
                            & (1 << description.color_index)
                            == 0
                        {
                            self.description_notes[clue_index].segments[place_index].block_states =
                                BlockStates::Blocked(description.number - 1);
                        } else {
                            if let BlockStates::Blocked(i) = self.description_notes[clue_index]
                                .segments[place_index - 1]
                                .block_states
                            {
                                if i != 0 {
                                    self.description_notes[clue_index].segments[place_index]
                                        .block_states = BlockStates::Blocked(i - 1)
                                }
                            }
                        }
                    }

                    self.description_notes[clue_index].segments[place_index].left_cases = 0;

                    if is_first_clue && is_first_place {
                        self.description_notes[clue_index].segments[place_index].left_cases = 1
                    } else {
                        let is_blank_possible = line_memo[min_index + place_index - 1] & 1 == 1;

                        if is_blank_possible && !is_first_place {
                            self.description_notes[clue_index].segments[place_index].left_cases =
                                self.description_notes[clue_index].segments[place_index - 1]
                                    .left_cases;
                        }

                        if !is_first_clue
                            && self.description_notes[clue_index - 1].segments[place_index]
                                .block_states
                                == BlockStates::Open
                            && (is_blank_possible
                                || !(line_clue[clue_index - 1].color_index
                                    == description.color_index))
                        {
                            self.description_notes[clue_index].segments[place_index].left_cases +=
                                self.description_notes[clue_index - 1].segments[place_index]
                                    .left_cases;
                        }
                    }
                }
            }

            for (clue_index, description) in line_clue.iter().enumerate().rev() {
                let min_index = self.description_notes[clue_index].min_index;
                let is_first_clue = clue_index == line_clue.len() - 1;
                for place_index in (0..self.description_notes[clue_index].segments.len()).rev() {
                    let is_first_place =
                        place_index == self.description_notes[clue_index].segments.len() - 1;

                    self.description_notes[clue_index].segments[place_index].right_cases = 0;

                    if is_first_clue && is_first_place {
                        self.description_notes[clue_index].segments[place_index].right_cases = 1;
                    } else {
                        let is_blank_possible =
                            line_memo[min_index + place_index + description.number] & 1 == 1;

                        if is_blank_possible && !is_first_place {
                            self.description_notes[clue_index].segments[place_index].right_cases =
                                self.description_notes[clue_index].segments[place_index + 1]
                                    .right_cases;
                        }

                        if !is_first_clue
                            && self.description_notes[clue_index + 1].segments[place_index]
                                .block_states
                                == BlockStates::Open
                            && (is_blank_possible
                                || !(line_clue[clue_index + 1].color_index
                                    == description.color_index))
                        {
                            self.description_notes[clue_index].segments[place_index].right_cases +=
                                self.description_notes[clue_index + 1].segments[place_index]
                                    .right_cases;
                        }
                    }
                }
            }

            for color_case in self.color_cases.iter_mut() {
                color_case.fill(0);
            }

            self.painting_count = 0;

            for (clue_index, description) in line_clue.iter().enumerate() {
                let min_index = self.description_notes[clue_index].min_index;
                for (place_index, segment) in self.description_notes[clue_index]
                    .segments
                    .iter()
                    .enumerate()
                {
                    let product = match segment.block_states {
                        BlockStates::Open => segment.left_cases * segment.right_cases,
                        _ => 0,
                    };

                    for index in
                        (min_index + place_index)..(min_index + place_index + description.number)
                    {
                        self.color_cases[index][description.color_index] += product;
                    }

                    if clue_index == 0 {
                        self.painting_count += product;
                    }
                }
            }

            if self.painting_count == 0 {
                return false;
            }

            for color_case in self.color_cases.iter_mut() {
                color_case[0] = self.painting_count;
                for paint_index in 1..color_case.len() {
                    color_case[0] -= color_case[paint_index];
                }
            }

            true
        }
    }

    pub fn get_color_case(&self, pixel_index: usize, color_index: usize) -> u128 {
        self.color_cases[pixel_index][color_index]
    }

    pub fn get_painting_count(&self) -> u128 {
        self.painting_count
    }
}

#[derive(Debug, Clone)]
struct DescriptionNote {
    min_index: usize,
    segments: Vec<SegmentNote>,
}

impl DescriptionNote {
    fn new(free: usize, min_index: usize) -> Self {
        Self {
            min_index,
            segments: vec![SegmentNote::default(); free],
        }
    }
}

#[derive(Debug, Clone)]
struct SegmentNote {
    block_states: BlockStates,
    left_cases: u128,
    right_cases: u128,
}

impl Default for SegmentNote {
    fn default() -> Self {
        Self {
            block_states: BlockStates::Open,
            left_cases: 0,
            right_cases: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum BlockStates {
    Blocked(usize /* caused index */),
    Open,
}
