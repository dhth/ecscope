const colorMap = new Map();
const colors = [
	"#fabd2f",
	"#b8bb26",
	"#83a598",
	"#d3869b",
	"#8ec07c",
	"#fe8019",
	"#fb4934",
	"#d79921",
];

const API_URL = "/api/deps";
const DEFAULT_REFRESH_INTERVAL = 5;
const MIN_REFRESH_INTERVAL = 5;
const MAX_REFRESH_INTERVAL = 300;
let refreshInterval = null;

function sanitizeHTML(str) {
	const tempDiv = document.createElement("div");
	tempDiv.textContent = str;
	return tempDiv.innerHTML;
}

function getColorForKey(key) {
	if (!colorMap.has(key)) {
		const color = colors[colorMap.size % colors.length];
		colorMap.set(key, color);
	}
	return colorMap.get(key);
}

function showErrorMessage(elementId, message) {
	const element = document.getElementById(elementId);
	element.innerHTML = `<p class="text-red-500">${sanitizeHTML(message)}</p>`;
}

async function fetchData() {
	const pageTitle = document.getElementById("page-title");
	pageTitle.classList.add("loading");

	try {
		const response = await fetch(API_URL);
		if (!response.ok) {
			throw new Error(
				`Failed to fetch data: ${response.statusText} (HTTP ${response.status})`,
			);
		}
		const data = await response.json();
		renderDeployments(data.deployments);
		renderErrors(data.errors);
	} catch (error) {
		console.error("Error fetching data:", error);
		showErrorMessage("deployments", `Error: ${error.message}`);
	} finally {
		pageTitle.classList.remove("loading");
	}
}

function renderDeployments(deployments) {
	const deploymentsDiv = document.getElementById("deployments");
	if (deployments.length === 0) {
		deploymentsDiv.innerHTML = `<p class="text-yellow-500">No deployments found.</p>`;
		return;
	}

	deployments.sort((a, b) => {
		if (a.service_name === b.service_name) {
			return a.keys.localeCompare(b.keys);
		}
		return a.service_name.localeCompare(b.service_name);
	});

	const table = document.createElement("table");
	table.className = "table-auto w-full mb-6";
	table.innerHTML = `
    <thead>
      <tr>
        <th>Service Name</th>
        <th>Keys</th>
        <th>Status</th>
        <th>Running</th>
        <th>Desired</th>
        <th>Pending</th>
        <th>Failed</th>
      </tr>
    </thead>
    <tbody>
      ${deployments
				.map((deployment) => {
					let rowClass = "";
					let serviceColor = "#282828";
					let keysColor = "#282828";
					if (deployment.failed_count > 0) {
						rowClass = "row-error";
					} else if (deployment.status === "ACTIVE") {
						rowClass = "row-active";
					} else if (deployment.running_count !== deployment.desired_count) {
						rowClass = "row-mismatch";
					} else if (deployment.status === "DRAINING") {
						rowClass = "row-draining";
					} else {
						serviceColor = getColorForKey(deployment.service_name);
						keysColor = getColorForKey(deployment.keys);
					}

					return `
          <tr class="${rowClass}">
            <td class="bold" style="color: ${serviceColor}">${sanitizeHTML(deployment.service_name)}</td>
            <td style="color: ${keysColor}">${sanitizeHTML(deployment.keys)}</td>
            <td>${sanitizeHTML(deployment.status)}</td>
            <td>${deployment.running_count}</td>
            <td>${deployment.desired_count}</td>
            <td>${deployment.pending_count}</td>
            <td>${deployment.failed_count}</td>
          </tr>
        `;
				})
				.join("")}
    </tbody>
  `;
	deploymentsDiv.innerHTML = "";
	deploymentsDiv.appendChild(table);
}

function renderErrors(errors) {
	const errorSection = document.getElementById("errors");
	if (errors.length === 0) {
		errorSection.innerHTML = "";
		return;
	}

	const table = document.createElement("table");
	table.className = "table-auto w-full";
	table.innerHTML = `
    <thead>
      <tr>
        <th style="width: 30%">Service Name</th>
        <th style="width: 20%">Keys</th>
        <th style="width: 50%">Error</th>
      </tr>
    </thead>
    <tbody>
      ${errors
				.map(
					(error) => `
        <tr class="error">
          <td class="bold" style="color: ${getColorForKey(error.service_name)}">${sanitizeHTML(error.service_name)}</td>
          <td style="color: ${getColorForKey(error.keys)}">${sanitizeHTML(error.keys)}</td>
          <td>${sanitizeHTML(error.error).replace(/\n/g, "<br>")}</td>
        </tr>
      `,
				)
				.join("")}
    </tbody>
  `;
	errorSection.innerHTML =
		'<h2 class="text-2xl font-bold mb-4 error">Errors</h2>';
	errorSection.appendChild(table);
}

function toggleRefresh(enabled) {
	const intervalPicker = document.getElementById("refresh-interval");
	const refreshButton = document.getElementById("manual-refresh");
	if (enabled) {
		fetchData();
		intervalPicker.disabled = false;
		refreshButton.disabled = true;
		const intervalInSeconds = parseInt(intervalPicker.value, 10);
		refreshInterval = setInterval(fetchData, intervalInSeconds * 1000);
	} else {
		clearInterval(refreshInterval);
		refreshInterval = null;
		intervalPicker.disabled = false;
		refreshButton.disabled = false;
	}
}

function updateRefreshInterval() {
	if (refreshInterval) {
		clearInterval(refreshInterval);
		const intervalInSeconds = parseInt(
			document.getElementById("refresh-interval").value,
			10,
		);
		refreshInterval = setInterval(fetchData, intervalInSeconds * 1000);
	}
}

function manualRefresh() {
	fetchData();
}

document.addEventListener("DOMContentLoaded", () => {
	const refreshToggle = document.getElementById("refresh-toggle");
	const intervalPicker = document.getElementById("refresh-interval");
	const refreshButton = document.getElementById("manual-refresh");

	refreshToggle.addEventListener("change", (event) => {
		toggleRefresh(event.target.checked);
	});

	intervalPicker.addEventListener("input", updateRefreshInterval);
	refreshButton.addEventListener("click", manualRefresh);
	intervalPicker.value = DEFAULT_REFRESH_INTERVAL;
	refreshButton.disabled = false;
	fetchData();
});
