import { useDataContext } from "@/plugins/store";
import { FC, useState, useMemo, useEffect } from "react";

interface Column {
  key: string;
  label: string;
  sort: boolean;
  width?: string;
  class?: string;
  check?: boolean;
}

interface TableProps {
  data: Record<string, unknown>[];
  columns: Column[];
}

const DataTable: FC<TableProps> = ({ data, columns }) => {
  const [sortColumn, setSortColumn] = useState<string>("");
  const [sortOrder, setSortOrder] = useState<"asc" | "desc">("asc");
  const [selectedRecords, setSelectedRecords] = useState<
    Record<string, unknown>[]
  >([]);
  const [selectAll, setSelectAll] = useState<boolean>(false);

  const { currentPage, itemsPerPage, searchQuery, updateTotalRecords } =
    useDataContext();

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
          ? a[sortColumn]?.toString().toLowerCase()
          : "";
      const compareB =
        typeof b[sortColumn] === "string"
          ? b[sortColumn]?.toString().toLowerCase()
          : "";
      if (compareA < compareB) return sortOrder === "asc" ? -1 : 1;
      if (compareA > compareB) return sortOrder === "asc" ? 1 : -1;
      return 0;
    });

    return sorted;
  }, [data, searchQuery, sortColumn, sortOrder]);

  useEffect(() => {
    updateTotalRecords(filteredRecords.length);
  }, [filteredRecords, updateTotalRecords]);

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
    if (!selectAll) {
      setSelectedRecords([...paginatedRecords]);
    } else {
      setSelectedRecords([]);
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

  return (
    <div className="h-screen w-screen">
      <div className="overflow-x-auto shadow-md bg-white rounded-sm">
        <table className="min-w-full bg-white border border-gray-200">
          <thead>
            <tr className="bg-gray-200 border-b border-gray-300">
              {columns.map((column) => (
                <th
                  key={column.key}
                  className="p-2 text-left text-gray-600 cursor-pointer hover:bg-gray-300 transition-colors text-md font-medium"
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
            {paginatedRecords.map(
              (
                record,
                index
              ) => (
                <tr
                  key={(record[columns[0].key] as string) || index}
                  className="border-b border-dashed border-gray-300 transition-all hover:shadow-md text-md"
                >
                  {columns.map((column) => (
                    <td
                      key={column.key}
                      className={`p-1 ${column.class || ""}`}
                    >
                      {column.check ? (
                        <input
                          type="checkbox"
                          className="cursor-pointer"
                          checked={selectedRecords.includes(record)}
                          onChange={() => handleSelectRecord(record)}
                        />
                      ) : (
                        <span
                          dangerouslySetInnerHTML={{
                            __html: record[column.key] as string,
                          }}
                        />
                      )}
                    </td>
                  ))}
                </tr>
              )
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default DataTable;
