import { gql } from '@apollo/client';

// ============================================================================
// PATIENT QUERIES
// ============================================================================

export const GET_PATIENTS = gql`
  query GetPatients(
    $page: Int
    $limit: Int
    $search: String
    $filters: PatientFilters
    $sort: PatientSort
  ) {
    patients(page: $page, limit: $limit, search: $search, filters: $filters, sort: $sort) {
      data {
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
        updatedAt
      }
      pagination {
        total
        page
        limit
        totalPages
      }
    }
  }
`;

export const GET_PATIENT = gql`
  query GetPatient($id: ID!) {
    patient(id: $id) {
      id
      patientId
      firstName
      lastName
      middleName
      dateOfBirth
      gender
      email
      phone
      alternatePhone
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
      createdAt
      updatedAt
      createdBy {
        id
        name
      }
    }
  }
`;

export const SEARCH_PATIENTS = gql`
  query SearchPatients($query: String!, $limit: Int) {
    searchPatients(query: $query, limit: $limit) {
      id
      patientId
      firstName
      lastName
      dateOfBirth
      gender
      phone
    }
  }
`;

// ============================================================================
// ORDER QUERIES
// ============================================================================

export const GET_ORDERS = gql`
  query GetOrders(
    $page: Int
    $limit: Int
    $search: String
    $filters: OrderFilters
    $sort: OrderSort
  ) {
    orders(page: $page, limit: $limit, search: $search, filters: $filters, sort: $sort) {
      data {
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
        totalTests
        completedTests
        doctor {
          id
          name
        }
        createdAt
      }
      pagination {
        total
        page
        limit
        totalPages
      }
    }
  }
`;

export const GET_ORDER = gql`
  query GetOrder($id: ID!) {
    order(id: $id) {
      id
      orderId
      patient {
        id
        patientId
        firstName
        lastName
        dateOfBirth
        gender
        phone
      }
      orderDate
      priority
      status
      tests {
        id
        testCode
        testName
        category
        price
        status
        sample {
          id
          sampleId
          status
        }
        result {
          id
          status
        }
      }
      doctor {
        id
        name
        specialty
        licenseNumber
      }
      clinicalInfo {
        diagnosis
        symptoms
        notes
      }
      billingInfo {
        totalAmount
        paidAmount
        balance
        paymentStatus
      }
      createdBy {
        id
        name
      }
      createdAt
      updatedAt
    }
  }
`;

// ============================================================================
// SAMPLE QUERIES
// ============================================================================

export const GET_SAMPLES = gql`
  query GetSamples(
    $page: Int
    $limit: Int
    $search: String
    $filters: SampleFilters
    $sort: SampleSort
  ) {
    samples(page: $page, limit: $limit, search: $search, filters: $filters, sort: $sort) {
      data {
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
        receivedDate
        status
        priority
        location
        collectedBy {
          id
          name
        }
        createdAt
      }
      pagination {
        total
        page
        limit
        totalPages
      }
    }
  }
`;

export const GET_SAMPLE = gql`
  query GetSample($id: ID!) {
    sample(id: $id) {
      id
      sampleId
      patient {
        id
        patientId
        firstName
        lastName
        dateOfBirth
        gender
      }
      order {
        id
        orderId
        tests {
          id
          testCode
          testName
        }
      }
      sampleType
      containerType
      volume
      collectionDate
      collectionTime
      receivedDate
      receivedTime
      status
      priority
      location
      storageConditions
      collectedBy {
        id
        name
      }
      receivedBy {
        id
        name
      }
      chainOfCustody {
        timestamp
        action
        user {
          id
          name
        }
        location
        notes
      }
      rejectionInfo {
        reason
        rejectedBy {
          id
          name
        }
        rejectedAt
        notes
      }
      createdAt
      updatedAt
    }
  }
`;

// ============================================================================
// RESULT QUERIES
// ============================================================================

export const GET_RESULTS = gql`
  query GetResults(
    $page: Int
    $limit: Int
    $search: String
    $filters: ResultFilters
    $sort: ResultSort
  ) {
    results(page: $page, limit: $limit, search: $search, filters: $filters, sort: $sort) {
      data {
        id
        sample {
          id
          sampleId
          patient {
            id
            patientId
            firstName
            lastName
          }
        }
        test {
          id
          testCode
          testName
        }
        status
        resultDate
        verifiedAt
        approvedAt
        isCritical
        createdAt
      }
      pagination {
        total
        page
        limit
        totalPages
      }
    }
  }
`;

