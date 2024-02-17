import logo from "../../assets/images/longLogo.svg";
import arrow from "../../assets/icons/up.svg";
import grid from "../../assets/images/grid.png";
import logoImg from "../../assets/images/footer.svg";
import { navLinks } from "./nav";
import * as Icon from "react-icons/fa6";
import { Button } from "@/components/ui/button";
import { Link } from "react-router-dom";

export const Footer = () => {
  const handleScrollToTop = () => {
    window.scrollTo({
      top: 0,
      right: 0,
      behaviour: "smooth",
    });
  };

  return (
    <footer
      className="font-body mt-[200px] flex w-full flex-col items-center justify-between
      gap-10 overflow-hidden px-10 pb-10 pt-20 lg:h-screen"
    >
      <section className="flex h-[80%] w-full flex-col items-center gap-6 pt-[0px]">
        <div
          style={{
            backgroundImage: `url(${grid})`,
            backgroundSize: "cover",
            backgroundPosition: "center",
          }}
          className="border-prop-white flex h-1/2 w-full items-center justify-center gap-6 rounded-[20px] border-[1px] p-10 lg:rounded-[40px]"
        >
          <h2 className="text-xl font-bold lg:text-4xl">
            Become a space <br /> owner today !
          </h2>
          <Button asChild>
            <Link to="register">Join us</Link>
          </Button>
        </div>

        <div className="flex h-1/2 w-full flex-col gap-6 lg:flex-row">
          <div className="bg-prop-mustard text-prop-black flex h-full flex-col items-center justify-center gap-3 rounded-[20px] p-10 lg:w-1/2 lg:rounded-[40px]">
            <h1 className="mb-4 text-6xl font-bold lg:text-8xl">Links</h1>
            <ul className="flex flex-wrap gap-3 lg:gap-10">
              {navLinks.map((link) => (
                <li key={link.name}>
                  <a
                    href={link.link}
                    className="text-sm hover:underline lg:text-base"
                  >
                    {link.name}
                  </a>
                </li>
              ))}
            </ul>
          </div>

          <div className="bg-prop-white text-prop-black relative flex h-full flex-col gap-10 rounded-[20px] p-10 lg:w-2/3 lg:rounded-[40px]">
            <p className="text-lg lg:w-1/2 lg:text-2xl">
              KM. 10 Idiroko Road, Canaan Land, Ota, Ogun State, Nigeria.
            </p>
            <span className="flex items-center gap-2">
              <Icon.FaSquareXTwitter className="h-[30px] w-[30px]" />
              <Icon.FaSquareInstagram className="h-[30px] w-[30px]" />
              <Icon.FaLinkedin className="h-[30px] w-[30px]" />
            </span>
            <img src={logoImg} className="absolute right-0 top-0 h-full" />
          </div>
        </div>
      </section>

      <section className="text-prop-white grid h-[20%] w-full grid-cols-2 items-end pt-[0px] font-thin opacity-75 lg:grid-cols-3">
        <p className="hidden lg:block">
          © 2024 Propspace. All rights reserved.
        </p>
        <img
          src={logo}
          className="w-[70%] justify-self-start lg:w-1/2 lg:justify-self-center"
        />
        <button
          onClick={handleScrollToTop}
          className="rotate-180 justify-self-end"
        >
          <img src={arrow} width={"40px"} />
        </button>
      </section>
    </footer>
  );
};
