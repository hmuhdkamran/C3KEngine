import { ChangeEvent, FC, useState } from "react";

// Generic interface with a type T
interface UserEditModuleProps<T> {
  card: T;
  onClose: () => void;
}

const UserEditModule: FC<UserEditModuleProps<{ [key: string]: any }>> = ({
  card,
  onClose,
}) => {
  const [isEditing, setIsEditing] = useState(false);
  const [editedCard, setEditedCard] = useState(card);

  const handleEditToggle = () => {
    setIsEditing(!isEditing);
  };

  const handleChange = (
    e: ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = e.target;
    setEditedCard((prevCard) => ({
      ...prevCard,
      [name]: value,
    }));
  };

  const handleSave = () => {
    console.log("Saved data", editedCard);
    setIsEditing(false);
  };

  return (
    <div className="border rounded-lg shadow-md p-4 bg-white dark:bg-gray-800 w-full lg:w-1/3 mx-auto">
      <div className="flex justify-between items-center">
        <div className="flex items-center">
          <span
            className={`${editedCard.iconClass} text-4xl text-indigo-500 mr-3`}
          ></span>
          {isEditing ? (
            <input
              type="text"
              name="title"
              value={editedCard.title}
              onChange={handleChange}
              className="border border-gray-300 rounded-md px-2 py-1 w-full"
            />
          ) : (
            <h3 className="text-xl font-semibold">{editedCard.title}</h3>
          )}
        </div>

        {/* Toggle between Edit and Close */}
        <button
          onClick={isEditing ? handleEditToggle : onClose}
          className="text-blue-500 hover:text-blue-700 focus:outline-none"
        >
          {isEditing ? "Cancel" : "Close"}
        </button>
      </div>

      <div className="mt-3">
        {isEditing ? (
          <textarea
            name="description"
            value={editedCard.description}
            onChange={handleChange}
            className="w-full border border-gray-300 rounded-md px-2 py-1"
          />
        ) : (
          <p>{editedCard.description}</p>
        )}
      </div>

      <div className="mt-3 flex justify-between">
        {isEditing ? (
          <select
            name="status"
            value={editedCard.status}
            onChange={handleChange}
            className="border border-gray-300 rounded-md px-2 py-1"
          >
            <option value="Installed">Installed</option>
            <option value="Not Installed">Not Installed</option>
            <option value="Pending">Pending</option>
          </select>
        ) : (
          <span
            className={`text-sm font-medium px-2 py-1 rounded ${
              editedCard.status === "Installed"
                ? "bg-green-100 text-green-800"
                : editedCard.status === "Pending"
                ? "bg-yellow-100 text-yellow-800"
                : "bg-red-100 text-red-800"
            }`}
          >
            {editedCard.status}
          </span>
        )}
        {isEditing ? (
          <button
            onClick={handleSave}
            className="px-4 py-2 bg-green-500 text-white rounded-md hover:bg-green-600"
          >
            Save
          </button>
        ) : (
          <button className="text-indigo-500 hover:text-indigo-600 focus:outline-none">
            {editedCard.buttonText}
          </button>
        )}
      </div>
    </div>
  );
};

export default UserEditModule;
