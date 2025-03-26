import React, { useState, useEffect } from 'react';
import PouchDB from 'pouchdb';
import MonthView from './MonthView';
import WeekView from './WeekView';
import YearView from './YearView';
import TaskView from './TaskView';
import SearchResults from './SearchResults';
import moment from 'moment-timezone'; 

const db = new PouchDB('task_manager_data');

function debounce(func, wait) {
  let timeout;
  return function executedFunction(...args) {
    const later = () => {
      clearTimeout(timeout);
      func(...args);
    };
    clearTimeout(timeout);
    timeout = setTimeout(later, wait);
  };
}

async function updateDoc(docId, newData) {
  try {
    const doc = await db.get(docId);
    const updatedDoc = {
      _id: docId,
      _rev: doc._rev,
      data: newData,
    };
    return await db.put(updatedDoc);
  } catch (err) {
    if (err.status === 404) {
      const doc = { _id: docId, data: newData };
      return await db.put(doc);
    } else if (err.status === 409) {
      const latestDoc = await db.get(docId);
      const updatedDoc = {
        _id: docId,
        _rev: latestDoc._rev,
        data: newData,
      };
      return await db.put(updatedDoc);
    } else {
      throw err;
    }
  }
}

const TaskManagerCalendar = () => {
  const [currentDate, setCurrentDate] = useState(new Date());
  const [currentView, setCurrentView] = useState('month'); // "month", "week", or "year"
  const [events, setEvents] = useState([]);
  const [weeklyGoals, setWeeklyGoals] = useState({});
  const [selectedTask, setSelectedTask] = useState(null);
  const [searchText, setSearchText] = useState('');
  
  // Create debounced version of updateDoc for weekly goals
  const debouncedUpdateWeeklyGoals = React.useCallback(
    debounce((goals) => {
      updateDoc('weeklyGoals', goals).catch(err => 
        console.error('Error updating weeklyGoals:', err)
      );
    }, 500), // 500ms delay to reduce database writes
    []
  );

  useEffect(() => {
    async function fetchData() {
      // Fetch events document
      try {
        const eventsDoc = await db.get('events');
        if (eventsDoc && eventsDoc.data) {
          const parsedEvents = eventsDoc.data.map((ev) => ({
            ...ev,
            start: new Date(ev.start),
            end: new Date(ev.end),
          }));
          setEvents(parsedEvents);
        } else {
          console.warn('Events document exists but has no data property');
          await db.put({ _id: 'events', data: [] });
        }
      } catch (err) {
        if (err.status === 404) {
          await db.put({ _id: 'events', data: [] });
        } else {
          console.error('Error fetching events:', err);
        }
      }

      // Fetch weekly goals with improved error handling
      try {
        const goalsDoc = await db.get('weeklyGoals');
        if (goalsDoc && goalsDoc.data) {
          setWeeklyGoals(goalsDoc.data);
        } else {
          console.warn('Weekly goals document exists but has no data property');
          await db.put({ _id: 'weeklyGoals', data: {} });
        }
      } catch (err) {
        if (err.status === 404) {
          console.log('Creating new weekly goals document');
          await db.put({ _id: 'weeklyGoals', data: {} });
        } else {
          console.error('Error fetching weeklyGoals:', err);
        }
      }
    }
    fetchData();
  }, []);

  useEffect(() => {
    async function updateEvents() {
      try {
        await updateDoc('events', events);
      } catch (err) {
        console.error('Error updating events:', err);
      }
    }
    updateEvents();
  }, [events]);

  // Use debounced function for weekly goals updates
  useEffect(() => {
    debouncedUpdateWeeklyGoals(weeklyGoals);
  }, [weeklyGoals, debouncedUpdateWeeklyGoals]);

  const navigate = (direction) => {
    let newDate = moment(currentDate);
    if (direction === 'today') {
      newDate = moment();
    } else {
      if (currentView === 'month') {
        newDate = direction === 'prev' ? newDate.subtract(1, 'month') : newDate.add(1, 'month');
      } else if (currentView === 'week') {
        newDate = direction === 'prev' ? newDate.subtract(1, 'week') : newDate.add(1, 'week');
      } else if (currentView === 'year') {
        newDate = direction === 'prev' ? newDate.subtract(1, 'year') : newDate.add(1, 'year');
      }
    }
    setCurrentDate(newDate.toDate());
  };

  const handleTaskShiftClick = (task) => {
    setSelectedTask(task);
  };

  const importData = (e) => {
    const file = e.target.files[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = async (event) => {
      try {
        if (file.name.toLowerCase().endsWith('.csv')) {
          const csvText = event.target.result;
          const newEvents = parseCsvToEvents(csvText);
          setEvents((prev) => [...prev, ...newEvents]);
        } else {
          const importedData = JSON.parse(event.target.result);
          if (importedData.events) {
            const importedEvents = importedData.events.map((ev) => ({
              ...ev,
              start: new Date(ev.start),
              end: new Date(ev.end),
            }));
            setEvents(importedEvents);
          }
          if (importedData.weeklyGoals) {
            setWeeklyGoals(importedData.weeklyGoals);
          }
        }
      } catch (err) {
        console.error('Error importing data:', err);
      }
    };
    reader.readAsText(file);
  };
  
function stripQuotes(str) {
  if (str.startsWith('"') && str.endsWith('"')) {
    return str.slice(1, -1);
  }
  return str;
}

function parseCsvToEvents(csvText) {
  const importDate = new Date();
  const todayMidnight = new Date(
    importDate.getFullYear(),
    importDate.getMonth(),
    importDate.getDate()
  );

  const lines = csvText.split(/\r?\n/).filter((line) => line.trim() !== '');
  if (lines.length < 2) {
    console.warn('CSV has no data rows.');
    return [];
  }

  const headers = lines[0].split(',');
  
  const taskNameIndex = headers.findIndex(
    (h) => h.trim().toLowerCase() === 'task name'
  );
  const taskContentIndex = headers.findIndex(
    (h) => h.trim().toLowerCase() === 'task content'
  );
  const dueDateIndex = headers.findIndex(
    (h) => h.trim().toLowerCase() === 'due date text'
  );

  if (taskNameIndex === -1 || taskContentIndex === -1 || dueDateIndex === -1) {
    console.error(
      'CSV missing required columns. Needed: "Task Name", "Task Content", "Due Date Text".'
    );
    return [];
  }

  let totalRows = 0;
  let validRows = 0;
  const newEvents = [];

  for (let i = 1; i < lines.length; i++) {
    totalRows++;
    const row = lines[i].split(',');

    if (row.length <= dueDateIndex) {
      console.warn('Skipping row with insufficient columns:', row);
      continue;
    }

    const rawTitle = row[taskNameIndex].trim();
    const rawContent = row[taskContentIndex].trim();
    const rawDueDateText = row[dueDateIndex].trim();
    const title = stripQuotes(rawTitle);
    const content = stripQuotes(rawContent);
    const dueDateStr = stripQuotes(rawDueDateText);

    let parsedMoment;
    if (/^\d+$/.test(dueDateStr)) {
      parsedMoment = moment(Number(dueDateStr));
    } else {
      parsedMoment = moment.tz(dueDateStr, 'MM/DD/YYYY, hh:mm:ss A z', 'America/Los_Angeles');
    }

    if (!parsedMoment.isValid()) {
      console.warn(`Could not parse date: ${dueDateStr}`);
      continue;
    }

    const startDate = parsedMoment.toDate();
    const endDate = new Date(startDate.getTime() + 60 * 60 * 1000);

    const isComplete = startDate < todayMidnight;

    const newEvent = {
      id: Date.now() + i,
      title: title || 'Untitled Task',
      start: startDate,
      end: endDate,
      comments: [
        {
          id: Date.now() + i,
          text: content || '',
        },
      ],
      completed: isComplete,
    };

    newEvents.push(newEvent);
    validRows++;
  }

  const percent = ((validRows / totalRows) * 100).toFixed(1);
  console.log(`Imported ${validRows} out of ${totalRows} tasks. That's ${percent}% success rate.`);
  return newEvents;
} 

  return (
    <div style={{ background: '#121212', color: '#fff', minHeight: '100vh' }}>
      <header
        style={{
          background: '#1f1f1f',
          padding: '1rem',
          display: 'flex',
          alignItems: 'center',
          flexWrap: 'wrap',
        }}
      >
        <div style={{ flex: 1, textAlign: 'left' }}>
          <button onClick={() => navigate('prev')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>
            Prev
          </button>
          <button onClick={() => navigate('today')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>
            Today
          </button>
          <button onClick={() => navigate('next')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>
            Next
          </button>
        </div>
        <div style={{ flex: 1, textAlign: 'center' }}>
          <button onClick={() => setCurrentView('month')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>
            Month
          </button>
          <button onClick={() => setCurrentView('week')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>
            Week
          </button>
          <button onClick={() => setCurrentView('year')} style={{ margin: '0.25rem', background: '#333', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}>
            Year
          </button>
        </div>
        <div style={{ flex: 1, textAlign: 'right' }}>
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
          <button
            onClick={() => {
              const dataToExport = { events, weeklyGoals };
              const jsonStr = JSON.stringify(dataToExport, null, 2);
              const blob = new Blob([jsonStr], { type: 'application/json' });
              const url = URL.createObjectURL(blob);
              const link = document.createElement('a');
              link.href = url;
              link.download = 'task-manager-data.json';
              document.body.appendChild(link);
              link.click();
              document.body.removeChild(link);
              setTimeout(() => URL.revokeObjectURL(url), 100);
            }}
            style={{ margin: '0.25rem', background: '#555', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}
          >
            Export
          </button>
          <input type="file" id="importFile" style={{ display: 'none' }} onChange={importData} />
          <button
            onClick={() => document.getElementById('importFile').click()}
            style={{ margin: '0.25rem', background: '#555', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}
          >
            Import
          </button>
        </div>
      </header>
      <main style={{ padding: '1rem' }}>
        {currentView === 'month' && (
          <MonthView
            events={events}
            weeklyGoals={weeklyGoals}
            setWeeklyGoals={setWeeklyGoals}
            currentDate={currentDate}
            setEvents={setEvents}
            onTaskShiftClick={handleTaskShiftClick}
          />
        )}
        {currentView === 'week' && (
          <WeekView
            events={events}
            currentDate={currentDate}
            setEvents={setEvents}
            onTaskShiftClick={handleTaskShiftClick}
          />
        )}
        {currentView === 'year' && (
          <YearView events={events} currentDate={currentDate} />
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
          onDeleteTask={(taskId) => {
            setEvents((prev) => prev.filter((ev) => ev.id !== taskId));
            setSelectedTask(null);
          }}
        />
      )}
      {searchText && (
        <SearchResults
          searchText={searchText}
          events={events}
          weeklyGoals={weeklyGoals}
          onClose={() => setSearchText('')}
          onTaskShiftClick={(task) => {
            setSelectedTask(task);
            setSearchText('');
          }}
        />
      )}
    </div>
  );
};

export default TaskManagerCalendar;