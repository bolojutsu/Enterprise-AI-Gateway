import { useState, useEffect } from 'react';

function App() {
  const [stats, setStats] = useState({ status: "Offline", active_models: [], total_requests_processed: 0 });
  const [loading, setLoading] = useState(false);

  // 1. Fetch live stats from the Rust Axum server (Port 3000)
  const fetchStats = async () => {
    try {
      const response = await fetch('http://127.0.0.1:3000/stats');
      const data = await response.json();
      setStats(data);
    } catch (error) {
      console.error("Gateway is likely offline:", error);
    }
  };

  useEffect(() => {
    fetchStats();
    // Refresh stats every 5 seconds
    const interval = setInterval(fetchStats, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div style={styles.container}>
      <header style={styles.header}>
        <h1>Enterprise AI Gateway</h1>
        <div style={{ color: stats.status === "Running" ? "#4ade80" : "#f87171" }}>
          ● {stats.status}
        </div>
      </header>

      <main style={styles.main}>
        <div style={styles.card}>
          <h3>System Metrics</h3>
          <p>Total Processed: <strong>{stats.total_requests_processed}</strong></p>
          <p>Models Online: <strong>{stats.active_models.join(", ")}</strong></p>
        </div>

        <div style={styles.card}>
          <h3>Workflow Trigger</h3>
          <p style={{ fontSize: '0.9rem', color: '#94a3b8' }}>
            To see the full flow, run your <b>Python Agent</b> now. 
            The stats above will update when Rust processes the gRPC call.
          </p>
        </div>
      </main>
    </div>
  );
}

const styles = {
  container: { minHeight: '100vh', backgroundColor: '#0f172a', color: 'white', fontFamily: 'sans-serif', padding: '40px' },
  header: { display: 'flex', justifyContent: 'space-between', alignItems: 'center', borderBottom: '1px solid #1e293b', paddingBottom: '20px' },
  main: { display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '20px', marginTop: '40px' },
  card: { backgroundColor: '#1e293b', padding: '20px', borderRadius: '12px', boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1)' }
};

export default App;