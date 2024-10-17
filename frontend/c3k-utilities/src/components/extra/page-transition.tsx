import { FC, ReactNode, useEffect, useState } from "react";
import { useLocation } from "react-router-dom";

interface PageTransitionProps {
  children: ReactNode;
}

const PageTransition: FC<PageTransitionProps> = ({ children }) => {
  const location = useLocation();
  const [isVisible, setIsVisible] = useState(true); // To control visibility of the component
  const [isTransitioning, setIsTransitioning] = useState(false); // To control transition state

  useEffect(() => {
    setIsTransitioning(true); // Start the transition
    setIsVisible(false); // Hide the component during transition

    const timeout = setTimeout(() => {
      setIsVisible(true); // Show the component after transition
      setIsTransitioning(false); // End the transition
    }, 500); // Match the duration of your CSS transition (fade and slide)

    return () => clearTimeout(timeout);
  }, [location.pathname]);

  return (
    <div
      className={`absolute top-0 w-screen transition-all duration-500 ease-in-out transform ${
        isTransitioning
          ? "opacity-0 translate-x-[20px] animate-fade-out"
          : "opacity-100 translate-x-0 animate-fade-in"
      }`}
      style={{ margin: 0 }} // Ensure no margin
    >
      {isVisible && children} {/* Render children only if visible */}
    </div>
  );
};

export default PageTransition;
