import React, { useMemo, useState, useRef, useEffect, useCallback } from 'react';
import moment from 'moment';

// Throttle function to limit how often a function is called
function throttle(func, limit) {
  let inThrottle;
  return function(...args) {
    if (!inThrottle) {
      func.apply(this, args);
      inThrottle = true;
      setTimeout(() => inThrottle = false, limit);
    }
  };
}

// Separate component for activity cell to prevent re-renders
const ActivityCell = React.memo(({ day, taskCount, tasksForDay, onMouseEnter, onMouseLeave }) => {
  const getCellColor = (count, maxCount) => {
    if (count === 0) return 'transparent';
    
    const intensity = count / maxCount;
    
    if (intensity < 0.2) return 'rgba(0, 100, 0, 0.2)';
    if (intensity < 0.4) return 'rgba(0, 130, 0, 0.4)';
    if (intensity < 0.6) return 'rgba(0, 160, 0, 0.6)';
    if (intensity < 0.8) return 'rgba(0, 190, 0, 0.8)';
    return 'rgba(0, 230, 0, 1.0)';
  };

  // For performance, we'll handle the hover only if there are tasks
  const handleMouseEnter = useCallback((e) => {
    if (taskCount > 0) {
      onMouseEnter(e, day.date.format('YYYY-MM-DD'), tasksForDay);
    }
  }, [taskCount, day.date, tasksForDay, onMouseEnter]);
  
  return (
    <div 
      onMouseEnter={handleMouseEnter}
      onMouseLeave={onMouseLeave}
      style={{
        width: '14px',
        height: '14px',
        margin: '1px',
        backgroundColor: getCellColor(taskCount, day.maxTaskCount),
        border: day.isCurrentYear ? 
          (day.isCurrentMonth ? '1px solid #444' : '1px solid #333') : 
          '1px solid #222',
        borderRadius: '2px',
        opacity: day.isCurrentYear ? 1 : 0.3,
        position: 'relative',
        cursor: taskCount > 0 ? 'pointer' : 'default'
      }}
    />
  );
});

