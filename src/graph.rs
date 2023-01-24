use crate::Zettel;
use petgraph::dot::{Config, Dot};
use petgraph::Graph;

/// Turn a Zettelkasten into a directed graph, using petgraph
fn gen_graph(zs: &[Zettel]) -> Graph<&str, &str>
{
    let mut graph = Graph::<&str, &str>::new();
    //let mut links: Vec<(&str, &str)> = vec![];
    for z in zs {
        let t_node = graph.add_node(&z.title);
        for l in &z.links {
            let l_node = graph.add_node(&l);
            graph.extend_with_edges(&[(t_node, l_node)]);
        }
    }
    graph
}

/// Turn a graph into its dot format, printing it to stdout
fn dot_output(g: Graph<&str, &str>)
{
    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}

/// Print the dot format obtained from the graph made from the given Zettelkasten
pub fn zk_graph_dot_output(zs: &[Zettel])
{
    dot_output(gen_graph(zs))
}
