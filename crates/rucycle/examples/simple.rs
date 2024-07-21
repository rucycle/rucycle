use rucycle::task::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let task1 = Task {
        id: "task1".to_string(),
        command: "echo Hello".to_string(),
        dependencies: vec![],
    };

    let task2 = Task {
        id: "task2".to_string(),
        command: "echo World".to_string(),
        dependencies: vec!["task1".to_string(), "task3".to_string()],
    };

    let task3 = Task {
        id: "task3".to_string(),
        command: "echo there".to_string(),
        dependencies: vec![],
    };

    let mut graph = TaskGraph::default();

    graph.insert(&task1);
    graph.insert(&task2);
    graph.insert(&task3);

    graph.run().await;
}
