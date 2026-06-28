import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'

const rootElement = document.getElementById('root')!;
const loader = document.getElementById('static-loader');

// Handle OAuth token from URL fragment
const handleOAuthCallback = () => {
  const hash = window.location.hash;
  if (hash && hash.startsWith('#token=')) {
    const token = hash.substring(7); // Remove '#token='
    localStorage.setItem('token', token);
    
    // clear the token from URL for security
    window.history.replaceState(null, '', window.location.pathname + window.location.search);
    
    const apiUrl = import.meta.env.VITE_API_URL || 'http://localhost:8080';
    
    // fetch user info with the new token
    fetch(`${apiUrl}/auth/me`, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    })
    .then(res => {
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      return res.json();
    })
    .then(data => {
      localStorage.setItem('user', JSON.stringify(data));
      // dispatch custom event to trigger auth state update
      window.dispatchEvent(new CustomEvent('oauth-login', { detail: { token, user: data } }));
    })
    .catch(() => {
      localStorage.removeItem('token');
      window.dispatchEvent(new CustomEvent('oauth-login-failed'));
    });
  }
};

handleOAuthCallback();

createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
)

requestAnimationFrame(() => {
  rootElement.style.opacity = '1';
  if (loader) {
    loader.style.opacity = '0';
    setTimeout(() => {
      if (loader && loader.parentElement) {
        loader.remove();
      }
    }, 500);
  }
});