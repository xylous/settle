use crate::Zettel;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;

/// Print the dot format obtained from the graph made from the given Zettelkasten
pub fn zk_graph_dot_output(zs: &[Zettel])
{
    dot_output(gen_graph(zs));
}

/// Print the JSON format obtained from the graph made from the given Zettelkasten
pub fn zk_graph_json_output(zs: &[Zettel])
{
    println!("{}", json_output(gen_graph(zs)));
}

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

/// Given a list of seen names and their index in the graph, return the pair that matches the name
/// of the node
fn find_seen_by_name(seen: Vec<(&str, NodeIndex)>, name: &str) -> (String, NodeIndex)
{
    let (n, i) = seen.into_iter()
                     .find(|(v, _)| v == &name)
                     .unwrap_or_default();
    (n.to_string(), i)
}

/// Turn a graph into its dot format, printing it to stdout
fn dot_output(g: Graph<&str, &str>)
{
    println!("{}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}

fn json_output(g: Graph<&str, &str>) -> String
{
    serde_json::to_string(&g).unwrap()
}

pub fn vizk(zs: &[Zettel])
{
    let jsongraph = json_output(gen_graph(zs));
    println!(
             r#"
             <!DOCTYPE html>
<html>

<head>
    <meta charset="utf-8">
    <title>vizk</title>
</head>

<style>
    html,
    body {{
        color: #fff;
        background-color: #121212;
        margin: 0;
        padding: 0;
        overflow: hidden;
    }}

    svg {{
        position: relative;
        z-index: 1;
        top: 0;
        left: 0;
        height: 100%;
        width: 100%;
    }}

    .top-div {{
        z-index: 2;
        position: absolute;
    }}

    .slider-description {{
        position: sticky;
    }}

    .no-select {{
        -webkit-touch-callout: none;
        -webkit-user-select: none;
        -khtml-user-select: none;
        -moz-user-select: none;
        -ms-user-select: none;
        user-select: none;
    }}
}}

</style>

<body>
    <!--The sliders go on top of the simulation nicely-->
    <div class="top-div" id="slider_container">
        <input type="range" min="0" max="5" step="0.1" value="2" class="slider" id="node_size">
        <input type="range" min="0" max="5" step="0.1" value="2" class="slider" id="link_thickness">
        <input type="range" min="30" max="100" step="1" value="30" class="slider" id="link_distance">
        <input type="range" min="0" max="5" step="0.1" value="2" class="slider" id="link_force">
        <input type="range" min="0" max="5" step="0.1" value="2" class="slider" id="repulsion_force">
        <input type="range" min="0" max="1" step="0.01" value="0.5" class="slider" id="center_force">
    </div>
    <div style="position: absolute; top: 15px; left: 15px">
        <p class="no-select" class="slider-description" id="node_size_description"></p>
        <p class="no-select" class="slider-description" id="link_thickness_description"></p>
        <p class="no-select" class="slider-description" id="link_distance_description"></p>
        <p class="no-select" class="slider-description" id="link_force_description"></p>
        <p class="no-select" class="slider-description" id="repulsion_force_description"></p>
        <p class="no-select" class="slider-description" id="center_force_description"></p>
    </div>
    <script src="https://cdn.jsdelivr.net/npm/d3@7"></script>
    <script src="https://unpkg.com/d3-simple-slider"></script>
    <script type="module">
        const width = window.innerWidth;
        const height = window.innerHeight;

        const raw_json_input = {};
        const raw_nodes = raw_json_input.nodes;
        const raw_links = raw_json_input.edges;

        let graph = {{
            nodes: raw_nodes.map((n) => {{return {{name: n}}}}),
            links: raw_links.map((l) => {{return {{source: l[0], target: l[1]}}}})
        }};

        const zoomTransformExtentFactor = 1;
        const zoomScaleExtentFactor = 1;

        const nodeSizeSlider = document.getElementById("node_size");
        const linkThicknessSlider = document.getElementById("link_thickness");
        const linkDistanceSlider = document.getElementById("link_distance");
        const linkForceSlider = document.getElementById("link_force");
        const repulsionForceSlider = document.getElementById("repulsion_force");
        const centerForceSlider = document.getElementById("center_force");

        // not completely immobile but also not too spasmodic
        const averageEntropyTarget = 0.1;
        // The normal repulsion force is usually too weak
        const repulsionForceFactor = 20;

        let nodeSizeFactor = nodeSizeSlider.value; // 0 to 5
        let linkThickness = linkThicknessSlider.value; // 0 to 5
        let linkDistance = linkDistanceSlider.value; // 30 to 100
        let attractionForceStrength = linkForceSlider.value; // 0 to 5
        let repulsionForceStrength = repulsionForceSlider.value * repulsionForceFactor; // 0 to 5
        let centerForceStrength = centerForceSlider.value; // 0 to 1

        let linkOpacity = 1;
        let linkColor = "black";

        let nodeBorderColor = "white";
        let nodeBorderSize = 0.5;
        let highlightSourceColor = "purple";
        let highlightUnrelated = "gray";
        let highlightRegularNode = "yellow";

        const svg = d3.select("body").append("svg")
            .attr("width", width)
            .attr("height", height);

        const g = svg.append("g")

        // On every tick, update the positions of the nodes and links
        let tick = () => {{
            nodeSelection
                .attr("transform", (d) => `translate(${{d.x}} ${{d.y}})`)
            textSelection
                .attr("transform", (d) => `translate(${{d.x}} ${{d.y}})`)
            linkSelection
                .attr("x1", d => d.source.x)
                .attr("y1", d => d.source.y)
                .attr("x2", d => d.target.x)
                .attr("y2", d => d.target.y)
        }}

        let dragStart = (event) => {{
            if (!event.active) simulation.alphaTarget(0.3).restart();
            event.subject.fx = event.x;
            event.subject.fy = event.y;
        }}

        let handleDrag = (event) => {{
            event.subject.fx = event.x;
            event.subject.fy = event.y;
        }}

        let dragEnd = (event) => {{
            if (!event.active) simulation.alphaTarget(0);
            event.subject.fx = null;
            event.subject.fy = null;
        }}

        let handleZoom = (event) => {{
            g.attr("transform", event.transform)
        }}

        let computeNodeSize = (d) => {{
            return d.size ? d.size * nodeSizeFactor : nodeSizeFactor
        }}

        // Count the number of links that the nodes have; used for scaling up
        // nodes
        graph.links.forEach((link) => {{
            if (!graph.nodes[link.source]["size"]) graph.nodes[link.source]["size"] = 0;
            if (!graph.nodes[link.target]["size"]) graph.nodes[link.target]["size"] = 0;
            graph.nodes[link.source]["size"]++;
            graph.nodes[link.target]["size"]++;
        }});

        const nodeSelection = g
            .selectAll()
            .data(graph.nodes)
            .join("circle")
            .attr("stroke", nodeBorderColor)
            .attr("stroke-width", nodeBorderSize)
            .attr("id", (d, id) => id)
            .attr("r", (d) => computeNodeSize(d))
            .style("fill", (d) => highlightRegularNode)

        // the text layer is separate to not allow dragging a node by its name
        // or to complicate highlighting code
        const textSelection = g
            .selectAll()
            .data(graph.nodes)
            .join("text")
            .text((d) => d.name)
            .attr("x", (d) => d.x)
            .attr("y", (d) => d.y)
            .attr("text-anchor", "middle")
            .attr("dy", (d) => d.size * 2 + 18)
            .style("fill", "white")
            .classed("no-select", true)

        const linkSelection = g
            .selectAll()
            .data(graph.links)
            .join("line")
            .attr("stroke-width", linkThickness)
            .attr("stroke-opacity", linkOpacity)
            .attr("stroke", linkColor)
            .lower()

        nodeSelection.call(d3.drag()
            .on("start", dragStart)
            .on("drag", handleDrag)
            .on("end", dragEnd));

        const linkedByIndex = {{}};
        graph.links.forEach(d => {{
            linkedByIndex[`${{d.source}},${{d.target}}`] = 1;
        }});

        let isConnected = (a, b) => {{
            return linkedByIndex[`${{a}},${{b}}`] || linkedByIndex[`${{b}},${{a}}`] || a === b;
        }}

        nodeSelection.on('mouseover', function (d) {{
            let current = d3.select(this)
            let c_id = current.attr("id")
            nodeSelection
                .filter(n => !isConnected(c_id, n.index))
                .style("fill", highlightUnrelated)
            current.style("fill", highlightSourceColor)
            linkSelection
                .style("stroke", highlightSourceColor)
            linkSelection
                .filter(l => !(c_id == l.source.index || c_id == l.target.index))
                .style("stroke", linkColor)
                .style("stroke-width", linkThickness / 2)
                .style("stroke-opacity", linkOpacity / 2)
        }})
        nodeSelection.on('mouseout', (d) => {{
            nodeSelection.style("fill", highlightRegularNode)
            linkSelection
                .style("stroke", linkColor)
                .style("stroke-width", linkThickness)
                .style("stroke-opacity", linkOpacity)
        }})

        const simulation = d3.forceSimulation(graph.nodes)
            // Move the nodes to the center when the simulation starts
            .force("center", d3.forceCenter(width / 2, height / 2))
            // Attract nodes to center
            .force("centerX", d3.forceX(width / 2).strength(centerForceStrength))
            .force("centerY", d3.forceY(height / 2).strength(centerForceStrength))
            // Attract linked nodes
            .force("links", d3.forceLink(graph.links)
                .strength(attractionForceStrength)
                .distance(linkDistance))
            // Repulse all nodes from each other by some force
            .force("repulsion",
            d3.forceManyBody().strength(-repulsionForceStrength))
            // Repulse nodes if they collide
            .force("collide", d3.forceCollide().radius((d) => d.size * nodeSizeFactor + 2))
            .on("tick", tick);

        svg.call(d3.zoom()
            .translateExtent([[zoomTransformExtentFactor * (-width), zoomTransformExtentFactor * (-height)],
                            [2 * zoomTransformExtentFactor * width, 2 * zoomTransformExtentFactor * height]])
            .scaleExtent([zoomScaleExtentFactor * 0.5, zoomScaleExtentFactor * 10])
            .on("zoom", handleZoom)
            .filter(() => {{
                return (event.button == 0 || event.button == 1)
            }})
        );

        let nodeSizeSliderDescription = document.getElementById("node_size_description");
        nodeSizeSliderDescription.innerHTML = "Node size: " + nodeSizeSlider.value;
        nodeSizeSlider.oninput = () => {{
            nodeSizeSliderDescription.innerHTML = "Node size: " + nodeSizeSlider.value;
            nodeSizeFactor = nodeSizeSlider.value;
            nodeSelection.attr("r", (d) => computeNodeSize(d));
        }}

        let linkThicknessSliderDescription = document.getElementById("link_thickness_description");
        linkThicknessSliderDescription.innerHTML = "Link thickness: " + linkThicknessSlider.value;
        linkThicknessSlider.oninput = () => {{
            linkThicknessSliderDescription.innerHTML = "Link thickness: " + linkThicknessSlider.value;
            linkThickness = linkThicknessSlider.value;
            linkSelection.style("stroke-width", linkThickness);
        }}

        let linkDistanceSliderDescription = document.getElementById("link_distance_description");
        linkDistanceSliderDescription.innerHTML = "Link distance: " + linkDistanceSlider.value;
        linkDistanceSlider.oninput = () => {{
            linkDistanceSliderDescription.innerHTML = "Link distance: " + linkDistanceSlider.value;
            linkDistance = linkDistanceSlider.value;
            simulation.force("links", d3.forceLink(graph.links)
                .strength(attractionForceStrength)
                .distance(linkDistance / (attractionForceStrength > 0 ? attractionForceStrength : 1)));
            simulation.alphaTarget(averageEntropyTarget)
        }}

        let linkForceSliderDescription = document.getElementById("link_force_description");
        linkForceSliderDescription.innerHTML = "Link force: " + linkForceSlider.value;
        linkForceSlider.oninput = () => {{
            linkForceSliderDescription.innerHTML = "Link force: " + linkForceSlider.value;
            attractionForceStrength = linkForceSlider.value;
            simulation.force("links", d3.forceLink(graph.links)
                .strength(attractionForceStrength)
                .distance(linkDistance / (attractionForceStrength > 0 ? attractionForceStrength : 1)));
            simulation.alphaTarget(averageEntropyTarget)
        }}

        let repulsionForceSliderDescription = document.getElementById("repulsion_force_description");
        repulsionForceSliderDescription.innerHTML = "Repel force: " + repulsionForceSlider.value;
        repulsionForceSlider.oninput = () => {{
            repulsionForceSliderDescription.innerHTML = "Repel force: " + repulsionForceSlider.value;
            repulsionForceStrength = repulsionForceFactor * repulsionForceSlider.value;
            simulation.force("charge", d3.forceManyBody().strength(-repulsionForceStrength));
            simulation.alphaTarget(averageEntropyTarget)
        }}

        let centerForceSliderDescription = document.getElementById("center_force_description");
        centerForceSliderDescription.innerHTML = "Center force: " + centerForceSlider.value;
        centerForceSlider.oninput = () => {{
            centerForceSliderDescription.innerHTML = "Center force: " + centerForceSlider.value;
            centerForceStrength = centerForceSlider.value;
            simulation.force("centerX", d3.forceX(width / 2).strength(centerForceStrength));
            simulation.force("centerY", d3.forceY(height / 2).strength(centerForceStrength));
            simulation.alphaTarget(averageEntropyTarget)
        }}
    </script>
</body>
</html>"#,
             jsongraph
    );
}
