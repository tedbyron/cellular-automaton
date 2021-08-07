//! Types of rules for cellular automata.

/// A list containing values that represent the number of live, first-generation
/// neighbors that must exist for a dead cell to come to life.
///
/// # Examples
///
/// ```
/// let b: BirthRule = vec![3];
/// assert!(b.iter().all(|n| (1..=8).contains(n)));
///
/// ```
pub type BirthRule = Vec<i8>;

/// A list containing values that represent the number of live, first-generation
/// neighbors that must exist for a live, first-generation cell to remain alive.
///
/// # Examples
///
/// ```
/// let s: SurvivalRule = vec![2, 3];
/// assert!(s.iter().all(|n| (-1..=8).contains(n)));
/// ```
pub type SurvivalRule = Vec<i8>;

/// The maximum value that a cell can have.
///
/// # Examples
///
/// ```
/// let s: GenerationRule = 2;
/// assert!(s >= 2);
/// ```
pub type GenerationRule = i8;
