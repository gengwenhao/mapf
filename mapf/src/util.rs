/*
 * Copyright (C) 2022 Open Source Robotics Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/

pub(crate) fn triangular_for<Item>(
    iterable: impl Iterator<Item = Item> + Clone,
    mut f: impl FnMut(&Item, Item),
) {
    let mut outer_iter = iterable.into_iter();
    while let Some(outer_value) = outer_iter.next() {
        let mut inner_iter = outer_iter.clone();
        while let Some(inner_value) = inner_iter.next() {
            f(&outer_value, inner_value);
        }
    }
}

pub(crate) struct Minimum<T: Clone, F: Fn(&T, &T) -> std::cmp::Ordering> {
    value: Option<T>,
    f: F,
}

impl<T: Clone, F: Fn(&T, &T) -> std::cmp::Ordering> Minimum<T, F> {
    pub(crate) fn new(f: F) -> Self {
        Self { value: None, f }
    }

    pub(crate) fn consider(&mut self, other: &T) -> bool {
        if let Some(value) = &self.value {
            if std::cmp::Ordering::Less == (self.f)(other, value) {
                self.value = Some(other.clone());
                return true;
            }
        } else {
            self.value = Some(other.clone());
            return true;
        }

        return false;
    }

    pub(crate) fn consider_take(&mut self, other: T) -> bool {
        if let Some(value) = &self.value {
            if std::cmp::Ordering::Less == (self.f)(&other, value) {
                self.value = Some(other);
                return true;
            }
        } else {
            self.value = Some(other);
            return true;
        }

        return false;
    }

    pub(crate) fn result(self) -> Option<T> {
        self.value
    }

    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }
}
