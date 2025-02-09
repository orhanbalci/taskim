import React, { useState, useEffect, useRef } from 'react';

const modalOverlayStyle = {
  position: 'fixed',
  top: 0,
  left: 0,
  right: 0,
  bottom: 0,
  background: 'rgba(0,0,0,0.7)',
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  zIndex: 1000,
};

const modalContentStyle = {
  background: '#1e1e1e',
  padding: '1rem',
  borderRadius: '8px',
  width: '80%',
  maxWidth: '800px',
  maxHeight: '90vh',
  display: 'flex',
  flexDirection: 'column',
};

const panelContainerStyle = {
  display: 'flex',
  flex: 1,
  marginTop: '1rem',
  gap: '1rem',
};

const panelStyle = {
  flex: 1,
  background: '#2e2e2e',
  borderRadius: '4px',
  padding: '0.5rem',
  overflowY: 'auto',
};

const inputStyle = {
  width: '100%',
  padding: '0.5rem',
  background: '#444',
  border: 'none',
  outline: 'none',
  color: '#fff',
  borderRadius: '4px',
  marginTop: '0.5rem',
};

const TaskView = ({ task, onClose, onUpdateTask }) => {
  const [subtaskInput, setSubtaskInput] = useState('');
  const [commentInput, setCommentInput] = useState('');
  const modalRef = useRef(null);

  // Close on Escape key.
  useEffect(() => {
    const handleKeyDown = (e) => {
      if (e.key === 'Escape') onClose();
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onClose]);

  // Close on clicking outside.
  const handleOverlayClick = (e) => {
    if (modalRef.current && !modalRef.current.contains(e.target)) {
      onClose();
    }
  };

  const addSubtask = () => {
    if (!subtaskInput.trim()) return;
    const newSubtask = { id: Date.now(), title: subtaskInput };
    const updatedTask = { ...task, subtasks: [...(task.subtasks || []), newSubtask] };
    onUpdateTask(updatedTask);
    setSubtaskInput('');
  };

  const addComment = () => {
    if (!commentInput.trim()) return;
    const newComment = { id: Date.now(), text: commentInput };
    const updatedTask = { ...task, comments: [...(task.comments || []), newComment] };
    onUpdateTask(updatedTask);
    setCommentInput('');
  };

  return (
    <div style={modalOverlayStyle} onClick={handleOverlayClick}>
      <div style={modalContentStyle} ref={modalRef}>
        <h2 style={{ margin: 0, color: '#fff' }}>{task.title}</h2>
        <div style={panelContainerStyle}>
          {/* Subtasks panel */}
          <div style={panelStyle}>
            {(task.subtasks || []).map((subtask) => (
              <div key={subtask.id} style={{ display: 'flex', alignItems: 'center', padding: '0.25rem 0', borderBottom: '1px solid #333' }}>
                <span style={{ marginRight: '0.5rem', fontSize: '1.2rem', color: '#fff' }}>○</span>
                <span style={{ color: '#fff' }}>{subtask.title}</span>
              </div>
            ))}
            <input
              type="text"
              placeholder="Add subtask"
              value={subtaskInput}
              onChange={(e) => setSubtaskInput(e.target.value)}
              onKeyDown={(e) => { if (e.key === 'Enter') addSubtask(); }}
              style={inputStyle}
            />
          </div>
          {/* Comments panel */}
          <div style={panelStyle}>
            {(task.comments || []).map((comment) => (
              <div key={comment.id} style={{ padding: '0.25rem 0', borderBottom: '1px solid #333', color: '#fff' }}>
                {comment.text}
              </div>
            ))}
            <input
              type="text"
              placeholder="Add comment"
              value={commentInput}
              onChange={(e) => setCommentInput(e.target.value)}
              onKeyDown={(e) => { if (e.key === 'Enter') addComment(); }}
              style={inputStyle}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default TaskView;
