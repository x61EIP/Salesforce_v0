

# Status Page Active Response Coordination (SPARC)

### Salesforce Apex Actions & Infrastructure Automation

## Project Overview

The **Status Page Active Response Coordination (SPARC)** is a proactive customer-experience and field-service workflow solution designed for telecommunications environments. 

SPARC automatically partitions affected subscribers, stages high-priority field service dispatches, and broadcasts proactive notifications, effectively neutralizing inbound support ticket spikes and lowering Mean Time to Repair (MTTR).

---

## Architectural Workflow

```
       [ Network Telemetry / Edge API ]
                       │
                       ▼ (Webhook Payload)
 ┌───────────────────────────────────────────────┐
 │       1. Salesforce Apex REST Endpoint        │
 └──────────────────────┬────────────────────────┘
                        │ (Instantiates Platform Event)
                        ▼
 ┌───────────────────────────────────────────────┐
 │      2. Target SQL || SOQL Geolocation Query  │
 └──────────────────────┬────────────────────────┘
                        │ (Isolates Impacted Customer Node IDs)
                        ▼
 ┌───────────────────────────────────────────────┐
 │        3. Apex Routing Control                │
 └──────────────────────┬────────────────────────┘
                        │
         ┌──────────────┴──────────────┐
         ▼ (Internal Routing)          ▼ (External Routing)
 ┌──────────────────────────────┐┌──────────────────────────────┐
 │ Salesforce Field Service     ││ Digital Engagement Gateway   │
 │ • Stages Work Orders         ││ • Dispatches SMS Alerts      │
 │ • Allocates Assets/Route     ││ • "We are aware & on it"     │
 └──────────────────────────────┘└──────────────────────────────┘

```

---

## Measurable Business Impact

* **Support Center Mitigation:** Proactively alerting affected subscribers via SMS drops customer service call queues to near zero for anticipated localized events.
* **Optimized Dispatch (MTTR):** Technicians are staged and routed with appropriate equipment before the asset undergoes critical failure, ensuring higher first-time-fix rates.
* **Controlled Change Management:** Following standard DevOps practices, all metadata modifications, Flow structures, and Apex source changes are tracked with Git.



