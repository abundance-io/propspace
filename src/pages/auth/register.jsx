import bg from "../../assets/images/register.png";
import logo from "../../assets/images/longLogo.svg";
import { Button } from "@/components/ui/button";
import { Link } from "react-router-dom";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";

export const Register = () => {
  return (
    <section className="flex h-screen flex-col items-center lg:flex-row">
      <div
        className="hidden h-full w-1/2 pt-[0px] lg:block"
        style={{
          backgroundImage: `url(${bg})`,
          backgroundSize: "cover",
          backgroundPosition: "center",
          backgroundRepeat: "no-repeat",
        }}
      ></div>
      <div className="flex h-full flex-col items-center gap-16 p-10 lg:w-1/2 lg:justify-center lg:px-[10%]">
        <img src={logo} className="mb-[30px] w-[50%]" />
        <div className="w-full">
          <h1 className="text-2xl font-semibold lg:text-4xl">
            Hey, Welcome! 👋
          </h1>
          <p className="text-muted-foreground">
            Enter your details to create an account.
          </p>
        </div>
        <div className="flex w-full flex-col gap-8">
          <div className="w-full space-y-2">
            <Label htmlFor="email">Email</Label>
            <Input
              type="email"
              id="email"
              placeholder="Email"
              className="bg-prop-black ring-offset-prop-mustard focus:border-prop-mustard"
            />
          </div>
          <div className="w-full space-y-2">
            <Label htmlFor="password">Password</Label>
            <Input
              type="password"
              id="password"
              placeholder="Password"
              className="bg-prop-black ring-offset-prop-mustard focus:border-prop-mustard"
            />
          </div>
          <div className="w-full space-y-2">
            <Label htmlFor="password">Confirm Password</Label>
            <Input
              type="password"
              id="password"
              placeholder="Password"
              className="bg-prop-black ring-offset-prop-mustard focus:border-prop-mustard"
            />
          </div>
          <div className="flex w-full">
            <Button variant="default" size="full">
              Register
            </Button>
          </div>
          <span className="flex w-full justify-center">
            <Link
              to="/"
              className="text-muted-foreground hover:text-prop-mustard text-sm"
            >
              Back to Home
            </Link>
          </span>
        </div>
      </div>
    </section>
  );
};
