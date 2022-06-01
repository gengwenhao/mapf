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

use std::sync::Arc;
use super::node;

pub trait Goal<N: node::Node> {
    fn is_satisfied(&self, node: &N) -> bool;
}

pub trait Solution<C: node::Cost>: Clone {
    fn cost(&self) -> C;
}

pub trait Expander {

    /// The type of Node supported by this Expander
    type Node: node::Node;

    /// The type of Start conditions supported by this Expander
    type Start;

    /// The type of Goal conditions supported by this Expander
    type Goal: Goal<Self::Node>;

    /// The representation of solutions that can be produced by this Expander
    type Solution: Solution<<Self::Node as node::Node>::Cost>;

    /// An initial set of nodes, produced from a Start object
    type InitialNodes<'a>: Iterator<Item=Result<Arc<Self::Node>, Self::Error>> where Self: 'a;

    /// An expansion that can be generated by this Expander
    type Expansion<'a>: Iterator<Item=Result<Arc<Self::Node>, Self::Error>> where Self: 'a;

    /// The type of error that the expander can produce
    type Error: std::fmt::Debug + Clone;

    /// Generate an initial set of nodes based on the given start conditions
    fn start<'a>(
        &'a self,
        start: &'a Self::Start,
        goal: Option<&'a Self::Goal>,
    ) -> Self::InitialNodes<'a>;

    /// Expand the given node
    fn expand<'a>(
        &'a self,
        parent: &Arc<Self::Node>,
        goal: Option<&'a Self::Goal>,
    ) -> Self::Expansion<'a>;

    /// Make a Solution for the given solution node
    fn make_solution(&self, solution_node: &Arc<Self::Node>) -> Result<Self::Solution, Self::Error>;
}

/// The Reversible trait can be implemented by Expanders that support expanding
/// in reverse from a goal. Bidirectional algorithms can take advantage of this
/// trait.
pub trait Reversible: Expander where Self::Node: node::Reversible {
    type Reverse: Expander<Node=<Self::Node as node::Reversible>::Reverse, Start=Self::Goal>;

    /// Create a reverse expander for the algorithm to use.
    fn reverse(&self) -> Arc<Self::Reverse>;

    /// Make a solution from a (Forward, Reverse) expansion node pair.
    fn make_bidirectional_solution(
        &self,
        forward_solution_node: &Arc<Self::Node>,
        reverse_solution_node: &Arc<<Self::Reverse as Expander>::Node>
    ) -> Result<Self::Solution, Self::Error>;
}

pub type NodeOf<E> = <E as Expander>::Node;
pub type CostOf<E> = <NodeOf<E> as node::Node>::Cost;
pub type ReverseOf<E> = <E as Reversible>::Reverse;
pub type SolutionOf<E> = <E as Expander>::Solution;
