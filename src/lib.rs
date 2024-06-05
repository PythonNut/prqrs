use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Instant;
use numpy::PyArrayDyn;

#[pyclass]
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Item {
    #[pyo3(get, set)]
    priority: i64,
    #[pyo3(get, set)]
    value: i64,
}

#[pymethods]
impl Item {
    #[new]
    fn new(priority: i64, value: i64) -> Self {
        Item { priority, value }
    }
}

#[pyclass]
#[derive(Clone)]
struct PriorityQueue {
    heap: BinaryHeap<Reverse<Item>>,
}

#[pymethods]
impl PriorityQueue {
    #[new]
    fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    #[staticmethod]
    fn from_numpy(array: &PyArrayDyn<i64>) -> PyResult<Self> {
        let array = unsafe { array.as_array() };
        let vec: Vec<Reverse<Item>> = array
            .iter()
            .enumerate()
            .map(|(i, &priority)| Reverse(Item { priority, value: i as i64 }))
            .collect();
        let heap = BinaryHeap::from(vec);
        Ok(PriorityQueue { heap })
    }

    fn push(&mut self, item: Item) {
        self.heap.push(Reverse(item));
    }

    fn pop(&mut self) -> Option<Item> {
        self.heap.pop().map(|Reverse(item)| item)
    }

    fn __len__(&self) -> usize {
        self.heap.len()
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

#[pymodule]
fn prqrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Item>()?;
    m.add_class::<PriorityQueue>()?;
    // m.add_function(wrap_pyfunction!(benchmark_enqueue, m)?)?;
    // m.add_function(wrap_pyfunction!(benchmark_dequeue, m)?)?;
    Ok(())
}
