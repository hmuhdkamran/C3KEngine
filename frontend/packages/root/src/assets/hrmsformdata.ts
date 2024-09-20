interface FormField {
  label: string;
  type: string;
  options?: string[];
}

interface FormConfig {
  title: string;
  fields: FormField[];
}

export const hrmsform: Record<string, FormConfig> = {
  'Employee Management': {
    title: 'Employee Management',
    fields: [
      { label: 'Employee Name', type: 'text' },
      { label: 'Position', type: 'text' },
      { label: 'Department', type: 'text' },
      { label: 'Hire Date', type: 'date' },
      { label: 'Employment Type', type: 'select', options: ['Full-Time', 'Part-Time', 'Contract'] },
      { label: 'Supervisor', type: 'text' },
    ],
  },
  'Payroll': {
    title: 'Payroll',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Salary', type: 'number' },
      { label: 'Tax Rate', type: 'number' },
      { label: 'Payment Date', type: 'date' },
      { label: 'Bonus Amount', type: 'number' },
      { label: 'Deductions', type: 'number' },
    ],
  },
  'Attendance Tracking': {
    title: 'Attendance Tracking',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Check-in Time', type: 'time' },
      { label: 'Check-out Time', type: 'time' },
      { label: 'Date', type: 'date' },
      { label: 'Overtime Hours', type: 'number' },
      { label: 'Total Hours', type: 'number' },
      { label: 'Notes', type: 'text' },
    ],
  },
  'Recruitment': {
    title: 'Recruitment',
    fields: [
      { label: 'Candidate Name', type: 'text' },
      { label: 'Position Applied', type: 'text' },
      { label: 'Interview Date', type: 'date' },
      { label: 'Status', type: 'select', options: ['Scheduled', 'Completed', 'Rejected', 'Hired'] },
      { label: 'Interview Feedback', type: 'text' },
      { label: 'Resume', type: 'file' },
      { label: 'Notes', type: 'text' },
    ],
  },
  'Benefits Management': {
    title: 'Benefits Management',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Benefit Type', type: 'select', options: ['Health Insurance', 'Retirement', 'Annual Leave'] },
      { label: 'Coverage Amount', type: 'number' },
      { label: 'Effective Date', type: 'date' },
      { label: 'Dependent Coverage', type: 'text' },
      { label: 'Enrollment Status', type: 'select', options: ['Active', 'Pending', 'Expired'] },
    ],
  },
  'Training': {
    title: 'Training',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Training Program', type: 'text' },
      { label: 'Training Date', type: 'date' },
      { label: 'Completion Status', type: 'select', options: ['Completed', 'Pending', 'Failed'] },
      { label: 'Trainer Name', type: 'text' },
      { label: 'Feedback', type: 'text' },
      { label: 'Certifications', type: 'text' },
    ],
  },
  'Performance Management': {
    title: 'Performance Management',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Performance Rating', type: 'select', options: ['1 - Poor', '2 - Fair', '3 - Good', '4 - Excellent'] },
      { label: 'Review Date', type: 'date' },
      { label: 'Comments', type: 'text' },
      { label: 'Next Review Date', type: 'date' },
      { label: 'Goals Set', type: 'text' },
      { label: 'Improvement Areas', type: 'text' },
    ],
  },
  'Employee Self-Service': {
    title: 'Employee Self-Service',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Change Request', type: 'select', options: ['Address Change', 'Contact Information', 'Bank Details'] },
      { label: 'Request Date', type: 'date' },
      { label: 'Approval Status', type: 'select', options: ['Approved', 'Pending', 'Rejected'] },
      { label: 'Remarks', type: 'text' },
    ],
  },
  'Time-Off Management': {
    title: 'Time-Off Management',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Leave Type', type: 'select', options: ['Sick Leave', 'Casual Leave', 'Annual Leave'] },
      { label: 'Start Date', type: 'date' },
      { label: 'End Date', type: 'date' },
      { label: 'Approval Status', type: 'select', options: ['Approved', 'Pending', 'Rejected'] },
    ],
  },
  'Expense Management': {
    title: 'Expense Management',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Expense Category', type: 'select', options: ['Travel', 'Meals', 'Office Supplies', 'Other'] },
      { label: 'Amount', type: 'number' },
      { label: 'Submission Date', type: 'date' },
      { label: 'Approval Status', type: 'select', options: ['Approved', 'Rejected', 'Pending'] },
    ],
  },
  'Onboarding': {
    title: 'Onboarding',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Start Date', type: 'date' },
      { label: 'Assigned Mentor', type: 'text' },
      { label: 'Orientation Completed', type: 'select', options: ['Yes', 'No'] },
      { label: 'First Week Goals', type: 'text' },
    ],
  },
  'Offboarding': {
    title: 'Offboarding',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Exit Interview Date', type: 'date' },
      { label: 'Reason for Leaving', type: 'text' },
      { label: 'Last Working Day', type: 'date' },
      { label: 'Asset Return Status', type: 'select', options: ['Returned', 'Pending'] },
    ],
  },
  'Employee Surveys': {
    title: 'Employee Surveys',
    fields: [
      { label: 'Survey Title', type: 'text' },
      { label: 'Employee ID', type: 'text' },
      { label: 'Completion Date', type: 'date' },
      { label: 'Satisfaction Rating', type: 'select', options: ['1 - Very Dissatisfied', '2 - Dissatisfied', '3 - Neutral', '4 - Satisfied', '5 - Very Satisfied'] },
      { label: 'Additional Feedback', type: 'text' },
    ],
  },
  'Health & Safety': {
    title: 'Health & Safety',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Incident Reported', type: 'text' },
      { label: 'Incident Date', type: 'date' },
      { label: 'Resolution Status', type: 'select', options: ['Resolved', 'Unresolved'] },
      { label: 'Safety Training Completed', type: 'select', options: ['Yes', 'No'] },
    ],
  },
  'Compliance Management': {
    title: 'Compliance Management',
    fields: [
      { label: 'Regulation Name', type: 'text' },
      { label: 'Compliance Status', type: 'select', options: ['Compliant', 'Non-Compliant'] },
      { label: 'Last Audit Date', type: 'date' },
      { label: 'Next Audit Date', type: 'date' },
    ],
  },
  'Project Management': {
    title: 'Project Management',
    fields: [
      { label: 'Project Name', type: 'text' },
      { label: 'Project Manager', type: 'text' },
      { label: 'Start Date', type: 'date' },
      { label: 'End Date', type: 'date' },
    ],
  },
  'Document Management': {
    title: 'Document Management',
    fields: [
      { label: 'Document Name', type: 'text' },
      { label: 'Upload Date', type: 'date' },
      { label: 'Document Type', type: 'select', options: ['Policy', 'Procedure', 'Form'] },
      { label: 'Access Level', type: 'select', options: ['Public', 'Restricted'] },
    ],
  },
  'Succession Planning': {
    title: 'Succession Planning',
    fields: [
      { label: 'Position', type: 'text' },
      { label: 'Potential Successor', type: 'text' },
      { label: 'Readiness Level', type: 'select', options: ['Ready Now', '1-2 Years', '3-5 Years'] },
      { label: 'Development Needs', type: 'text' },
    ],
  },
  'HR Analytics': {
    title: 'HR Analytics',
    fields: [
      { label: 'Metric Name', type: 'text' },
      { label: 'Value', type: 'number' },
      { label: 'Date', type: 'date' },
      { label: 'Comments', type: 'text' },
    ],
  },
  'Employee Engagement': {
    title: 'Employee Engagement',
    fields: [
      { label: 'Engagement Initiative', type: 'text' },
      { label: 'Employee ID', type: 'text' },
      { label: 'Date of Engagement', type: 'date' },
      { label: 'Feedback', type: 'textarea' },
    ],
  },
};
