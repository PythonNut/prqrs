use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Instant;

#[pyclass]
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Item {
    #[pyo3(get, set)]
    value: i64,
    #[pyo3(get, set)]
    priority: i64,
}

#[pymethods]
impl Item {
    #[new]
    fn new(value: i64, priority: i64) -> Self {
        Item { value, priority }
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

    fn push(&mut self, item: Item) {
        self.heap.push(Reverse(item));
    }

    fn pop(&mut self) -> Option<Item> {
        self.heap.pop().map(|Reverse(item)| item)
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

#[pyfunction]
fn benchmark_enqueue(pq: &mut PriorityQueue, items: Vec<Item>) -> String {
    let start = Instant::now();
    for item in items {
        pq.push(item);
    }
    let duration = start.elapsed();
    format!("Enqueue time: {:?}", duration)
}

#[pyfunction]
fn benchmark_dequeue(pq: &mut PriorityQueue) -> String {
    let start = Instant::now();
    let mut count = 0;
    while pq.pop().is_some() {
        count += 1;
    }
    let duration = start.elapsed();
    format!("Dequeue time: {:?}, Count: {}", duration, count)
}

#[pymodule]
fn prqrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Item>()?;
    m.add_class::<PriorityQueue>()?;
    m.add_function(wrap_pyfunction!(benchmark_enqueue, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_dequeue, m)?)?;
    Ok(())
}
