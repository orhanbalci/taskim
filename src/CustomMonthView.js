// src/CustomMonthView.js
import React from 'react';
import moment from 'moment';
import { useDrop } from 'react-dnd';
import DraggableTask from './DraggableTask';

/**
 * DayCell Component
 * Renders a single day cell as a drop target and displays tasks for that day.
 */
const DayCell = ({ day, dayEvents, updateTaskDate, handleAddTask }) => {
  // Use the useDrop hook at the top level of the component.
  const [{ isOver }, drop] = useDrop({
    accept: 'TASK',
    drop: (item) => {
      updateTaskDate(item.event.id, day);
    },
    collect: (monitor) => ({
      isOver: monitor.isOver(),
    }),
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
        {day.format('D')}
      </div>
      <div style={{ fontSize: '0.8rem' }}>
        {dayEvents.map((ev) => (
          <DraggableTask key={ev.id} event={ev} />
        ))}
      </div>
    </td>
  );
};

/**
 * CustomMonthView Component
 * Renders a custom month view calendar with an integrated weekly goal input row and draggable tasks.
 */
const CustomMonthView = ({ events, weeklyGoals, setWeeklyGoals, currentDate, setEvents }) => {
  // Determine the grid boundaries for the current month.
  const startOfMonth = moment(currentDate).startOf('month');
  const endOfMonth = moment(currentDate).endOf('month');
  const startDate = moment(startOfMonth).startOf('week'); // beginning of the week that contains the 1st
  const endDate = moment(endOfMonth).endOf('week'); // end of the week that contains the last day

  // Build an array of weeks; each week is an array of 7 days.
  let day = startDate.clone();
  const weeks = [];
  while (day.isBefore(endDate)) {
    const week = [];
    for (let i = 0; i < 7; i++) {
      week.push(day.clone());
      day.add(1, 'day');
    }
    weeks.push(week);
  }

  // Update a task's date when it is dropped on a new day.
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

  // Handler to add a new task via double-click.
  const handleAddTask = (day) => {
    const title = prompt(`Enter task title for ${day.format('MMM D, YYYY')}:`);
    if (title) {
      const newEvent = {
        id: Date.now(), // or use another unique id generator
        title,
        start: day.toDate(),
        end: moment(day).add(1, 'hour').toDate(),
      };
      setEvents((prevEvents) => [...prevEvents, newEvent]);
    }
  };

  // Update the weekly goal for the given week key.
  const updateWeeklyGoal = (weekKey, value) => {
    setWeeklyGoals((prevGoals) => ({ ...prevGoals, [weekKey]: value }));
  };

  return (
    <div style={{ background: '#121212', padding: '1rem', color: '#fff' }}>
      <h2 style={{ textAlign: 'center', marginBottom: '1rem' }}>
        {moment(currentDate).format('MMMM YYYY')}
      </h2>
      <table
        style={{
          width: '100%',
          borderCollapse: 'collapse',
          tableLayout: 'fixed',
          fontFamily:
            "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif",
        }}
      >
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
            // Use ISO week key (e.g., "2025-07") for the weekly goal.
            const weekKey = week[0].format('GGGG-ww');
            const weekGoal = weeklyGoals[weekKey] || '';
            return (
              <React.Fragment key={`${weekKey}-${wi}`}>
                {/* Editable Weekly Goal Row */}
                <tr>
                  <td
                    colSpan={7}
                    style={{
                      background: '#2e2e2e',
                      padding: '0.5rem',
                      textAlign: 'left',
                      border: '1px solid #333',
                    }}
                  >
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
                {/* Dates Row */}
                <tr>
                  {week.map((day, di) => {
                    // Filter events for this day.
                    const dayEvents = events.filter((event) =>
                      moment(event.start).isSame(day, 'day')
                    );
                    return (
                      <DayCell
                        key={di}
                        day={day}
                        dayEvents={dayEvents}
                        updateTaskDate={updateTaskDate}
                        handleAddTask={handleAddTask}
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
