import React from 'react';

const modalOverlayStyle = {
  position: 'fixed',
  top: 0,
  left: 0,
  width: '100%',
  height: '100%',
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
  maxWidth: '600px',
  maxHeight: '80vh',
  overflowY: 'auto',
};

const SearchResults = ({ searchText, events, weeklyGoals, onClose, onTaskDoubleClick }) => {
  // Simple fuzzy search using case-insensitive substring matching.
  const lowerSearch = searchText.toLowerCase();

  // Find matching tasks (title, subtasks, comments)
  const matchingTasks = events.filter((task) => {
    const inTitle = task.title.toLowerCase().includes(lowerSearch);
    const inSubtasks =
      (task.subtasks || []).some((st) => st.title.toLowerCase().includes(lowerSearch));
    const inComments =
      (task.comments || []).some((c) => c.text.toLowerCase().includes(lowerSearch));
    return inTitle || inSubtasks || inComments;
  });

  // Find matching weekly goals (the keys are weeks; values are goal strings)
  const matchingGoals = Object.entries(weeklyGoals)
    .filter(([, goal]) => goal.toLowerCase().includes(lowerSearch))
    .map(([weekKey, goal]) => ({ weekKey, goal }));

  return (
    <div style={modalOverlayStyle} onClick={onClose}>
      <div style={modalContentStyle} onClick={(e) => e.stopPropagation()}>
        <h2>Search Results for "{searchText}"</h2>
        <h3>Tasks</h3>
        {matchingTasks.length === 0 ? (
          <p>No matching tasks found.</p>
        ) : (
          matchingTasks.map((task) => (
            <div
              key={task.id}
              onDoubleClick={() => onTaskDoubleClick(task)}
              style={{ padding: '0.5rem', borderBottom: '1px solid #333', cursor: 'pointer' }}
            >
              {task.title}
            </div>
          ))
        )}
        <h3>Weekly Goals</h3>
        {matchingGoals.length === 0 ? (
          <p>No matching goals found.</p>
        ) : (
          matchingGoals.map(({ weekKey, goal }) => (
            <div key={weekKey} style={{ padding: '0.5rem', borderBottom: '1px solid #333' }}>
              <strong>Week {weekKey.split('-')[1]}:</strong> {goal}
            </div>
          ))
        )}
        <button
          onClick={onClose}
          style={{ marginTop: '1rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}
        >
          Close
        </button>
      </div>
    </div>
  );
};

export default SearchResults;
