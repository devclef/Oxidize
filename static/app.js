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

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    const filterSelect = document.getElementById('type-filter');
    filterSelect.addEventListener('change', fetchAccounts);
    fetchAccounts();
});
