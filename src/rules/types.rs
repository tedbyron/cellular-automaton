//! Types of rules for cellular automata.

/// A list containing values that represent the number of state-1 neighbors that
/// must exist for a state-0 (dead) cell to be born.
///
/// # Examples
///
/// ```
/// let b: BirthRule = vec![3];
/// assert!(b.iter().all(|n| (1..=8).contains(n)));
///
/// ```
pub type BirthRule = Vec<i8>;

/// A list containing values that represent the number of state-1 neighbors that
/// must exist for a state-1 cell to remain in state-1.
///
/// # Examples
///
/// ```
/// let s: SurvivalRule = vec![2, 3];
/// assert!(s.iter().all(|n| (-1..=8).contains(n)));
/// ```
pub type SurvivalRule = Vec<i8>;

/// The number of possible cell states.
///
/// # Examples
///
/// ```
/// let s: GenerationRule = 2;
/// assert!(s >= 2);
/// ```
pub type GenerationRule = i8;
