import React, { useState, useRef, useEffect } from 'react';

const CanvasPrompt = ({ placeholder, onSubmit, onCancel }) => {
  const [input, setInput] = useState('');
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas) {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      const ctx = canvas.getContext('2d');
      ctx.fillStyle = 'rgba(0, 0, 0, 0.3)';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
    }
  }, []);

  const handleKeyDown = (e) => {
    if (e.key === 'Enter') {
      onSubmit(input);
    } else if (e.key === 'Escape') {
      onCancel();
    }
  };

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100%',
        height: '100%',
        zIndex: 9999,
      }}
    >
      {/* The translucent background via a canvas */}
      <canvas ref={canvasRef} style={{ position: 'absolute', top: 0, left: 0 }} />

      {/* A minimal "modal" container for the input */}
      <div
        style={{
          position: 'absolute',
          top: '50%',
          left: '50%',
          transform: 'translate(-50%, -50%)',
          backgroundColor: 'rgba(255, 255, 255, 0.1)',
          borderRadius: '8px',
          boxShadow: '0 8px 20px rgba(0,0,0,0.4)',
          width: '400px',
          maxWidth: '90%',
          padding: '1rem',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'stretch',
        }}
      >
        <input
          type="text"
          autoFocus
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder={placeholder || ''}
          style={{
            backgroundColor: 'rgba(0, 0, 0, 0.2)',
            color: '#fff',
            border: '1px solid #555',
            borderRadius: '4px',
            padding: '0.5rem',
            fontSize: '1rem',
            outline: 'none',
          }}
        />
      </div>
    </div>
  );
};

export default CanvasPrompt;
