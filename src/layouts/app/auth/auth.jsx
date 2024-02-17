import { Outlet } from "react-router-dom";

export const AuthLayout = () => {
  return (
    <main className="bg-prop-black text-prop-white">
      <Outlet />
    </main>
  );
};
