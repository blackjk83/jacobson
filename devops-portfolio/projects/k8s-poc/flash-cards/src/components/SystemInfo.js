import React, { useState, useEffect } from 'react';
import './SystemInfo.css';

const SystemInfo = () => {
  const [systemInfo, setSystemInfo] = useState(null);
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchSystemInfo = async () => {
      try {
        setLoading(true);
        // First get public IP from external service
        const ipResponse = await fetch('https://api.ipify.org?format=json');
        const ipData = await ipResponse.json();

        // Then get system info from our backend
        const sysResponse = await fetch('/api/system-info', {
          headers: {
            'Accept': 'application/json',
            'Cache-Control': 'no-cache',
            'X-Real-IP': ipData.ip
          }
        });

        if (!sysResponse.ok) {
          throw new Error(`HTTP error! status: ${sysResponse.status}`);
        }

        const data = await sysResponse.json();
        console.log('System info:', data);
        setSystemInfo(data);
        setError(null);
      } catch (err) {
        console.error('Error fetching info:', err);
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchSystemInfo();
  }, []);

  if (loading) return <div className="system-info loading">Loading system information...</div>;
  if (error) return <div className="system-info error">Error: {error}</div>;
  if (!systemInfo) return null;

  return (
    <div className="system-info">
      <h2>System Information</h2>
      <div className="info-grid">
        <div className="info-section">
          <h3>Environment</h3>
          <div className="info-item">
            <label>Host OS:</label>
            <span>{systemInfo.os}</span>
          </div>
          <div className="info-item">
            <label>Architecture:</label>
            <span>{systemInfo.arch}</span>
          </div>
          <div className="info-item">
            <label>Environment:</label>
            <span>{systemInfo.containerized}</span>
          </div>
        </div>

        <div className="info-section">
          <h3>Container Details</h3>
          <div className="info-item">
            <label>Pod Name:</label>
            <span>{systemInfo.podName}</span>
          </div>
          <div className="info-item">
            <label>Node Name:</label>
            <span>{systemInfo.nodeName}</span>
          </div>
          <div className="info-item">
            <label>Pod IP:</label>
            <span>{systemInfo.podIp}</span>
          </div>
          <div className="info-item">
            <label>Host IP:</label>
            <span>{systemInfo.hostIp}</span>
          </div>
          <div className="info-item">
            <label>Namespace:</label>
            <span>{systemInfo.namespace}</span>
          </div>
        </div>

        <div className="info-section highlight">
          <h3>Visitor Information</h3>
          <div className="info-item">
            <label>IP Address:</label>
            <span>{systemInfo.visitor.ip}</span>
          </div>
          <div className="info-item">
            <label>Browser:</label>
            <span>{systemInfo.visitor.browser}</span>
          </div>
          <div className="info-item">
            <label>Location:</label>
            <span>{systemInfo.visitor.location}</span>
          </div>
          <div className="info-item">
            <label>Timezone:</label>
            <span>{systemInfo.visitor.timezone}</span>
          </div>
          <div className="info-item special">
            <label>Visitor #:</label>
            <span>{systemInfo.visitor.visitorNumber}</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SystemInfo; 