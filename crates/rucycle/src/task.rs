use petgraph::graphmap::DiGraphMap;

pub struct Task {
    pub id: String,
    pub command: String,
    pub dependencies: Vec<String>,
}

pub fn build_task_graph(tasks: &[Task]) -> DiGraphMap<&str, ()> {
    let mut graph = DiGraphMap::new();

    for task in tasks {
        let id = task.id.as_str();
        graph.add_node(id);

        for dep in &task.dependencies {
            graph.add_edge(dep.as_str(), id, ());
        }
    }

    graph
}
