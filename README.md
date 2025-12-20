# Streaming API Messages

This document lists the message structures used in the streaming data analysis system.

## Input Message

### MetricEvent (from dc_metrics topic)
- `eventId`: UUID - Unique identifier for the event
- `hostId`: String - Identifier of the host
- `zone`: String - Zone where the host is located
- `timestamp`: long - Timestamp of the event
- `metric`: MetricType - Type of metric (enum: CPU_USAGE, MEM_USAGE, DISK_IO_READ, DISK_IO_WRITE, NET_IN, NET_OUT, CPU_TEMP)
- `value`: double - Metric value
- `unit`: String - Unit of measurement
- `tags`: Map<String, Object> - Additional tags

## Output Messages

### Alert (to dc_alerts topic) dc_alerts dc_host_status dc_sla_compliance dc_zone_load  dc_cpu_trend
- `alertId`: UUID - Unique identifier for the alert
- `hostId`: String - Identifier of the host
- `zone`: String - Zone where the host is located
- `timestamp`: long - Timestamp of the alert
- `metric`: MetricType - Type of metric that triggered the alert
- `value`: double - Actual metric value
- `threshold`: double - Threshold value that was exceeded
- `message`: String - Alert message

### HostStatus (to dc_host_status topic)
- `key`: String - Composite key in format "hostId:zone"
- `metric`: String - Metric type (e.g., "CPU_USAGE")
- `avg`: Double - Average value of the metric over the window

### SlaCompliance (to dc_sla_compliance topic)
- `hostId`: String - Identifier of the host
- `metric`: String - Metric type
- `violationPercentage`: Double - Percentage of time the metric violated SLA thresholds

### ZoneLoad (to dc_zone_load topic)
- `zone`: String - Zone identifier
- `totalCpuUsage`: Double - Total CPU usage across all hosts in the zone

### CpuTrend (to dc_cpu_trend topic)
- `hostId`: String - Identifier of the host
- `totalCpuUsage`: Double - Total CPU usage for the host over the hour
