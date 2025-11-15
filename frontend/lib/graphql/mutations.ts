import { gql } from '@apollo/client';

// ============================================================================
// PATIENT MUTATIONS
// ============================================================================

export const CREATE_PATIENT = gql`
  mutation CreatePatient($input: CreatePatientInput!) {
    createPatient(input: $input) {
      id
      patientId
      firstName
      lastName
      dateOfBirth
      gender
      email
      phone
      status
      createdAt
    }
  }
`;

export const UPDATE_PATIENT = gql`
  mutation UpdatePatient($id: ID!, $input: UpdatePatientInput!) {
    updatePatient(id: $id, input: $input) {
      id
      patientId
      firstName
      lastName
      dateOfBirth
      gender
      email
      phone
      address {
        street
        city
        state
        postalCode
        country
      }
      emergencyContact {
        name
        relationship
        phone
      }
      medicalHistory {
        allergies
        medications
        conditions
        notes
      }
      insuranceInfo {
        provider
        policyNumber
        groupNumber
        validUntil
      }
      status
      updatedAt
    }
  }
`;

export const DELETE_PATIENT = gql`
  mutation DeletePatient($id: ID!) {
    deletePatient(id: $id) {
      success
      message
    }
  }
`;

export const MERGE_PATIENTS = gql`
  mutation MergePatients($sourceId: ID!, $targetId: ID!) {
    mergePatients(sourceId: $sourceId, targetId: $targetId) {
      id
      patientId
      firstName
      lastName
    }
  }
`;

// ============================================================================
// ORDER MUTATIONS
// ============================================================================

export const CREATE_ORDER = gql`
  mutation CreateOrder($input: CreateOrderInput!) {
    createOrder(input: $input) {
      id
      orderId
      patient {
        id
        patientId
        firstName
        lastName
      }
      orderDate
      priority
      status
      tests {
        id
        testCode
        testName
      }
      doctor {
        id
        name
      }
      createdAt
    }
  }
`;

export const UPDATE_ORDER = gql`
  mutation UpdateOrder($id: ID!, $input: UpdateOrderInput!) {
    updateOrder(id: $id, input: $input) {
      id
      orderId
      priority
      status
      tests {
        id
        testCode
        testName
        status
      }
      updatedAt
    }
  }
`;

export const CANCEL_ORDER = gql`
  mutation CancelOrder($id: ID!, $reason: String!) {
    cancelOrder(id: $id, reason: $reason) {
      id
      orderId
      status
      cancellationReason
      cancelledAt
    }
  }
`;

export const ADD_TESTS_TO_ORDER = gql`
  mutation AddTestsToOrder($orderId: ID!, $testIds: [ID!]!) {
    addTestsToOrder(orderId: $orderId, testIds: $testIds) {
      id
      orderId
      tests {
        id
        testCode
        testName
      }
    }
  }
`;

// ============================================================================
// SAMPLE MUTATIONS
// ============================================================================

export const COLLECT_SAMPLE = gql`
  mutation CollectSample($input: CollectSampleInput!) {
    collectSample(input: $input) {
      id
      sampleId
      patient {
        id
        patientId
        firstName
        lastName
      }
      order {
        id
        orderId
      }
      sampleType
      collectionDate
      status
      collectedBy {
        id
        name
      }
      createdAt
    }
  }
`;

export const RECEIVE_SAMPLE = gql`
  mutation ReceiveSample($id: ID!, $input: ReceiveSampleInput!) {
    receiveSample(id: $id, input: $input) {
      id
      sampleId
      receivedDate
      receivedTime
      status
      receivedBy {
        id
        name
      }
      updatedAt
    }
  }
`;

export const REJECT_SAMPLE = gql`
  mutation RejectSample($id: ID!, $reason: String!, $notes: String) {
    rejectSample(id: $id, reason: $reason, notes: $notes) {
      id
      sampleId
      status
      rejectionInfo {
        reason
        rejectedBy {
          id
          name
        }
        rejectedAt
        notes
      }
    }
  }
`;

