import React from 'react';
import ReactDOM from 'react-dom/client'; // Note the new import for React 18
import App from './App';
import './index.css';

// Import DndProvider and the HTML5Backend
import { DndProvider } from 'react-dnd';
import { HTML5Backend } from 'react-dnd-html5-backend';

const container = document.getElementById('root');
const root = ReactDOM.createRoot(container);
root.render(
  <DndProvider backend={HTML5Backend}>
    <App />
  </DndProvider>
);
