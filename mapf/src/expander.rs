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

use std::rc::Rc;
use super::node;

pub trait Goal<Node: node::Node> {
    fn is_satisfied(&self, node: &Node) -> bool;
}

pub trait Expander {

    /// The type of Node supported by this Expander
    type Node: node::Node;

    /// The type of Start conditions supported by this Expander
    type Start;

    /// The type of Goal conditions supported by this Expander
    type Goal: Goal<Self::Node>;

    /// Options for how this Expander will behave. This may include things like
    /// situational constraints.
    type Options: Clone;

    /// The representation of solutions that can be produced by this Expander
    type Solution;

    /// An expansion that can be generated by this Expander
    type Expansion: Iterator<Item=Rc<Self::Node>>;

    /// The default (recommended) options for this Expander
    fn default_options(&self) -> Self::Options;

    /// Generate an initial set of nodes based on the given start conditions
    fn start(
        &self,
        start: &Self::Start,
        goal: &Self::Goal
    // ) -> Self::Expansion;
    ) -> Self::Expansion;

    /// Expand the given node
    fn expand(
        &self,
        parent: &Rc<Self::Node>,
        goal: &Self::Goal,
        options: &Self::Options
    ) -> Self::Expansion;

    /// Make a Solution for the given solution node
    fn make_solution(&self, solution_node: &Rc<Self::Node>, options: &Self::Options) -> Self::Solution;
}

/// The Reversible trait can be implemented by Expanders that support expanding
/// in reverse from a goal. Bidirectional algorithms can take advantage of this.
pub trait Reversible<Reverse: Expander>: Expander {

    /// Create a reverse expander for the algorithm to use.
    /// Note: Reverse::Start must be equivalent to the Forward Expander's Goal.
    fn reverse(&self) -> Rc<Reverse>;

    /// Make a solution from a (Forward, Reverse) expansion node pair.
    fn make_bidirectional_solution(
        &self,
        forward_solution_node: &Rc<Self::Node>,
        reverse_solution_node: &Rc<Reverse::Node>
    ) -> Self::Solution;
}

pub type Cost<E> = <<E as Expander>::Node as node::Node>::Cost;
