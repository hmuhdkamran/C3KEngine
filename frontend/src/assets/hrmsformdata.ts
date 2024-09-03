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
    ],
  },
  'Payroll': {
    title: 'Payroll',
    fields: [
      { label: 'Employee ID', type: 'text' },
      { label: 'Salary', type: 'number' },
      { label: 'Tax Rate', type: 'number' },
      { label: 'Payment Date', type: 'date' },
    ],
  },
   'Attendance Tracking':{
        title: 'Attendance Tracking',
        fields: [
          { label: 'Employee ID', type: 'text' },
          { label: 'Check-in Time', type: 'time' },
          { label: 'Check-out Time', type: 'time' },
          { label: 'Date', type: 'date' },
        ]
      },
     'Recruitment':
       {
        title: 'Recruitment',
        fields: [
          { label: 'Candidate Name', type: 'text' },
          { label: 'Position Applied', type: 'text' },
          { label: 'Interview Date', type: 'date' },
          { label: 'Status', type: 'text' },
        ]
      },
     'Benefits Management':
       {
        title: 'Benefits Management',
        fields: [
          { label: 'Employee ID', type: 'text' },
          { label: 'Benefit Type', type: 'select', options: ['Health Insurance', 'Retirement', 'Annual Leave'] },
          { label: 'Coverage Amount', type: 'number' },
          { label: 'Effective Date', type: 'date' },
        ]
      },
     'Training':
       {
        title: 'Training',
        fields: [
          { label: 'Employee ID', type: 'text' },
          { label: 'Training Program', type: 'text' },
          { label: 'Training Date', type: 'date' },
          { label: 'Completion Status', type: 'select', options: ['Completed', 'Pending', 'Failed'] },
        ]
      },
     'Performance Management':
       {
        title: 'Performance Management',
        fields: [
          { label: 'Employee ID', type: 'text' },
          { label: 'Performance Rating', type: 'select', options: ['1 - Poor', '2 - Fair', '3 - Good', '4 - Excellent'] },
          { label: 'Review Date', type: 'date' },
          { label: 'Comments', type: 'text' },
        ]
      },
     'Employee Self-Service':
       {
        title: 'Employee Self-Service',
        fields: [
          { label: 'Employee ID', type: 'text' },
          { label: 'Change Request', type: 'select', options: ['Address Change', 'Contact Information', 'Bank Details'] },
          { label: 'Request Date', type: 'date' },
          { label: 'Approval Status', type: 'select', options: ['Approved', 'Pending', 'Rejected'] },
        ]
      },
     'Time-Off Management':
       {
        title: 'Time-Off Management',
        fields: [
          { label: 'Employee ID', type: 'text' },
          { label: 'Leave Type', type: 'select', options: ['Sick Leave', 'Casual Leave', 'Annual Leave'] },
          { label: 'Start Date', type: 'date' },
          { label: 'End Date', type: 'date' },
        ]
      },
     'Expense Management':
       {
        title: 'Expense Management',
        fields: [
          { label: 'Employee ID', type: 'text' },
          { label: 'Expense Category', type: 'select', options: ['Travel', 'Meals', 'Office Supplies', 'Other'] },
          { label: 'Amount', type: 'number' },
          { label: 'Submission Date', type: 'date' },
        ]
      },
};
