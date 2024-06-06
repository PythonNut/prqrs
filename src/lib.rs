use numpy::PyArrayDyn;
use ordered_float::NotNan;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::BinaryHeap;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct NotNanF64(NotNan<f64>);
impl FromPyObject<'_> for NotNanF64 {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let f: f64 = ob.extract()?;
        return Ok(NotNanF64 {
            0: NotNan::new(f).unwrap(),
        });
    }
}
impl IntoPy<PyObject> for NotNanF64 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0.into_inner().into_py(py)
    }
}

#[pyclass]
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Item {
    #[pyo3(get, set)]
    priority: NotNanF64,
    #[pyo3(get, set)]
    value: i64,
}

#[pymethods]
impl Item {
    #[new]
    fn new(p: f64, value: i64) -> Self {
        Item {
            priority: NotNanF64 {
                0: NotNan::new(p).unwrap(),
            },
            value,
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct PriorityQueue {
    heap: BinaryHeap<Item>,
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
    fn from_numpy(array: &PyArrayDyn<f64>) -> PyResult<Self> {
        let array = unsafe { array.as_array() };
        let vec: Vec<Item> = array
            .iter()
            .enumerate()
            .map(|(i, &p)| Item {
                priority: NotNanF64 {
                    0: NotNan::new(p).unwrap(),
                },
                value: i as i64,
            })
            .collect();
        let heap = BinaryHeap::from(vec);
        Ok(PriorityQueue { heap })
    }

    fn push(&mut self, item: Item) {
        self.heap.push(item);
    }

    fn push_batch(&mut self, items: &PyArrayDyn<i64>, priorities: &PyArrayDyn<f64>) {
        let items = unsafe { items.as_array() };
        let priorities = unsafe { priorities.as_array() };
        items.iter().zip(priorities.iter()).for_each(|(&i, &p)| self.push(Item { priority: NotNanF64 {0: NotNan::new(p).unwrap()}, value: i as i64 }));
    }

    fn pop(&mut self) -> Option<Item> {
        self.heap.pop()
    }

    fn peek(&mut self) -> Option<Item> {
        self.heap.peek().map(|item| item.clone())
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
