# Frontend

## Usage of DataTable
```ts
const columns = [
  { key: "check", label: "check", sort: false, check: true },
  { key: "title", label: "Title", sort: true },
  {
    key: "action",
    label: "Action",
    sort: false,
    width: "100px",
    class: "text-center",
    render: (record: unknown) => (
      <div className="flex justify-center space-x-2">
        <button
          className="bg-blue-500 text-white px-2 py-1 rounded-md"
          onClick={() => handleEdit(record)}
        >
          Edit
        </button>
        <button
          className="bg-red-500 text-white px-2 py-1 rounded-md"
          onClick={() => handleDelete(record)}
        >
          Delete
        </button>
      </div>
    ),
  },
];

const handleEdit = (record: any) => {
  console.log("Editing:", JSON.stringify(record));
};

const handleDelete = (record: any) => {
  console.log("Deleting:", JSON.stringify(record));
};

const [_selectedRecords, setSelectedRecords] = useState<Record<string, unknown>[]>([]);

const handleSelectionChange = (selected: Record<string, unknown>[]) => {
  setSelectedRecords(selected);
};
```

```html
<DataTable data={hrmsCards} columns={columns} onSelectionChange={handleSelectionChange} />
```

