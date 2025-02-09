import React from 'react';
import moment from 'moment';
import { useDrop } from 'react-dnd';
import DraggableTask from './DraggableTask';

// The DayCell component renders a single <td> cell.
const DayCell = ({ day, dayEvents, updateTaskDate, handleAddTask, onTaskDoubleClick }) => {
  // Always call useDrop at the top level.
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
          <DraggableTask key={ev.id} event={ev} onDoubleClick={onTaskDoubleClick} />
        ))}
      </div>
    </td>
  );
};

const CustomMonthView = ({ events, weeklyGoals, setWeeklyGoals, currentDate, setEvents, onTaskDoubleClick }) => {
  // Calculate grid boundaries.
  const startOfMonth = moment(currentDate).startOf('month');
  const endOfMonth = moment(currentDate).endOf('month');
  const startDate = moment(startOfMonth).startOf('week'); // start at Sunday
  const endDate = moment(endOfMonth).endOf('week');         // end at Saturday

  // Build weeks array (each week is an array of 7 moment objects).
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

  // Update a task’s date when dropped on a new day.
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

  // Allow adding a new task via double-click.
  const handleAddTask = (day) => {
    const title = prompt(`Enter task title for ${day.format('MMM D, YYYY')}:`);
    if (title) {
      const newEvent = {
        id: Date.now(),
        title,
        start: day.toDate(),
        end: moment(day).add(1, 'hour').toDate(),
      };
      setEvents([...events, newEvent]);
    }
  };

  // Update the weekly goal for this week.
  const updateWeeklyGoal = (weekKey, value) => {
    setWeeklyGoals((prevGoals) => ({ ...prevGoals, [weekKey]: value }));
  };

  return (
    <div style={{ background: '#121212', padding: '1rem', color: '#fff' }}>
      <h2 style={{ textAlign: 'center', marginBottom: '1rem' }}>
        {moment(currentDate).format('MMMM YYYY')}
      </h2>
      <table style={{ width: '100%', borderCollapse: 'collapse', tableLayout: 'fixed' }}>
        <thead>
          <tr>
            {['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'].map((dayName) => (
              <th
                key={dayName}
                style={{ border: '1px solid #333', padding: '0.5rem', background: '#1e1e1e', color: '#fff' }}
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
                {/* Editable weekly goal row */}
                <tr>
                  <td colSpan="7" style={{ background: '#2e2e2e', padding: '0.5rem', border: '1px solid #333' }}>
                    <span style={{ marginRight: '0.5rem' }}>
                      Week {week[0].isoWeek()} Goal:
                    </span>
                    <input
                      type="text"
                      value={weekGoal}
                      onChange={(e) => updateWeeklyGoal(weekKey, e.target.value)}
                      placeholder="Enter goal..."
                      style={{
                        background: '#444',
                        color: '#fff',
                        border: 'none',
                        outline: 'none',
                        padding: '0.25rem',
                        width: '70%',
                      }}
                    />
                  </td>
                </tr>
                {/* Dates row */}
                <tr>
                  {week.map((d, di) => {
                    const dayEvents = events.filter((ev) => moment(ev.start).isSame(d, 'day'));
                    return (
                      <DayCell
                        key={di}
                        day={d}
                        dayEvents={dayEvents}
                        updateTaskDate={updateTaskDate}
                        handleAddTask={handleAddTask}
                        onTaskDoubleClick={onTaskDoubleClick}
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

export default CustomMonthView;