export const UPDATE_SAMPLE_STATUS = gql`
  mutation UpdateSampleStatus($id: ID!, $status: SampleStatus!, $location: String) {
    updateSampleStatus(id: $id, status: $status, location: $location) {
      id
      sampleId
      status
      location
      updatedAt
    }
  }
`;

export const UPDATE_SAMPLE_LOCATION = gql`
  mutation UpdateSampleLocation($id: ID!, $location: String!) {
    updateSampleLocation(id: $id, location: $location) {
      id
      sampleId
      location
      updatedAt
    }
  }
`;

// ============================================================================
// RESULT MUTATIONS
// ============================================================================

export const ENTER_RESULT = gql`
  mutation EnterResult($input: EnterResultInput!) {
    enterResult(input: $input) {
      id
      sample {
        id
        sampleId
      }
      test {
        id
        testCode
        testName
      }
      parameters {
        id
        parameterName
        value
        unit
        referenceRange
        isNormal
        isCritical
      }
      status
      resultDate
      isCritical
      enteredBy {
        id
        name
      }
      createdAt
    }
  }
`;

export const VERIFY_RESULT = gql`
  mutation VerifyResult($id: ID!, $comments: String) {
    verifyResult(id: $id, comments: $comments) {
      id
      status
      verifiedBy {
        id
        name
      }
      verifiedAt
      comments
    }
  }
`;

export const APPROVE_RESULT = gql`
  mutation ApproveResult($id: ID!, $interpretation: String) {
    approveResult(id: $id, interpretation: $interpretation) {
      id
      status
      approvedBy {
        id
        name
      }
      approvedAt
      interpretation
    }
  }
`;

export const REJECT_RESULT = gql`
  mutation RejectResult($id: ID!, $reason: String!) {
    rejectResult(id: $id, reason: $reason) {
      id
      status
      rejectionReason
      rejectedBy {
        id
        name
      }
      rejectedAt
    }
  }
`;

export const AMEND_RESULT = gql`
  mutation AmendResult($id: ID!, $input: AmendResultInput!) {
    amendResult(id: $id, input: $input) {
      id
      parameters {
        id
        parameterName
        value
        unit
      }
      amendments {
        id
        reason
        previousValue
        newValue
        amendedBy {
          id
          name
        }
        amendedAt
      }
      updatedAt
    }
  }
`;

export const BATCH_ENTER_RESULTS = gql`
  mutation BatchEnterResults($results: [EnterResultInput!]!) {
    batchEnterResults(results: $results) {
      success
      count
      errors {
        index
        message
      }
    }
  }
`;

// ============================================================================
// REPORT MUTATIONS
// ============================================================================

export const GENERATE_REPORT = gql`
  mutation GenerateReport($input: GenerateReportInput!) {
    generateReport(input: $input) {
      id
      reportNumber
      patient {
        id
        patientId
        firstName
        lastName
      }
      order {
        id
        orderId
      }
      reportType
      status
      generatedAt
      pdfUrl
    }
  }
`;

export const APPROVE_REPORT = gql`
  mutation ApproveReport($id: ID!, $signature: String) {
    approveReport(id: $id, signature: $signature) {
      id
      reportNumber
      status
      approvedBy {
        id
        name
        signature
      }
      approvedAt
    }
  }
`;

export const DELIVER_REPORT = gql`
  mutation DeliverReport($id: ID!, $deliveryMethod: DeliveryMethod!, $recipient: String!) {
    deliverReport(id: $id, deliveryMethod: $deliveryMethod, recipient: $recipient) {
      id
      reportNumber
      deliveryMethod
      deliveredAt
      deliveryStatus
    }
  }
`;

export const AMEND_REPORT = gql`
  mutation AmendReport($id: ID!, $reason: String!, $changes: JSON!) {
    amendReport(id: $id, reason: $reason, changes: $changes) {
      id
      reportNumber
      status
      amendments {
        id
        reason
        amendedBy {
          id
          name
        }
        amendedAt
      }
    }
  }
`;

// ============================================================================
// QC MUTATIONS
// ============================================================================

export const ENTER_QC_RUN = gql`
  mutation EnterQCRun($input: EnterQCRunInput!) {
    enterQCRun(input: $input) {
      id
      test {
        id
        testCode
        testName
      }
      material {
        id
        name
        lotNumber
      }
      runDate
      value
      mean
      sd
      cv
      status
      rulesViolated
      operator {
        id
        name
      }
      createdAt
    }
  }
`;

