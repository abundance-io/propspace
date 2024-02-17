import { Outlet } from "react-router-dom";
import { Nav } from "../../components/global/nav";
import { Footer } from "../../components/global/footer";

export const WebsiteLayout = () => {
  return (
    <main className="bg-prop-black text-prop-white">
      <Nav />
      <Outlet />
      <Footer />
    </main>
  );
};
