import React, { useState, useEffect } from 'react';
import moment from 'moment';
import CustomMonthView from './CustomMonthView';
import WeekView from './WeekView';
import QuarterView from './QuarterView';
import TaskView from './TaskView';
import SearchResults from './SearchResults';

const TaskManagerCalendar = () => {
  // Use today's date as the initial current date.
  const [currentDate, setCurrentDate] = useState(new Date());
  const [currentView, setCurrentView] = useState('month'); // "month", "week", or "quarter"
  const [events, setEvents] = useState([]);
  const [weeklyGoals, setWeeklyGoals] = useState({});
  const [dailyGoals] = useState({}); // For week view; not used in this example.
  const [selectedTask, setSelectedTask] = useState(null);
  const [searchText, setSearchText] = useState('');

  // Load persistent data from localStorage when the component mounts.
  useEffect(() => {
    const storedEvents = localStorage.getItem('events');
    if (storedEvents) setEvents(JSON.parse(storedEvents));

    const storedGoals = localStorage.getItem('weeklyGoals');
    if (storedGoals) setWeeklyGoals(JSON.parse(storedGoals));
  }, []);

  // Persist events to localStorage when they change.
  useEffect(() => {
    localStorage.setItem('events', JSON.stringify(events));
  }, [events]);

  // Persist weekly goals.
  useEffect(() => {
    localStorage.setItem('weeklyGoals', JSON.stringify(weeklyGoals));
  }, [weeklyGoals]);

  // Navigation function: adjust the currentDate based on view and direction.
  const navigate = (direction) => {
    let newDate = moment(currentDate);
    if (direction === 'today') {
      newDate = moment();
    } else {
      if (currentView === 'month') {
        newDate = direction === 'prev' ? newDate.subtract(1, 'month') : newDate.add(1, 'month');
      } else if (currentView === 'week') {
        newDate = direction === 'prev' ? newDate.subtract(1, 'week') : newDate.add(1, 'week');
      } else if (currentView === 'quarter') {
        newDate = direction === 'prev' ? newDate.subtract(3, 'month') : newDate.add(3, 'month');
      }
    }
    setCurrentDate(newDate.toDate());
  };

  // Open the task view modal when a task is double-clicked.
  const handleTaskDoubleClick = (task) => {
    setSelectedTask(task);
  };

  // (For brevity, the search functionality here uses a simple search.)
  return (
    <div style={{ background: '#121212', color: '#fff', minHeight: '100vh' }}>
      <header
        style={{
          background: '#1f1f1f',
          padding: '1rem',
          display: 'flex',
          flexWrap: 'wrap',
          alignItems: 'center',
          justifyContent: 'space-between',
        }}
      >
        <div>
          <button onClick={() => navigate('prev')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>Prev</button>
          <button onClick={() => navigate('today')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>Today</button>
          <button onClick={() => navigate('next')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>Next</button>
        </div>
        <div>
          <button onClick={() => setCurrentView('month')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>Month View</button>
          <button onClick={() => setCurrentView('week')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>Week View</button>
          <button onClick={() => setCurrentView('quarter')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>Quarter View</button>
        </div>
        <div>
          <input
            type="text"
            placeholder="Search..."
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            style={{
              padding: '0.5rem',
              borderRadius: '4px',
              border: 'none',
              outline: 'none',
              background: '#333',
              color: '#fff',
              margin: '0.25rem',
            }}
          />
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
            onTaskDoubleClick={handleTaskDoubleClick}
          />
        )}
        {currentView === 'week' && (
          <WeekView
            events={events}
            dailyGoals={dailyGoals}
            currentDate={currentDate}
            setEvents={setEvents}
            onTaskDoubleClick={handleTaskDoubleClick}
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
      {selectedTask && (
        <TaskView
          task={selectedTask}
          onClose={() => setSelectedTask(null)}
          onUpdateTask={(updatedTask) => {
            setEvents((prev) =>
              prev.map((ev) => (ev.id === updatedTask.id ? updatedTask : ev))
            );
          }}
        />
      )}
      {searchText && (
        <SearchResults
          searchText={searchText}
          events={events}
          weeklyGoals={weeklyGoals}
          onClose={() => setSearchText('')}
          onTaskDoubleClick={(task) => {
            setSelectedTask(task);
            setSearchText('');
          }}
        />
      )}
    </div>
  );
};

export default TaskManagerCalendar;