export const GET_RESULT = gql`
  query GetResult($id: ID!) {
    result(id: $id) {
      id
      sample {
        id
        sampleId
        patient {
          id
          patientId
          firstName
          lastName
          dateOfBirth
          gender
        }
        order {
          id
          orderId
        }
      }
      test {
        id
        testCode
        testName
        category
        methodology
        referenceRanges {
          minAge
          maxAge
          gender
          normalMin
          normalMax
          unit
        }
      }
      parameters {
        id
        parameterName
        value
        unit
        referenceRange
        isNormal
        isCritical
        flags
      }
      status
      resultDate
      interpretation
      comments
      isCritical
      deltaCheck {
        previousValue
        currentValue
        percentChange
        flag
      }
      enteredBy {
        id
        name
      }
      verifiedBy {
        id
        name
      }
      approvedBy {
        id
        name
      }
      verifiedAt
      approvedAt
      createdAt
      updatedAt
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
    }
  }
`;

// ============================================================================
// REPORT QUERIES
// ============================================================================

export const GET_REPORTS = gql`
  query GetReports(
    $page: Int
    $limit: Int
    $search: String
    $filters: ReportFilters
    $sort: ReportSort
  ) {
    reports(page: $page, limit: $limit, search: $search, filters: $filters, sort: $sort) {
      data {
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
        deliveredAt
        createdAt
      }
      pagination {
        total
        page
        limit
        totalPages
      }
    }
  }
`;

export const GET_REPORT = gql`
  query GetReport($id: ID!) {
    report(id: $id) {
      id
      reportNumber
      patient {
        id
        patientId
        firstName
        lastName
        dateOfBirth
        gender
        phone
        address {
          street
          city
          state
          postalCode
        }
      }
      order {
        id
        orderId
        orderDate
        doctor {
          id
          name
          specialty
          licenseNumber
        }
      }
      results {
        id
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
        interpretation
      }
      reportType
      status
      template
      content
      generatedAt
      generatedBy {
        id
        name
      }
      approvedBy {
        id
        name
        signature
      }
      approvedAt
      deliveryMethod
      deliveredAt
      deliveryStatus
      pdfUrl
      createdAt
      updatedAt
    }
  }
`;

// ============================================================================
// TEST CATALOG QUERIES
// ============================================================================

export const GET_TEST_CATALOG = gql`
  query GetTestCatalog($search: String, $category: String) {
    testCatalog(search: $search, category: $category) {
      id
      testCode
      testName
      category
      department
      sampleType
      methodology
      turnaroundTime
      price
      isActive
      description
    }
  }
`;

export const GET_TEST_CATEGORIES = gql`
  query GetTestCategories {
    testCategories {
      id
      name
      code
      testCount
    }
  }
`;

// ============================================================================
// QC QUERIES
// ============================================================================

export const GET_QC_RUNS = gql`
  query GetQCRuns(
    $page: Int
    $limit: Int
    $testId: ID
    $filters: QCFilters
  ) {
    qcRuns(page: $page, limit: $limit, testId: $testId, filters: $filters) {
      data {
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
          level
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
      pagination {
        total
        page
        limit
        totalPages
      }
    }
  }
`;

export const GET_QC_STATISTICS = gql`
  query GetQCStatistics($testId: ID!, $materialId: ID!, $dateRange: DateRange) {
    qcStatistics(testId: $testId, materialId: $materialId, dateRange: $dateRange) {
      mean
      sd
      cv
      runs {
        date
        value
      }
      controlLimits {
        mean
        plus1SD
        minus1SD
        plus2SD
        minus2SD
        plus3SD
        minus3SD
      }
    }
  }
`;

export const GET_QC_MATERIALS = gql`
  query GetQCMaterials($testId: ID) {
    qcMaterials(testId: $testId) {
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

// ============================================================================
// EQUIPMENT QUERIES
// ============================================================================

export const GET_EQUIPMENT = gql`
  query GetEquipment($page: Int, $limit: Int, $search: String) {
    equipment(page: $page, limit: $limit, search: $search) {
      data {
        id
        equipmentId
        name
        manufacturer
        model
        serialNumber
        department
        location
        status
        lastMaintenanceDate
        nextMaintenanceDate
        createdAt
      }
      pagination {
        total
        page
        limit
        totalPages
      }
    }
  }
