import { FC, useState, useMemo, useEffect, ReactNode } from "react";
import { RootState } from "../../plugins/store";
import { useDispatch, useSelector } from "react-redux";
import { updateTotalRecords } from '../../plugins/store/dataSlice';

interface Column {
  key: string;
  label: string;
  sort: boolean;
  width?: string;
  class?: string;
  check?: boolean;
  render?: (record: Record<string, unknown>) => ReactNode;
}

interface TableProps {
  data: Record<string, unknown>[];
  columns: Column[];
  onSelectionChange?: (selected: Record<string, unknown>[]) => void;
}

const DataTable: FC<TableProps> = ({ data, columns, onSelectionChange }) => {
  const dispatch = useDispatch();
  const { currentPage, itemsPerPage, searchQuery } = useSelector((state: RootState) => state.data);
  
  const [sortColumn, setSortColumn] = useState<string>("");
  const [sortOrder, setSortOrder] = useState<"asc" | "desc">("asc");
  const [selectedRecords, setSelectedRecords] = useState<
    Record<string, unknown>[]
  >([]);
  const [selectAll, setSelectAll] = useState<boolean>(false);

  const filteredRecords = useMemo(() => {
    if (!data || data.length === 0) {
      return [];
    }

    const query = searchQuery.toLowerCase();

    const filtered = data.filter((record) =>
      Object.values(record).some((value) => {
        if (typeof value === "string" || typeof value === "number") {
          return value.toString().toLowerCase().includes(query);
        }
        return false;
      })
    );

    const sorted = filtered.sort((a, b) => {
      const compareA =
        typeof a[sortColumn] === "string"
          ? (a[sortColumn] as string).toLowerCase()
          : typeof a[sortColumn] === "number"
          ? (a[sortColumn] as number).toString().toLowerCase()
          : ""; // Default to empty string if not string or number
    
      const compareB =
        typeof b[sortColumn] === "string"
          ? (b[sortColumn] as string).toLowerCase()
          : typeof b[sortColumn] === "number"
          ? (b[sortColumn] as number).toString().toLowerCase()
          : ""; // Default to empty string if not string or number
    
      if (compareA < compareB) return sortOrder === "asc" ? -1 : 1;
      if (compareA > compareB) return sortOrder === "asc" ? 1 : -1;
      return 0;
    });

    return sorted;
  }, [data, searchQuery, sortColumn, sortOrder]);

  useEffect(() => {
    dispatch(updateTotalRecords(filteredRecords.length));
  }, [filteredRecords.length, dispatch]);

  const paginatedRecords = useMemo(() => {
    const start = (currentPage - 1) * itemsPerPage;
    const end = start + itemsPerPage;
    return filteredRecords.slice(start, end);
  }, [filteredRecords, currentPage, itemsPerPage]);

  const changeSort = (column: string, sort: boolean = true) => {
    if (!sort) return;
    if (sortColumn === column) {
      setSortOrder(sortOrder === "asc" ? "desc" : "asc");
    } else {
      setSortColumn(column);
      setSortOrder("asc");
    }
  };

  const toggleSelectAll = () => {
    setSelectAll(!selectAll);
    const newSelectedRecords = !selectAll ? [...paginatedRecords] : [];
    setSelectedRecords(newSelectedRecords);

    if (onSelectionChange) {
      onSelectionChange(newSelectedRecords);
    }
  };

  const handleSelectRecord = (record: Record<string, unknown>) => {
    setSelectedRecords((prevSelected) => {
      if (prevSelected.includes(record)) {
        return prevSelected.filter((r) => r !== record);
      } else {
        return [...prevSelected, record];
      }
    });
  };

  const renderColumnContent = (
    record: Record<string, unknown>,
    column: Column
  ): ReactNode => {
    if (column.render) {
      return column.render(record);
    }
    return (record[column.key] as ReactNode) || null;
  };

  return (
    <table>
      <thead>
        <tr className="bg-gray-200 border-b border-gray-300">
          {columns.map((column) => (
            <th
              key={column.key}
              onClick={() => changeSort(column.key, column.sort)}
            >
              {column.check ? (
                <input
                  type="checkbox"
                  className="cursor-pointer"
                  checked={selectAll}
                  onChange={toggleSelectAll}
                />
              ) : (
                <>
                  {column.label}
                  {sortColumn === column.key && column.sort && (
                    <span className="ml-1 text-md">
                      {sortOrder === "asc" ? "↑" : "↓"}
                    </span>
                  )}
                </>
              )}
            </th>
          ))}
        </tr>
      </thead>

      <tbody>
        {paginatedRecords.map((record, index) => (
          <tr
            key={(record[columns[0].key] as string) || index}
          >
            {columns.map((column) => (
              <td key={column.key} className={`p-1 ${column.class || ""}`}>
                {column.check ? (
                  <div className="flex items-center justify-center h-full">
                    <input
                      type="checkbox"
                      className="cursor-pointer"
                      checked={selectedRecords.includes(record)}
                      onChange={() => handleSelectRecord(record)}
                    />
                  </div>
                ) : (
                  renderColumnContent(record, column)
                )}
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default DataTable;
