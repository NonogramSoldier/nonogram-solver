use super::*;

#[derive(Debug)]
pub struct GridProbability<'a> {
    parent: Option<&'a GridProbability<'a>>,
    line_probabilities: FxHashMap<LineId, LineProbability>,
}

impl<'a> GridProbability<'a> {
    pub fn new(parent: Option<&'a GridProbability<'a>>) -> Self {
        Self {
            parent,
            line_probabilities: Default::default(),
        }
    }

    fn line_solve(
        &mut self,
        line_id: LineId,
        line_memo: Vec<PixelMemo>,
        line_clue: Vec<Description>,
        resources: &SolveResources,
    ) -> bool {
        let solve_line = self
            .line_probabilities
            .entry(line_id)
            .or_insert(LineProbability::new(
                match line_id {
                    LineId::Row(_) => resources.get_width(),
                    LineId::Column(_) => resources.get_height(),
                },
                resources.get_color_num(),
            ));
        
        solve_line.solve(line_memo, line_clue, *resources.get_free(line_id).unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct LineProbability {
    color_cases: Vec<ColorCases>,
    painting_count: u128,
}

impl LineProbability {
    pub fn new(length: usize, color_num: usize) -> Self {
        Self {
            color_cases: vec![ColorCases::new(color_num); length],
            painting_count: 0,
        }
    }

    pub fn solve(
        &mut self,
        line_memo: Vec<PixelMemo>,
        line_clue: Vec<Description>,
        free: usize,
    ) -> bool {
        if line_clue.len() == 0 {
            for (index, memo) in line_memo.iter().enumerate() {
                if memo.blank_possibility == Possibility::Impossible {
                    return false;
                }
                self.color_cases[index].blank_num = 1;
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
                            if line_memo[min_index + index].paint_possibilities
                                [description.color_index]
                                == Possibility::Impossible
                            {
                                segment_note.block_states = BlockStates::Blocked(index);
                                break;
                            }
                        }
                    } else {
                        if line_memo[min_index + place_index + description.number - 1]
                            .paint_possibilities[description.color_index]
                            == Possibility::Impossible
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
                        segment_note.left_case = 1;
                    } else {
                        let is_blank_possible = line_memo[min_index + place_index - 1]
                            .blank_possibility
                            == Possibility::Possible;

                        if is_blank_possible && !is_first_place {
                            segment_note.left_case = segments[place_index - 1].left_case;
                        }

                        if !is_first_clue
                            && description_notes[clue_index - 1].segments[place_index].block_states
                                == BlockStates::Open
                            && (is_blank_possible
                                || !(line_clue[clue_index - 1].color_index
                                    == description.color_index))
                        {
                            segment_note.left_case +=
                                description_notes[clue_index - 1].segments[place_index].left_case;
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
                        description_notes[clue_index].segments[place_index].right_case = 1;
                    } else {
                        let is_blank_possible = line_memo
                            [min_index + place_index + description.number]
                            .blank_possibility
                            == Possibility::Possible;

                        if is_blank_possible && !is_first_place {
                            description_notes[clue_index].segments[place_index].right_case =
                                description_notes[clue_index].segments[place_index + 1].right_case;
                        }

                        if !is_first_clue
                            && description_notes[clue_index + 1].segments[place_index].block_states
                                == BlockStates::Open
                            && (is_blank_possible
                                || !(line_clue[clue_index + 1].color_index
                                    == description.color_index))
                        {
                            description_notes[clue_index].segments[place_index].right_case +=
                                description_notes[clue_index + 1].segments[place_index].right_case;
                        }
                    }
                }
            }

            for color_case in self.color_cases.iter_mut() {
                color_case.paint_nums.fill(0);
            }

            self.painting_count = 0;

            for (clue_index, description) in line_clue.iter().enumerate() {
                let min_index = description_notes[clue_index].min_index;
                for (place_index, segment) in
                    description_notes[clue_index].segments.iter().enumerate()
                {
                    let product = match segment.block_states {
                        BlockStates::Open => segment.left_case * segment.right_case,
                        _ => 0,
                    };

                    for in_segment in 0..description.number {
                        self.color_cases[min_index + place_index + in_segment].paint_nums
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
                color_case.blank_num = self.painting_count;
                for paint_case in color_case.paint_nums.iter() {
                    color_case.blank_num -= paint_case;
                }
            }
            // println!("{:#?}", description_notes);
            true
        }
    }
}

#[derive(Debug)]
struct DescriptionNote {
    min_index: usize,
    segments: Vec<SegmentNote>,
}

impl DescriptionNote {
    fn new(min_index: usize) -> Self {
        Self {
            min_index,
            segments: Default::default(),
        }
    }
}

#[derive(Debug)]
struct SegmentNote {
    block_states: BlockStates,
    left_case: u128,
    right_case: u128,
}

impl Default for SegmentNote {
    fn default() -> Self {
        Self {
            block_states: BlockStates::Open,
            left_case: 0,
            right_case: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum BlockStates {
    Blocked(usize /* caused index */),
    Open,
}

#[derive(Debug, Clone)]
struct ColorCases {
    blank_num: u128,
    paint_nums: Vec<u128>,
}

impl ColorCases {
    fn new(color_num: usize) -> Self {
        Self {
            blank_num: 0,
            paint_nums: vec![0; color_num],
        }
    }
}
