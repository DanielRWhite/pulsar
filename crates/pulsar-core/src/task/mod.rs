pub struct Chunk { }

pub struct SharedTaskState<T>(T);

pub struct Task {
        shared_state: SharedTaskState<dyn std::any::Any>,
        builder: dyn FnOnce(dyn std::any::Any) -> Vec<Chunk>,
        chunker: dyn FnOnce(dyn std::any::Any) -> Box<dyn std::any::Any>,
        gatherer: Option<dyn FnOnce(dyn std::any::Any) -> Box<dyn std::any::Any>>
}