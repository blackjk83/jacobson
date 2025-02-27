const express = require('express');
const os = require('os');
const geoip = require('geoip-lite');
const useragent = require('express-useragent');
const axios = require('axios');

const app = express();
const port = 3000;

// Add logging middleware
app.use((req, res, next) => {
  console.log(`${new Date().toISOString()} - ${req.method} ${req.url}`);
  next();
});

// Middleware
app.use(useragent.express());

// Get real IP helper
const getRealIP = (req) => {
  return req.headers['x-real-ip'] || 
         req.headers['x-forwarded-for']?.split(',')[0] || 
         req.connection.remoteAddress.replace('::ffff:', '');
};

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({ status: 'ok' });
});

// System info endpoint
app.get('/api/system-info', async (req, res) => {
  try {
    const ip = getRealIP(req);
    const geo = geoip.lookup(ip);
    const ua = useragent.parse(req.headers['user-agent']);

    // Get additional network info
    const networkInterfaces = os.networkInterfaces();
    const interfaces = Object.keys(networkInterfaces)
      .filter(iface => !iface.includes('lo'))
      .reduce((acc, iface) => {
        acc[iface] = networkInterfaces[iface]
          .filter(details => details.family === 'IPv4')
          .map(details => details.address);
        return acc;
      }, {});

    const systemInfo = {
      os: `${os.type()} ${os.release()}`,
      arch: os.arch(),
      platform: os.platform(),
      containerized: process.env.KUBERNETES_SERVICE_HOST ? 'Kubernetes' : 'Docker',
      podName: process.env.POD_NAME || 'N/A',
      nodeName: process.env.NODE_NAME || 'N/A',
      podIp: process.env.POD_IP || 'N/A',
      hostIp: process.env.HOST_IP || 'N/A',
      namespace: process.env.NAMESPACE || 'N/A',
      network: {
        interfaces,
        hostname: os.hostname(),
        uptime: os.uptime()
      },
      visitor: {
        ip,
        browser: `${ua.browser} ${ua.version} on ${ua.os}`,
        location: geo ? `${geo.city || 'Unknown City'}, ${geo.country}` : 'Unknown Location',
        timezone: geo?.timezone || 'Unknown',
        isp: geo?.org || 'Unknown ISP',
        visitorNumber: global.visitorCounter = (global.visitorCounter || 0) + 1
      }
    };

    console.log('Sending system info:', systemInfo);
    res.json(systemInfo);
  } catch (error) {
    console.error('Error getting system info:', error);
    res.status(500).json({ error: 'Failed to get system information' });
  }
});

// Start server
app.listen(port, '0.0.0.0', () => {
  console.log(`Server running on http://0.0.0.0:${port}`);
}); 