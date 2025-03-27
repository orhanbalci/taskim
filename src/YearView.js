import React, { useMemo, useState, useRef, useEffect } from 'react';
import moment from 'moment';

const YearView = ({ events, currentDate, weeklyGoals }) => {
  const year = moment(currentDate).year();
  const [hoveredTasks, setHoveredTasks] = useState(null);
  const tableContainerRef = useRef(null);
  const currentWeekRowRef = useRef(null);
  
  // Generate all days for the year
  const yearData = useMemo(() => {
    const firstDay = moment([year, 0, 1]);
    const lastDay = moment([year, 11, 31]);
    
    // Start from the first Sunday before or on January 1st
    const startDate = moment(firstDay).startOf('week');
    // End on the last Saturday after or on December 31st
    const endDate = moment(lastDay).endOf('week');

    const days = [];
    const currentDay = startDate.clone();
    
    // Initialize weeks and days
    while (currentDay.isSameOrBefore(endDate, 'day')) {
      days.push({
        date: currentDay.clone(),
        isCurrentMonth: currentDay.month() === moment(currentDate).month(),
        isCurrentYear: currentDay.year() === year,
      });
      currentDay.add(1, 'day');
    }

    return days;
  }, [year, currentDate]);
  
  // Count completed tasks per day
  const taskCounts = useMemo(() => {
    const counts = {};
    events.forEach(event => {
      if (event.completed) {
        const dateKey = moment(event.start).format('YYYY-MM-DD');
        counts[dateKey] = (counts[dateKey] || 0) + 1;
      }
    });
    return counts;
  }, [events]);
  
  // Find max tasks for color scaling
  const maxTaskCount = useMemo(() => {
    return Math.max(1, ...Object.values(taskCounts));
  }, [taskCounts]);
  
  // Group days by week number
  const weeks = useMemo(() => {
    const result = [];
    let currentWeek = [];
    
    yearData.forEach((day, index) => {
      currentWeek.push(day);
      
      // Start a new week after every 7 days
      if (currentWeek.length === 7) {
        result.push(currentWeek);
        currentWeek = [];
      }
    });
    
    return result;
  }, [yearData]);
  
  // Function to determine cell color based on task count
  const getCellColor = (date) => {
    const dateKey = date.format('YYYY-MM-DD');
    const count = taskCounts[dateKey] || 0;
    
    if (count === 0) return 'transparent';
    
    // Calculate color intensity (0-1)
    const intensity = count / maxTaskCount;
    
    if (intensity < 0.2) return 'rgba(0, 100, 0, 0.2)';
    if (intensity < 0.4) return 'rgba(0, 130, 0, 0.4)';
    if (intensity < 0.6) return 'rgba(0, 160, 0, 0.6)';
    if (intensity < 0.8) return 'rgba(0, 190, 0, 0.8)';
    return 'rgba(0, 230, 0, 1.0)';
  };
  
  // Generate weekly summary data
  const weeklySummary = useMemo(() => {
    const summary = [];
    
    // Group weeks by week number
    const weeklyEvents = {};
    
    // Process events to group them by week
    events.forEach(event => {
      if (event.completed && moment(event.start).year() === year) {
        const weekKey = moment(event.start).format('GGGG-ww');
        if (!weeklyEvents[weekKey]) {
          weeklyEvents[weekKey] = [];
        }
        weeklyEvents[weekKey].push(event);
      }
    });
    
    // Create summary for each week
    const processedWeeks = new Set();
    weeks.forEach(week => {
      const firstDay = week[0].date;
      if (firstDay.year() === year) {
        const weekKey = firstDay.format('GGGG-ww');
        
        // Avoid duplicate weeks
        if (!processedWeeks.has(weekKey)) {
          processedWeeks.add(weekKey);
          
          const weekNumber = firstDay.isoWeek();
          const goal = weeklyGoals[weekKey] || '';
          const completedTasks = weeklyEvents[weekKey] || [];
          
          summary.push({
            weekKey,
            weekNumber,
            startDate: firstDay.format('MMM D'),
            endDate: moment(week[6].date).format('MMM D'),
            goal,
            tasksCount: completedTasks.length,
            isCurrentWeek: moment().isSame(firstDay, 'week')
          });
        }
      }
    });
    
    return summary;
  }, [weeks, year, events, weeklyGoals]);
  
  // Get tasks completed on a specific date
  const getCompletedTasksForDate = (date) => {
    const dateStr = date.format('YYYY-MM-DD');
    return events.filter(event => 
      event.completed && 
      moment(event.start).format('YYYY-MM-DD') === dateStr
    );
  };
  
  // Month labels for the top of the graph
  const monthLabels = useMemo(() => {
    const months = [];
    for (let i = 0; i < 12; i++) {
      months.push(moment([year, i, 1]).format('MMM'));
    }
    return months;
  }, [year]);
  
  // Calculate month positions for proper alignment
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

  // Auto-scroll to current week after render
  useEffect(() => {
    if (tableContainerRef.current && currentWeekRowRef.current) {
      // Scroll to current week with a small offset to show a bit of context above
      currentWeekRowRef.current.scrollIntoView({
        behavior: 'smooth',
        block: 'center'
      });
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
      
      <div style={{ display: 'flex', marginBottom: '0', marginLeft: '2rem', position: 'relative', height: '25px' }}>
        {/* Month labels with absolute positioning for alignment */}
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
      
      <div style={{ display: 'flex' }}>
        {/* Day of week labels */}
        <div style={{ marginRight: '0.5rem', display: 'flex', flexDirection: 'column', justifyContent: 'space-around' }}>
          {['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'].map(day => (
            <div key={day} style={{ height: '14px', fontSize: '10px', textAlign: 'right' }}>{day}</div>
          ))}
        </div>
        
        {/* Calendar grid */}
        <div style={{ display: 'flex', flexWrap: 'nowrap', overflowX: 'auto' }}>
          {weeks.map((week, weekIndex) => (
            <div key={weekIndex} style={{ display: 'flex', flexDirection: 'column', margin: '0 1px' }}>
              {week.map((day) => {
                const dateKey = day.date.format('YYYY-MM-DD');
                const tasksForDay = getCompletedTasksForDate(day.date);
                const taskCount = tasksForDay.length;
                
                return (
                  <div 
                    key={dateKey}
                    onMouseEnter={(e) => taskCount > 0 && setHoveredTasks({ 
                      date: dateKey, 
                      tasks: tasksForDay,
                      position: { x: e.clientX, y: e.clientY }
                    })}
                    onMouseLeave={() => setHoveredTasks(null)}
                    style={{
                      width: '14px',
                      height: '14px',
                      margin: '1px',
                      backgroundColor: getCellColor(day.date),
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
              })}
            </div>
          ))}
        </div>
      </div>
      
      {/* Custom tooltip for hovered tasks */}
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
        {events.filter(event => event.completed && moment(event.start).year() === year).length} tasks completed in {year}
      </div>
      
      {/* Weekly summary table */}
      <div style={{ marginTop: '2rem' }}>
        <h3 style={{ marginBottom: '1rem' }}>Weekly Summary</h3>
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
      </div>
    </div>
  );
};

export default YearView;