export const CREATE_QC_MATERIAL = gql`
  mutation CreateQCMaterial($input: CreateQCMaterialInput!) {
    createQCMaterial(input: $input) {
      id
      name
      manufacturer
      catalogNumber
      lotNumber
      level
      expiryDate
      targetValue
      allowableSD
      isActive
    }
  }
`;

export const UPDATE_QC_MATERIAL = gql`
  mutation UpdateQCMaterial($id: ID!, $input: UpdateQCMaterialInput!) {
    updateQCMaterial(id: $id, input: $input) {
      id
      name
      lotNumber
      expiryDate
      targetValue
      allowableSD
      isActive
      updatedAt
    }
  }
`;

export const INVESTIGATE_QC_FAILURE = gql`
  mutation InvestigateQCFailure($id: ID!, $investigation: String!, $correctiveAction: String!) {
    investigateQCFailure(id: $id, investigation: $investigation, correctiveAction: $correctiveAction) {
      id
      investigation {
        findings
        correctiveAction
        investigatedBy {
          id
          name
        }
        investigatedAt
      }
    }
  }
`;

// ============================================================================
// EQUIPMENT MUTATIONS
// ============================================================================

export const CREATE_EQUIPMENT = gql`
  mutation CreateEquipment($input: CreateEquipmentInput!) {
    createEquipment(input: $input) {
      id
      equipmentId
      name
      manufacturer
      model
      serialNumber
      department
      location
      status
      createdAt
    }
  }
`;

export const UPDATE_EQUIPMENT = gql`
  mutation UpdateEquipment($id: ID!, $input: UpdateEquipmentInput!) {
    updateEquipment(id: $id, input: $input) {
      id
      equipmentId
      name
      location
      status
      updatedAt
    }
  }
`;

export const LOG_MAINTENANCE = gql`
  mutation LogMaintenance($equipmentId: ID!, $input: LogMaintenanceInput!) {
    logMaintenance(equipmentId: $equipmentId, input: $input) {
      id
      date
      type
      description
      performedBy {
        id
        name
      }
      cost
      vendor
      nextDue
    }
  }
`;

export const LOG_CALIBRATION = gql`
  mutation LogCalibration($equipmentId: ID!, $input: LogCalibrationInput!) {
    logCalibration(equipmentId: $equipmentId, input: $input) {
      id
      date
      standard
      result
      performedBy {
        id
        name
      }
      nextDue
    }
  }
`;

export const UPDATE_EQUIPMENT_STATUS = gql`
  mutation UpdateEquipmentStatus($id: ID!, $status: EquipmentStatus!, $notes: String) {
    updateEquipmentStatus(id: $id, status: $status, notes: $notes) {
      id
      equipmentId
      status
      updatedAt
    }
  }
`;

// ============================================================================
// INVENTORY MUTATIONS
// ============================================================================

export const CREATE_INVENTORY_ITEM = gql`
  mutation CreateInventoryItem($input: CreateInventoryItemInput!) {
    createInventoryItem(input: $input) {
      id
      itemCode
      itemName
      category
      currentStock
      reorderLevel
      unit
      location
      createdAt
    }
  }
`;

export const UPDATE_INVENTORY_ITEM = gql`
  mutation UpdateInventoryItem($id: ID!, $input: UpdateInventoryItemInput!) {
    updateInventoryItem(id: $id, input: $input) {
      id
      itemCode
      itemName
      currentStock
      reorderLevel
      updatedAt
    }
  }
`;

export const STOCK_IN = gql`
  mutation StockIn($itemId: ID!, $input: StockInInput!) {
    stockIn(itemId: $itemId, input: $input) {
      id
      type
      quantity
      date
      lotNumber
      expiryDate
      balanceAfter
      user {
        id
        name
      }
    }
  }
`;

export const STOCK_OUT = gql`
  mutation StockOut($itemId: ID!, $input: StockOutInput!) {
    stockOut(itemId: $itemId, input: $input) {
      id
      type
      quantity
      date
      reason
      balanceAfter
      user {
        id
        name
      }
    }
  }
`;

