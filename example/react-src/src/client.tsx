import React from 'react';
import ReactDOM from 'react-dom';
import App from './App.tsx';

// Hydrate the <App /> component on the client-side
ReactDOM.hydrate(<App />, document.getElementById('root'));