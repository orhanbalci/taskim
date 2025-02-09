// src/TaskManagerCalendar.js
import React, { useState } from 'react';
import WeekView from './WeekView';
import QuarterView from './QuarterView';
import CustomMonthView from './CustomMonthView';

const TaskManagerCalendar = () => {
  const [currentView, setCurrentView] = useState('month'); // "month", "week", or "quarter"
  const [currentDate, setCurrentDate] = useState(new Date());

  const [events, setEvents] = useState([
    {
      id: 0,
      title: 'Initial Task',
      start: new Date(),
      end: new Date(new Date().getTime() + 60 * 60 * 1000),
    },
  ]);

  // Weekly goals keyed by ISO week (e.g., "2025-07")
  const [weeklyGoals, setWeeklyGoals] = useState({
    '2025-07': 'Build this app',
  });

  // Daily goals for week view
  const [dailyGoals, setDailyGoals] = useState({
    '2025-02-08': 'Call client',
  });

  const headerStyle = {
    background: '#1f1f1f',
    padding: '1rem',
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
  };

  const buttonStyle = {
    background: '#333',
    color: '#fff',
    border: 'none',
    padding: '0.5rem 1rem',
    margin: '0 0.5rem',
    cursor: 'pointer',
    borderRadius: '4px',
  };

  return (
    <div style={{ background: '#121212', color: '#fff', minHeight: '100vh' }}>
      <header style={headerStyle}>
        <div>
          <button style={buttonStyle} onClick={() => setCurrentView('month')}>
            Month View
          </button>
          <button style={buttonStyle} onClick={() => setCurrentView('week')}>
            Week View
          </button>
          <button style={buttonStyle} onClick={() => setCurrentView('quarter')}>
            Quarter View
          </button>
        </div>
      </header>
      <main style={{ padding: '1rem' }}>
        {currentView === 'month' && (
          <CustomMonthView
            events={events}
            weeklyGoals={weeklyGoals}
            setWeeklyGoals={setWeeklyGoals}
            currentDate={currentDate}
            setEvents={setEvents}
          />
        )}
        {currentView === 'week' && (
          <WeekView
            events={events}
            dailyGoals={dailyGoals}
            currentDate={currentDate}
            onNavigate={(date) => setCurrentDate(date)}
            setEvents={setEvents}
          />
        )}
        {currentView === 'quarter' && (
          <QuarterView
            events={events}
            weeklyGoals={weeklyGoals}
            currentDate={currentDate}
          />
        )}
      </main>
    </div>
  );
};

export default TaskManagerCalendar;
