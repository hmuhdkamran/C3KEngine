import { FC } from "react";
import { config } from "@/plugins/config";

const Footer: FC = () => {
  const currentYear = new Date().getFullYear();

  return (
    <>
      <footer className="fixed text-xs w-auto justify-end bottom-0 right-0 bg-gray-100 py-2 px-4 flex items-center">
        <p className="text-gray-500">
          Copyright © {currentYear} {config.application} by
          <a
            href="https://combinesoft.net"
            className="text-gray-500 hover:text-gray-800"
            target="_blank"
          >
            {config.company}
          </a>
        </p>
      </footer>
    </>
  );
};

export default Footer;