`;

export const GET_EQUIPMENT_DETAIL = gql`
  query GetEquipmentDetail($id: ID!) {
    equipment(id: $id) {
      id
      equipmentId
      name
      manufacturer
      model
      serialNumber
      purchaseDate
      warrantyExpiry
      department
      location
      status
      specifications
      lastMaintenanceDate
      nextMaintenanceDate
      maintenanceSchedule {
        id
        frequency
        taskDescription
        lastPerformed
        nextDue
      }
      maintenanceHistory {
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
      }
      calibrationHistory {
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
      createdAt
      updatedAt
    }
  }
`;

// ============================================================================
// INVENTORY QUERIES
// ============================================================================

export const GET_INVENTORY = gql`
  query GetInventory($page: Int, $limit: Int, $search: String, $filters: InventoryFilters) {
    inventory(page: $page, limit: $limit, search: $search, filters: $filters) {
      data {
        id
        itemCode
        itemName
        category
        currentStock
        reorderLevel
        unit
        location
        expiryDate
        status
        lastRestocked
      }
      pagination {
        total
        page
        limit
        totalPages
      }
    }
  }
`;

export const GET_INVENTORY_ITEM = gql`
  query GetInventoryItem($id: ID!) {
    inventoryItem(id: $id) {
      id
      itemCode
      itemName
      category
      manufacturer
      catalogNumber
      currentStock
      reorderLevel
      maxStock
      unit
      unitPrice
      location
      storageConditions
      expiryDate
      status
      lots {
        id
        lotNumber
        quantity
        expiryDate
        receivedDate
        location
      }
      transactions {
        id
        type
        quantity
        date
        reason
        user {
          id
          name
        }
        balanceAfter
      }
      lastRestocked
      createdAt
      updatedAt
    }
  }
`;

// ============================================================================
// BILLING QUERIES
// ============================================================================

export const GET_INVOICES = gql`
  query GetInvoices($page: Int, $limit: Int, $search: String, $filters: InvoiceFilters) {
    invoices(page: $page, limit: $limit, search: $search, filters: $filters) {
      data {
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
        paidAmount
        balance
        status
        createdAt
      }
      pagination {
        total
        page
        limit
        totalPages
      }
    }
  }
`;

export const GET_INVOICE = gql`
  query GetInvoice($id: ID!) {
    invoice(id: $id) {
      id
      invoiceNumber
      patient {
        id
        patientId
        firstName
        lastName
        phone
        email
        address {
          street
          city
          state
          postalCode
        }
      }
      order {
        id
        orderId
        orderDate
      }
      invoiceDate
      dueDate
      items {
        id
        description
        quantity
        unitPrice
        discount
        tax
        amount
      }
      subtotal
      discount
      tax
      totalAmount
      paidAmount
      balance
      status
      payments {
        id
        date
        amount
        method
        reference
        receivedBy {
          id
          name
        }
      }
      notes
      terms
      createdAt
      updatedAt
    }
  }
`;

// ============================================================================
// USER QUERIES
// ============================================================================

export const GET_USERS = gql`
  query GetUsers($page: Int, $limit: Int, $search: String, $filters: UserFilters) {
    users(page: $page, limit: $limit, search: $search, filters: $filters) {
      data {
        id
        email
        firstName
        lastName
        role
        department
        status
        lastLogin
        createdAt
      }
      pagination {
        total
        page
        limit
        totalPages
      }
    }
  }
`;

export const GET_USER = gql`
  query GetUser($id: ID!) {
    user(id: $id) {
      id
      email
      firstName
      lastName
      phone
      role
      department
      permissions
      status
      lastLogin
      createdAt
      updatedAt
    }
  }
`;

// ============================================================================
// ORGANIZATION QUERIES
// ============================================================================

export const GET_ORGANIZATION = gql`
  query GetOrganization {
    organization {
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
      settings {
        timezone
        dateFormat
        currency
        language
      }
      integrations {
        abdm {
          enabled
          facilityId
        }
        whatsapp {
          enabled
          businessId
        }
        payment {
          enabled
          gateway
        }
      }
    }
  }
`;

// ============================================================================
// DASHBOARD QUERIES
// ============================================================================

export const GET_DASHBOARD_STATS = gql`
  query GetDashboardStats($dateRange: DateRange) {
    dashboardStats(dateRange: $dateRange) {
      totalPatients
      totalOrders
      pendingSamples
      pendingResults
      criticalResults
      todayRevenue
      trends {
        patients
        orders
        revenue
      }
      recentActivity {
        type
        description
        timestamp
        user {
          id
          name
        }
      }
    }
  }
`;

// ============================================================================
// NOTIFICATION QUERIES
// ============================================================================

export const GET_NOTIFICATIONS = gql`
  query GetNotifications($page: Int, $limit: Int, $unreadOnly: Boolean) {
    notifications(page: $page, limit: $limit, unreadOnly: $unreadOnly) {
      data {
        id
        type
        title
        message
        read
        createdAt
        metadata
      }
      pagination {
        total
        page
        limit
        totalPages
      }
      unreadCount
    }
  }
`;
