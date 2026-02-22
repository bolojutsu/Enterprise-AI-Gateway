import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer, Cell } from 'recharts';
import { Activity, Trophy, Cpu, Zap, ShieldCheck } from 'lucide-react';
import './index.css';

const BAR_COLORS = ['#8b5cf6', '#3b82f6', '#10b981', '#f59e0b'];

function App() {
  const [leaderboard, setLeaderboard] = useState([]);
  const [stats, setStats] = useState({ total_requests_processed: 0 });
  const [logs, setLogs] = useState([]);

  const refreshData = async () => {
    try {
      const lbRes = await axios.get('http://localhost:3000/leaderboard');
      const statsRes = await axios.get('http://localhost:3000/stats');
      const logsRes = await axios.get('http://localhost:3000/logs');
      setLeaderboard(lbRes.data.leaderboard);
      setStats(statsRes.data);
      setLogs(logsRes.data.logs);
    } catch (e) {
      console.warn("Gateway is offline. Check if Rust server is running on port 3000.");
    }
  };

  useEffect(() => {
    refreshData();
    const timer = setInterval(refreshData, 3000);
    return () => clearInterval(timer);
  }, []);

  return (
    <div className="app-container">
      <header className="main-header">
        <div className="branding">
          <Cpu className="brand-icon" size={32} />
          <h1>AI GATEWAY <span className="accent">DASHBOARD</span></h1>
        </div>
        <div className="system-status">
          <div className="badge">
            <Activity className="pulse-icon" size={14} /> SYSTEM LIVE
          </div>
          <div className="stat-counter">
            <span className="count">{stats.total_requests_processed}</span>
            <span className="label">TOTAL REQS</span>
          </div>
        </div>
      </header>

      <main className="dashboard-layout">
        <section className="chart-section card">
          <div className="card-title">
            <Trophy size={18} className="icon-gold" />
            <h2>Model Race Leaderboard</h2>
          </div>
          <div className="chart-container">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={leaderboard}>
                <XAxis dataKey="winner" axisLine={false} tickLine={false} tick={{fill: '#64748b'}} />
                <Tooltip 
                  cursor={{fill: 'rgba(255,255,255,0.03)'}}
                  contentStyle={{backgroundColor: '#1e293b', border: 'none', borderRadius: '8px'}}
                />
                <Bar dataKey="win_count" radius={[4, 4, 0, 0]}>
                  {leaderboard.map((_, index) => (
                    <Cell key={`cell-${index}`} fill={BAR_COLORS[index % BAR_COLORS.length]} />
                  ))}
                </Bar>
              </BarChart>
            </ResponsiveContainer>
          </div>
        </section>

        <aside className="sidebar">
          <div className="info-card card">
            <Zap size={20} className="icon-blue" />
            <h3>Active Pattern</h3>
            <p><strong>Race Mode:</strong> OpenAI vs Gemini vs Claude vs Grok.</p>
            <p className="description">The gateway selects the first provider to respond, minimizing latency for the Python Agent.</p>
          </div>

          <div className="security-card card">
            <ShieldCheck size={20} className="icon-green" />
            <h3>Grounded Research</h3>
            <p>Web-access via <strong>Tavily</strong> is enabled for queries tagged as 'research'.</p>
          </div>
        </aside>


        <section className="logs-section card">
          <div className="card-title">
            <Activity size={18} className="icon-blue" />
            <h2>Live Request Feed</h2>
          </div>
          <div className="logs-list">
            {logs.map((log, i) => (
              <div key={i} className="log-item">
                <div className="log-meta">
                  <span className="log-winner">{log.winner}</span>
                  <span className="log-latency">{log.latency}ms</span>
                </div>
                <p className="log-prompt"><strong>Prompt:</strong> {log.prompt}</p>
                <p className="log-response text-dim">{log.response.substring(0, 120)}...</p>
              </div>
            ))}
          </div>
        </section>







      </main>
    </div>
  );
}

export default App;