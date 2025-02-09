// src/QuarterView.js
import React from 'react';
import moment from 'moment';

const QuarterView = ({ events, weeklyGoals, currentDate }) => {
  const quarterStart = moment(currentDate).startOf('quarter');
  const quarterEnd = moment(currentDate).endOf('quarter');

  let weeks = [];
  let weekStart = quarterStart.clone().startOf('week');
  while (weekStart.isBefore(quarterEnd)) {
    weeks.push(weekStart.clone());
    weekStart.add(1, 'week');
  }

  // Count tasks in a given week by checking if an event’s start falls within the week.
  const countTasksInWeek = (weekStart) => {
    const weekEnd = moment(weekStart).endOf('week');
    return events.filter((event) => {
      const eventStart = moment(event.start);
      return eventStart.isBetween(weekStart, weekEnd, null, '[]');
    }).length;
  };

  return (
    <div>
      <h2 style={{ textAlign: 'center', color: '#fff' }}>
        Quarter View – {moment(currentDate).format('Q')} Quarter {moment(currentDate).year()}
      </h2>
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
          gap: '1rem',
        }}
      >
        {weeks.map((weekStart) => {
          const weekKey = weekStart.format('GGGG-ww');
          const goal = weeklyGoals[weekKey] || 'No goal set';
          const taskCount = countTasksInWeek(weekStart);
          return (
            <div
              key={weekKey}
              style={{
                background: '#2e2e2e',
                padding: '1rem',
                borderRadius: '4px',
                color: '#fff',
              }}
            >
              <h3 style={{ margin: '0 0 0.5rem 0' }}>Week {weekStart.format('ww')}</h3>
              <p style={{ margin: '0.5rem 0' }}>
                <strong>Goal:</strong> {goal}
              </p>
              <p style={{ margin: '0.5rem 0' }}>
                <strong>Tasks Scheduled:</strong> {taskCount}
              </p>
              <p style={{ margin: '0.5rem 0', fontSize: '0.8rem' }}>
                {weekStart.format('MMM D')} – {moment(weekStart).add(6, 'days').format('MMM D')}
              </p>
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default QuarterView;
