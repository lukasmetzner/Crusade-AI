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

function transformData(resourcesData) {
  // Extracting x-axis labels using the 'tick' values
  const labels = resourcesData.map(obj => obj.tick).reverse();

  // Remove 'tick' from the keys
  const keys = Object.keys(resourcesData[0]).filter(key => key !== 'tick');

  const datasets = keys.map((key, index) => ({
    label: key,
    data: resourcesData.map(obj => obj[key]).reverse(),
    fill: false,
    borderColor: colors[index],
    tension: 0.1
  }));
  return [labels, datasets];
}

function buildGraph() {
  let ctx = document.getElementById('myChart');

  getResources().then(resourcesData => {
    const [labels, datasets] = transformData(resourcesData);

    graph = new Chart(ctx, {
      type: 'line',
      data: {
        labels: labels,
        datasets: datasets
      },
      options: {
        animation: {
          duration: 0 // Disables the initial animation
        },
        hover: {
          animationDuration: 0 // Disables the hover pop-up animation
        },
        scales: {
          x: {
            beginAtZero: true
          },
          y: {
            type: 'logarithmic',
            beginAtZero: true,
            ticks: {
              callback: function (value, index, values) {
                if (value === 10 || value === 100 || value === 1000 || value === 10000) {
                  return value;
                }
              }
            }
          }
        }
      }
    });
  })
    .catch((err) => {
      console.log("Could not build graph!");
      console.log(err);
      return;
    });
}

function updateGraph() {
  getResources().then(resourcesData => {
    const [keys, datasets] = transformData(resourcesData);
    graph.data.labels = keys;
    graph.data.datasets = datasets;
    graph.update();
  });
}

window.addEventListener("DOMContentLoaded", () => {
  buildGraph();
  graphInterval = setInterval(updateGraph, 2000);
});