// Separate component for the weekly summary table
const WeeklySummaryTable = React.memo(({ weeklySummary, tableContainerRef, currentWeekRowRef }) => {
  return (
    <div 
      ref={tableContainerRef}
      style={{
        maxHeight: '300px',
        overflowY: 'auto',
        border: '1px solid #333',
        borderRadius: '4px'
      }}
    >
      <table style={{ width: '100%', borderCollapse: 'collapse' }}>
        <thead style={{ position: 'sticky', top: 0, backgroundColor: '#1e1e1e', zIndex: 1 }}>
          <tr>
            <th style={{ padding: '8px', textAlign: 'left', borderBottom: '1px solid #333' }}>Week</th>
            <th style={{ padding: '8px', textAlign: 'left', borderBottom: '1px solid #333' }}>Dates</th>
            <th style={{ padding: '8px', textAlign: 'left', borderBottom: '1px solid #333' }}>Goal</th>
            <th style={{ padding: '8px', textAlign: 'right', borderBottom: '1px solid #333' }}>Tasks</th>
          </tr>
        </thead>
        <tbody>
          {weeklySummary.map(week => (
            <tr 
              key={week.weekKey} 
              ref={week.isCurrentWeek ? currentWeekRowRef : null}
              style={{ 
                backgroundColor: week.isCurrentWeek ? '#2a3225' : 'transparent' 
              }}
            >
              <td style={{ padding: '8px', borderBottom: '1px solid #333' }}>
                {week.weekNumber}
              </td>
              <td style={{ padding: '8px', borderBottom: '1px solid #333' }}>
                {week.startDate} - {week.endDate}
              </td>
              <td style={{ padding: '8px', borderBottom: '1px solid #333', maxWidth: '300px', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                {week.goal || '-'}
              </td>
              <td style={{ padding: '8px', borderBottom: '1px solid #333', textAlign: 'right' }}>
                {week.tasksCount}
              </td>
            </tr>
          ))}
          {weeklySummary.length === 0 && (
            <tr>
              <td colSpan={4} style={{ padding: '16px', textAlign: 'center', color: '#888' }}>
                No data for this year
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
});

// Main component
const YearView = ({ events, currentDate, weeklyGoals }) => {
  const year = moment(currentDate).year();
  const [hoveredTasks, setHoveredTasks] = useState(null);
  const tableContainerRef = useRef(null);
  const currentWeekRowRef = useRef(null);
  
  // Cache completed tasks by day for faster lookup
  const completedTasksByDay = useMemo(() => {
    const tasksByDay = {};
    events.forEach(event => {
      if (event.completed) {
        const dateKey = moment(event.start).format('YYYY-MM-DD');
        if (!tasksByDay[dateKey]) {
          tasksByDay[dateKey] = [];
        }
        tasksByDay[dateKey].push(event);
      }
    });
    return tasksByDay;
  }, [events]);
  
  // Count tasks per day for color calculation
  const taskCounts = useMemo(() => {
    const counts = {};
    Object.entries(completedTasksByDay).forEach(([dateKey, tasks]) => {
      counts[dateKey] = tasks.length;
    });
    return counts;
  }, [completedTasksByDay]);
  
  // Find max task count for color scaling
  const maxTaskCount = useMemo(() => {
    return Math.max(1, ...Object.values(taskCounts));
  }, [taskCounts]);
  
  // Generate year data - days and weeks
  const { yearData, weeks } = useMemo(() => {
    const firstDay = moment([year, 0, 1]);
    const lastDay = moment([year, 11, 31]);
    
    const startDate = moment(firstDay).startOf('week');
    const endDate = moment(lastDay).endOf('week');

    const days = [];
    const currentDay = startDate.clone();
    
    while (currentDay.isSameOrBefore(endDate, 'day')) {
      days.push({
        date: currentDay.clone(),
        isCurrentMonth: currentDay.month() === moment(currentDate).month(),
        isCurrentYear: currentDay.year() === year,
        maxTaskCount: maxTaskCount, // Pass down maxTaskCount to avoid prop drilling
      });
      currentDay.add(1, 'day');
    }

    // Group by week
    const weekGroups = [];
    for (let i = 0; i < days.length; i += 7) {
      weekGroups.push(days.slice(i, i + 7));
    }

    return { yearData: days, weeks: weekGroups };
  }, [year, currentDate, maxTaskCount]);
  
  // Generate weekly summary data with better caching
  const weeklySummary = useMemo(() => {
    // Create a map of week keys for fast lookup
    const weeklyEventCounts = {}; 
    
    // Pre-calculate week tasks in one pass
    Object.entries(completedTasksByDay).forEach(([dateKey, tasks]) => {
      const date = moment(dateKey);
      if (date.year() === year) {
        const weekKey = date.format('GGGG-ww');
        if (!weeklyEventCounts[weekKey]) {
          weeklyEventCounts[weekKey] = 0;
        }
        weeklyEventCounts[weekKey] += tasks.length;
      }
    });
    
    // Generate summary with pre-calculated data
    const summary = [];
    const processedWeeks = new Set();
    
    weeks.forEach(week => {
      const firstDay = week[0].date;
      if (firstDay.year() === year) {
        const weekKey = firstDay.format('GGGG-ww');
        
        if (!processedWeeks.has(weekKey)) {
          processedWeeks.add(weekKey);
          
          summary.push({
            weekKey,
            weekNumber: firstDay.isoWeek(),
            startDate: firstDay.format('MMM D'),
            endDate: moment(week[6].date).format('MMM D'),
            goal: weeklyGoals[weekKey] || '',
            tasksCount: weeklyEventCounts[weekKey] || 0,
            isCurrentWeek: moment().isSame(firstDay, 'week')
          });
        }
      }
    });
    
    return summary;
  }, [weeks, year, completedTasksByDay, weeklyGoals]);
  
  // Calculate month positions for labels
  const monthPositions = useMemo(() => {
    const positions = [];
    
    // Find the starting week for each month
    for (let month = 0; month < 12; month++) {
      const firstDayOfMonth = moment([year, month, 1]);
      
      // Find which week contains this first day
      let weekIndex = 0;
      let found = false;
      
      for (let i = 0; i < weeks.length; i++) {
        for (let j = 0; j < weeks[i].length; j++) {
          if (weeks[i][j].date.isSame(firstDayOfMonth, 'day')) {
            weekIndex = i;
            found = true;
            break;
          }
        }
        if (found) break;
      }
      
      positions.push({
        month,
        label: moment([year, month, 1]).format('MMM'),
        weekIndex,
      });
    }
    
    return positions;
  }, [weeks, year]);

  // Throttled mouse handlers to prevent excessive updates
  const handleMouseEnter = useCallback(
    throttle((e, dateKey, tasks) => {
      setHoveredTasks({ 
        date: dateKey, 
        tasks: tasks,
        position: { x: e.clientX, y: e.clientY }
      });
    }, 50), // 50ms throttle to improve performance
    []
  );
  
  const handleMouseLeave = useCallback(() => {
    setHoveredTasks(null);
  }, []);

  // Auto-scroll to current week
  useEffect(() => {
    if (tableContainerRef.current && currentWeekRowRef.current) {
      setTimeout(() => {
        currentWeekRowRef.current?.scrollIntoView({
          behavior: 'smooth',
          block: 'center'
        });
      }, 100); // Small delay to ensure render is complete
    }
  }, []);

  return (
    <div style={{
      padding: '1rem',
      background: '#121212',
      color: '#fff',
      fontFamily: 'sans-serif'
    }}>
      <h2 style={{ textAlign: 'center', marginBottom: '1.5rem' }}>
        {year} Activity
      </h2>
      
      {/* Month labels section */}
      <div style={{ display: 'flex', marginBottom: '0', marginLeft: '2rem', position: 'relative', height: '25px' }}>
        {monthPositions.map((monthData) => (
          <div 
            key={monthData.label} 
            style={{ 
              position: 'absolute',
              left: `${monthData.weekIndex * 16}px`,
              textAlign: 'center',
              fontWeight: moment(currentDate).month() === monthData.month ? 'bold' : 'normal',
              fontSize: '12px',
            }}
          >
            {monthData.label}
          </div>
        ))}
      </div>
      
      {/* Activity grid */}
      <div style={{ display: 'flex' }}>
        {/* Day of week labels */}
        <div style={{ marginRight: '0.5rem', display: 'flex', flexDirection: 'column', justifyContent: 'space-around' }}>
          {['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'].map(day => (
            <div key={day} style={{ height: '14px', fontSize: '10px', textAlign: 'right' }}>{day}</div>
          ))}
        </div>
        
        {/* Calendar grid with optimized rendering */}
        <div style={{ display: 'flex', flexWrap: 'nowrap', overflowX: 'auto' }}>
          {weeks.map((week, weekIndex) => (
            <div key={weekIndex} style={{ display: 'flex', flexDirection: 'column', margin: '0 1px' }}>
              {week.map((day) => {
                const dateKey = day.date.format('YYYY-MM-DD');
                const tasksForDay = completedTasksByDay[dateKey] || [];
                const taskCount = tasksForDay.length;
                
                return (
                  <ActivityCell 
                    key={dateKey}
                    day={day}
                    taskCount={taskCount}
                    tasksForDay={tasksForDay}
                    onMouseEnter={handleMouseEnter}
                    onMouseLeave={handleMouseLeave}
                  />
                );
              })}
            </div>
          ))}
        </div>
      </div>
      
      {/* Custom tooltip */}
      {hoveredTasks && (
        <div style={{
          position: 'fixed',
          top: hoveredTasks.position.y + 10,
          left: hoveredTasks.position.x + 10,
          backgroundColor: '#333',
          border: '1px solid #555',
          borderRadius: '4px',
          padding: '8px',
          zIndex: 1000,
          maxWidth: '300px',
          boxShadow: '0 2px 10px rgba(0,0,0,0.5)'
        }}>
          <div style={{ fontWeight: 'bold', marginBottom: '5px' }}>
            {moment(hoveredTasks.date).format('MMMM D, YYYY')}
          </div>
          <div style={{ fontSize: '12px' }}>
            {hoveredTasks.tasks.map(task => (
              <div key={task.id} style={{ margin: '3px 0', display: 'flex', alignItems: 'center' }}>
                <span style={{ 
                  display: 'inline-block', 
                  width: '8px', 
                  height: '8px', 
                  backgroundColor: 'rgba(0, 230, 0, 1.0)', 
                  borderRadius: '50%',
                  marginRight: '5px'
                }}></span>
                {task.title}
              </div>
            ))}
          </div>
        </div>
      )}
      
      {/* Legend */}
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', marginTop: '1.5rem', gap: '0.5rem' }}>
        <span>Less</span>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'transparent', border: '1px solid #333', borderRadius: '2px' }}></div>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'rgba(0, 100, 0, 0.2)', border: '1px solid #333', borderRadius: '2px' }}></div>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'rgba(0, 130, 0, 0.4)', border: '1px solid #333', borderRadius: '2px' }}></div>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'rgba(0, 160, 0, 0.6)', border: '1px solid #333', borderRadius: '2px' }}></div>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'rgba(0, 190, 0, 0.8)', border: '1px solid #333', borderRadius: '2px' }}></div>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'rgba(0, 230, 0, 1.0)', border: '1px solid #333', borderRadius: '2px' }}></div>
        <span>More</span>
      </div>
      
      <div style={{ textAlign: 'center', marginTop: '1rem', fontSize: '0.9rem', color: '#aaa' }}>
        {Object.values(completedTasksByDay).flat().filter(event => moment(event.start).year() === year).length} tasks completed in {year}
      </div>
      
      {/* Weekly summary section using memoized component */}
      <div style={{ marginTop: '2rem' }}>
        <h3 style={{ marginBottom: '1rem' }}>Weekly Summary</h3>
        <WeeklySummaryTable 
          weeklySummary={weeklySummary}
          tableContainerRef={tableContainerRef}
          currentWeekRowRef={currentWeekRowRef}
        />
      </div>
    </div>
  );
};

export default YearView;
