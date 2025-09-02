pub trait FloatIterExt {
    fn float_min(&mut self) -> f32;
    fn float_max(&mut self) -> f32;
}

impl<T> FloatIterExt for T
where T: Iterator<Item = f32>
{
    fn float_min(&mut self) -> f32 {
        self.fold(f32::NAN, f32::min)
    }

    fn float_max(&mut self) -> f32 {
        self.fold(f32::NAN, f32::max)
    }
}

pub trait Apply<T, Ret> {
    fn apply(self, action: impl FnMut(T) -> Ret);
    fn apply2<U, Second: IntoIterator<Item = U>>(self, second: Second, action: impl FnMut(T, U) -> Ret);
}

impl<T, I: IntoIterator<Item = T>, Ret> Apply<T, Ret> for I {
    fn apply(self, mut action: impl FnMut(T) -> Ret) {
        for item in self {
            action(item);
        }
    }

    fn apply2<U, Second: IntoIterator<Item = U>>(self, second: Second, mut action: impl FnMut(T, U) -> Ret) {
        for (item, second) in self.into_iter().zip(second) {
            action(item, second);
        }
    }
}

pub trait Toggle {
    fn toggle(&mut self) -> bool;
}

/// Returns old value
impl Toggle for bool {
    fn toggle(&mut self) -> bool {
        *self = !*self;
        !*self
    }
}

// #[cfg(test)]
// mod test {
//     use fake::{Fake, Faker};
//
//     use crate::misc::{Apply, Toggle};
//
//     #[test]
//     fn apply_arr() {
//         let mut ve = vec![];
//         [1, 2, 3, 4, 5].apply(|a| {
//             ve.push(a);
//         });
//         assert_eq!(&ve, &[1, 2, 3, 4, 5]);
//     }
//
//     #[test]
//     fn apply_tuple() {
//         let mut num = vec![];
//         let mut ch = vec![];
//         [(1, '5'), (2, '4'), (3, '3'), (4, '2'), (5, '1')].apply(|(n, c)| {
//             num.push(n);
//             ch.push(c.clone());
//         });
//         assert_eq!(&num, &[1, 2, 3, 4, 5]);
//         assert_eq!(&ch, &['5', '4', '3', '2', '1']);
//     }
//
//     #[test]
//     fn apply2_arr() {
//         let mut num = vec![];
//         let mut ch = vec![];
//         [1, 2, 3, 4, 5].apply2(['5', '4', '3', '2', '1'], |n, c| {
//             num.push(n);
//             ch.push(c);
//         });
//         assert_eq!(&num, &[1, 2, 3, 4, 5]);
//         assert_eq!(&ch, &['5', '4', '3', '2', '1']);
//     }
//
//     #[test]
//     fn toggle() {
//         let mut val = Faker.fake::<bool>();
//
//         for _ in 0..10 {
//             let prev = val;
//             assert_eq!(val.toggle(), prev);
//             assert_eq!(val, !prev);
//         }
//     }
// }
