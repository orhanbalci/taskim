// src/GoalManager.js
import React, { useState } from 'react';

const GoalManager = () => {
  const [goals, setGoals] = useState({
    quarter: '',
    month: '',
    week: '',
  });

  const handleChange = (e) => {
    const { name, value } = e.target;
    setGoals((prevGoals) => ({ ...prevGoals, [name]: value }));
  };

  const handleSave = () => {
    // In a real app, you’d persist these goals to your backend or local storage.
    alert('Goals saved:\n' + JSON.stringify(goals, null, 2));
  };

  return (
    <div style={{ padding: '1rem' }}>
      <h2>Set Your Goals</h2>
      <div>
        <label>
          Quarterly Goal:
          <input
            type="text"
            name="quarter"
            value={goals.quarter}
            onChange={handleChange}
            placeholder="e.g., Complete Q1 project"
            style={{ marginLeft: '0.5rem' }}
          />
        </label>
      </div>
      <div style={{ marginTop: '0.5rem' }}>
        <label>
          Monthly Goal:
          <input
            type="text"
            name="month"
            value={goals.month}
            onChange={handleChange}
            placeholder="e.g., Finish 5 tasks"
            style={{ marginLeft: '0.5rem' }}
          />
        </label>
      </div>
      <div style={{ marginTop: '0.5rem' }}>
        <label>
          Weekly Goal:
          <input
            type="text"
            name="week"
            value={goals.week}
            onChange={handleChange}
            placeholder="e.g., Organize your week"
            style={{ marginLeft: '0.5rem' }}
          />
        </label>
      </div>
      <button onClick={handleSave} style={{ marginTop: '1rem' }}>
        Save Goals
      </button>
    </div>
  );
};

export default GoalManager;
