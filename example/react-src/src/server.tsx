import React from 'react';
import ReactDOMServer from 'react-dom/server';
import App from './App.tsx';

const html = ReactDOMServer.renderToString(<App />);
console.log(html);