export const ADJUST_STOCK = gql`
  mutation AdjustStock($itemId: ID!, $input: AdjustStockInput!) {
    adjustStock(itemId: $itemId, input: $input) {
      id
      type
      quantity
      date
      reason
      balanceAfter
      user {
        id
        name
      }
    }
  }
`;

// ============================================================================
// BILLING MUTATIONS
// ============================================================================

export const CREATE_INVOICE = gql`
  mutation CreateInvoice($input: CreateInvoiceInput!) {
    createInvoice(input: $input) {
      id
      invoiceNumber
      patient {
        id
        patientId
        firstName
        lastName
      }
      order {
        id
        orderId
      }
      invoiceDate
      dueDate
      totalAmount
      balance
      status
      createdAt
    }
  }
`;

export const RECORD_PAYMENT = gql`
  mutation RecordPayment($invoiceId: ID!, $input: RecordPaymentInput!) {
    recordPayment(invoiceId: $invoiceId, input: $input) {
      id
      invoiceNumber
      paidAmount
      balance
      status
      payments {
        id
        date
        amount
        method
        reference
      }
    }
  }
`;

export const ISSUE_REFUND = gql`
  mutation IssueRefund($invoiceId: ID!, $amount: Float!, $reason: String!, $method: String!) {
    issueRefund(invoiceId: $invoiceId, amount: $amount, reason: $reason, method: $method) {
      id
      invoiceNumber
      refunds {
        id
        date
        amount
        reason
        method
        issuedBy {
          id
          name
        }
      }
    }
  }
`;

export const SEND_PAYMENT_REMINDER = gql`
  mutation SendPaymentReminder($invoiceId: ID!, $method: String!) {
    sendPaymentReminder(invoiceId: $invoiceId, method: $method) {
      success
      message
      sentAt
    }
  }
`;

// ============================================================================
// USER MUTATIONS
// ============================================================================

export const CREATE_USER = gql`
  mutation CreateUser($input: CreateUserInput!) {
    createUser(input: $input) {
      id
      email
      firstName
      lastName
      role
      department
      permissions
      status
      createdAt
    }
  }
`;

export const UPDATE_USER = gql`
  mutation UpdateUser($id: ID!, $input: UpdateUserInput!) {
    updateUser(id: $id, input: $input) {
      id
      email
      firstName
      lastName
      phone
      role
      department
      permissions
      status
      updatedAt
    }
  }
`;

export const DELETE_USER = gql`
  mutation DeleteUser($id: ID!) {
    deleteUser(id: $id) {
      success
      message
    }
  }
`;

export const DEACTIVATE_USER = gql`
  mutation DeactivateUser($id: ID!) {
    deactivateUser(id: $id) {
      id
      status
      deactivatedAt
    }
  }
`;

export const ACTIVATE_USER = gql`
  mutation ActivateUser($id: ID!) {
    activateUser(id: $id) {
      id
      status
      activatedAt
    }
  }
`;

export const RESET_USER_PASSWORD = gql`
  mutation ResetUserPassword($id: ID!, $newPassword: String!) {
    resetUserPassword(id: $id, newPassword: $newPassword) {
      success
      message
    }
  }
`;

// ============================================================================
// ORGANIZATION MUTATIONS
// ============================================================================

export const UPDATE_ORGANIZATION = gql`
  mutation UpdateOrganization($input: UpdateOrganizationInput!) {
    updateOrganization(input: $input) {
      id
      name
      email
      phone
      website
      address {
        street
        city
        state
        postalCode
        country
      }
      logo
      licenseNumber
      accreditation
      updatedAt
    }
  }
`;

export const UPDATE_ORGANIZATION_SETTINGS = gql`
  mutation UpdateOrganizationSettings($input: UpdateOrganizationSettingsInput!) {
    updateOrganizationSettings(input: $input) {
      id
      settings {
        timezone
        dateFormat
        currency
        language
      }
      updatedAt
    }
  }
`;

