# User Flow Diagrams - All Personas

## Table of Contents
1. [Patient (Self-Service)](#1-patient-self-service)
2. [Front Desk Staff](#2-front-desk-staff)
3. [Phlebotomist / Sample Collector](#3-phlebotomist--sample-collector)
4. [Lab Technician](#4-lab-technician)
5. [Pathologist](#5-pathologist)
6. [Lab Director / Manager](#6-lab-director--manager)
7. [Quality Manager](#7-quality-manager)
8. [Billing Staff](#8-billing-staff)

---

# 1. Patient (Self-Service)

## 1.1 Patient Registration via WhatsApp

```mermaid
flowchart TD
    START([Patient Starts WhatsApp Chat]) --> GREETING[Receive Welcome Message]
    GREETING --> MENU{Select Option}

    MENU -->|New User| REGISTER[Start Registration]
    MENU -->|Existing User| LOGIN[Provide Mobile Number]

    REGISTER --> NAME[Share Full Name]
    NAME --> DOB[Share Date of Birth]
    DOB --> GENDER[Select Gender]
    GENDER --> CITY[Share City/Location]
    CITY --> CONFIRM[Review Details]

    CONFIRM --> VALID{Details<br/>Correct?}
    VALID -->|No| NAME
    VALID -->|Yes| CREATE[Create Account]
    CREATE --> MRN[Receive Patient ID]
    MRN --> MAIN_MENU[Show Main Menu]

    LOGIN --> VERIFY[System Verifies Mobile]
    VERIFY --> FOUND{Patient<br/>Found?}
    FOUND -->|Yes| MAIN_MENU
    FOUND -->|No| REGISTER

    MAIN_MENU --> OPTIONS{Choose Action}
    OPTIONS -->|Book Test| BOOK_FLOW[Go to Test Booking]
    OPTIONS -->|View Reports| VIEW_REPORTS[Show Available Reports]
    OPTIONS -->|Track Order| TRACK_ORDER[Enter Order Number]
    OPTIONS -->|Help| HELP[Show Help Menu]
```

## 1.2 Test Booking via Web Portal

```mermaid
flowchart TD
    START([Patient Opens Portal]) --> AUTH{Logged In?}
    AUTH -->|No| LOGIN[Login/Register]
    AUTH -->|Yes| DASHBOARD[View Dashboard]

    LOGIN --> DASHBOARD
    DASHBOARD --> BOOK[Click 'Book Test']
    BOOK --> SEARCH[Search for Tests]

    SEARCH --> SELECT[Select Tests/Profiles]
    SELECT --> MORE{Add More<br/>Tests?}
    MORE -->|Yes| SEARCH
    MORE -->|No| CART[View Cart]

    CART --> RECOMMENDATIONS{View AI<br/>Recommendations?}
    RECOMMENDATIONS -->|Yes| SUGGEST[See Suggested Tests]
    SUGGEST --> ADD{Add to<br/>Cart?}
    ADD -->|Yes| CART
    ADD -->|No| PROCEED
    RECOMMENDATIONS -->|No| PROCEED[Proceed to Checkout]

    PROCEED --> SCHEDULE[Select Date & Time]
    SCHEDULE --> LOCATION{Sample<br/>Collection?}
    LOCATION -->|Home Visit| ADDRESS[Enter Address]
    LOCATION -->|Lab Visit| LAB[Select Lab Location]

    ADDRESS --> PAYMENT_SUMMARY
    LAB --> PAYMENT_SUMMARY[Review Order Summary]

    PAYMENT_SUMMARY --> DISCOUNT{Apply<br/>Discount?}
    DISCOUNT -->|Yes| COUPON[Enter Coupon Code]
    COUPON --> PAYMENT_SUMMARY
    DISCOUNT -->|No| PAYMENT[Select Payment Method]

    PAYMENT --> PAY_METHOD{Payment<br/>Type?}
    PAY_METHOD -->|UPI| UPI[Pay via UPI]
    PAY_METHOD -->|Card| CARD[Enter Card Details]
    PAY_METHOD -->|Wallet| WALLET[Select Wallet]

    UPI --> PROCESS[Process Payment]
    CARD --> PROCESS
    WALLET --> PROCESS

    PROCESS --> SUCCESS{Payment<br/>Success?}
    SUCCESS -->|Yes| CONFIRM[Order Confirmed]
    SUCCESS -->|No| RETRY[Retry Payment]
    RETRY --> PAYMENT

    CONFIRM --> RECEIPT[Receive Confirmation]
    RECEIPT --> TRACK[Track Order Status]
    TRACK --> END([End])
```

## 1.3 View & Download Reports

```mermaid
flowchart TD
    START([Patient Receives Notification]) --> CHANNEL{Notification<br/>Channel?}

    CHANNEL -->|WhatsApp| WA[Click WhatsApp Link]
    CHANNEL -->|SMS| SMS[Click SMS Link]
    CHANNEL -->|Email| EMAIL[Click Email Link]
    CHANNEL -->|Portal| PORTAL[Login to Portal]

    WA --> AUTH{Authenticated?}
    SMS --> AUTH
    EMAIL --> AUTH
    PORTAL --> REPORTS

    AUTH -->|No| OTP[Enter OTP]
    AUTH -->|Yes| REPORTS[View Reports List]
    OTP --> VERIFY{OTP<br/>Valid?}
    VERIFY -->|No| OTP
    VERIFY -->|Yes| REPORTS

    REPORTS --> SELECT[Select Report]
    SELECT --> VIEW[View Report Online]

    VIEW --> ACTIONS{Choose Action}
    ACTIONS -->|Download PDF| DOWNLOAD[Download Report]
    ACTIONS -->|Share| SHARE[Share with Doctor]
    ACTIONS -->|Print| PRINT[Print Report]
    ACTIONS -->|Close| END

    DOWNLOAD --> SAVE[Save to Device]
    SHARE --> SELECT_CONTACT[Select Contact Method]
    SELECT_CONTACT --> SHARE_EMAIL{Share Via?}
    SHARE_EMAIL -->|Email| SEND_EMAIL[Send via Email]
    SHARE_EMAIL -->|WhatsApp| SEND_WA[Share on WhatsApp]
    PRINT --> PRINT_PREVIEW[Show Print Preview]
    PRINT_PREVIEW --> PRINT_DOC[Print Document]

    SAVE --> END([End])
    SEND_EMAIL --> END
    SEND_WA --> END
    PRINT_DOC --> END
```

---

# 2. Front Desk Staff

## 2.1 Patient Check-in & Order Creation

```mermaid
flowchart TD
    START([Patient Arrives]) --> GREET[Greet Patient]
    GREET --> SEARCH[Search Patient Record]

    SEARCH --> EXISTS{Patient<br/>Exists?}
    EXISTS -->|Yes| VERIFY[Verify Identity]
    EXISTS -->|No| REGISTER[Register New Patient]

    REGISTER --> COLLECT_INFO[Collect Demographics]
    COLLECT_INFO --> PHOTO_ID{Has Photo<br/>ID?}
    PHOTO_ID -->|Yes| AADHAAR{Aadhaar<br/>Verification?}
    PHOTO_ID -->|No| MANUAL_ENTRY[Manual Entry]

    AADHAAR -->|Yes| SEND_OTP[Send Aadhaar OTP]
    SEND_OTP --> VERIFY_OTP{OTP<br/>Valid?}
    VERIFY_OTP -->|Yes| AUTO_FILL[Auto-fill Details]
    VERIFY_OTP -->|No| MANUAL_ENTRY
    AADHAAR -->|No| MANUAL_ENTRY

    AUTO_FILL --> ADDRESS[Capture Address]
    MANUAL_ENTRY --> ADDRESS
    ADDRESS --> CONTACT[Capture Contact Details]
    CONTACT --> SAVE_PATIENT[Save Patient Record]
    SAVE_PATIENT --> PRINT_LABEL[Print Patient Label]

    VERIFY --> ORDER_CREATE[Create New Order]
    PRINT_LABEL --> ORDER_CREATE

    ORDER_CREATE --> PRESCRIPTION{Has<br/>Prescription?}
    PRESCRIPTION -->|Yes| SCAN_SCRIPT[Scan Prescription]
    SCAN_SCRIPT --> SELECT_TESTS
    PRESCRIPTION -->|No| SELECT_TESTS[Select Tests Manually]

    SELECT_TESTS --> SEARCH_TEST[Search Test Catalog]
    SEARCH_TEST --> ADD_TEST[Add Tests to Order]
    ADD_TEST --> MORE{More<br/>Tests?}
    MORE -->|Yes| SEARCH_TEST
    MORE -->|No| PRIORITY[Set Priority]

    PRIORITY --> PHYSICIAN{Add Referring<br/>Physician?}
    PHYSICIAN -->|Yes| DOCTOR_INFO[Enter Doctor Details]
    PHYSICIAN -->|No| CALCULATE
    DOCTOR_INFO --> CALCULATE[Calculate Total]

    CALCULATE --> INSURANCE{Insurance<br/>Claim?}
    INSURANCE -->|Yes| VERIFY_INS[Verify Coverage]
    VERIFY_INS --> COPAY{Co-pay<br/>Required?}
    COPAY -->|Yes| APPLY_COPAY[Apply Co-payment]
    COPAY -->|No| COLLECT_PAYMENT
    INSURANCE -->|No| DISCOUNT{Apply<br/>Discount?}
    DISCOUNT -->|Yes| ENTER_DISC[Enter Discount]
    ENTER_DISC --> COLLECT_PAYMENT
    DISCOUNT -->|No| COLLECT_PAYMENT[Collect Payment]
    APPLY_COPAY --> COLLECT_PAYMENT

    COLLECT_PAYMENT --> PAY_METHOD{Payment<br/>Method?}
    PAY_METHOD -->|Cash| CASH_PAYMENT[Accept Cash]
    PAY_METHOD -->|Card| CARD_PAYMENT[Process Card]
    PAY_METHOD -->|UPI| UPI_PAYMENT[Scan UPI QR]
    PAY_METHOD -->|Credit| CREDIT_ACCOUNT[Add to Credit]

    CASH_PAYMENT --> PRINT_RECEIPT
    CARD_PAYMENT --> PRINT_RECEIPT
    UPI_PAYMENT --> PRINT_RECEIPT
    CREDIT_ACCOUNT --> PRINT_RECEIPT[Print Receipt]

    PRINT_RECEIPT --> SAMPLE_COLLECTION{Sample<br/>Collection?}
    SAMPLE_COLLECTION -->|Now| DIRECT_TO_PHLEBOTOMY[Direct to Phlebotomy]
    SAMPLE_COLLECTION -->|Later| SCHEDULE[Schedule Appointment]

    DIRECT_TO_PHLEBOTOMY --> END([End])
    SCHEDULE --> END
```

## 2.2 Handling Patient Queries

```mermaid
flowchart TD
    START([Patient Approaches Desk]) --> QUERY_TYPE{Query Type?}

    QUERY_TYPE -->|Report Status| CHECK_STATUS[Check Order Status]
    QUERY_TYPE -->|Report Collection| COLLECT_REPORT[Verify Patient Identity]
    QUERY_TYPE -->|Payment Issue| PAYMENT_QUERY[Check Payment Records]
    QUERY_TYPE -->|Test Information| TEST_INFO[Search Test Details]

    CHECK_STATUS --> SYSTEM[Look up in System]
    SYSTEM --> STATUS{Order<br/>Status?}
    STATUS -->|Pending| INFORM_PENDING[Inform TAT]
    STATUS -->|Ready| INFORM_READY[Notify Report Ready]
    STATUS -->|Delayed| ESCALATE[Escalate to Supervisor]

    COLLECT_REPORT --> VERIFY_ID{Valid<br/>ID?}
    VERIFY_ID -->|Yes| CHECK_READY{Report<br/>Ready?}
    VERIFY_ID -->|No| REQUEST_ID[Request Valid ID]
    REQUEST_ID --> VERIFY_ID

    CHECK_READY -->|Yes| PRINT_REPORT[Print Report]
    CHECK_READY -->|No| INFORM_PENDING

    PRINT_REPORT --> SIGNATURE[Obtain Acknowledgment]
    SIGNATURE --> HAND_OVER[Hand Over Report]

    PAYMENT_QUERY --> CHECK_PAYMENT[Check Payment Status]
    CHECK_PAYMENT --> PAID{Already<br/>Paid?}
    PAID -->|Yes| SHOW_RECEIPT[Show Receipt]
    PAID -->|No| COLLECT_DUE[Collect Payment]

    TEST_INFO --> EXPLAIN[Explain Test Details]
    EXPLAIN --> PROVIDE_PREP[Provide Preparation Instructions]

    INFORM_PENDING --> END([End])
    INFORM_READY --> END
    ESCALATE --> END
    HAND_OVER --> END
    SHOW_RECEIPT --> END
    COLLECT_DUE --> END
    PROVIDE_PREP --> END
```

---

# 3. Phlebotomist / Sample Collector

## 3.1 Sample Collection Process

```mermaid
flowchart TD
    START([Receive Patient]) --> CHECK_ORDER[Verify Order]
    CHECK_ORDER --> VALID{Order<br/>Valid?}
    VALID -->|No| CONTACT_DESK[Contact Front Desk]
    VALID -->|Yes| VERIFY_PATIENT[Verify Patient Identity]

    VERIFY_PATIENT --> MATCH{Identity<br/>Matches?}
    MATCH -->|No| CONTACT_DESK
    MATCH -->|Yes| CHECK_PREP[Check Preparation Requirements]

    CHECK_PREP --> FASTING{Fasting<br/>Required?}
    FASTING -->|Yes| FASTED{Patient<br/>Fasted?}
    FASTED -->|No| INFORM_RESCHEDULE[Inform Need to Reschedule]
    INFORM_RESCHEDULE --> END_RESCHEDULE([End - Reschedule])
    FASTED -->|Yes| PROCEED_COLLECTION
    FASTING -->|No| PROCEED_COLLECTION[Proceed with Collection]

    PROCEED_COLLECTION --> BARCODE[Generate Sample Barcode]
    BARCODE --> PRINT_LABEL[Print Sample Label]
    PRINT_LABEL --> PREPARE[Prepare Collection Materials]

    PREPARE --> SELECT_SITE[Select Venipuncture Site]
    SELECT_SITE --> SANITIZE[Sanitize Site]
    SANITIZE --> COLLECT[Collect Blood Sample]

    COLLECT --> SUCCESS{Collection<br/>Successful?}
    SUCCESS -->|No| RETRY{Retry<br/>Possible?}
    RETRY -->|Yes| SELECT_SITE
    RETRY -->|No| INFORM_DIFFICULT[Inform Supervisor]
    INFORM_DIFFICULT --> END_DIFFICULT([End - Need Assistance])

    SUCCESS -->|Yes| LABEL_TUBE[Apply Label to Tube]
    LABEL_TUBE --> VISUAL_INSPECT[Visual Inspection]

    VISUAL_INSPECT --> QUALITY{Sample<br/>Quality OK?}
    QUALITY -->|No| REJECT_REASON{Rejection<br/>Reason?}
    REJECT_REASON -->|Hemolyzed| MARK_HEMOLYZED
    REJECT_REASON -->|Clotted| MARK_CLOTTED
    REJECT_REASON -->|Insufficient| MARK_INSUFFICIENT

    MARK_HEMOLYZED --> REJECT_SAMPLE[Mark Sample as Rejected]
    MARK_CLOTTED --> REJECT_SAMPLE
    MARK_INSUFFICIENT --> REJECT_SAMPLE

    REJECT_SAMPLE --> RECOLLECT{Recollect<br/>Now?}
    RECOLLECT -->|Yes| SELECT_SITE
    RECOLLECT -->|No| SCHEDULE_RECOLLECT[Schedule Recollection]
    SCHEDULE_RECOLLECT --> END_REJECT([End - Rejected])

    QUALITY -->|Yes| MIX_SAMPLE[Mix Sample (if anticoagulant)]
    MIX_SAMPLE --> RECORD_DETAILS[Record Collection Details]
    RECORD_DETAILS --> UPDATE_SYSTEM[Update System Status]
    UPDATE_SYSTEM --> TRANSPORT[Place in Transport Container]
    TRANSPORT --> THANK_PATIENT[Thank Patient & Apply Bandage]
    THANK_PATIENT --> SEND_TO_LAB[Send to Laboratory]
    SEND_TO_LAB --> END([End])
```

## 3.2 Home Visit Sample Collection

```mermaid
flowchart TD
    START([Receive Home Visit Request]) --> PLAN[Plan Route for Multiple Visits]
    PLAN --> PREPARE[Prepare Collection Kit]
    PREPARE --> CHECK_LIST[Check Equipment List]
    CHECK_LIST --> DEPART[Depart for Visit]

    DEPART --> ARRIVE[Arrive at Patient Location]
    ARRIVE --> CALL_PATIENT[Call Patient]
    CALL_PATIENT --> PATIENT_READY{Patient<br/>Available?}
    PATIENT_READY -->|No| RESCHEDULE[Reschedule Visit]
    PATIENT_READY -->|Yes| ENTER[Enter Patient Home]

    ENTER --> VERIFY_ID[Verify Patient Identity]
    VERIFY_ID --> SETUP[Setup Collection Area]
    SETUP --> COLLECT_SAMPLE[Collect Sample]
    COLLECT_SAMPLE --> LABEL[Label Sample]
    LABEL --> STORAGE[Store in Cool Box]

    STORAGE --> MORE_VISITS{More Visits<br/>Pending?}
    MORE_VISITS -->|Yes| NEXT[Navigate to Next Location]
    NEXT --> ARRIVE
    MORE_VISITS -->|No| RETURN[Return to Laboratory]

    RETURN --> HANDOVER[Hand Over Samples]
    HANDOVER --> UPDATE[Update Collection Status]
    UPDATE --> END([End])

    RESCHEDULE --> MORE_VISITS
```

---

# 4. Lab Technician

## 4.1 Sample Receiving & Processing

```mermaid
flowchart TD
    START([Receive Samples]) --> SCAN[Scan Sample Barcode]
    SCAN --> VALID{Sample<br/>Valid?}
    VALID -->|No| INVESTIGATE[Investigate Issue]
    VALID -->|Yes| VISUAL[Visual Inspection]

    VISUAL --> QUALITY{Quality<br/>OK?}
    QUALITY -->|No| REJECT_FLOW[Rejection Workflow]
    QUALITY -->|Yes| REGISTER[Register Sample Receipt]

    REGISTER --> CHECK_REQUIREMENTS[Check Test Requirements]
    CHECK_REQUIREMENTS --> PROCESSING{Processing<br/>Required?}

    PROCESSING -->|Centrifuge| CENTRIFUGE[Centrifuge Sample]
    PROCESSING -->|Aliquot| ALIQUOT[Create Aliquots]
    PROCESSING -->|None| ROUTE_SAMPLE

    CENTRIFUGE --> SEPARATE[Separate Serum/Plasma]
    SEPARATE --> TRANSFER[Transfer to Secondary Tube]
    TRANSFER --> LABEL_SECONDARY[Label Secondary Tube]
    LABEL_SECONDARY --> ROUTE_SAMPLE

    ALIQUOT --> CALCULATE_VOL[Calculate Required Volume]
    CALCULATE_VOL --> CREATE_ALIQUOTS[Create Multiple Aliquots]
    CREATE_ALIQUOTS --> LABEL_ALIQUOTS[Label Each Aliquot]
    LABEL_ALIQUOTS --> ROUTE_SAMPLE[Route Sample to Equipment]

    ROUTE_SAMPLE --> SYSTEM_ROUTE[System Suggests Equipment]
    SYSTEM_ROUTE --> CONFIRM_ROUTE{Accept<br/>Suggestion?}
    CONFIRM_ROUTE -->|No| MANUAL_SELECT[Select Equipment Manually]
    CONFIRM_ROUTE -->|Yes| ASSIGN_EQUIPMENT[Assign to Equipment]
    MANUAL_SELECT --> ASSIGN_EQUIPMENT

    ASSIGN_EQUIPMENT --> LOAD{Auto-Loader<br/>Available?}
    LOAD -->|Yes| AUTO_LOAD[Load on Auto-Loader]
    LOAD -->|No| MANUAL_LOAD[Manual Loading]

    AUTO_LOAD --> MONITOR[Monitor Processing]
    MANUAL_LOAD --> MONITOR

    MONITOR --> RESULTS_IN{Results<br/>Received?}
    RESULTS_IN -->|No| WAIT[Wait for Results]
    WAIT --> RESULTS_IN
    RESULTS_IN -->|Yes| VERIFY_RESULTS[Preliminary Verification]

    VERIFY_RESULTS --> STORE[Store Remaining Sample]
    STORE --> LOCATION[Record Storage Location]
    LOCATION --> END([End])

    REJECT_FLOW --> DOCUMENT[Document Rejection]
    DOCUMENT --> NOTIFY[Notify Front Desk]
    NOTIFY --> END
    INVESTIGATE --> END
```

## 4.2 Manual Result Entry

```mermaid
flowchart TD
    START([Receive Manual Test Result]) --> LOGIN[Login to System]
    LOGIN --> SEARCH[Search for Sample]
    SEARCH --> FOUND{Sample<br/>Found?}
    FOUND -->|No| CHECK_BARCODE[Verify Barcode]
    CHECK_BARCODE --> SEARCH
    FOUND -->|Yes| OPEN_RESULT[Open Result Entry Form]

    OPEN_RESULT --> SELECT_TEST[Select Test]
    SELECT_TEST --> ENTER_VALUE[Enter Result Value]
    ENTER_VALUE --> UNIT{Correct<br/>Unit?}
    UNIT -->|No| SELECT_UNIT[Select Unit]
    SELECT_UNIT --> VALIDATION
    UNIT -->|Yes| VALIDATION[System Validates Range]

    VALIDATION --> RANGE{Within<br/>Range?}
    RANGE -->|No| WARNING[Show Warning]
    WARNING --> CONFIRM{Confirm<br/>Value?}
    CONFIRM -->|No| ENTER_VALUE
    CONFIRM -->|Yes| SAVE_RESULT
    RANGE -->|Yes| CRITICAL{Critical<br/>Value?}

    CRITICAL -->|Yes| ALERT[Critical Value Alert]
    ALERT --> RECHECK[Recheck Result]
    RECHECK --> CONFIRMED{Confirmed?}
    CONFIRMED -->|No| ENTER_VALUE
    CONFIRMED -->|Yes| NOTIFY_PATH[Notify Pathologist]
    NOTIFY_PATH --> SAVE_RESULT

    CRITICAL -->|No| DELTA{Delta Check<br/>Flag?}
    DELTA -->|Yes| REVIEW_PREVIOUS[Review Previous Results]
    REVIEW_PREVIOUS --> DELTA_CONFIRM{Confirm<br/>Large Change?}
    DELTA_CONFIRM -->|No| ENTER_VALUE
    DELTA_CONFIRM -->|Yes| ADD_COMMENT[Add Comment]
    ADD_COMMENT --> SAVE_RESULT

    DELTA -->|No| SAVE_RESULT[Save Result]
    SAVE_RESULT --> MORE{More<br/>Tests?}
    MORE -->|Yes| SELECT_TEST
    MORE -->|No| COMPLETE[Mark Sample Complete]
    COMPLETE --> END([End])
```

---

# 5. Pathologist

## 5.1 Result Review & Verification

```mermaid
flowchart TD
    START([Login to System]) --> DASHBOARD[View Pathologist Dashboard]
    DASHBOARD --> PENDING[View Pending Verifications]

    PENDING --> FILTER{Filter by?}
    FILTER -->|Critical| CRITICAL_LIST[Critical Values]
    FILTER -->|Abnormal| ABNORMAL_LIST[Abnormal Results]
    FILTER -->|All| ALL_LIST[All Pending]

    CRITICAL_LIST --> SELECT
    ABNORMAL_LIST --> SELECT
    ALL_LIST --> SELECT[Select Result to Review]

    SELECT --> VIEW[View Complete Result]
    VIEW --> CONTEXT[Review Patient Context]

    CONTEXT --> CHECK_HISTORY[Check Previous Results]
    CHECK_HISTORY --> DELTA_REVIEW{Significant<br/>Change?}
    DELTA_REVIEW -->|Yes| INVESTIGATE[Investigate Cause]
    INVESTIGATE --> VALID{Result<br/>Valid?}
    VALID -->|No| REQUEST_RETEST[Request Retest]
    REQUEST_RETEST --> END_RETEST([End - Awaiting Retest])

    DELTA_REVIEW -->|No| REFERENCE[Check Reference Range]
    VALID -->|Yes| REFERENCE

    REFERENCE --> INTERPRETATION[Clinical Interpretation]
    INTERPRETATION --> ABNORMAL{Abnormal<br/>Result?}
    ABNORMAL -->|Yes| ADD_COMMENT[Add Clinical Comment]
    ADD_COMMENT --> CRITICAL_CHECK
    ABNORMAL -->|No| CRITICAL_CHECK{Critical<br/>Value?}

    CRITICAL_CHECK -->|Yes| PHONE_CALL[Call Referring Physician]
    PHONE_CALL --> DOCUMENT_CALL[Document Communication]
    DOCUMENT_CALL --> APPROVE
    CRITICAL_CHECK -->|No| APPROVE[Approve Result]

    APPROVE --> DIGITAL_SIGN{Sign<br/>Required?}
    DIGITAL_SIGN -->|Yes| APPLY_SIGNATURE[Apply Digital Signature]
    APPLY_SIGNATURE --> VERIFY_COMPLETE
    DIGITAL_SIGN -->|No| VERIFY_COMPLETE[Mark as Verified]

    VERIFY_COMPLETE --> ALL_DONE{All Tests<br/>Complete?}
    ALL_DONE -->|Yes| TRIGGER_REPORT[Trigger Report Generation]
    ALL_DONE -->|No| NEXT[Next Result]

    TRIGGER_REPORT --> REVIEW_REPORT[Review Generated Report]
    REVIEW_REPORT --> REPORT_OK{Report<br/>OK?}
    REPORT_OK -->|No| EDIT_REPORT[Edit Report]
    EDIT_REPORT --> REVIEW_REPORT
    REPORT_OK -->|Yes| SIGN_REPORT[Sign Report]

    SIGN_REPORT --> AUTHORIZE_DELIVERY[Authorize Delivery]
    AUTHORIZE_DELIVERY --> END([End])

    NEXT --> SELECT
```

## 5.2 Handling Critical Values

```mermaid
flowchart TD
    START([Critical Value Alert Received]) --> REVIEW[Review Critical Result]
    REVIEW --> VERIFY[Verify Result Accuracy]

    VERIFY --> RECHECK{Needs<br/>Recheck?}
    RECHECK -->|Yes| REQUEST_REPEAT[Request Repeat Analysis]
    REQUEST_REPEAT --> AWAIT[Await Repeat Result]
    AWAIT --> CONFIRMED{Same<br/>Result?}
    CONFIRMED -->|No| INVESTIGATE[Investigate Discrepancy]
    CONFIRMED -->|Yes| PROCEED_NOTIFICATION

    RECHECK -->|No| PROCEED_NOTIFICATION[Proceed to Notification]

    PROCEED_NOTIFICATION --> PATIENT_INFO[Get Patient Contact Info]
    PATIENT_INFO --> PHYSICIAN_INFO[Get Physician Contact Info]

    PHYSICIAN_INFO --> CALL_PHYSICIAN[Call Referring Physician]
    CALL_PHYSICIAN --> REACHED{Physician<br/>Reached?}
    REACHED -->|No| TRY_ALTERNATE[Try Alternate Number]
    TRY_ALTERNATE --> RETRY_COUNT{Max<br/>Attempts?}
    RETRY_COUNT -->|No| CALL_PHYSICIAN
    RETRY_COUNT -->|Yes| CALL_PATIENT[Call Patient Directly]

    REACHED -->|Yes| INFORM[Inform Critical Result]
    INFORM --> RECOMMENDATIONS[Provide Recommendations]
    RECOMMENDATIONS --> DOCUMENT[Document Communication]

    CALL_PATIENT --> PATIENT_REACHED{Patient<br/>Reached?}
    PATIENT_REACHED -->|Yes| INFORM_PATIENT[Inform Patient]
    INFORM_PATIENT --> ADVISE[Advise Immediate Medical Attention]
    ADVISE --> DOCUMENT

    PATIENT_REACHED -->|No| EMERGENCY[Escalate to Emergency Protocol]
    EMERGENCY --> DOCUMENT

    DOCUMENT --> RECORD_TIME[Record Communication Time]
    RECORD_TIME --> RECORD_PERSON[Record Person Notified]
    RECORD_PERSON --> RECORD_RESPONSE[Record Their Response]
    RECORD_RESPONSE --> SAVE_LOG[Save Communication Log]

    SAVE_LOG --> APPROVE_RESULT[Approve Critical Result]
    APPROVE_RESULT --> EXPEDITE_REPORT[Expedite Report Delivery]
    EXPEDITE_REPORT --> END([End])

    INVESTIGATE --> END
```

---

# 6. Lab Director / Manager

## 6.1 Daily Operations Monitoring

```mermaid
flowchart TD
    START([Login to System]) --> DASHBOARD[View Director Dashboard]
    DASHBOARD --> KPI[Review Key Metrics]

    KPI --> METRICS{Review Area?}
    METRICS -->|Operations| OPS_METRICS[Sample Volume, TAT, Backlogs]
    METRICS -->|Quality| QUALITY_METRICS[QC Status, Rejections]
    METRICS -->|Financial| FINANCIAL[Revenue, Collections]
    METRICS -->|Equipment| EQUIPMENT[Equipment Status]

    OPS_METRICS --> SAMPLE_VOL{Sample Volume<br/>Normal?}
    SAMPLE_VOL -->|No| INVESTIGATE_VOL[Investigate Volume Change]
    SAMPLE_VOL -->|Yes| TAT_CHECK{TAT<br/>Compliant?}
    TAT_CHECK -->|No| IDENTIFY_BOTTLENECK[Identify Bottlenecks]
    IDENTIFY_BOTTLENECK --> ASSIGN_RESOURCES[Assign Additional Resources]
    TAT_CHECK -->|Yes| BACKLOG{Backlog<br/>Present?}
    BACKLOG -->|Yes| PRIORITIZE[Prioritize Critical Orders]
    BACKLOG -->|No| CONTINUE

    QUALITY_METRICS --> QC_STATUS{QC<br/>Passed?}
    QC_STATUS -->|No| REVIEW_QC_FAILURE[Review QC Failures]
    REVIEW_QC_FAILURE --> CORRECTIVE_ACTION[Initiate Corrective Action]
    QC_STATUS -->|Yes| REJECTION_RATE{Rejection<br/>Rate Normal?}
    REJECTION_RATE -->|No| ANALYZE_REJECTIONS[Analyze Rejection Reasons]
    ANALYZE_REJECTIONS --> TRAINING[Arrange Training]
    REJECTION_RATE -->|Yes| CONTINUE[Continue Monitoring]

    FINANCIAL --> REVENUE_TARGET{Meeting<br/>Target?}
    REVENUE_TARGET -->|No| REVENUE_ANALYSIS[Analyze Revenue Gaps]
    REVENUE_ANALYSIS --> MARKETING[Plan Marketing Initiatives]
    REVENUE_TARGET -->|Yes| COLLECTION{Collection<br/>Efficient?}
    COLLECTION -->|No| FOLLOW_UP[Follow Up Pending Payments]
    COLLECTION -->|Yes| CONTINUE

    EQUIPMENT --> EQP_STATUS{All Equipment<br/>Online?}
    EQP_STATUS -->|No| MAINTENANCE[Check Maintenance Status]
    MAINTENANCE --> ESCALATE{Urgent<br/>Repair?}
    ESCALATE -->|Yes| CALL_VENDOR[Call Service Vendor]
    ESCALATE -->|No| SCHEDULE_MAINTENANCE[Schedule Maintenance]
    EQP_STATUS -->|Yes| CALIBRATION{Calibration<br/>Due?}
    CALIBRATION -->|Yes| PLAN_CALIBRATION[Plan Calibration Schedule]
    CALIBRATION -->|No| CONTINUE

    CONTINUE --> ALERTS[Review System Alerts]
    ALERTS --> CRITICAL_ALERTS{Critical<br/>Alerts?}
    CRITICAL_ALERTS -->|Yes| HANDLE_ALERT[Handle Critical Issues]
    CRITICAL_ALERTS -->|No| REPORTS[Review Reports]

    REPORTS --> MEETINGS[Attend Meetings]
    MEETINGS --> APPROVE_REQUESTS[Approve Pending Requests]
    APPROVE_REQUESTS --> END([End Day])

    INVESTIGATE_VOL --> CONTINUE
    ASSIGN_RESOURCES --> CONTINUE
    PRIORITIZE --> CONTINUE
    CORRECTIVE_ACTION --> CONTINUE
    TRAINING --> CONTINUE
    MARKETING --> CONTINUE
    FOLLOW_UP --> CONTINUE
    CALL_VENDOR --> CONTINUE
    SCHEDULE_MAINTENANCE --> CONTINUE
    PLAN_CALIBRATION --> CONTINUE
    HANDLE_ALERT --> CONTINUE
```

## 6.2 Performance Review & Decision Making

```mermaid
flowchart TD
    START([Monthly Review Meeting]) --> AGENDA[Review Meeting Agenda]
    AGENDA --> OPERATIONAL[Review Operational Metrics]

    OPERATIONAL --> TAT_REVIEW[TAT Performance Analysis]
    TAT_REVIEW --> TAT_TREND{TAT<br/>Improving?}
    TAT_TREND -->|No| ROOT_CAUSE[Conduct Root Cause Analysis]
    ROOT_CAUSE --> SOLUTIONS[Identify Solutions]
    SOLUTIONS --> PROCESS_CHANGE{Process<br/>Change Needed?}
    PROCESS_CHANGE -->|Yes| APPROVE_CHANGE[Approve Process Changes]
    PROCESS_CHANGE -->|No| TECHNOLOGY{Technology<br/>Upgrade?}
    TECHNOLOGY -->|Yes| BUDGET_REVIEW[Review Budget]
    BUDGET_REVIEW --> APPROVE_INVESTMENT[Approve Investment]
    TECHNOLOGY -->|No| TRAINING_NEED[Identify Training Needs]

    TAT_TREND -->|Yes| VOLUME_ANALYSIS[Volume Analysis]
    VOLUME_ANALYSIS --> CAPACITY{At<br/>Capacity?}
    CAPACITY -->|Yes| EXPANSION_PLAN[Plan Capacity Expansion]
    EXPANSION_PLAN --> HIRE{Hire More<br/>Staff?}
    HIRE -->|Yes| RECRUITMENT[Approve Recruitment]
    HIRE -->|No| EQUIPMENT_ADD[Add Equipment]
    CAPACITY -->|No| QUALITY_REVIEW

    QUALITY_REVIEW --> QC_PERFORMANCE[Review QC Metrics]
    QC_PERFORMANCE --> QC_COMPLIANT{QC<br/>Compliant?}
    QC_COMPLIANT -->|No| QC_ISSUES[Identify QC Issues]
    QC_ISSUES --> CAPA_REVIEW[Review CAPA Status]
    CAPA_REVIEW --> APPROVE_CAPA[Approve CAPA Plans]
    QC_COMPLIANT -->|Yes| ACCREDITATION[Review Accreditation Status]

    ACCREDITATION --> NABL_STATUS{NABL<br/>Compliance?}
    NABL_STATUS -->|No| GAP_ANALYSIS[Conduct Gap Analysis]
    GAP_ANALYSIS --> REMEDIATION[Plan Remediation]
    REMEDIATION --> ASSIGN_OWNER[Assign Ownership]
    NABL_STATUS -->|Yes| FINANCIAL_REVIEW

    FINANCIAL_REVIEW --> REVENUE[Review Revenue]
    REVENUE --> PROFIT{Profitable?}
    PROFIT -->|No| COST_ANALYSIS[Analyze Costs]
    COST_ANALYSIS --> REDUCE_COST[Identify Cost Reduction Opportunities]
    REDUCE_COST --> PRICING{Review<br/>Pricing?}
    PRICING -->|Yes| PRICING_STRATEGY[Revise Pricing Strategy]
    PRICING -->|No| NEGOTIATE[Negotiate with Vendors]

    PROFIT -->|Yes| GROWTH[Plan Growth Initiatives]
    GROWTH --> MARKET_ANALYSIS[Analyze Market Opportunities]
    MARKET_ANALYSIS --> NEW_TESTS{Launch New<br/>Tests?}
    NEW_TESTS -->|Yes| TEST_VALIDATION[Plan Test Validation]
    NEW_TESTS -->|No| PARTNERSHIPS{Explore<br/>Partnerships?}
    PARTNERSHIPS -->|Yes| PARTNERSHIP_PLAN[Develop Partnership Strategy]
    PARTNERSHIPS -->|No| DOCUMENT_DECISIONS

    APPROVE_CHANGE --> DOCUMENT_DECISIONS
    APPROVE_INVESTMENT --> DOCUMENT_DECISIONS
    TRAINING_NEED --> DOCUMENT_DECISIONS
    RECRUITMENT --> DOCUMENT_DECISIONS
    EQUIPMENT_ADD --> DOCUMENT_DECISIONS
    APPROVE_CAPA --> DOCUMENT_DECISIONS
    ASSIGN_OWNER --> DOCUMENT_DECISIONS
    PRICING_STRATEGY --> DOCUMENT_DECISIONS
    NEGOTIATE --> DOCUMENT_DECISIONS
    TEST_VALIDATION --> DOCUMENT_DECISIONS
    PARTNERSHIP_PLAN --> DOCUMENT_DECISIONS[Document Decisions]

    DOCUMENT_DECISIONS --> COMMUNICATE[Communicate to Team]
    COMMUNICATE --> TRACK[Setup Tracking Metrics]
    TRACK --> END([End Meeting])
```

---

# 7. Quality Manager

## 7.1 Daily QC Review

```mermaid
flowchart TD
    START([Start of Day]) --> LOGIN[Login to QC Module]
    LOGIN --> TODAY_QC[View Today's QC Schedule]

    TODAY_QC --> PENDING{Pending<br/>QC Runs?}
    PENDING -->|Yes| REMIND[Remind Technicians]
    REMIND --> AWAIT
    PENDING -->|No| REVIEW[Review Completed QC]

    AWAIT[Wait for QC Completion] --> COMPLETE{QC<br/>Complete?}
    COMPLETE -->|No| AWAIT
    COMPLETE -->|Yes| REVIEW

    REVIEW --> SELECT_TEST[Select Test/Equipment]
    SELECT_TEST --> VIEW_DATA[View QC Data]
    VIEW_DATA --> LJ_CHART[Review Levy-Jennings Chart]

    LJ_CHART --> WESTGARD[Apply Westgard Rules]
    WESTGARD --> VIOLATION{Rule<br/>Violation?}

    VIOLATION -->|Yes| VIOLATION_TYPE{Violation<br/>Type?}
    VIOLATION_TYPE -->|1_3s| CRITICAL[Critical - Stop Testing]
    VIOLATION_TYPE -->|2_2s| CRITICAL
    VIOLATION_TYPE -->|R_4s| CRITICAL
    VIOLATION_TYPE -->|10_x| WARNING[Warning - Monitor]
    VIOLATION_TYPE -->|4_1s| WARNING

    CRITICAL --> ALERT_TECH[Alert Technicians]
    ALERT_TECH --> STOP_EQUIPMENT[Mark Equipment Out of Control]
    STOP_EQUIPMENT --> INVESTIGATE_CAUSE[Investigate Root Cause]

    INVESTIGATE_CAUSE --> CAUSE{Cause<br/>Identified?}
    CAUSE -->|Reagent| CHANGE_REAGENT[Change Reagent Lot]
    CAUSE -->|Calibration| RECALIBRATE[Perform Recalibration]
    CAUSE -->|Equipment| MAINTENANCE_REQUIRED[Schedule Maintenance]
    CAUSE -->|Unknown| DETAILED_INVESTIGATION[Detailed Investigation]

    CHANGE_REAGENT --> RERUN_QC[Rerun QC]
    RECALIBRATE --> RERUN_QC
    MAINTENANCE_REQUIRED --> WAIT_MAINTENANCE[Wait for Maintenance]
    WAIT_MAINTENANCE --> RERUN_QC

    RERUN_QC --> QC_PASS{QC<br/>Passed?}
    QC_PASS -->|Yes| RESUME[Resume Testing]
    QC_PASS -->|No| ESCALATE[Escalate to Director]

    WARNING --> DOCUMENT_WARNING[Document Warning]
    DOCUMENT_WARNING --> MONITOR_TREND[Monitor Trend]
    MONITOR_TREND --> ACCEPT_RUN[Accept QC Run]

    VIOLATION -->|No| IN_CONTROL[Equipment In Control]
    IN_CONTROL --> ACCEPT_RUN

    ACCEPT_RUN --> CREATE_CAPA{CAPA<br/>Required?}
    CREATE_CAPA -->|Yes| INITIATE_CAPA[Initiate CAPA]
    CREATE_CAPA -->|No| MORE_EQUIPMENT{More<br/>Equipment?}

    MORE_EQUIPMENT -->|Yes| SELECT_TEST
    MORE_EQUIPMENT -->|No| REPORT[Generate QC Summary Report]

    REPORT --> DISTRIBUTE[Distribute to Management]
    DISTRIBUTE --> END([End])

    RESUME --> ACCEPT_RUN
    ESCALATE --> END
    INITIATE_CAPA --> MORE_EQUIPMENT
    DETAILED_INVESTIGATION --> END
```

## 7.2 Audit Preparation & Conduct

```mermaid
flowchart TD
    START([Audit Scheduled]) --> AUDIT_TYPE{Audit<br/>Type?}
    AUDIT_TYPE -->|Internal| INTERNAL_AUDIT
    AUDIT_TYPE -->|NABL| NABL_AUDIT
    AUDIT_TYPE -->|Client| CLIENT_AUDIT

    INTERNAL_AUDIT --> PLAN[Create Audit Plan]
    NABL_AUDIT --> PLAN
    CLIENT_AUDIT --> PLAN

    PLAN --> CHECKLIST[Prepare Audit Checklist]
    CHECKLIST --> ASSIGN_AUDITOR[Assign Auditors]
    ASSIGN_AUDITOR --> NOTIFY_DEPT[Notify Departments]

    NOTIFY_DEPT --> PRE_AUDIT[Pre-Audit Document Review]
    PRE_AUDIT --> DOCUMENTS{Documents<br/>Complete?}
    DOCUMENTS -->|No| REQUEST_DOCS[Request Missing Documents]
    REQUEST_DOCS --> AWAIT_DOCS[Wait for Documents]
    AWAIT_DOCS --> PRE_AUDIT
    DOCUMENTS -->|Yes| OPENING_MEETING[Conduct Opening Meeting]

    OPENING_MEETING --> FACILITY_TOUR[Facility Tour]
    FACILITY_TOUR --> INTERVIEW[Interview Staff]

    INTERVIEW --> OBSERVE[Observe Processes]
    OBSERVE --> RECORDS[Review Records]
    RECORDS --> FINDINGS{Non-Conformances<br/>Found?}

    FINDINGS -->|Yes| CLASSIFY{Severity?}
    CLASSIFY -->|Critical| CRITICAL_NC[Document Critical NC]
    CLASSIFY -->|Major| MAJOR_NC[Document Major NC]
    CLASSIFY -->|Minor| MINOR_NC[Document Minor NC]
    CLASSIFY -->|Observation| OBS[Document Observation]

    CRITICAL_NC --> IMMEDIATE_ACTION[Require Immediate Action]
    IMMEDIATE_ACTION --> NEXT_AREA
    MAJOR_NC --> NEXT_AREA
    MINOR_NC --> NEXT_AREA
    OBS --> NEXT_AREA

    FINDINGS -->|No| NEXT_AREA{More Areas<br/>to Audit?}
    NEXT_AREA -->|Yes| FACILITY_TOUR
    NEXT_AREA -->|No| CLOSING_MEETING[Conduct Closing Meeting]

    CLOSING_MEETING --> PRESENT_FINDINGS[Present Findings]
    PRESENT_FINDINGS --> AUDIT_REPORT[Prepare Audit Report]

    AUDIT_REPORT --> DISTRIBUTE_REPORT[Distribute Report]
    DISTRIBUTE_REPORT --> CAPA_REQUIRED{CAPA<br/>Required?}

    CAPA_REQUIRED -->|Yes| CREATE_CAPA[Create CAPA]
    CREATE_CAPA --> ASSIGN_RESPONSIBLE[Assign Responsible Person]
    ASSIGN_RESPONSIBLE --> SET_DEADLINE[Set Completion Deadline]
    SET_DEADLINE --> TRACK_CAPA[Track CAPA Progress]

    TRACK_CAPA --> CAPA_COMPLETE{CAPA<br/>Completed?}
    CAPA_COMPLETE -->|No| REMIND[Send Reminder]
    REMIND --> TRACK_CAPA
    CAPA_COMPLETE -->|Yes| VERIFY_EFFECTIVENESS[Verify Effectiveness]

    VERIFY_EFFECTIVENESS --> EFFECTIVE{Effective?}
    EFFECTIVE -->|No| REVISE_CAPA[Revise CAPA]
    REVISE_CAPA --> TRACK_CAPA
    EFFECTIVE -->|Yes| CLOSE_FINDING[Close Finding]

    CLOSE_FINDING --> FOLLOW_UP_AUDIT[Schedule Follow-up Audit]
    CAPA_REQUIRED -->|No| FOLLOW_UP_AUDIT

    FOLLOW_UP_AUDIT --> END([End])
```

---

# 8. Billing Staff

## 8.1 Invoice Generation & Payment Processing

```mermaid
flowchart TD
    START([Order Completed]) --> AUTO_GEN{Auto-Generate<br/>Invoice?}
    AUTO_GEN -->|Yes| GENERATE[System Generates Invoice]
    AUTO_GEN -->|No| MANUAL[Create Invoice Manually]

    MANUAL --> SELECT_ORDER[Select Order]
    SELECT_ORDER --> REVIEW_TESTS[Review Tests Performed]
    REVIEW_TESTS --> GENERATE

    GENERATE --> CALCULATE[Calculate Totals]
    CALCULATE --> GST[Apply GST]
    GST --> INVOICE_NUM[Assign Invoice Number]
    INVOICE_NUM --> PREVIEW[Preview Invoice]

    PREVIEW --> CORRECT{Details<br/>Correct?}
    CORRECT -->|No| EDIT[Edit Invoice]
    EDIT --> PREVIEW
    CORRECT -->|Yes| INSURANCE{Insurance<br/>Claim?}

    INSURANCE -->|Yes| VERIFY_COV[Verify Coverage]
    VERIFY_COV --> COVERED{Fully<br/>Covered?}
    COVERED -->|Yes| SUBMIT_CLAIM[Submit Claim to TPA]
    COVERED -->|No| CALCULATE_PATIENT[Calculate Patient Portion]
    CALCULATE_PATIENT --> SPLIT_INVOICE[Split Insurance/Patient]
    SPLIT_INVOICE --> SUBMIT_CLAIM

    SUBMIT_CLAIM --> TPA_APPROVAL{TPA<br/>Approved?}
    TPA_APPROVAL -->|No| APPEAL[Appeal Rejection]
    TPA_APPROVAL -->|Yes| RECEIVE_PAYMENT[Receive Insurance Payment]
    RECEIVE_PAYMENT --> PATIENT_DUE{Patient<br/>Balance?}
    PATIENT_DUE -->|Yes| COLLECT_BALANCE
    PATIENT_DUE -->|No| FINALIZE

    INSURANCE -->|No| PAYMENT_STATUS{Already<br/>Paid?}
    PAYMENT_STATUS -->|Yes| RECONCILE[Reconcile Payment]
    PAYMENT_STATUS -->|No| SEND_INVOICE[Send Invoice to Patient]

    SEND_INVOICE --> CHANNEL{Delivery<br/>Channel?}
    CHANNEL -->|Email| EMAIL_INV[Email Invoice]
    CHANNEL -->|WhatsApp| WA_INV[WhatsApp Invoice]
    CHANNEL -->|SMS| SMS_INV[SMS Payment Link]
    CHANNEL -->|Print| PRINT_INV[Print Invoice]

    EMAIL_INV --> AWAIT_PAYMENT
    WA_INV --> AWAIT_PAYMENT
    SMS_INV --> AWAIT_PAYMENT
    PRINT_INV --> COLLECT_NOW{Collect<br/>Now?}
    COLLECT_NOW -->|Yes| COLLECT_PAYMENT
    COLLECT_NOW -->|No| AWAIT_PAYMENT[Await Payment]

    AWAIT_PAYMENT --> RECEIVED{Payment<br/>Received?}
    RECEIVED -->|No| REMINDER{Send<br/>Reminder?}
    REMINDER -->|Yes| REMIND_PATIENT[Send Payment Reminder]
    REMIND_PATIENT --> AWAIT_PAYMENT
    REMINDER -->|No| OVERDUE{Overdue?}
    OVERDUE -->|Yes| COLLECTION_PROCESS[Initiate Collection Process]
    OVERDUE -->|No| AWAIT_PAYMENT

    RECEIVED -->|Yes| VERIFY_PAYMENT[Verify Payment Details]
    COLLECT_PAYMENT[Collect Payment] --> METHOD{Payment<br/>Method?}
    METHOD -->|Cash| CASH[Accept Cash]
    METHOD -->|Card| CARD[Process Card Payment]
    METHOD -->|UPI| UPI[Receive UPI Payment]
    METHOD -->|Online| ONLINE[Process Online Payment]

    CASH --> RECEIPT
    CARD --> RECEIPT
    UPI --> RECEIPT
    ONLINE --> RECEIPT
    VERIFY_PAYMENT --> RECEIPT[Generate Receipt]

    RECEIPT --> SEND_RECEIPT[Send Receipt to Patient]
    SEND_RECEIPT --> UPDATE_SYSTEM[Update Payment Status]
    UPDATE_SYSTEM --> COLLECT_BALANCE[Collect Balance if Any]

    COLLECT_BALANCE --> FINALIZE[Finalize Invoice]
    RECONCILE --> FINALIZE

    FINALIZE --> E_INVOICE{E-Invoice<br/>Required?}
    E_INVOICE -->|Yes| GENERATE_EINV[Generate E-Invoice]
    GENERATE_EINV --> IRN[Obtain IRN]
    IRN --> QR[Generate QR Code]
    QR --> ARCHIVE
    E_INVOICE -->|No| ARCHIVE[Archive Invoice]

    ARCHIVE --> END([End])
    APPEAL --> END
    COLLECTION_PROCESS --> END
```

## 8.2 Credit Management & Collections

```mermaid
flowchart TD
    START([Daily Collections Review]) --> OVERDUE[Generate Overdue Report]
    OVERDUE --> AGING[Aging Analysis]

    AGING --> SEGMENT{Overdue<br/>Period?}
    SEGMENT -->|0-30 days| GENTLE[Gentle Reminder]
    SEGMENT -->|31-60 days| FIRM[Firm Reminder]
    SEGMENT -->|61-90 days| STRICT[Strict Notice]
    SEGMENT -->|>90 days| ESCALATE[Escalate to Collection]

    GENTLE --> EMAIL_REMINDER[Send Email Reminder]
    EMAIL_REMINDER --> SMS_REMINDER[Send SMS Reminder]
    SMS_REMINDER --> TRACK_GENTLE[Track Response]

    FIRM --> CALL_PATIENT[Call Patient]
    CALL_PATIENT --> REACHED{Patient<br/>Reached?}
    REACHED -->|No| MULTIPLE_ATTEMPTS[Multiple Contact Attempts]
    REACHED -->|Yes| DISCUSS[Discuss Payment]

    DISCUSS --> AGREE{Payment<br/>Agreed?}
    AGREE -->|Yes| PAYMENT_PLAN[Setup Payment Plan]
    AGREE -->|No| REASONS[Understand Reasons]
    REASONS --> DISPUTE{Billing<br/>Dispute?}
    DISPUTE -->|Yes| RESOLVE_DISPUTE[Resolve Dispute]
    DISPUTE -->|No| NEGOTIATE[Negotiate Terms]

    STRICT --> FINAL_NOTICE[Send Final Notice]
    FINAL_NOTICE --> GRACE[7-Day Grace Period]
    GRACE --> PAID{Paid?}
    PAID -->|Yes| UPDATE_PAID[Update as Paid]
    PAID -->|No| ESCALATE

    ESCALATE --> LEGAL{Legal<br/>Action?}
    LEGAL -->|Yes| LEGAL_NOTICE[Issue Legal Notice]
    LEGAL -->|No| WRITE_OFF{Write Off?}
    WRITE_OFF -->|Yes| BAD_DEBT[Mark as Bad Debt]
    WRITE_OFF -->|No| HOLD[Put on Hold]

    PAYMENT_PLAN --> SCHEDULE[Schedule Installments]
    SCHEDULE --> MONITOR[Monitor Payments]
    MONITOR --> INSTALLMENT{Installment<br/>Paid?}
    INSTALLMENT -->|Yes| MORE{More<br/>Installments?}
    MORE -->|Yes| MONITOR
    MORE -->|No| COMPLETE[Mark as Complete]

    INSTALLMENT -->|No| MISSED[Missed Payment]
    MISSED --> FOLLOWUP[Follow Up]
    FOLLOWUP --> RENEGOTIATE{Renegotiate?}
    RENEGOTIATE -->|Yes| SCHEDULE
    RENEGOTIATE -->|No| ESCALATE

    TRACK_GENTLE --> RESPONSE{Response<br/>Received?}
    RESPONSE -->|Yes| CHECK_PAYMENT{Payment<br/>Made?}
    CHECK_PAYMENT -->|Yes| UPDATE_PAID
    CHECK_PAYMENT -->|No| FIRM
    RESPONSE -->|No| WAIT[Wait 7 Days]
    WAIT --> FIRM

    MULTIPLE_ATTEMPTS --> MAX_ATTEMPTS{Max<br/>Attempts?}
    MAX_ATTEMPTS -->|No| CALL_PATIENT
    MAX_ATTEMPTS -->|Yes| STRICT

    UPDATE_PAID --> REPORT[Update Collection Report]
    COMPLETE --> REPORT
    RESOLVE_DISPUTE --> REPORT
    NEGOTIATE --> REPORT
    BAD_DEBT --> REPORT
    HOLD --> REPORT
    LEGAL_NOTICE --> REPORT

    REPORT --> MORE_OVERDUE{More Overdue<br/>Accounts?}
    MORE_OVERDUE -->|Yes| AGING
    MORE_OVERDUE -->|No| SUMMARY[Generate Summary Report]

    SUMMARY --> SUBMIT[Submit to Management]
    SUBMIT --> END([End])
```

---

## Summary

This comprehensive documentation provides detailed user flow diagrams for all 8 major user personas in the LIS/LIMS system:

1. **Patient (Self-Service)**: WhatsApp registration, web booking, report access
2. **Front Desk Staff**: Patient check-in, order creation, query handling
3. **Phlebotomist**: Sample collection (clinic & home visit)
4. **Lab Technician**: Sample processing, result entry
5. **Pathologist**: Result review, critical value handling
6. **Lab Director**: Operations monitoring, strategic decision-making
7. **Quality Manager**: QC review, audit management
8. **Billing Staff**: Invoice generation, payment processing, collections

Each flow diagram shows:
- Decision points and branches
- Error handling and edge cases
- System interactions
- Integration touchpoints
- Compliance checkpoints
- User experience optimization

These flows support NABL ISO 15189:2022 compliance and are optimized for the Indian healthcare market with features like WhatsApp integration, UPI payments, and Aadhaar verification.

---

**Document Version:** 1.0
**Last Updated:** 2025-11-05
**Status:** Complete
