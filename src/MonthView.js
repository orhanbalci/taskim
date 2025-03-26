import React, { useState, useCallback } from 'react';
import moment from 'moment';
import { useDrop } from 'react-dnd';
import DraggableTask from './DraggableTask';
import CanvasPrompt from './CanvasPrompt';

// New dedicated component for weekly goal input with local state management
const WeeklyGoalInput = React.memo(({ weekKey, initialValue, onUpdate }) => {
  const [localValue, setLocalValue] = useState(initialValue || '');
  
  // Update local state without triggering parent updates
  const handleChange = (e) => {
    setLocalValue(e.target.value);
  };
  
  // Only update parent state when user finishes typing
  const handleBlur = () => {
    // Only trigger update if value actually changed
    if (localValue !== initialValue) {
      onUpdate(weekKey, localValue);
    }
  };
  
  // If prop changes from outside, update local state
  React.useEffect(() => {
    setLocalValue(initialValue || '');
  }, [initialValue]);
  
  return (
    <input
      type="text"
      value={localValue}
      onChange={handleChange}
      onBlur={handleBlur}
      placeholder=""
      style={{
        background: 'transparent',
        color: '#fff',
        border: 'none',
        outline: 'none',
        padding: '0.25rem',
        width: '100%',
        opacity: localValue ? '1' : '0.3',
      }}
      onFocus={(e) => {
        e.target.style.opacity = '1';
        e.target.style.background = '#444';
      }}
      onBlur={(e) => {
        e.target.style.background = 'transparent';
        e.target.style.opacity = e.target.value ? '1' : '0.3';
        handleBlur();
      }}
    />
  );
});

const DayCell = ({ day, dayEvents, updateTaskDate, handleAddTask, onTaskShiftClick }) => {
  const [{ isOver }, drop] = useDrop({
    accept: 'TASK',
    drop: (item) => {
      if (day) {
        updateTaskDate(item.event.id, day);
      }
    },
    collect: (monitor) => ({ isOver: monitor.isOver() }),
  });

  return (
    <td
      ref={drop}
      style={{
        border: '1px solid #333',
        verticalAlign: 'top',
        height: '100px',
        padding: '0.5rem',
        background: isOver ? '#3a3a3a' : '#1e1e1e',
        cursor: 'pointer',
      }}
      onDoubleClick={() => handleAddTask(day)}
    >
      <div style={{ fontWeight: 'bold', marginBottom: '0.25rem' }}>
        {day ? day.format('D') : ''}
      </div>
      <div style={{ fontSize: '0.8rem' }}>
        {(dayEvents || []).map((ev) => (
          <DraggableTask key={ev.id} event={ev} onShiftClick={onTaskShiftClick} />
        ))}
      </div>
    </td>
  );
};

const MonthView = ({
  events,
  weeklyGoals,
  setWeeklyGoals,
  currentDate,
  setEvents,
  onTaskShiftClick,
}) => {
  const [promptData, setPromptData] = useState(null);

  const startOfMonth = moment(currentDate).startOf('month');
  const endOfMonth = moment(currentDate).endOf('month');
  const startDate = moment(startOfMonth).startOf('week');
  const endDate = moment(endOfMonth).endOf('week');

  let day = startDate.clone();
  const weeks = [];
  while (day.isBefore(endDate) || day.isSame(endDate, 'day')) {
    const week = [];
    for (let i = 0; i < 7; i++) {
      week.push(day.clone());
      day.add(1, 'day');
    }
    weeks.push(week);
  }

  const updateTaskDate = (taskId, newDay) => {
    setEvents((prevEvents) =>
      prevEvents.map((ev) => {
        if (ev.id === taskId) {
          const oldStart = moment(ev.start);
          const newStart = moment(newDay)
            .hour(oldStart.hour())
            .minute(oldStart.minute())
            .second(oldStart.second());
          const duration = moment(ev.end).diff(oldStart);
          const newEnd = newStart.clone().add(duration, 'ms');
          return { ...ev, start: newStart.toDate(), end: newEnd.toDate() };
        }
        return ev;
      })
    );
  };

  const handleAddTask = (day) => {
    setPromptData({ day, message: `Enter task title for ${day.format('MMM D, YYYY')}:` });
  };

  const submitPrompt = (title) => {
    if (title && promptData) {
      const day = promptData.day;
      const newEvent = {
        id: Date.now(),
        title,
        start: day.toDate(),
        end: moment(day).add(1, 'hour').toDate(),
      };
      setEvents([...events, newEvent]);
    }
    setPromptData(null);
  };

  // Memoized callback for updating weekly goals
  const updateWeeklyGoal = useCallback((weekKey, value) => {
    setWeeklyGoals(prev => ({ ...prev, [weekKey]: value }));
  }, [setWeeklyGoals]);

  return (
    <div style={{ background: '#121212', padding: '0rem', color: '#fff', position: 'relative' }}>
      {promptData && (
        <CanvasPrompt
          message={promptData.message}
          onSubmit={submitPrompt}
          onCancel={() => setPromptData(null)}
        />
      )}
      <h2 style={{ textAlign: 'center', marginBottom: '1rem' }}>
        {moment(currentDate).format('MMMM YYYY')}
      </h2>
      <table style={{ width: '100%', borderCollapse: 'collapse', tableLayout: 'fixed' }}>
        <thead>
          <tr>
            {['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'].map((dayName) => (
              <th
                key={dayName}
                style={{
                  border: '1px solid #333',
                  padding: '0.5rem',
                  background: '#1e1e1e',
                  color: '#fff',
                }}
              >
                {dayName}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {weeks.map((week, wi) => {
            const weekKey = week[0].format('GGGG-ww');
            const weekGoal = weeklyGoals[weekKey] || '';
            return (
              <React.Fragment key={`${weekKey}-${wi}`}>
                <tr>
                  <td
                    colSpan="7"
                    style={{ background: '#2e2e2e', padding: '0.5rem', border: '1px solid #333' }}
                  >
                    <WeeklyGoalInput 
                      weekKey={weekKey}
                      initialValue={weekGoal}
                      onUpdate={updateWeeklyGoal}
                    />
                  </td>
                </tr>
                <tr>
                  {week.map((d, di) => {
                    const dayEvents = events.filter((ev) =>
                      moment(ev.start).isSame(d, 'day')
                    );
                    return (
                      <DayCell
                        key={di}
                        day={d}
                        dayEvents={dayEvents}
                        updateTaskDate={updateTaskDate}
                        handleAddTask={handleAddTask}
                        onTaskShiftClick={onTaskShiftClick}
                      />
                    );
                  })}
                </tr>
              </React.Fragment>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};

export default MonthView;
