use std::collections::HashMap;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputCellID(usize);

/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: T,
}

struct ComputeCell<'a, T> {
    value: T,
    func: Box<dyn 'a + Fn(&[T]) -> T>,
    dependencies: Vec<CellID>,
    callback_id: usize,
    callbacks: HashMap<usize, Box<dyn 'a + FnMut(T)>>,
}

#[derive(Default)]
pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    input_cells: Vec<InputCell<T>>,
    compute_cells: Vec<ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: vec![],
            compute_cells: vec![],
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, value: T) -> InputCellID {
        let id = self.input_cells.len();
        let cell = InputCellID(id);
        self.input_cells.push(InputCell { value });
        cell
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let failed = dependencies.iter().find(|d| match d {
            CellID::Input(input) => self.input_cells.get(input.0).is_none(),
            CellID::Compute(compute) => self.compute_cells.get(compute.0).is_none(),
        });
        match failed {
            Some(cell_id) => Err(*cell_id),
            None => {
                let cell_id = ComputeCellID(self.compute_cells.len());
                let data = dependencies
                    .iter()
                    .map(|ci| self.value(*ci))
                    .flatten()
                    .collect::<Vec<_>>();
                let compute_cell = ComputeCell {
                    value: compute_func(&data[..]),
                    func: Box::new(compute_func),
                    dependencies: dependencies.to_vec(),
                    callback_id: 0,
                    callbacks: HashMap::new(),
                };
                self.compute_cells.push(compute_cell);
                Ok(cell_id)
            }
        }
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(InputCellID(index)) => self.input_cells.get(index).map(|c| c.value),
            CellID::Compute(ComputeCellID(index)) => self.compute_cells.get(index).map(|c| c.value),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, value: T) -> bool {
        if let Some(cell) = self.input_cells.get_mut(id.0) {
            cell.value = value;

            let mut compute_values: Vec<_> = self.compute_cells.iter().map(|c| c.value).collect();
            for (i, compute_cell) in self.compute_cells.iter_mut().enumerate() {
                let mut dep_values = Vec::new();
                for &dep in &compute_cell.dependencies {
                    dep_values.push(match dep {
                        CellID::Input(InputCellID(index)) => self.input_cells[index].value,
                        CellID::Compute(ComputeCellID(index)) => compute_values[index],
                    })
                }
                let new_value = (compute_cell.func)(&dep_values);
                if new_value != compute_values[i] {
                    compute_values[i] = new_value;
                    for callback in compute_cell.callbacks.values_mut() {
                        callback(new_value)
                    }
                }
            }
            for (i, &compute_value) in compute_values.iter().enumerate() {
                self.compute_cells[i].value = compute_value;
            }
            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: 'a + FnMut(T) -> ()>(
        &mut self,
        ComputeCellID(index): ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        match self.compute_cells.get_mut(index) {
            None => None,
            Some(cell) => {
                let id = cell.callback_id;
                cell.callback_id += 1;
                cell.callbacks.insert(id, Box::new(callback));
                Some(CallbackID(id))
            }
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        ComputeCellID(index): ComputeCellID,
        CallbackID(callback_id): CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        match self.compute_cells.get_mut(index) {
            None => Err(RemoveCallbackError::NonexistentCell),
            Some(cell) => cell
                .callbacks
                .remove(&callback_id)
                .map_or(Err(RemoveCallbackError::NonexistentCallback), |_| Ok(())),
        }
    }
}
