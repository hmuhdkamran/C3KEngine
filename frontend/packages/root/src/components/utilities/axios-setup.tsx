import { useNavigate } from "react-router-dom";
import { useEffect } from "react";
import { useAxios } from "c3k-utilities";

const AxiosSetupComponent: React.FC = () => {
  const navigate = useNavigate();

  useEffect(() => {
    useAxios(navigate);
  }, [navigate]);

  return null;
};

export default AxiosSetupComponent;
