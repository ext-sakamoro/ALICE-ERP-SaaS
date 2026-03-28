# ALICE-ERP-SaaS

Enterprise Resource Planning API — inventory, production scheduling, payroll (HRM), and learning management (LMS) via the ALICE SaaS architecture.

## Architecture

```
Client
  └─ API Gateway (:8145) — JWT auth, rate limiting, proxy
       └─ Core Engine (:9145) — inventory, production, HRM/payroll, LMS
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /health | Health check |
| GET/POST | /api/v1/erp/inventory | Inventory query and update |
| GET/POST | /api/v1/erp/production | Production schedule and orders |
| GET/POST | /api/v1/erp/payroll | Payroll processing (HRM) |
| GET/POST | /api/v1/erp/courses | Learning management system (LMS) |
| GET | /api/v1/erp/stats | Request statistics |

## Quick Start

```bash
cd services/core-engine && cargo run
cd services/api-gateway && cargo run
```

## License

AGPL-3.0-or-later
