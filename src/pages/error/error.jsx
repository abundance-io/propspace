import { useNavigate, useRouteError } from "react-router-dom";

export const Error = () => {
  const error = useRouteError();
  const navigate = useNavigate();

  return (
    <main className="flex h-screen flex-col items-center justify-center gap-20">
      <section className="flex flex-row items-center justify-center gap-10 pt-[0px]">
        <h1 className="text-9xl font-bold">{error.status} </h1>
        <span className="text-mustard text-9xl ">|</span>
        <p className="text-2xl">
          {error.statusText} {error.data}
        </p>
      </section>

      <section className="pt-[0px]">
        <button
          onClick={() => navigate(-1)}
          className="hover:text-mustard text-xl hover:underline"
        >
          Go Back
        </button>
      </section>
    </main>
  );
};
