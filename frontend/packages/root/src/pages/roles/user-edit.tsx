import { FC, useState, useEffect } from "react";

type UserEditModuleProps = {
  card: any;
  onClose: () => void;
  mode: "view" | "edit" | "add";
};

const UserEditModule: FC<UserEditModuleProps> = ({ card, onClose, mode }) => {
  const [openSection, setOpenSection] = useState(false);
  const [formValues, setFormValues] = useState({
    title: card ? card.title : "",
    description: card ? card.description : "",
    status: card ? card.status : "Activate",
  });
  
  const [isOpen, setIsOpen] = useState(false);
  const isViewMode = mode === "view";
  const isAddMode = mode === "add";

  useEffect(() => {
    setIsOpen(true);
  }, []);

  const toggleDropdown = () => {
    setOpenSection(!openSection);
  };

  const handleSave = () => {
    console.log("Data saved:", formValues);
    handleClose();
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormValues({ ...formValues, [name]: value });
  };

  const handleClose = () => {
    setIsOpen(false);
    setTimeout(onClose, 300);
  };

  return (
    <div className="fixed inset-0 z-50 flex justify-end">
      <div className={`fixed inset-0 bg-black transition-opacity duration-300 pointer-events-none ${
          isOpen ? "opacity-50" : "opacity-0"
        }`}
      ></div>
      <div className={`relative bg-white w-[600px] min-h-screen flex flex-col shadow-lg transform transition-transform duration-300 ${
          isOpen ? "translate-x-0" : "translate-x-full"
        }`}
      >
        <div className="flex justify-between items-center p-4 border-b">
          <h2 className="text-lg font-semibold">
            {isViewMode ? "View" : isAddMode ? "Add" : "Edit"}
          </h2>
          <button onClick={handleClose}>
            <span className="icon-[fluent--dismiss-20-filled] h-4 w-4"></span>
          </button>
        </div>
        <div className="p-4 flex-grow">
          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">Title</label>
            <input
              name="title"
              type="text"
              value={formValues.title}
              onChange={handleInputChange}
              className="w-full px-2 py-1 input-complete"
              disabled={isViewMode}
            />
          </div>
          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">Description</label>
            <textarea
              name="description"
              value={formValues.description}
              onChange={handleInputChange}
              className="w-full px-2 py-1 input-complete"
              disabled={isViewMode}
            />
          </div>
          <div className="mb-4 relative">
            <label className="block text-sm font-medium mb-2">Status</label>
            <div className="relative flex items-center">
              <select
                name="status"
                value={formValues.status}
                onChange={handleInputChange}
                className="w-full px-2 py-1 input-complete appearance-none"
                disabled={isViewMode}
                onClick={toggleDropdown}
              >
                <option value="Activate">Activate</option>
                <option value="Installed">Installed</option>
              </select>
              <span className="absolute inset-y-0 right-2 flex items-center pointer-events-none">
                <span
                  className={
                    openSection
                      ? "icon-[mdi--chevron-up] text-gray-600"
                      : "icon-[mdi--chevron-down] text-gray-500"
                  }
                ></span>
              </span>
            </div>
          </div>
        </div>
        {!isViewMode && (
          <div className="flex justify-end items-center p-4 border-t space-x-2">
            <button onClick={handleClose} className="px-3 py-1 btn-secondary">
              Cancel
            </button>
            <button onClick={handleSave} className="px-3 py-1 btn-primary">
              {isAddMode ? "Save" : "Save"}
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default UserEditModule;
