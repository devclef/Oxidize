async function fetchAccounts() {
    const app = document.getElementById('app');
    const typeFilter = document.getElementById('type-filter').value;
    
    app.innerHTML = '<div class="loading">Loading accounts...</div>';
    
    try {
        let url = '/api/accounts';
        if (typeFilter !== 'all') {
            url += `?type=${typeFilter}`;
        }
        
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Error: ${response.status} ${response.statusText}`);
        }
        const accounts = await response.json();

        if (accounts.length === 0) {
            app.innerHTML = '<div class="loading">No accounts found for this filter.</div>';
            return;
        }

        let html = '<div class="account-list">';
        accounts.forEach(account => {
            const isNegative = parseFloat(account.balance) < 0;
            html += `
                <div class="account-card">
                    <input type="checkbox" class="account-select" value="${account.id}">
                    <div class="account-info">
                        <span class="account-name">${account.name}</span>
                        <span class="account-type-tag">${account.account_type}</span>
                    </div>
                    <span class="account-balance ${isNegative ? 'negative' : ''}">
                        ${account.currency}${account.balance}
                    </span>
                </div>
            `;
        });
        html += '</div>';
        app.innerHTML = html;
    } catch (error) {
        app.innerHTML = `<div class="error">Failed to load accounts: ${error.message}</div>`;
        console.error('Fetch error:', error);
    }
}

async function fetchChartData() {
    const chartContainer = document.querySelector('.chart-container');
    const chartError = document.getElementById('chart-error');
    
    // Clear previous errors
    chartError.innerHTML = '';
    
    const selectedCheckboxes = document.querySelectorAll('.account-select:checked');
    const selectedIds = Array.from(selectedCheckboxes).map(cb => cb.value);
    
    try {
        let url = '/api/accounts/balance-history';
        if (selectedIds.length > 0) {
            const params = new URLSearchParams();
            selectedIds.forEach(id => params.append('accounts[]', id));
            url += `?${params.toString()}`;
        }
        
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Error: ${response.status} ${response.statusText}`);
        }
        const history = await response.json();
        
        if (!history || history.length === 0) {
            // If we selected specific accounts and got nothing, show error
            if (selectedIds.length > 0) {
                 chartError.innerHTML = '<div class="error">No data returned for selected accounts.</div>';
            } else {
                 chartContainer.style.display = 'none';
            }
            return;
        }

        chartContainer.style.display = 'block';
        renderChart(history);
    } catch (error) {
        console.error('Fetch chart error:', error);
        chartError.innerHTML = `<div class="error">Failed to load chart data: ${error.message}</div>`;
    }
}

let balanceChart = null;

function renderChart(history) {
    const ctx = document.getElementById('balanceChart').getContext('2d');
    
    // Destroy existing chart to avoid memory leaks
    if (balanceChart) {
        balanceChart.destroy();
    }
    
    // Extract labels from the first dataset that has entries
    let labels = [];
    const firstDataset = history.find(ds => ds.entries && (Array.isArray(ds.entries) ? ds.entries.length > 0 : Object.keys(ds.entries).length > 0));
    if (firstDataset) {
        if (Array.isArray(firstDataset.entries)) {
            labels = firstDataset.entries.map(e => e.key);
        } else {
            labels = Object.keys(firstDataset.entries);
        }
    }

    const datasets = history.map((ds, index) => {
        let data = [];
        if (Array.isArray(ds.entries)) {
            data = ds.entries.map(e => parseFloat(e.value || 0));
        } else {
            data = Object.values(ds.entries).map(v => {
                if (typeof v === 'object' && v !== null) {
                    return parseFloat(v.value || 0);
                }
                return parseFloat(v);
            });
        }

        const colors = [
            '#3498db', '#2ecc71', '#e74c3c', '#f1c40f', '#9b59b6', 
            '#1abc9c', '#e67e22', '#34495e', '#7f8c8d', '#d35400'
        ];
        const color = colors[index % colors.length];

        return {
            label: ds.label,
            data: data,
            borderColor: color,
            backgroundColor: color + '20', // Add transparency
            borderWidth: 2,
            tension: 0.1,
            fill: false
        };
    });

    balanceChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: datasets
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    position: 'bottom',
                }
            },
            scales: {
                y: {
                    beginAtZero: false,
                    ticks: {
                        callback: function(value) {
                            return value.toLocaleString();
                        }
                    }
                },
                x: {
                    ticks: {
                        maxRotation: 45,
                        minRotation: 45
                    }
                }
            }
        }
    });
}

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    const fetchAccountsBtn = document.getElementById('fetch-accounts-btn');
    const updateChartBtn = document.getElementById('update-chart-btn');
    const app = document.getElementById('app');

    app.innerHTML = '<div class="loading">Select a type and click "Fetch Accounts" to begin.</div>';

    fetchAccountsBtn.addEventListener('click', fetchAccounts);
    updateChartBtn.addEventListener('click', fetchChartData);
    
    // Initial chart load
    fetchChartData();
});
