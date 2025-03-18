import React, { useState, useEffect } from "react";

const ClockWidget: React.FC = () => {
    const [date, setDate] = useState(new Date());
    const [sysInfo, setSysInfo] = useState({
        uptime: 0,
        cpu_usage: 0,
        total_memory: 0,
        used_memory: 0
    });

    useEffect(() => {
        const interval = setInterval(() => {
            setDate(new Date());
            fetchSysInfo();
        }, 1000);

        return () => clearInterval(interval);
    }, []);

    const fetchSysInfo = async () => {
        try {
            const response = await fetch("http://localhost:3001/sysinfo");
            const data = await response.json();
            setSysInfo(data);
        } catch (error) {
            console.error("Error fetching system info:", error);
        }
    };

    return (
        <div style={{
            border: "1px solid #ccc",
            padding: "10px",
            borderRadius: "8px",
            width: "300px",
            backgroundColor: "#282c34",
            color: "#ffffff",
            fontFamily: "Arial, sans-serif",
            textAlign: "center"
        }}>
            <h2>Clock & System Info</h2>
            <p>Local Time: {date.toLocaleTimeString()}</p>
            <p>UTC Time: {date.toUTCString()}</p>
            <p>ISO Format: {date.toISOString()}</p>
            <p>Timezone Offset: {date.getTimezoneOffset()} minutes</p>
            <p>Milliseconds: {date.getMilliseconds()}</p>
            <hr />
            <h3>System Info</h3>
            <p>Uptime: {sysInfo.uptime} sec</p>
            <p>CPU Usage: {sysInfo.cpu_usage.toFixed(2)}%</p>
            <p>Memory Usage: {sysInfo.used_memory} / {sysInfo.total_memory} KB</p>
        </div>
    );
};

export default ClockWidget;