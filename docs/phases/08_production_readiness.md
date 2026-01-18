# Phase 8: Production Readiness Checklists

**Goal:** Operational excellence.

## 1. Health Checks

- [ ] Implement Liveness Probes (`/health/live` - returns 200 if process running)
- [ ] Implement Readiness Probes (`/health/ready` - returns 200 if connected to DB/Redis)
- [ ] Configure Docker Healthcheck / K8s Probes

## 2. Autoscaling Integration

- [ ] Define CPU/Memory resource requests/limits
- [ ] Configure HPA (Horizontal Pod Autoscaler) based on CPU or Custom Metric (Queue Length)

## 3. Disaster Recovery

- [ ] Automate DB Backups (Wal-G / AWS RDS automated backups)
- [ ] Document Restore Procedure
- [ ] Chaos Testing: Randomly kill pods/containers and verify zero (or minimal) data loss

## 4. Final Verification

- [ ] Verify SLOs: 99.9% Availability, P99 < 500ms
