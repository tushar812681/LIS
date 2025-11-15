# Workflow Documentation

This directory contains comprehensive workflow diagrams for all LIS/LIMS processes.

## Core Process Workflows

1. **[Patient Registration](patient-registration.md)** - Complete patient onboarding flow
2. **[Sample Collection](sample-collection.md)** - Sample collection and labeling process
3. **[Test Processing](test-processing.md)** - From order to result entry
4. **[Quality Control](quality-control.md)** - IQC and EQC workflows
5. **[Result Verification](result-verification.md)** - Auto-verification and manual review
6. **[Report Generation](report-generation.md)** - Report creation and delivery
7. **[Billing](billing.md)** - Invoice generation and payment processing
8. **[NABL Compliance](nabl-compliance.md)** - Compliance and audit workflows

## User Flow Diagrams

- **[Lab Technician Flows](user-flows/lab-technician.md)**
- **[Pathologist Flows](user-flows/pathologist.md)**
- **[Front Desk Flows](user-flows/front-desk.md)**
- **[Lab Director Flows](user-flows/lab-director.md)**
- **[Patient Flows](user-flows/patient.md)**
- **[Admin Flows](user-flows/admin.md)**

## Reading the Diagrams

- **Rectangles**: Process steps
- **Diamonds**: Decision points
- **Cylinders**: Database operations
- **Clouds**: External services
- **Arrows**: Flow direction
- **Dashed lines**: Asynchronous operations
- **Parallel**: Concurrent processes

## Notation

```
→   Sequential flow
⇒   Event-driven flow
||  Parallel execution
◇   Decision point
⬡   Manual step
⬢   Automated step
```
