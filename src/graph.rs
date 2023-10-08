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
    let titles: Vec<String> = zs.iter().map(|z| z.title.clone()).collect();
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
            let is_ghost = if titles.contains(l) { "" } else { "ghost" };
            graph.add_edge(t_idx, l_idx, is_ghost);
        }
    }
    graph
}

/// Given a list of seen names and their index in the graph, return the pair that matches the name
/// of the node
fn find_seen_by_name(seen: Vec<(&str, NodeIndex)>, name: &str) -> (String, NodeIndex)
{
    let (n, i) = seen
        .into_iter()
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
        r#"<!DOCTYPE html>
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

    canvas {{
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
        <input type="range" min="0" max="5" step="0.1" value="1" class="slider" id="link_thickness">
        <input type="range" min="30" max="300" step="1" value="150" class="slider" id="link_distance">
        <input type="range" min="0" max="5" step="0.1" value="1" class="slider" id="link_force">
        <input type="range" min="0" max="10" step="0.1" value="5" class="slider" id="repulsion_force">
        <input type="range" min="0" max="1" step="0.01" value="0.05" class="slider" id="center_force">
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
    <script type="module">
        const raw_json_input = {};

        const width = window.innerWidth;
        const height = window.innerHeight;

        const nodeSizeSlider = document.getElementById("node_size");
        const linkThicknessSlider = document.getElementById("link_thickness");
        const linkDistanceSlider = document.getElementById("link_distance");
        const linkForceSlider = document.getElementById("link_force");
        const repulsionForceSlider = document.getElementById("repulsion_force");
        const centerForceSlider = document.getElementById("center_force");

        const desiredSimulationEntropy = 0.075;

        // used for detecting nodes on drag and hover
        const maxNodeDistanceFromCursor = 10;

        const repulsionForceFactor = 20; // multiplier for the repulsion force, since the default
                                         // is too weak

        let nodeSizeFactor = nodeSizeSlider.value;
        let linkThickness = linkThicknessSlider.value;
        let linkDistance = linkDistanceSlider.value;
        let attractionForceStrength = linkForceSlider.value;
        let repulsionForceStrength = repulsionForceSlider.value * repulsionForceFactor;
        let centerForceStrength = centerForceSlider.value;

        const nodeColor = "gray";
        const linkColor = "gray";
        const textColor = "white";
        const highlightColor = "purple";

        const nodeOpacity = 1;
        const textOpacity = 0.8;
        const linkOpacity = 0.6;
        const unhighlightedOpacity = 0.3;
        const ghostOpacity = 0.4;
        const unhighlightedGhostColor = "\#373737";

        const renderRegular = 0;
        const renderHighlight = 1;
        const renderUnhighlight = 2;

        let graph = {{
            nodes: raw_json_input.nodes.map((n) => {{return {{name: n, render: renderRegular, exists: true}}}}),
            links: raw_json_input.edges.map((l) => {{return {{source: l[0], target: l[1], render: renderRegular}}}})
        }};

        raw_json_input.edges.forEach((l) => {{
            if (l[2] == "ghost") {{
                graph.nodes[l[1]].exists = false;
            }}
        }});

        const canvas = d3.select("body").append("canvas")
            .attr("width", width)
            .attr("height", height);
        const context = canvas.node().getContext("2d");

        let currentTransform = d3.zoomIdentity;
        let render = (transform = d3.zoomTransform(context.canvas)) => {{
            context.clearRect(0, 0, width, height);
            context.save();

            context.translate(transform.x, transform.y);
            context.scale(transform.k, transform.k);
            currentTransform = transform;

            if (linkThickness > 0) {{
                let arrowHeadLength = linkThickness * 3;

                const drawArrowHead = (d) => {{
                    let arrowAngle = Math.atan2(d.target.y - d.source.y, d.target.x - d.source.x);
                    context.moveTo(d.target.x, d.target.y);
                    context.lineTo(d.target.x - arrowHeadLength * Math.cos(arrowAngle - Math.PI / 7),
                               d.target.y - arrowHeadLength * Math.sin(arrowAngle - Math.PI / 7));

                    context.moveTo(d.target.x - arrowHeadLength * Math.cos(arrowAngle + Math.PI / 7),
                               d.target.y - arrowHeadLength * Math.sin(arrowAngle + Math.PI / 7));

                    context.lineTo(d.target.x, d.target.y);
                    context.lineTo(d.target.x - arrowHeadLength * Math.cos(arrowAngle - Math.PI / 7),
                               d.target.y - arrowHeadLength * Math.sin(arrowAngle - Math.PI / 7));
                }}

                // highlighted links and regular links have to be handled differently, since color
                // is set on a global basis and not on a per-stroke basis, and so there would only
                // be a single color if done in one pass
                context.beginPath();
                context.lineWidth = linkThickness;
                graph.links
                    .filter((d) => d.render == renderHighlight)
                    .forEach((d) => {{
                        context.strokeStyle = highlightColor;
                        context.globalAlpha = 1;
                        context.moveTo(d.source.x, d.source.y);
                        context.lineTo(d.target.x, d.target.y);
                        drawArrowHead(d);
                }})
                context.stroke();
                graph.links
                    .filter((d) => d.render != renderHighlight)
                    .forEach((d) => {{
                        context.strokeStyle = linkColor;
                        // note that this only works because all links that are not highlighted
                        // have only one of the two possible properties
                        if (d.render == renderUnhighlight) {{
                            context.globalAlpha = unhighlightedOpacity;
                        }} else {{ // d.render == renderRegular
                            context.globalAlpha = linkOpacity;
                        }}
                        context.moveTo(d.source.x, d.source.y);
                        context.lineTo(d.target.x, d.target.y);
                        drawArrowHead(d);
                }})
                context.stroke();
            }}

            graph.nodes.forEach((d) => {{
                let currentTextColor = textColor;
                let currentTextOpacity = textOpacity;
                let currentNodeColor = nodeColor;
                let currentNodeOpacity = nodeOpacity;
                if (d.exists) {{
                    switch(d.render) {{
                        case renderHighlight:
                            currentNodeColor = highlightColor;
                            break;
                        case renderUnhighlight:
                            currentNodeOpacity = unhighlightedOpacity;
                            currentTextOpacity = unhighlightedOpacity;
                            break;
                        // the default case is handled by the parameters set before the `if`
                        // statement
                    }}
                }} else {{
                    switch(d.render) {{
                        case renderHighlight:
                            currentNodeColor = highlightColor;
                            break;
                        case renderUnhighlight:
                            currentNodeOpacity = unhighlightedOpacity;
                            currentNodeColor = unhighlightedGhostColor;
                            currentTextOpacity = unhighlightedOpacity;
                            break;
                        default:
                            currentNodeOpacity = ghostOpacity;
                    }}
                }}

                context.fillStyle = currentNodeColor;
                context.globalAlpha = currentNodeOpacity;
                context.beginPath();
                context.moveTo(d.x + 5, d.y);
                context.arc(d.x, d.y, computeNodeSize(d), 0, 2 * Math.PI);
                context.fill();

                context.fillStyle = currentTextColor;
                context.globalAlpha = currentTextOpacity;
                context.textAlign = "center";
                context.fillText(d.name, d.x, d.y + 5 + 2 * Math.PI)
            }})
            context.restore();
        }}

        let computeNodeSize = (d) => {{
            return d.size > 1 ? Math.log(d.size) * nodeSizeFactor : nodeSizeFactor
        }}

        // Count the number of links that the nodes have; used for scaling up
        // nodes
        graph.links.forEach((link) => {{
            if (!graph.nodes[link.source]["size"]) graph.nodes[link.source]["size"] = 0;
            if (!graph.nodes[link.target]["size"]) graph.nodes[link.target]["size"] = 0;
            graph.nodes[link.source]["size"]++;
            graph.nodes[link.target]["size"]++;
        }});

        const linkedByIndex = {{}};
        graph.links.forEach(d => {{
            linkedByIndex[`${{d.source}},${{d.target}}`] = 1;
        }});
        let isConnected = (a, b) => {{
            return a == b || linkedByIndex[`${{a}},${{b}}`] || linkedByIndex[`${{b}},${{a}}`];
        }}

        const simulation = d3.forceSimulation(graph.nodes);

        const updateSimulationParameters = () => {{
            // Move the nodes to the center when the simulation starts
            simulation.force("center", d3.forceCenter(width / 2, height / 2))
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
                .force("collide", d3.forceCollide().radius((d) => computeNodeSize(d) * 4))
                .on("tick", render);
            simulation.alphaTarget(desiredSimulationEntropy)
            simulation.restart();
        }}

        updateSimulationParameters();

        let nodeSizeSliderDescription = document.getElementById("node_size_description");
        let linkThicknessSliderDescription = document.getElementById("link_thickness_description");
        let linkDistanceSliderDescription = document.getElementById("link_distance_description");
        let linkForceSliderDescription = document.getElementById("link_force_description");
        let repulsionForceSliderDescription = document.getElementById("repulsion_force_description");
        let centerForceSliderDescription = document.getElementById("center_force_description");

        // display the slider values at the beginning of the simulation
        nodeSizeSliderDescription.innerHTML = "Node size: " + nodeSizeSlider.value;
        linkThicknessSliderDescription.innerHTML = "Link thickness: " + linkThicknessSlider.value;
        linkDistanceSliderDescription.innerHTML = "Link distance: " + linkDistanceSlider.value;
        linkForceSliderDescription.innerHTML = "Link force: " + linkForceSlider.value;
        repulsionForceSliderDescription.innerHTML = "Repel force: " + repulsionForceSlider.value;
        centerForceSliderDescription.innerHTML = "Center force: " + centerForceSlider.value;

        nodeSizeSlider.oninput = () => {{
            nodeSizeSliderDescription.innerHTML = "Node size: " + nodeSizeSlider.value;
            nodeSizeFactor = nodeSizeSlider.value;
            updateSimulationParameters(); // node size affects the collision force
            render();
        }}

        linkThicknessSlider.oninput = () => {{
            linkThicknessSliderDescription.innerHTML = "Link thickness: " + linkThicknessSlider.value;
            linkThickness = linkThicknessSlider.value;
            render();
        }}

        linkDistanceSlider.oninput = () => {{
            linkDistanceSliderDescription.innerHTML = "Link distance: " + linkDistanceSlider.value;
            linkDistance = linkDistanceSlider.value;
            updateSimulationParameters();
        }}

        linkForceSlider.oninput = () => {{
            linkForceSliderDescription.innerHTML = "Link force: " + linkForceSlider.value;
            attractionForceStrength = linkForceSlider.value;
            updateSimulationParameters();
        }}

        repulsionForceSlider.oninput = () => {{
            repulsionForceSliderDescription.innerHTML = "Repel force: " + repulsionForceSlider.value;
            repulsionForceStrength = repulsionForceFactor * repulsionForceSlider.value;
            updateSimulationParameters();
        }}

        centerForceSlider.oninput = () => {{
            centerForceSliderDescription.innerHTML = "Center force: " + centerForceSlider.value;
            centerForceStrength = centerForceSlider.value;
            updateSimulationParameters();
        }}

        const drag = (circles, canvas) => {{
            let dragStart = (event) => {{
                if (!event.subject.active) {{
                    simulation.alphaTarget(desiredSimulationEntropy);
                    simulation.restart();
                }};
                event.subject.active = true;
            }}

            let handleDrag = (event) => {{
                const transform = d3.zoomTransform(canvas);
                event.subject.circle.fx = transform.invertX(event.x);
                event.subject.circle.fy = transform.invertY(event.y);
            }}

            let dragEnd = (event) => {{
                if (!event.subject.active) simulation.alphaTarget(0);
                event.subject.circle.fx = null;
                event.subject.circle.fy = null;
                event.subject.active = false;
            }}

            return d3.drag()
                .subject((event) => {{
                    const transform = d3.zoomTransform(canvas);
                    const subject = getClosestNode(transform.invertX(event.x), transform.invertY(event.y));
                    return subject
                        ? {{
                            circle: subject,
                            x: transform.applyX(subject.x),
                            y: transform.applyY(subject.y)
                        }}
                        : null;
                }})
                .on("start", dragStart)
                .on("drag", handleDrag)
                .on("end", dragEnd);
        }}

        const getClosestNode = (x, y) => {{
            let subject = null;
            let distance = maxNodeDistanceFromCursor;
            for (const c of graph.nodes) {{
                let d = Math.hypot(x - c.x, y - c.y);
                if (d <= distance) {{
                    distance = d;
                    subject = c;
                }}
            }}
            return subject;
        }}

        // Highlight nodes on hover and un-highlight them on de-hover
        let lastClosest = null;
        d3.select(context.canvas).on("mousemove", (event) => {{
            let absMouseX = ((event.layerX || event.offsetX) - currentTransform.x) / currentTransform.k;
            let absMouseY = ((event.layerY || event.offsetY) - currentTransform.y) / currentTransform.k;
            let closest = getClosestNode(absMouseX, absMouseY);
            if (closest === null) {{
                graph.nodes.forEach((d) => {{d.render = renderRegular}});
                graph.links.forEach((d) => {{d.render = renderRegular}});
            }} else if (lastClosest === null || closest.index != lastClosest.index){{
                closest.render = renderHighlight;
                graph.links.forEach((d) => {{
                    if (closest.index === d.target.index || closest.index == d.source.index) {{
                        d.render = renderHighlight
                    }} else {{
                        d.render = renderUnhighlight;
                    }}
                }})
                graph.nodes.forEach((d) => {{
                    if (!isConnected(closest.index, d.index)) {{
                        d.render = renderUnhighlight;
                    }}
                }})
            }}
            lastClosest = closest;
            render(event.transform);
        }})

        d3.select(context.canvas).call(drag(graph.nodes, context.canvas)
            .on("start.render drag.render end.render", (event) => render(event.transform)));

        d3.select(context.canvas).call(d3.zoom()
            .on("zoom", (event) => render(event.transform)))
    </script>
</body>
</html>"#,
        jsongraph
    );
}
