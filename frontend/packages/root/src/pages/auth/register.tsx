import { FC, useState } from "react";
import { Link } from "react-router-dom";
import { config } from "@/plugins/config";
import VNodeRenderer from "@/components/extra/node-renderer";

const RegisterForm: FC = () => {
  const [name, setName] = useState<string>("");
  const [email, setEmail] = useState<string>("");
  const [password, setPassword] = useState<string>("");

  return (
    <div
      className="min-h-screen py-6 flex flex-col justify-center sm:py-12 relative h-full md:flex items-center p-10 
            overflow-hidden bg-violet-900 text-white bg-no-repeat bg-cover"
    >
      <div className="absolute bg-gradient-to-b from-violet-500 to-purple-300 opacity-75 inset-0 z-0"></div>
      <ul className="circles">
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
        <li></li>
      </ul>
      <div className="relative py-3 sm:max-w-xl sm:mx-auto w-full">
        <div className="absolute inset-0 bg-gradient-to-r from-violet-900 to-purple-900 shadow-lg transform -skew-y-6 sm:skew-y-0 sm:-rotate-6 sm:rounded-3xl transition-transform hover:scale-105 duration-500"></div>
        <div className="relative bg-white shadow-lg sm:rounded-3xl sm:p-5 transition-transform hover:scale-105 duration-700">
          <div className="flex flex-col bg-white">
            <div className="flex justify-center md:justify-start md:pl-6 md:-mb-12">
              <a href="/" className="text-white font-bold text-xl p-2">
                <VNodeRenderer nodes={config.logo} />
              </a>
            </div>
            <div className="flex flex-col justify-center md:justify-start px-2 my-auto md:pt-0 md:px-12">
              <p className="text-center text-2xl text-gray-800">Join Us.</p>
              <div className="flex flex-col pt-3 md:pt-8">
                <div className="flex flex-col pt-4">
                  <label htmlFor="name" className="text-lg text-gray-800">
                    Name
                  </label>
                  <input
                    type="text"
                    id="name"
                    placeholder="John Smith"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    className="input-bottom"
                  />
                </div>

                <div className="flex flex-col pt-4">
                  <label htmlFor="email" className="text-lg text-gray-800">
                    Email
                  </label>
                  <input
                    type="email"
                    id="email"
                    placeholder="your@email.com"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    className="input-bottom"
                  />
                </div>

                <div className="flex flex-col pt-4">
                  <label htmlFor="password" className="text-lg text-gray-800">
                    Password
                  </label>
                  <input
                    type="password"
                    id="password"
                    placeholder="Password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    className="input-bottom"
                  />
                </div>

                <div className="flex flex-col pt-4">
                  <label
                    htmlFor="confirm-password"
                    className="text-lg text-gray-800"
                  >
                    Confirm Password
                  </label>
                  <input
                    type="password"
                    id="confirm-password"
                    placeholder="Password"
                    className="input-bottom"
                  />
                </div>

                <input
                  type="submit"
                  value="Register"
                  className="btn-gradient mt-6"
                />
              </div>
              <div className="text-center pt-12 pb-12">
                <p className="text-gray-800">
                  Already have an account?
                  <Link
                    to="/auth/login"
                    className="underline font-semibold text-violet-700"
                  >
                    Login here.
                  </Link>
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default RegisterForm;
