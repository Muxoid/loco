// Function to get the token from cookies
function getCookie(name) {
    let cookieArray = document.cookie.split('; ');
    let cookie = cookieArray.find(row => row.startsWith(name + '='));
    return cookie ? cookie.split('=')[1] : null;
}

// Function to update HTMX request parameters
function updateHtmxRequests() {
    document.body.addEventListener('htmx:configRequest', function(evt) {
        const authToken = getCookie('token');  // Assuming the cookie is named 'token'
        if (authToken) {
            evt.detail.headers['Authorization'] = 'Bearer ' + authToken;
        }
        // You can add other modifications to the parameters or headers here
    });
}

// Initialize the HTMX request modifications
document.addEventListener('DOMContentLoaded', updateHtmxRequests);
document.body.addEventListener('htmx:beforeRequest', function(event) {
    // Add or modify headers globally
    event.detail.headers['Authorization'] = 'Bearer ' + authToken;  // Replace YOUR_ACCESS_TOKEN
});
