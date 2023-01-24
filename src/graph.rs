use crate::Zettel;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;

/// Turn a Zettelkasten into a directed graph, using petgraph
fn gen_graph(zs: &[Zettel]) -> Graph<&str, &str>
{
    let mut graph = Graph::<&str, &str>::new();
    let mut idxs = vec![];
    for z in zs {
        // basically, figure out if we've seen the current Zettel, that is, if it's already
        // added to the graph, because every entry should be added only once
        let (_, seen_idx) = find_seen_by_name(idxs.clone(), &z.title);
        let t_idx = if seen_idx == NodeIndex::default() {
            let idx = graph.add_node(&z.title);
            idxs.push((&z.title, idx));
            idx
        } else {
            seen_idx
        };

        // the same is also checked for with every link
        for l in &z.links {
            let (_, seen_idx) = find_seen_by_name(idxs.clone(), l);
            let l_idx = if seen_idx == NodeIndex::default() {
                let idx = graph.add_node(l);
                idxs.push((l, idx));
                idx
            } else {
                seen_idx
            };
            graph.add_edge(t_idx, l_idx, "");
        }
    }
    graph
}

fn find_seen_by_name(idxs: Vec<(&str, NodeIndex)>, name: &str) -> (String, NodeIndex)
{
    let (n, k) = idxs.into_iter()
                     .find(|(v, _)| v == &name)
                     .unwrap_or_default();
    (n.to_string(), k)
}

/// Turn a graph into its dot format, printing it to stdout
fn dot_output(g: Graph<&str, &str>)
{
    println!("{}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}

/// Print the dot format obtained from the graph made from the given Zettelkasten
pub fn zk_graph_dot_output(zs: &[Zettel])
{
    dot_output(gen_graph(zs));
}
