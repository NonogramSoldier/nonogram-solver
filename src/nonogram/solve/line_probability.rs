use super::*;

#[derive(Debug)]
pub struct LineProbability {
    color_cases: Vec<Vec<u128>>,
    painting_count: u128,
}

impl LineProbability {
    pub fn new(length: usize, color_num: usize) -> Self {
        Self {
            color_cases: vec![vec![0; color_num]; length],
            painting_count: 0,
        }
    }

    pub fn solve(&mut self, line_memo: &Vec<usize>, line_clue: &LineClue, free: usize) -> bool {
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
            let mut description_notes: Vec<DescriptionNote> = Default::default();
            for (clue_index, description) in line_clue.iter().enumerate() {
                let is_first_clue = clue_index == 0;
                let min_index = if is_first_clue {
                    0
                } else {
                    description_notes[clue_index - 1].min_index
                        + line_clue[clue_index - 1].number
                        + if line_clue[clue_index - 1].color_index == description.color_index {
                            1
                        } else {
                            0
                        }
                };

                let mut segments: Vec<SegmentNote> = Default::default();
                for place_index in 0..free {
                    let is_first_place = place_index == 0;
                    let mut segment_note: SegmentNote = Default::default();

                    if is_first_place {
                        for index in (0..description.number).rev() {
                            if line_memo[min_index + index] & (1 << description.color_index) == 0 {
                                segment_note.block_states = BlockStates::Blocked(index);
                                break;
                            }
                        }
                    } else {
                        if line_memo[min_index + place_index + description.number - 1]
                            & (1 << description.color_index)
                            == 0
                        {
                            segment_note.block_states =
                                BlockStates::Blocked(description.number - 1);
                        } else {
                            if let BlockStates::Blocked(i) = segments[place_index - 1].block_states
                            {
                                if i != 0 {
                                    segment_note.block_states = BlockStates::Blocked(i - 1)
                                }
                            }
                        }
                    }

                    if is_first_clue && is_first_place {
                        segment_note.left_cases = 1;
                    } else {
                        let is_blank_possible = line_memo[min_index + place_index - 1] & 1 == 1;

                        if is_blank_possible && !is_first_place {
                            segment_note.left_cases = segments[place_index - 1].left_cases;
                        }

                        if !is_first_clue
                            && description_notes[clue_index - 1].segments[place_index].block_states
                                == BlockStates::Open
                            && (is_blank_possible
                                || !(line_clue[clue_index - 1].color_index
                                    == description.color_index))
                        {
                            segment_note.left_cases +=
                                description_notes[clue_index - 1].segments[place_index].left_cases;
                        }
                    }

                    segments.push(segment_note);
                }

                description_notes.push(DescriptionNote {
                    min_index,
                    segments,
                })
            }

            for (clue_index, description) in line_clue.iter().enumerate().rev() {
                let min_index = description_notes[clue_index].min_index;
                let is_first_clue = clue_index == line_clue.len() - 1;
                for place_index in (0..free).rev() {
                    let is_first_place = place_index == free - 1;

                    if is_first_clue && is_first_place {
                        description_notes[clue_index].segments[place_index].right_cases = 1;
                    } else {
                        let is_blank_possible =
                            line_memo[min_index + place_index + description.number] & 1 == 1;

                        if is_blank_possible && !is_first_place {
                            description_notes[clue_index].segments[place_index].right_cases =
                                description_notes[clue_index].segments[place_index + 1].right_cases;
                        }

                        if !is_first_clue
                            && description_notes[clue_index + 1].segments[place_index].block_states
                                == BlockStates::Open
                            && (is_blank_possible
                                || !(line_clue[clue_index + 1].color_index
                                    == description.color_index))
                        {
                            description_notes[clue_index].segments[place_index].right_cases +=
                                description_notes[clue_index + 1].segments[place_index].right_cases;
                        }
                    }
                }
            }

            for color_case in self.color_cases.iter_mut() {
                color_case.fill(0);
            }

            self.painting_count = 0;

            for (clue_index, description) in line_clue.iter().enumerate() {
                let min_index = description_notes[clue_index].min_index;
                for (place_index, segment) in
                    description_notes[clue_index].segments.iter().enumerate()
                {
                    let product = match segment.block_states {
                        BlockStates::Open => segment.left_cases * segment.right_cases,
                        _ => 0,
                    };

                    for in_segment in 0..description.number {
                        self.color_cases[min_index + place_index + in_segment]
                            [description.color_index] += product;
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

#[derive(Debug)]
struct DescriptionNote {
    min_index: usize,
    segments: Vec<SegmentNote>,
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq, Eq)]
enum BlockStates {
    Blocked(usize /* caused index */),
    Open,
}
