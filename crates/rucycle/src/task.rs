use std::collections::{HashMap, HashSet};

use log::{info, warn};
use petgraph::graphmap::DiGraphMap;
use tokio::process::Command;

pub struct Task {
    pub id: String,
    pub command: String,
    pub dependencies: Vec<String>,
}

pub struct TaskGraph<'a> {
    graph: DiGraphMap<&'a str, ()>,
    tasks: HashMap<&'a str, &'a Task>,
}

impl<'a> Default for TaskGraph<'a> {
    fn default() -> Self {
        TaskGraph {
            graph: DiGraphMap::new(),
            tasks: HashMap::new(),
        }
    }
}

impl<'a> TaskGraph<'a> {
    pub fn insert(&mut self, task: &'a Task) {
        self.tasks.insert(&task.id, task);
        self.graph.add_node(&task.id);

        for dep in &task.dependencies {
            self.graph.add_edge(dep.as_str(), &task.id, ());
        }
    }

    fn schedule_tasks(&self) -> Vec<&str> {
        let mut schedule = vec![];
        let mut visited = HashSet::new();

        for node in self.graph.nodes() {
            self.visit_node(node, &mut visited, &mut schedule);
        }

        schedule.reverse();
        schedule
    }

    fn visit_node<'b>(
        &'b self,
        node: &'b str,
        visited: &mut HashSet<&'b str>,
        schedule: &mut Vec<&'b str>,
    ) {
        if !visited.contains(node) {
            visited.insert(node);
            for neighbor in self.graph.neighbors(node) {
                self.visit_node(neighbor, visited, schedule);
            }
            schedule.push(node);
        }
    }

    async fn execute_task(task: &Task) -> Result<(), Box<dyn std::error::Error>> {
        info!("Executing task {}", task.id);
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(&task.command);
        let status = cmd.status().await?;

        if status.success() {
            info!("Task {} completed successfully", task.id);
            Ok(())
        } else {
            warn!("Task {} failed", task.id);
            Err(Box::from("Task failed"))
        }
    }

    pub async fn run(&self) {
        let schedule = self.schedule_tasks();

        for task_id in schedule {
            let task = self.tasks.get(task_id).unwrap();
            TaskGraph::execute_task(task).await.unwrap();
        }
    }
}