export const UPDATE_INTEGRATION_SETTINGS = gql`
  mutation UpdateIntegrationSettings($integration: String!, $settings: JSON!) {
    updateIntegrationSettings(integration: $integration, settings: $settings) {
      id
      integrations
      updatedAt
    }
  }
`;

// ============================================================================
// NOTIFICATION MUTATIONS
// ============================================================================

export const MARK_NOTIFICATION_READ = gql`
  mutation MarkNotificationRead($id: ID!) {
    markNotificationRead(id: $id) {
      id
      read
      readAt
    }
  }
`;

export const MARK_ALL_NOTIFICATIONS_READ = gql`
  mutation MarkAllNotificationsRead {
    markAllNotificationsRead {
      success
      count
    }
  }
`;

export const DELETE_NOTIFICATION = gql`
  mutation DeleteNotification($id: ID!) {
    deleteNotification(id: $id) {
      success
      message
    }
  }
`;

// ============================================================================
// AUTHENTICATION MUTATIONS (Already exist in auth pages, included for completeness)
// ============================================================================

export const LOGIN_MUTATION = gql`
  mutation Login($email: String!, $password: String!) {
    login(email: $email, password: $password) {
      token
      user {
        id
        email
        firstName
        lastName
        role
        permissions
        organizationId
      }
    }
  }
`;

export const REGISTER_MUTATION = gql`
  mutation Register($input: RegisterInput!) {
    register(input: $input) {
      token
      user {
        id
        email
        firstName
        lastName
        role
        organizationId
      }
    }
  }
`;

export const FORGOT_PASSWORD_MUTATION = gql`
  mutation ForgotPassword($email: String!) {
    forgotPassword(email: $email) {
      success
      message
    }
  }
`;

export const RESET_PASSWORD_MUTATION = gql`
  mutation ResetPassword($token: String!, $newPassword: String!) {
    resetPassword(token: $token, newPassword: $newPassword) {
      success
      message
    }
  }
`;

export const CHANGE_PASSWORD = gql`
  mutation ChangePassword($currentPassword: String!, $newPassword: String!) {
    changePassword(currentPassword: $currentPassword, newPassword: $newPassword) {
      success
      message
    }
  }
`;

export const UPDATE_PROFILE = gql`
  mutation UpdateProfile($input: UpdateProfileInput!) {
    updateProfile(input: $input) {
      id
      email
      firstName
      lastName
      phone
      updatedAt
    }
  }
`;

// ==============================================================================
// ADDITIONAL MUTATIONS FOR PHASE 3 MODULES
// ==============================================================================

export const CREATE_QC_RUN = gql`
  mutation CreateQCRun($input: CreateQCRunInput!) {
    createQCRun(input: $input) {
      id
      testId
      lotNumber
      level
      measuredValue
      meanValue
      standardDeviation
      zScore
      qcStatus
      performedAt
      performedBy {
        firstName
        lastName
      }
    }
  }
`;

export const DELETE_QC_RUN = gql`
  mutation DeleteQCRun($id: ID!) {
    deleteQCRun(id: $id) {
      success
      message
    }
  }
`;

export const SCHEDULE_MAINTENANCE = gql`
  mutation ScheduleMaintenance($input: ScheduleMaintenanceInput!) {
    scheduleMaintenance(input: $input) {
      id
      equipmentId
      maintenanceType
      scheduledDate
      description
      status
    }
  }
`;

export const DELETE_EQUIPMENT = gql`
  mutation DeleteEquipment($id: ID!) {
    deleteEquipment(id: $id) {
      success
      message
    }
  }
`;

export const RECORD_TRANSACTION = gql`
  mutation RecordTransaction($itemId: ID!, $input: RecordTransactionInput!) {
    recordTransaction(itemId: $itemId, input: $input) {
      id
      transactionType
      quantity
      reason
      reference
      balanceAfter
      createdAt
    }
  }
`;

export const DELETE_INVENTORY_ITEM = gql`
  mutation DeleteInventoryItem($id: ID!) {
    deleteInventoryItem(id: $id) {
      success
      message
    }
  }
`;

export const DELETE_INVOICE = gql`
  mutation DeleteInvoice($id: ID!) {
    deleteInvoice(id: $id) {
      success
      message
    }
  }
`;
