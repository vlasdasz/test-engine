use std::str::FromStr;

use gm::flat::Point;
use itertools::Itertools;

use crate::input::TouchEvent;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Touch {
    pub id:       u64,
    pub position: Point,
    pub event:    TouchEvent,
}

impl Touch {
    pub fn is_began(&self) -> bool {
        self.event == TouchEvent::Began
    }

    pub fn is_moved(&self) -> bool {
        self.event == TouchEvent::Moved
    }

    pub fn is_ended(&self) -> bool {
        self.event == TouchEvent::Ended
    }
}

impl Touch {
    pub fn vec_from_str(s: &str) -> Vec<Self> {
        s.split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(|s| s.parse().unwrap())
            .collect()
    }
}

impl ToString for Touch {
    fn to_string(&self) -> String {
        format!(
            "{:>4} {:>4} {}",
            self.position.x,
            self.position.y,
            self.event.to_string()
        )
    }
}

impl FromStr for Touch {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s.split_whitespace().collect_vec();

        let touch = Touch {
            id:       0,
            position: Point {
                x: vals[0].parse()?,
                y: vals[1].parse()?,
            },
            event:    vals[2].parse()?,
        };

        Ok(touch)
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::{input::TouchEvent, Touch};

    #[test]
    fn touch_to_string() {
        let touches = [
            Touch {
                id:       0,
                position: (0, 0).into(),
                event:    TouchEvent::Began,
            },
            Touch {
                id:       0,
                position: (2000, 10).into(),
                event:    TouchEvent::Ended,
            },
            Touch {
                id:       0,
                position: (100, 4000).into(),
                event:    TouchEvent::Ended,
            },
            Touch {
                id:       0,
                position: (1, 4000).into(),
                event:    TouchEvent::Moved,
            },
            Touch {
                id:       0,
                position: (4000, 1).into(),
                event:    TouchEvent::Moved,
            },
        ];

        let result = touches.into_iter().map(|t| t.to_string()).join("\n");

        assert_eq!(
            result,
            r#"   0    0 ↓
2000   10 ↑
 100 4000 ↑
   1 4000 →
4000    1 →"#
        );

        assert_eq!(touches.as_slice(), &Touch::vec_from_str(&result));

        assert_eq!(
            touches.as_slice(),
            &Touch::vec_from_str(
                r#"
                                       0    0 ↓
                                    2000   10 ↑
                                     100 4000 ↑
                                       1 4000 →
                                    4000    1 →
                "#
            )
        );

        assert_eq!(
            vec![Touch {
                id:       0,
                position: (10, 20).into(),
                event:    TouchEvent::Began,
            }],
            Touch::vec_from_str("10 20 ↓")
        );
    }
}
