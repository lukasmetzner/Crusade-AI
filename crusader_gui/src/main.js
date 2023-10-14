const { invoke } = window.__TAURI__.tauri;

let graphInterval;
let graph;

const colors = [
  'rgba(255, 99, 132, 1)',
  'rgba(54, 162, 235, 1)',
  'rgba(255, 206, 86, 1)',
  'rgba(75, 192, 192, 1)',
  'rgba(153, 102, 255, 1)',
  'rgba(255, 159, 64, 1)',
  'rgba(255, 99, 64, 1)',
  'rgba(54, 99, 235, 1)',
  'rgba(255, 206, 132, 1)',
  'rgba(75, 192, 75, 1)',
  'rgba(153, 50, 255, 1)'
];


async function getResources() {
  return await invoke("get_resources");
}

function buildGraph() {
  const ctx = document.getElementById('myChart');

  getResources().then(resourcesData => {
    const labels = Object.keys(resourcesData);
    const datasets = labels
      .filter(label => label !== "tick")
      .map((label, index) => {
        return {
            label: label,
            data: [resourcesData[label]],
            borderColor: colors[index],
            fill: false
        };
      });

    const chartData = {
        labels: [0],
        datasets: datasets
    };
  
    graph = new Chart(ctx, {
      type: 'line',
      data: chartData,
    });
  });
}

function updateGraph() {
  getResources().then(resourcesData => {
    graph.data.labels.push(resourcesData.tick);
    graph.data.datasets.forEach((dataset, index) => {
      if (Object.keys(resourcesData)[index] !== "tick") {
          dataset.data.push(Object.values(resourcesData)[index]);
      }
  });
    graph.update();
  });
}

window.addEventListener("DOMContentLoaded", () => {
  buildGraph();
  graphInterval = setInterval(updateGraph, 2000);
});
