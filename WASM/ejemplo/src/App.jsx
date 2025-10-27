import init, { Nodo } from './pkg/saludos_wasm.js';

import { useState, useEffect, useRef } from 'react'
import * as d3 from 'd3';
import './App.css'

function App() {
    // para guardar los datos
    const svgRef = useRef();

    var colores = {
      'A' : '#eb25c0ff',
      'B' : '#2563eb',
      'C' : '#eb253fff',
      'D' : '#8feb25ff',
      'E' : '#25ebdeff',
      'F' : '#2563eb',
    }

    const graph = {
      nodes: [
        { id: 'A' },
        { id: 'B' },
        { id: 'C' },
        { id: 'D' },
        { id: 'E' },
        { id: 'F' },
      ],
      links: [
        { source: 'A', target: 'B' },
        { source: 'A', target: 'C' },
        { source: 'B', target: 'D' },
        { source: 'C', target: 'E' },
        { source: 'A', target: 'F' },
        { source: 'C', target: 'F' },
      ],
    };

    // carga el wasm
    async function loadWasm() {
        await init();
      }

    useEffect(() => {
      loadWasm();
      drawGraph();
    }, []);

    const cambiarColor = () => {
      colores['A'] = '#000000';
      const a = new Nodo('A', []);
      drawGraph();
    }

    // Función para dibujar el grafo
  const drawGraph = () => {
    const width = 800;
    const height = 600;

    const svg = d3.select(svgRef.current);
    svg.selectAll('*').remove(); // Limpia si se vuelve a renderizar

    const simulation = d3.forceSimulation(graph.nodes)
      .force('link', d3.forceLink(graph.links).id(d => d.id).distance(100))
      .force('charge', d3.forceManyBody().strength(-300))
      .force('center', d3.forceCenter(width / 2, height / 2));

    const link = svg.append('g')
      .attr('stroke', '#aaa')
      .selectAll('line')
      .data(graph.links)
      .enter().append('line')
      .attr('stroke-width', 2);

    const node = svg.append('g')
      .selectAll('circle')
      .data(graph.nodes)
      .enter().append('circle')
      .attr('r', 10)
      .attr('fill', d => colores[d.id])
      .call(d3.drag()
        .on('start', dragstarted)
        .on('drag', dragged)
        .on('end', dragended)
      );

    const label = svg.append('g')
      .selectAll('text')
      .data(graph.nodes)
      .enter().append('text')
      .text(d => d.id)
      .attr('font-size', 12)
      .attr('color', '#fefefeff')
      .attr('dx', 14)
      .attr('dy', 4);

    simulation.on('tick', () => {
      link
        .attr('x1', d => d.source.x)
        .attr('y1', d => d.source.y)
        .attr('x2', d => d.target.x)
        .attr('y2', d => d.target.y);

      node
        .attr('cx', d => d.x)
        .attr('cy', d => d.y);

      label
        .attr('x', d => d.x)
        .attr('y', d => d.y);
    });

    function dragstarted(event, d) {
      if (!event.active) simulation.alphaTarget(0.3).restart();
      d.fx = d.x;
      d.fy = d.y;
    }

    function dragged(event, d) {
      d.fx = event.x;
      d.fy = event.y;
    }

    function dragended(event, d) {
      if (!event.active) simulation.alphaTarget(0);
      d.fx = null;
      d.fy = null;
    }
  };
    
    return (
    <div>
      <h1>WebAssembly con Rust y Vite</h1>
      <div>
        <h2 style={{ marginTop: '2rem' }}>Grafo simple (D3.js)</h2>
        <svg ref={svgRef} width={800} height={600} style={{ border: '1px solid #ccc', backgroundColor: '#fefefe', borderRadius: '8px' }}></svg>
      </div>
      <button onClick={cambiarColor}>Cambio</button>
    </div>
    );
}

export default App